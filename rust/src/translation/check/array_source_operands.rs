//! Actual source receiver/key bindings around existing typed reads.
use super::{Checker, Env, Expr, Node};
use crate::translation::reference_array_projection_rules::accept_reference_array_source_operands;

impl Checker {
    pub(super) fn capture_array_source_operands(
        &mut self,
        array: Expr,
        index: Expr,
        env: &Env,
    ) -> (Expr, Expr, Vec<(String, Expr)>) {
        let reference = match &array.node {
            Node::ReferenceRead { reference } => Some(reference.as_ref()),
            _ => None,
        };
        if !accept_reference_array_source_operands(
            self.language.as_str(),
            array.kind(),
            reference.map_or("", |value| value.ty.kind()),
            reference.is_some_and(|value| value.ty.is_reference()),
        ) {
            return (array, index, Vec::new());
        }
        let reference = reference.unwrap();
        let mut inner = env.clone();
        let mut bindings = Vec::new();
        for (prefix, value) in [
            ("ml_array_read_value", reference.clone()),
            ("ml_array_read_index", index),
        ] {
            let name = loop {
                let name = self.fresh_name(prefix);
                if !inner.has(&name) {
                    break name;
                }
            };
            inner.bind_local(&name, value.ty.clone());
            bindings.push((name, value));
        }
        let owner = Expr::new(
            Node::Var {
                name: bindings[0].0.clone(),
            },
            bindings[0].1.ty.clone(),
        );
        let key = Expr::new(
            Node::Var {
                name: bindings[1].0.clone(),
            },
            bindings[1].1.ty.clone(),
        );
        (
            Expr::new(
                Node::ReferenceRead {
                    reference: Box::new(owner),
                },
                array.ty,
            ),
            key,
            bindings,
        )
    }
}
pub(super) fn wrap_array_source_operands(mut body: Expr, bindings: Vec<(String, Expr)>) -> Expr {
    for (name, value) in bindings.into_iter().rev() {
        let ty = body.ty.clone();
        body = Expr::new(
            Node::Let {
                name,
                value: Box::new(value),
                body: Box::new(body),
            },
            ty,
        );
    }
    body
}
