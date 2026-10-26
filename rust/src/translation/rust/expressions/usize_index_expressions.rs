//! Actual source ancestry/ownership transport; admission is generated from JS.
use super::{Result, RustParser, SExpr, SNode, Type, unsupported};
use crate::translation::rust_array_index_rules::read_rust_scalar_array_index_operand_error;
use crate::translation::rust_usize_index_expression_rules::read_rust_usize_index_expression_error;
impl RustParser {
    pub(super) fn usize_index_expression(&self, index: &SExpr, path: &[String]) -> Result<String> {
        let (mut kinds, mut operators, mut atoms, mut literals, mut parents) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
        let mut pending = vec![(index, -1.0, 0usize)];
        while let Some((node, parent, depth)) = pending.pop() {
            if kinds.len() >= 4096 || depth > 128 {
                return Err(unsupported(
                    "Rust usize index expression",
                    if depth > 128 {
                        "the index expression exceeds its source depth limit"
                    } else {
                        "the index expression exceeds its source node limit"
                    },
                    index.span,
                ));
            }
            let structural = matches!(
                &node.node,
                SNode::Binary { .. } | SNode::If { .. } | SNode::Match { .. }
            );
            let origin = if structural {
                super::SourceOrigin::unknown()
            } else {
                self.origins.origin(node, path)
            };
            let position = serde_json::Number::from(kinds.len())
                .as_f64()
                .expect("bounded source position");
            let owner = origin.pointer_integer_owner();
            let (bits, signed) = match origin.ty.as_ref() {
                Some(Type::Fixed { bits, signed }) => (f64::from(*bits), *signed),
                _ => (0.0, false),
            };
            kinds.push(node.node.kind().to_owned());
            operators.push(match &node.node {
                SNode::Binary { op, .. } => op.name().to_owned(),
                _ => String::new(),
            });
            parents.push(parent);
            atoms.push(
                read_rust_scalar_array_index_operand_error(
                    false,
                    origin.ty.as_ref().map_or("", Type::kind),
                    bits,
                    signed,
                    &owner,
                    serde_json::Number::from(origin.wrappers.len())
                        .as_f64()
                        .expect("actual source wrapper count"),
                    origin.proven,
                )
                .is_empty(),
            );
            literals.push(matches!(
                &node.node,
                SNode::Num {
                    ty: None,
                    negative: false,
                    ..
                }
            ));
            match &node.node {
                SNode::Binary { left, right, .. } => {
                    pending.push((right, position, depth + 1));
                    pending.push((left, position, depth + 1));
                }
                SNode::If {
                    then, otherwise, ..
                } => {
                    pending.push((otherwise, position, depth + 1));
                    pending.push((then, position, depth + 1));
                }
                SNode::Match { rows, .. } => {
                    for row in rows.iter().rev() {
                        pending.push((&row.body, position, depth + 1));
                    }
                }
                _ => {}
            }
        }
        Ok(read_rust_usize_index_expression_error(
            &kinds, &operators, &atoms, &literals, &parents,
        ))
    }
}
