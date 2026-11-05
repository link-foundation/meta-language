//! Actual source primitive owners and literal syntax into shared decisions.
use super::{SourceOrigin, Type};
use crate::translation::{
    rust_primitive_expression_rules as rules,
    source_signatures::SourceOwner,
    surface::{SExpr, SNode, UnaryOp},
};
fn count(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}
fn literal(node: Option<&SExpr>) -> &'static str {
    let Some(node) = node else {
        return "";
    };
    let (number, negative) = match &node.node {
        SNode::Num { .. } => (node, false),
        SNode::Unary {
            op: UnaryOp::Neg,
            arg,
        } if matches!(arg.node, SNode::Num { .. }) => (arg.as_ref(), true),
        _ => return "",
    };
    match &number.node {
        SNode::Num { ty: None, .. } => {
            if negative {
                "negative integer numeral"
            } else {
                "integer numeral"
            }
        }
        SNode::Num {
            ty: Some(Type::Float),
            ..
        } => {
            if negative {
                "negative floating numeral"
            } else {
                "floating numeral"
            }
        }
        _ => "",
    }
}
fn facts(origin: &SourceOrigin) -> (SourceOrigin, bool, String, bool) {
    let mut origin = origin.clone();
    if origin.source_owner.is_none()
        && origin.ty == Some(Type::Float)
        && origin.float_source_model == Some("binary64 scalar")
    {
        origin.source_owner = Some(SourceOwner {
            kind: "primitive".to_owned(),
            path: vec!["f64".to_owned()],
            arguments: Vec::new(),
        });
    }
    let path = origin
        .source_owner
        .as_ref()
        .map_or_else(String::new, |owner| owner.path.join("::"));
    let (bits, signed) = match &origin.ty {
        Some(Type::Fixed { bits, signed }) => (bits.to_string(), *signed),
        _ => (String::new(), false),
    };
    let known = rules::accept_rust_primitive_arithmetic_origin(
        origin.ty.as_ref().map_or("", Type::kind),
        &bits,
        signed,
        origin
            .source_owner
            .as_ref()
            .map_or("", |owner| owner.kind.as_str()),
        &path,
        origin
            .source_owner
            .as_ref()
            .map_or(-1.0, |owner| count(owner.arguments.len())),
        origin.proven,
        count(origin.wrappers.len()),
    );
    (origin, known, path, signed)
}
pub(super) fn select_primitive_expression_origin(
    operation: &str,
    nodes: &[&SExpr],
    operands: &[SourceOrigin],
) -> Option<SourceOrigin> {
    let left = operands.first().map(facts);
    let right = operands.get(1).map(facts);
    let selected = rules::select_rust_primitive_arithmetic_origin(
        operation,
        left.as_ref().is_some_and(|value| value.1),
        right.as_ref().is_some_and(|value| value.1),
        left.as_ref().map_or("", |value| value.2.as_str()),
        right.as_ref().map_or("", |value| value.2.as_str()),
        left.as_ref().is_some_and(|value| value.3),
        right.as_ref().is_some_and(|value| value.3),
        literal(nodes.first().copied()),
        literal(nodes.get(1).copied()),
    );
    if selected == 0.0 {
        left.map(|value| value.0)
    } else if selected == 1.0 {
        right.map(|value| value.0)
    } else {
        None
    }
}
