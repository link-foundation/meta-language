//! Typed node transport; the generated source rule owns admitted result roles.
use super::{Expr, Node, Type};
use crate::translation::abrupt_result_rules::read_abrupt_result_coercion_form;

pub(super) fn coerce_abrupt_result(value: &Expr, ty: &Type) -> Option<Expr> {
    let operation = match &value.node {
        Node::Math { op, .. } => op.as_str(),
        _ => "",
    };
    let form = read_abrupt_result_coercion_form(value.kind(), operation);
    let node = match (form.as_str(), &value.node) {
        ("leaf", _) => value.node.clone(),
        ("body", Node::Let { name, value, body }) => Node::Let {
            name: name.clone(),
            value: value.clone(),
            body: Box::new(coerce_abrupt_result(body, ty)?),
        },
        ("body", Node::Print { text, body }) => Node::Print {
            text: text.clone(),
            body: Box::new(coerce_abrupt_result(body, ty)?),
        },
        (
            "branches",
            Node::If {
                cond,
                then,
                otherwise,
            },
        ) => Node::If {
            cond: cond.clone(),
            then: Box::new(coerce_abrupt_result(then, ty)?),
            otherwise: Box::new(coerce_abrupt_result(otherwise, ty)?),
        },
        _ => return None,
    };
    Some(Expr::new(node, ty.clone()))
}
