//! The loaded program the executor interprets: every expression compiled to
//! a typed tree with its terminals as byte matchers, every operation checked
//! for its context, every rule call resolved to a rule index or an external
//! token. `load.rs` builds it; `executor.rs` runs it.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::operations::{Condition, Statement};
use super::text::{decode_at, fold_case, quote_text};
use crate::grammar::{ByteClassItem, RuleKind};

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
    pub(super) node_kind: Name,
    pub(super) kind: RuleKind,
    pub(super) expression: Expr,
    pub(super) action: Option<Vec<Statement>>,
    pub(super) modes: Option<Vec<Name>>,
    pub(super) lexical_priority: i64,
}

/// One external scanner.
#[derive(Debug)]
pub(super) struct Scanner {
    pub(super) operations: Vec<Statement>,
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
    pub(super) start: Option<String>,
    pub(super) rules: Vec<Rule>,
    pub(super) rule_index: HashMap<String, usize>,
    pub(super) external: HashMap<String, usize>,
    pub(super) scanners: Vec<Scanner>,
    pub(super) conflicts: HashSet<String>,
    pub(super) trivia: Vec<Trivia>,
}

/// A grammar with every language it embeds.
#[derive(Debug)]
pub(super) struct Compiled {
    pub(super) programs: Vec<Program>,
    pub(super) languages: HashMap<String, usize>,
    pub(super) main: usize,
}
