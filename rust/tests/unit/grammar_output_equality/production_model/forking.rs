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
// Bind every copied declaration without relaxing native lints.
const _: fn() = || {
    let _ = self::Lead::Literal;
    let _ = self::Lead::Ref;
    let _ = |value: &self::Lead| match value {
        self::Lead::Literal(field_0) => {
            let _ = (self::Lead::Literal, field_0);
        }
        self::Lead::Ref(field_0) => {
            let _ = (self::Lead::Ref, field_0);
        }
    };
};
