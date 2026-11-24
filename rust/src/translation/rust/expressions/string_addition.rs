//! Actual source projections to the shared standard String addition decisions.
use super::{BinaryOp, Result, RustParser, SExpr, SNode, STRING, SourceOrigin, Type, unsupported};
use crate::translation::rust_string_addition_rules as rules;
use crate::translation::source_signatures::SourceOwner;
fn count(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}
impl RustParser {
    pub(super) fn read_string_addition(
        &mut self,
        node: &SExpr,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        let SNode::Binary {
            op, left, right, ..
        } = &node.node
        else {
            return Ok(None);
        };
        let left = self.origins.origin(left, path);
        let right = self.origins.origin(right, path);
        let ordinary = self.ordinary_completion_context()
            && self.local_value_state_model.is_empty()
            && self.type_namespace_model.is_empty()
            && self.const_function_model.is_empty()
            && !self.scalar_loop_active;
        if !rules::accept_rust_string_addition_candidate(
            op.name(),
            ordinary,
            left.ty.as_ref().map_or("", Type::kind),
            right.ty.as_ref().map_or("", Type::kind),
        ) {
            return Ok(None);
        }
        if !self.origins.collecting {
            for (side, origin) in [("left", &left), ("right", &right)] {
                let owner = origin.source_owner.clone().unwrap_or_else(|| {
                    self.named_source_type_owner(
                        &[if origin.text_source_model == Some("string slice") {
                            "str".to_owned()
                        } else {
                            "String".to_owned()
                        }],
                        Vec::new(),
                    )
                });
                let known = rules::accept_rust_string_addition_owner(
                    side,
                    &owner.kind,
                    &owner.path.join("::"),
                    count(owner.arguments.len()),
                );
                if !rules::accept_rust_string_addition_operand(
                    side,
                    origin.ty.as_ref().map_or("", Type::kind),
                    origin.text_source_model.unwrap_or(""),
                    count(origin.wrappers.len()),
                    origin.proven,
                    origin.wrappers.iter().all(|layer| layer == "reference"),
                    known,
                ) {
                    return Err(unsupported(
                        "Rust string addition",
                        "an actual owned standard String and one immutable standard str or String reference are required",
                        node.span,
                    ));
                }
            }
        }
        let mut value = node.clone();
        if let SNode::Binary { op, .. } = &mut value.node {
            *op = BinaryOp::Concat;
        }
        let mut origin = SourceOrigin::with_type(STRING, path, Vec::new());
        origin.text_source_model = Some("owned string");
        origin.copy_evidence = None;
        origin.source_owner = Some(SourceOwner {
            kind: "library".to_owned(),
            path: vec!["std".to_owned(), "string".to_owned(), "String".to_owned()],
            arguments: Vec::new(),
        });
        self.origins.consume_owned_binding(&left, node.span)?;
        self.origins.check_owned_origin(&right, node.span)?;
        self.origins.mark_owned_string_constructor(&value, origin);
        Ok(Some(value))
    }
}
