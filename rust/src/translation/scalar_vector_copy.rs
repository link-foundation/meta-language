//! Structural checked-IR projection into the shared full-copy value policy.
use super::ir::{Expr, LitValue, Node};
use super::types::Type;
pub(crate) fn read_rust_scalar_vector_copy_value<'a>(
    language: &str,
    expression: &'a Expr,
) -> Option<&'a Expr> {
    let Node::Method { object, op, args } = &expression.node else {
        return None;
    };
    let element = match &object.ty {
        Type::Array { element, .. } => element.kind(),
        _ => "",
    };
    let bound = |index: usize| {
        args.get(index).map_or(("", ""), |value| match &value.node {
            Node::Lit {
                value: LitValue::Text(text),
            } => (value.ty.kind(), text.as_str()),
            _ => ("", ""),
        })
    };
    let (start_kind, start) = bound(0);
    let (end_kind, end) = bound(1);
    super::rust_scalar_vector_copy_rules::accept_rust_scalar_vector_copy_value(
        language,
        expression.kind(),
        op,
        object.ty.kind(),
        element,
        object.ty.is_reference(),
        u32::try_from(args.len()).map_or(f64::INFINITY, f64::from),
        start_kind,
        start,
        end_kind,
        end,
    )
    .then_some(object.as_ref())
}
