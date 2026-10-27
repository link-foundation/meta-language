//! Actual path, literal, type and origin projection into shared method rules.
use super::{
    Result, RustParser, SExpr, SNode, SourceOrigin, Span, TokenKind, Type, UnaryOp,
    rust_fixed_type, type_error, unsupported,
};
use crate::translation::rust_primitive_associated_call_rules as rules;
use crate::translation::source_signatures::SourceOwner;
fn count(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}
impl RustParser {
    pub(in super::super) fn primitive_associated_import(
        &self,
        segments: &[String],
        path: &[String],
    ) -> bool {
        let root = segments.first().cloned().unwrap_or_default();
        let shadowed = self.origins.lookup(&[root.clone()], path).is_some()
            || self.origins.scope.contains_key(&root);
        let mut qualified = segments.to_vec();
        qualified.push("min".to_owned());
        !rules::read_rust_primitive_associated_owner(
            &qualified,
            self.ordinary_completion_context(),
            shadowed,
            false,
        )
        .is_empty()
            || !crate::translation::rust_primitive_from_rules::read_rust_primitive_from_owner(
                &qualified,
                self.ordinary_completion_context(),
                shadowed,
                false,
            )
            .is_empty()
    }
    pub(super) fn read_primitive_associated_call(
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
        let method = resolved.last().cloned().unwrap_or_default();
        let owner = if method == "from" {
            crate::translation::rust_primitive_from_rules::read_rust_primitive_from_owner(
                &resolved,
                self.ordinary_completion_context(),
                shadowed,
                absolute,
            )
        } else {
            rules::read_rust_primitive_associated_owner(
                &resolved,
                self.ordinary_completion_context(),
                shadowed,
                absolute,
            )
        };
        let conversion =
            crate::translation::rust_primitive_from_rules::accept_rust_primitive_from_method(
                &owner, &method,
            );
        let arity = if conversion {
            1.0
        } else {
            rules::read_rust_primitive_associated_arity(&owner, &method)
        };
        if arity < 0.0 {
            return Ok(None);
        }
        if !self.cursor.is_at("(", offset) {
            return Err(unsupported(
                "Rust primitive associated call",
                "a primitive associated function value needs its own callable interpretation",
                Some(Span {
                    start: start.start,
                    end: self.cursor.peek_at(offset).end,
                }),
            ));
        }
        for _ in 0..=offset {
            self.cursor.advance();
        }
        let mut values = self.call_arguments(path)?;
        let close = self.cursor.expect(")", Some("primitive associated call"))?;
        let range = Some(Span {
            start: start.start,
            end: close.end,
        });
        if count(values.len()) != arity {
            return Err(type_error(
                format!(
                    "primitive associated {method} expects {arity} arguments but got {}",
                    values.len()
                ),
                range,
            ));
        }
        if self.origins.collecting {
            let mut token = start.clone();
            token.value = method.clone();
            if let Some(checked) =
                self.read_checked_integer(&values[0], &token, &values[1..], range, path)?
            {
                return Ok(Some(checked));
            }
            return Ok(Some(SExpr::new(
                SNode::App {
                    func: Box::new(SExpr::new(SNode::Name { path: resolved }, range)),
                    args: values,
                },
                range,
            )));
        }
        let mut receiver = values.remove(0);
        if conversion {
            return self
                .project_primitive_from(receiver, &owner, range, path)
                .map(Some);
        }
        let (form, typed) = match &receiver.node {
            SNode::Num { ty, .. } => ("numeral", ty.is_some()),
            SNode::Unary {
                op: UnaryOp::Neg,
                arg,
            } => match &arg.node {
                SNode::Num { ty, .. } => ("negative numeral", ty.is_some()),
                _ => ("", true),
            },
            _ => ("", true),
        };
        if rules::accept_rust_primitive_associated_literal(&owner, form, typed) {
            let Some(ty) = rust_fixed_type(&owner) else {
                return Err(type_error(
                    "a selected integer primitive lost its actual type",
                    range,
                ));
            };
            match &mut receiver.node {
                SNode::Num { ty: actual, .. } => *actual = Some(ty.clone()),
                SNode::Unary { arg, .. } => {
                    if let SNode::Num { ty: actual, .. } = &mut arg.node {
                        *actual = Some(ty.clone());
                    }
                }
                _ => {}
            }
            let mut origin = SourceOrigin::with_type(ty, path, Vec::new());
            origin.source_owner = Some(SourceOwner {
                kind: "primitive".to_owned(),
                path: vec![owner.clone()],
                arguments: Vec::new(),
            });
            self.origins.mark(&receiver, origin);
        }
        let origin = self.origins.origin(&receiver, path);
        let actual = origin.source_owner.clone().or_else(|| {
            matches!(receiver.node, SNode::Bool { .. }).then(|| SourceOwner {
                kind: "primitive".to_owned(),
                path: vec!["bool".to_owned()],
                arguments: Vec::new(),
            })
        });
        let (bits, signed) = match &origin.ty {
            Some(Type::Fixed { bits, signed }) => (bits.to_string(), *signed),
            _ => (String::new(), false),
        };
        if !rules::accept_rust_primitive_associated_receiver(
            &owner,
            origin.ty.as_ref().map_or("", Type::kind),
            &bits,
            signed,
            actual.as_ref().map_or("", |owner| owner.kind.as_str()),
            &actual
                .as_ref()
                .map_or_else(String::new, |owner| owner.path.join("::")),
            actual
                .as_ref()
                .map_or(-1.0, |owner| count(owner.arguments.len())),
            origin.proven,
            count(origin.wrappers.len()),
        ) {
            return Err(unsupported(
                "Rust primitive associated call",
                "the explicit receiver must retain the selected standard primitive type and source owner",
                range,
            ));
        }
        let mut token = start;
        token.value = method;
        if let Some(value) =
            self.read_primitive_selection(&receiver, &token, &values, range, path)?
        {
            return Ok(Some(value));
        }
        if let Some(value) =
            self.read_signed_integer_property(&receiver, &token, &values, range, path)?
        {
            return Ok(Some(value));
        }
        if let Some(value) =
            self.read_saturating_integer(&receiver, &token, &values, range, path)?
        {
            return Ok(Some(value));
        }
        if let Some(value) =
            self.read_saturating_multiplication(&receiver, &token, &values, range, path)?
        {
            return Ok(Some(value));
        }
        if let Some(value) = self.read_wrapping_integer(&receiver, &token, &values, range, path)? {
            return Ok(Some(value));
        }
        if let Some(checked) = self.read_checked_integer(&receiver, &token, &values, range, path)? {
            return Ok(Some(checked));
        }
        Err(type_error(
            "the registered primitive associated call lost its shared method interpretation",
            range,
        ))
    }
}
