//! The loaded program the executor interprets: every expression compiled to
//! a typed tree with its terminals as byte matchers, every operation checked
//! for its context, every rule call resolved to a rule index or an external
//! token. `load.rs` builds it; `executor.rs` runs it.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::forking::GrammarFacts;
use super::operations::{Condition, Statement};
use super::text::{decode_at, fold_case, quote_text};
use crate::grammar::{ByteClassItem, PrecedenceEntry, RuleKind};

/// A shared name: a rule, a node kind, a field, a mode or a language.
pub(super) type Name = Arc<str>;

/// One test of a character class.
#[derive(Debug)]
pub(super) enum ClassTest {
    Char(u32),
    Range(u32, u32),
    Property(regex::Regex),
}

impl ClassTest {
    fn test(&self, code_point: u32) -> bool {
        match self {
            Self::Char(value) => code_point == *value,
            Self::Range(low, high) => (*low..=*high).contains(&code_point),
            Self::Property(pattern) => char::from_u32(code_point)
                .is_some_and(|c| pattern.is_match(c.encode_utf8(&mut [0; 4]))),
        }
    }
}

/// A terminal compiled to a byte matcher.
#[derive(Debug)]
pub(super) enum Matcher {
    Literal(Vec<u8>),
    Insensitive {
        folded: String,
        count: usize,
    },
    CharRange(u32, u32),
    CharClass {
        negated: bool,
        tests: Vec<ClassTest>,
    },
    ByteClass {
        negated: bool,
        items: Vec<ByteClassItem>,
    },
    Any,
}

impl Matcher {
    /// The end of the match at `position`, or `None`.
    pub(super) fn matches(&self, bytes: &[u8], position: usize, end: usize) -> Option<usize> {
        match self {
            Self::Literal(literal) => (position + literal.len() <= end
                && bytes[position..position + literal.len()] == literal[..])
                .then_some(position + literal.len()),
            Self::Insensitive { folded, count } => {
                let mut cursor = position;
                let mut text = String::new();
                for _ in 0..*count {
                    if cursor >= end {
                        return None;
                    }
                    let (code_point, length) = decode_at(bytes, cursor, end);
                    text.push(char::from_u32(code_point?)?);
                    cursor += length;
                }
                (fold_case(&text) == *folded).then_some(cursor)
            }
            Self::CharRange(low, high) => {
                code_point_match(bytes, position, end, |c| (*low..=*high).contains(&c))
            }
            Self::CharClass { negated, tests } => code_point_match(bytes, position, end, |c| {
                tests.iter().any(|test| test.test(c)) != *negated
            }),
            Self::ByteClass { negated, items } => {
                if position >= end {
                    return None;
                }
                let byte = bytes[position];
                let contains = items.iter().any(|item| match item {
                    ByteClassItem::Byte(value) => byte == *value,
                    ByteClassItem::Range(low, high) => (*low..=*high).contains(&byte),
                });
                (contains != *negated).then_some(position + 1)
            }
            Self::Any => (position < end).then(|| position + decode_at(bytes, position, end).1),
        }
    }
}

fn code_point_match(
    bytes: &[u8],
    position: usize,
    end: usize,
    test: impl Fn(u32) -> bool,
) -> Option<usize> {
    if position >= end {
        return None;
    }
    let (code_point, length) = decode_at(bytes, position, end);
    code_point.filter(|c| test(*c)).map(|_| position + length)
}

/// The expectation a failed literal records.
pub(super) fn literal_expectation(value: &str) -> Name {
    Arc::from(quote_text(value))
}

/// The expectation a failed case-insensitive literal records.
pub(super) fn insensitive_expectation(value: &str) -> Name {
    Arc::from(format!("{}i", quote_text(value)))
}

/// The expectation a failed character range records.
pub(super) fn range_expectation(start: char, end: char) -> Name {
    Arc::from(format!(
        "{}..{}",
        quote_text(&start.to_string()),
        quote_text(&end.to_string())
    ))
}

