//! Actual checked operands into the shared scalar-string byte-count policy.
use super::diagnostics::{Result, unsupported};
use super::ir::{Expr, Node};
use super::{Language, rust_string_byte_length_rules as rules};
pub(super) fn validate(value: &Expr, language: Language, utf16: bool) -> Result<()> {
    let Node::Math { args, .. } = &value.node else {
        unreachable!("actual byte-count operation");
    };
    let error = rules::read_rust_string_byte_length_argument_error(
        language.as_str(),
        utf16,
        serde_json::Number::from(args.len())
            .as_f64()
            .expect("actual argument count"),
        false,
        args.first().map_or("", |arg| arg.ty.kind()),
    );
    if error.is_empty() {
        Ok(())
    } else {
        Err(unsupported("Rust String byte length", error, value.span))
    }
}

// Validate the actual representation after the selected shared scalar bridge.
pub(super) fn validate_with_scalar_bridge(
    value: &Expr,
    language: Language,
    utf16: bool,
    target: &str,
) -> Result<()> {
    validate(
        value,
        language,
        utf16
            && !super::rust_string_byte_view_rules::accept_rust_string_byte_scalar_bridge(
                target, utf16,
            ),
    )
}

include!("../../tests/internal/translation/rust_string_byte_lengths.rs");
