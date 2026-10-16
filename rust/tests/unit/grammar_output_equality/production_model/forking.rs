// Generated from complete owned Rust CST declarations. Do not edit by hand.
use super::program::Name;
/// A token a parser may see first, as tree-sitter's FIRST sets name it: a
/// literal by its text, a token rule, an external token or a rule a lexer
/// matches whole by its name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum Lead {
    Literal(Vec<u8>),
    Ref(Name),
}