/// The associativity of a precedence level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Associativity {
    Left,
    Right,
    None,
}

/// The precedence a result or a node is built under: its level, its name (a
/// named precedence ranks by the grammar's precedence orders), its
/// associativity and the rule it ranks for (the rule whose body holds it, or
/// for no precedence the node's own rule). See `compare_precedence`.
#[derive(Clone, Debug)]
pub(super) struct PrecedenceTag {
    pub(super) level: i64,
    pub(super) name: Option<Name>,
    pub(super) associativity: Associativity,
    pub(super) rule: Option<Name>,
}

impl PrecedenceTag {
    /// The precedence of a production reduced or shifted under none: level
    /// 0, ranked only by its rule's entries in the precedence orders.
    pub(super) const fn unranked(rule: Option<Name>) -> Self {
        Self {
            level: 0,
            name: None,
            associativity: Associativity::None,
            rule,
        }
    }

    /// Whether two precedences are the same, whichever rule they rank for.
    pub(super) fn same(a: Option<&Self>, b: Option<&Self>) -> bool {
        match (a, b) {
            (Some(a), Some(b)) => {
                a.level == b.level && a.name == b.name && a.associativity == b.associativity
            }
            (a, b) => a.is_none() && b.is_none(),
        }
    }
}

/// How precedence `a` compares with `b`, as tree-sitter's
/// `compare_precedence`: two levels compare when either is nonzero and
/// neither precedence is named; otherwise the first of the `orders` with an
/// entry for each decides, the earlier entry higher. A `name` entry stands
/// for a named precedence, a `rule` entry for any precedence of the rule's
/// productions. Equal when neither is higher. It mirrors comparePrecedence in
/// js/src/grammar-runtime/executor.js.
pub(super) fn compare_precedence(
    a: &PrecedenceTag,
    b: &PrecedenceTag,
    orders: &[Vec<PrecedenceEntry>],
) -> std::cmp::Ordering {
    if a.name.is_none() && b.name.is_none() && (a.level != 0 || b.level != 0) {
        return a.level.cmp(&b.level);
    }
    let matches = |entry: &PrecedenceEntry, tag: &PrecedenceTag| match entry {
        PrecedenceEntry::Name(name) => tag.name.as_deref() == Some(name.as_str()),
        PrecedenceEntry::Rule(rule) => tag.rule.as_deref() == Some(rule.as_str()),
    };
    for order in orders {
        let (mut left, mut right) = (None, None);
        for (position, entry) in order.iter().enumerate() {
            if matches(entry, a) {
                left = Some(position);
            }
            if matches(entry, b) {
                right = Some(position);
            }
            if let (Some(left), Some(right)) = (left, right) {
                return right.cmp(&left);
            }
        }
    }
    std::cmp::Ordering::Equal
}

/// The target of a rule call.
#[derive(Clone, Debug)]
pub(super) enum Target {
    Rule(usize),
    External(Name),
}

/// One compiled expression.
#[derive(Debug)]
pub(super) enum Expr {
    Empty,
    Terminal {
        matcher: Matcher,
        expectation: Name,
        /// The id of the literal where a scanner's `expected` asks about it,
        /// which the parse requests where it tries the literal.
        expected: Option<usize>,
    },
    Ref(Target),
    Seq(Vec<Self>),
    Choice {
        ordered: bool,
        items: Vec<Self>,
    },
    Repeat {
        item: Box<Self>,
        min: usize,
        max: Option<usize>,
    },
    And(Box<Self>),
    Not(Box<Self>),
    Capture {
        label: Option<Name>,
        item: Box<Self>,
    },
    Alias {
        name: Name,
        item: Box<Self>,
    },
    Precedence {
        level: i64,
        name: Option<Name>,
        associativity: Associativity,
        item: Box<Self>,
    },
    DynamicPrecedence {
        level: i64,
        item: Box<Self>,
    },
    LexicalPrecedence {
        level: i64,
        item: Box<Self>,
    },
    Longest(Vec<Self>),
    Token(Box<Self>),
    ImmediateToken(Box<Self>),
    Predicate {
        item: Box<Self>,
        condition: Box<Condition>,
    },
    Recover {
        item: Box<Self>,
        synchronize: Box<Self>,
    },
    Missing {
        item: Box<Self>,
        kind: Option<Name>,
        literal: bool,
    },
    Embed {
        language: Name,
        item: Box<Self>,
    },
}

