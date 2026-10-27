//! Source paths/types and existing shared exact bound definitions into ASTs.
use super::{
    Result, RustParser, SExpr, SNode, SourceOrigin, Span, TokenKind, Type, UnaryOp,
    rust_fixed_type, type_error, unsupported,
};
use crate::translation::rust_primitive_integer_constant_rules as rules;
use crate::translation::rust_saturating_integer_rules as bounds;
use crate::translation::source_signatures::SourceOwner;
impl RustParser {
    pub(super) fn read_primitive_integer_constant(
        &mut self,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        let start = self.peek();
        let absolute = self.cursor.is("::");
        let mut offset = usize::from(absolute);
        let mut segments = Vec::new();
        while self.cursor.peek_at(offset).kind == TokenKind::Identifier && segments.len() < 16 {
            segments.push(self.cursor.peek_at(offset).value.clone());
            offset += 1;
            if !self.cursor.is_at("::", offset) {
                break;
            }
            if self.cursor.peek_at(offset + 1).kind != TokenKind::Identifier {
                return Ok(None);
            }
            offset += 1;
        }
        if segments.len() < 2 {
            return Ok(None);
        }
        let resolved = if absolute {
            segments.clone()
        } else {
            self.resolve(&segments, path)
        };
        let root = segments.first().cloned().unwrap_or_default();
        let target = resolved.first().cloned().unwrap_or_default();
        let shadowed = !absolute
            && (self.origins.lookup(&[root.clone()], path).is_some()
                || self.origins.scope.contains_key(&root)
                || self.origins.lookup(&[target.clone()], path).is_some()
                || self.origins.scope.contains_key(&target));
        let owner = rules::read_rust_primitive_integer_constant_owner(
            &resolved,
            self.ordinary_completion_context(),
            shadowed,
            absolute,
        );
        let term = rules::read_rust_primitive_integer_constant_term(
            resolved.last().map_or("", String::as_str),
        );
        if owner.is_empty() || term.is_empty() {
            return Ok(None);
        }
        let range = Some(Span {
            start: start.start,
            end: self.cursor.peek_at(offset - 1).end,
        });
        if self.cursor.is_at("(", offset) {
            return Err(unsupported(
                "Rust primitive integer constant",
                "an associated integer constant requires value syntax rather than a call",
                range,
            ));
        }
        for _ in 0..offset {
            self.cursor.advance();
        }
        let source = rust_fixed_type(&owner).ok_or_else(|| {
            type_error(
                "the selected primitive constant lost its actual source type",
                range,
            )
        })?;
        let Type::Fixed { bits, signed } = source else {
            return Err(type_error(
                "the selected primitive constant requires an actual fixed integer source type",
                range,
            ));
        };
        let result_owner = rules::read_rust_primitive_integer_constant_result_owner(&owner, &term);
        let ty = rust_fixed_type(&result_owner).ok_or_else(|| {
            type_error(
                "the selected primitive constant lost its actual result type",
                range,
            )
        })?;
        let magnitude = if term == "width" {
            bits.to_string()
        } else {
            bounds::read_rust_saturating_integer_bound_magnitude(&term, f64::from(bits), signed)
        };
        if magnitude.is_empty() {
            return Err(type_error(
                "the shared primitive constant lost its exact magnitude",
                range,
            ));
        }
        let literal = SExpr::new(
            SNode::Num {
                value: magnitude,
                ty: Some(ty.clone()),
                negative: false,
                unit: false,
            },
            range,
        );
        let node = if term != "width"
            && bounds::accept_rust_saturating_integer_bound_negative(&term, signed)
        {
            SExpr::new(
                SNode::Unary {
                    op: UnaryOp::Neg,
                    arg: Box::new(literal),
                },
                range,
            )
        } else {
            literal
        };
        let mut origin = SourceOrigin::with_type(ty, path, Vec::new());
        origin.source_owner = Some(SourceOwner {
            kind: "primitive".to_owned(),
            path: vec![result_owner],
            arguments: Vec::new(),
        });
        self.origins.mark(&node, origin);
        Ok(Some(node))
    }
}
