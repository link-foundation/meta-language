//! Actual checked operands and prelude transport for the shared projection rule.
use super::{Expr, Node, Prelude, Result, RustEmitter};
use crate::translation::reference_array_projection_rules::{
    accept_reference_array_projection_index, read_reference_array_projection,
    read_reference_array_projection_suffix, render_reference_array_projection,
};

fn accepts_index(expression: &Expr) -> bool {
    let children = match &expression.node {
        Node::Binary { left, right, .. } => vec![accepts_index(left), accepts_index(right)],
        Node::Unary { arg, .. } | Node::Cast { arg, .. } => vec![accepts_index(arg)],
        Node::Math { args, .. } => args.iter().map(accepts_index).collect(),
        _ => Vec::new(),
    };
    accept_reference_array_projection_index(expression.kind(), expression.ty.kind(), &children)
}
impl RustEmitter<'_> {
    pub(super) fn reference_array_projection(
        &mut self,
        expression: &Expr,
    ) -> Result<Option<String>> {
        let (array, index) = match &expression.node {
            Node::Length { array } => (array.as_ref(), None),
            Node::Index { array, index, .. } | Node::OptionalIndex { array, index } => {
                (array.as_ref(), Some(index.as_ref()))
            }
            _ => return Ok(None),
        };
        let reference = match &array.node {
            Node::ReferenceRead { reference } => reference.as_ref(),
            _ => return Ok(None),
        };
        let operation = read_reference_array_projection(
            expression.kind(),
            array.kind(),
            reference.ty.kind(),
            reference.ty.is_reference(),
            expression.ty.kind(),
            index.is_some_and(accepts_index),
        );
        if operation.is_empty() {
            return Ok(None);
        }
        let receiver = self.expr(reference)?;
        let position = index
            .map(|value| self.expr(value))
            .transpose()?
            .unwrap_or_default();
        let initial = serde_json::Number::from(self.temporaries + 1)
            .as_f64()
            .expect("a bounded temporary is a Number");
        let suffix =
            read_reference_array_projection_suffix(initial, &[receiver.clone(), position.clone()])
                .to_string();
        self.temporaries = suffix
            .parse()
            .expect("the selected temporary suffix is an integer");
        if operation == "length" {
            if !expression.ty.is_float() {
                self.preludes.insert(Prelude::Big);
            }
        } else {
            self.preludes.insert(Prelude::Array);
            if operation == "optionalIndex" {
                self.standard_array = true;
            } else if !matches!(
                &expression.node,
                Node::Index {
                    proven_in_bounds: true,
                    ..
                }
            ) {
                self.state.array_read();
            }
        }
        Ok(Some(render_reference_array_projection(
            &operation,
            &receiver,
            &position,
            index.map_or("", |value| value.ty.kind()),
            expression.ty.kind(),
            &suffix,
        )))
    }
}