/// One loaded rule; an instance of a parameterized rule keeps its kind.
#[derive(Debug)]
pub(super) struct Rule {
    /// The rule's name, an instance's own.
    pub(super) name: Name,
    pub(super) node_kind: Name,
    pub(super) kind: RuleKind,
    pub(super) expression: Expr,
    pub(super) action: Option<Vec<Statement>>,
    pub(super) modes: Option<Vec<Name>>,
    pub(super) lexical_priority: i64,
}

/// The rank of a token of a `(matching longest)` grammar: its lexical
/// precedence, its specificity (2 for a literal, 0 for a pattern, one more
/// for an immediate token) and its order in the grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct TokenRank {
    pub(super) priority: i64,
    pub(super) specificity: u8,
    pub(super) order: usize,
}

/// The ranks of the tokens of a `(matching longest)` grammar, keyed by leaf
/// kind and, for a literal leaf, which has none, by text.
#[derive(Debug, Default)]
pub(super) struct TokenRanks {
    pub(super) kinds: HashMap<Name, TokenRank>,
    pub(super) literals: HashMap<Vec<u8>, TokenRank>,
    /// The texts of the literals the grammar takes as an immediate token
    /// (`(immediateToken (literal [))`), which a lexer prefers to the plain
    /// literal of the same text where both are valid (see `KeywordLexing`).
    pub(super) immediate: HashSet<Vec<u8>>,
}

/// One external scanner.
#[derive(Debug)]
pub(super) struct Scanner {
    pub(super) operations: Vec<Statement>,
    /// Whether the scanner asks what the parse expects (`expected`), so that
    /// it answers per context offset.
    pub(super) consults: bool,
}

/// One trivia expression: an `extra` or a rule on a channel other than `default`.
#[derive(Debug)]
pub(super) struct Trivia {
    pub(super) expression: Expr,
    pub(super) kind: Option<Name>,
    pub(super) modes: Option<Vec<Name>>,
}

/// One loaded grammar.
#[derive(Debug)]
pub(super) struct Program {
    pub(super) peg: bool,
    /// `(matching longest)`: the token ranks by which a tie between results
    /// goes to the tokens a lexer prefers.
    pub(super) token_ranks: Option<TokenRanks>,
    pub(super) start: Option<String>,
    pub(super) rules: Vec<Rule>,
    pub(super) rule_index: HashMap<String, usize>,
    pub(super) external: HashMap<String, usize>,
    pub(super) scanners: Vec<Scanner>,
    pub(super) conflicts: HashSet<String>,
    /// The orders of named precedences (see `compare_precedence`).
    pub(super) precedence_orders: Vec<Vec<PrecedenceEntry>>,
    /// The silent rules a `rule` entry of the orders names, whose reduction
    /// of one item alone the item records (see `child_parting`).
    pub(super) ranked_silent: HashSet<String>,
    pub(super) trivia: Vec<Trivia>,
    /// The rules and external tokens a scanner's `expected` asks about, each
    /// by the id of its item, which the parse requests where it calls them
    /// (see `Expectations` in executor.rs).
    pub(super) expected_references: HashMap<String, usize>,
    /// What the conflicts of two results ask of the rules (see
    /// `GrammarFacts`).
    pub(super) grammar: GrammarFacts,
}

/// A grammar with every language it embeds.
#[derive(Debug)]
pub(super) struct Compiled {
    pub(super) programs: Vec<Program>,
    pub(super) languages: HashMap<String, usize>,
    pub(super) main: usize,
}
