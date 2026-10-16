// Generated from complete owned Rust CST declarations. Do not edit by hand.
use std::sync::Arc;
/// A shared name: a rule, a node kind, a field, a mode or a language.
pub(super) type Name = Arc<str>;
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
/// The rank of a token of a `(matching longest)` grammar: its lexical
/// precedence, its specificity (2 for a literal, 0 for a pattern, one more
/// for an immediate token) and its order in the grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct TokenRank {
    pub(super) priority: i64,
    pub(super) specificity: u8,
    pub(super) order: usize,
}
