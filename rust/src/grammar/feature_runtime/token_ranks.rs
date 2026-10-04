//! The token ranks of a `(matching longest)` grammar, by which the native
//! executor orders two tokens over the same text as a lexer does, as
//! `tokenRanks` in `js/src/grammar-runtime/load.js`.

use std::collections::HashSet;
use std::sync::Arc;

use super::program::{Expr, Matcher, Rule, Target, TokenRank, TokenRanks};
use crate::grammar::RuleKind;

/// The rank of each token of a `(matching longest)` grammar, by which two
/// tokens over the same text conflict as a lexer orders them: the higher
/// lexical precedence, then (at one length) a literal over a pattern, an
/// immediate token by one more, then the earlier token. Tokens are numbered
/// in rule order as they first appear: a token rule at its own definition, a
/// literal, an inline token and an alias of either where they are used; a
/// token rule's kind ranks as the rule even where an alias took it first. It
/// mirrors tokenRanks in js/src/grammar-runtime/load.js.
pub(super) fn token_ranks(rules: &[Rule]) -> TokenRanks {
    let mut ranks = Ranking::default();
    let mut aliased = Vec::new();
    // A token rule ranks its own kind, over an alias of another token to it
    // that came first (Rust's `(alias identifier (literal default))`).
    let mut defined = HashSet::new();
    for rule in rules {
        if rule.kind != RuleKind::Token {
            ranks.walk(&rule.expression, &mut aliased);
        } else if defined.insert(rule.node_kind.clone()) {
            ranks.ranks.kinds.remove(rule.node_kind.as_ref());
            ranks.assign_kind(&rule.node_kind, rank_of(&rule.expression, 0));
        }
    }
    for (name, index) in aliased {
        if rules[index].kind == RuleKind::Token
            && !ranks.ranks.kinds.contains_key(name.as_ref())
            && let Some(rank) = ranks.ranks.kinds.get(&rules[index].node_kind).copied()
        {
            ranks.ranks.kinds.insert(name, rank);
        }
    }
    ranks.ranks
}

/// The ranks assigned so far and the next order.
#[derive(Default)]
struct Ranking {
    ranks: TokenRanks,
    order: usize,
}

impl Ranking {
    fn assign_kind(&mut self, kind: &Arc<str>, (priority, specificity): (i64, u8)) {
        if !self.ranks.kinds.contains_key(kind) {
            let order = self.next();
            self.ranks.kinds.insert(
                kind.clone(),
                TokenRank {
                    priority,
                    specificity,
                    order,
                },
            );
        }
    }

    fn assign_literal(&mut self, literal: &[u8], (priority, specificity): (i64, u8)) {
        if !self.ranks.literals.contains_key(literal) {
            let order = self.next();
            self.ranks.literals.insert(
                literal.to_vec(),
                TokenRank {
                    priority,
                    specificity,
                    order,
                },
            );
        }
    }

    const fn next(&mut self) -> usize {
        self.order += 1;
        self.order - 1
    }

    fn walk(&mut self, expr: &Expr, aliased: &mut Vec<(Arc<str>, usize)>) {
        match expr {
            Expr::Alias { name, item } if matches!(**item, Expr::Ref(_)) => {
                if let Expr::Ref(Target::Rule(index)) = **item {
                    aliased.push((name.clone(), index));
                }
            }
            Expr::Alias { name, item } if lexical(item) => self.assign_kind(name, rank_of(item, 0)),
            _ if lexical(expr) => {
                if let Some(literal) = bare(expr) {
                    self.assign_literal(literal, rank_of(expr, 0));
                }
            }
            Expr::Seq(items) | Expr::Choice { items, .. } | Expr::Longest(items) => {
                for item in items {
                    self.walk(item, aliased);
                }
            }
            Expr::Recover { item, synchronize } => {
                self.walk(item, aliased);
                self.walk(synchronize, aliased);
            }
            Expr::Repeat { item, .. }
            | Expr::And(item)
            | Expr::Not(item)
            | Expr::Capture { item, .. }
            | Expr::Alias { item, .. }
            | Expr::Precedence { item, .. }
            | Expr::DynamicPrecedence { item, .. }
            | Expr::LexicalPrecedence { item, .. }
            | Expr::Token(item)
            | Expr::ImmediateToken(item)
            | Expr::Predicate { item, .. }
            | Expr::Missing { item, .. }
            | Expr::Embed { item, .. } => self.walk(item, aliased),
            Expr::Empty | Expr::Terminal { .. } | Expr::Ref(_) => {}
        }
    }
}

/// A literal closed by lookaheads, such as a keyword, ranks as the literal.
fn closed(expr: &Expr) -> &Expr {
    if let Expr::Seq(items) = expr {
        let mut kept = items
            .iter()
            .filter(|item| !matches!(item, Expr::Not(_) | Expr::And(_)));
        if let (Some(only), None) = (kept.next(), kept.next()) {
            return closed(only);
        }
    }
    expr
}

/// The lexical precedence and the specificity of a token expression.
fn rank_of(expr: &Expr, priority: i64) -> (i64, u8) {
    match closed(expr) {
        Expr::LexicalPrecedence { level, item } => rank_of(item, *level),
        Expr::Token(item) => rank_of(item, priority),
        Expr::ImmediateToken(item) => {
            let (priority, specificity) = rank_of(item, priority);
            (priority, specificity + 1)
        }
        Expr::Terminal {
            matcher: Matcher::Literal(_),
            ..
        } => (priority, 2),
        _ => (priority, 0),
    }
}

fn lexical(expr: &Expr) -> bool {
    match expr {
        Expr::Terminal {
            matcher: Matcher::Literal(_),
            ..
        }
        | Expr::Token(_)
        | Expr::ImmediateToken(_) => true,
        Expr::LexicalPrecedence { item, .. } => lexical(item),
        _ => false,
    }
}

/// The literal a token expression is, under its token wrappers.
fn bare(expr: &Expr) -> Option<&[u8]> {
    match closed(expr) {
        Expr::Terminal {
            matcher: Matcher::Literal(literal),
            ..
        } => Some(literal),
        Expr::LexicalPrecedence { item, .. } | Expr::Token(item) | Expr::ImmediateToken(item) => {
            bare(item)
        }
        _ => None,
    }
}
