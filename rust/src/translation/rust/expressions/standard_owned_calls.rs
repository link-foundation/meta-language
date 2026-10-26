//! Actual qualified source calls into existing owned value projections.
use super::{
    Options, Result, RustParser, SExpr, SNode, STRING, SourceOrigin, Span, TokenKind, Type,
    type_error, unsupported,
};
use crate::translation::rust_read_only_query_rules::accept_rust_read_only_query_owner;
use crate::translation::rust_scalar_vector_copy_rules::{
    accept_rust_scalar_vector_copy_owner, accept_rust_scalar_vector_copy_receiver,
};
use crate::translation::rust_standard_owned_call_rules::{
    accept_rust_standard_owned_array_call, accept_rust_standard_owned_scalar_call,
    accept_rust_standard_owned_text_call, read_rust_standard_owned_call_role,
};
use crate::translation::source_signatures::SourceOwner;
fn count(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}
fn accept_standard_scalar_origin(origin: &SourceOrigin, owner: &SourceOwner) -> bool {
    if !owner.arguments.is_empty() {
        return false;
    }
    if owner.kind == "unit" {
        return owner.path.is_empty() && matches!(origin.ty, Some(Type::Unit));
    }
    if owner.kind != "primitive" || owner.path.len() != 1 {
        return false;
    }
    let (bits, signed) = match &origin.ty {
        Some(Type::Fixed { bits, signed }) => (bits.to_string(), *signed),
        _ => (String::new(), false),
    };
    crate::translation::source_signature_rules::accept_rust_source_signature_primitive(
        origin.ty.as_ref().map_or("", Type::kind),
        &owner.path[0],
        &bits,
        signed,
    )
}
impl RustParser {
    pub(super) fn read_standard_owned_call(&mut self, path: &[String]) -> Result<Option<SExpr>> {
        let start = self.peek();
        let mut offset = usize::from(self.cursor.is("::"));
        if self.cursor.peek_at(offset).value != "std" {
            return Ok(None);
        }
        let mut segments = Vec::new();
        while self.cursor.peek_at(offset).kind == TokenKind::Identifier && segments.len() < 16 {
            segments.push(self.cursor.peek_at(offset).value.clone());
            offset += 1;
            if !self.cursor.is_at("::", offset) {
                break;
            }
            offset += 1;
        }
        let shadowed = self.origins.lookup(&["std".to_owned()], path).is_some()
            || self.origins.scope.contains_key("std")
            || self
                .aliases
                .values()
                .any(|aliases| aliases.contains_key("std"));
        let role = read_rust_standard_owned_call_role(
            &segments.join("::"),
            self.ordinary_completion_context(),
            shadowed,
        );
        if role.is_empty() {
            return Ok(None);
        }
        if !self.cursor.is_at("(", offset) {
            return Err(unsupported(
                "Rust standard owned call",
                "a standard trait function value needs its own callable interpretation",
                Some(Span {
                    start: start.start,
                    end: self.cursor.peek_at(offset).end,
                }),
            ));
        }
        for _ in 0..=offset {
            self.cursor.advance();
        }
        let mut args = Vec::new();
        while !self.cursor.is(")") {
            args.push(self.expr(path, Options::default())?);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        let close = self.cursor.expect(")", Some("standard owned call"))?;
        let range = Some(Span {
            start: start.start,
            end: close.end,
        });
        if args.len() != 1 {
            return Err(type_error(
                "a standard owned call needs one actual borrowed argument",
                range,
            ));
        }
        if self.origins.collecting {
            return Ok(Some(SExpr::new(
                SNode::App {
                    func: Box::new(SExpr::new(SNode::Name { path: segments }, range)),
                    args,
                },
                range,
            )));
        }
        self.project_standard_owned_call(&role, &args[0], range, path)
    }

    fn project_standard_owned_call(
        &mut self,
        role: &str,
        receiver: &SExpr,
        range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        let origin = self.origins.origin(receiver, path);
        let owner = origin.source_owner.clone().or_else(|| {
            matches!(origin.ty, Some(Type::String)).then(|| {
                self.named_source_type_owner(
                    &[if origin.text_source_model == Some("string slice") {
                        "str".to_owned()
                    } else {
                        "String".to_owned()
                    }],
                    Vec::new(),
                )
            })
        });
        let known_text = owner.as_ref().is_some_and(|owner| {
            accept_rust_read_only_query_owner("string empty", &owner.kind, &owner.path.join("::"))
        });
        let references = origin.wrappers.iter().all(|layer| layer == "reference");
        if accept_rust_standard_owned_text_call(
            role,
            origin.ty.as_ref().map_or("", Type::kind),
            origin.text_source_model.unwrap_or(""),
            count(origin.wrappers.len()),
            origin.proven,
            references,
            known_text,
        ) {
            return Ok(Some(
                self.mark_standard_owned_value(receiver, origin, range, path, true),
            ));
        }
        let known_scalar = owner
            .as_ref()
            .is_some_and(|owner| accept_standard_scalar_origin(&origin, owner));
        if accept_rust_standard_owned_scalar_call(
            role,
            origin.ty.as_ref().map_or("", Type::kind),
            count(origin.wrappers.len()),
            origin.proven,
            references,
            known_scalar,
        ) {
            return Ok(Some(
                self.mark_standard_owned_value(receiver, origin, range, path, false),
            ));
        }
        let known_array = owner.as_ref().is_some_and(|owner| {
            accept_rust_scalar_vector_copy_owner(
                &owner.kind,
                &owner.path.join("::"),
                count(owner.path.len()),
                count(owner.arguments.len()),
            )
        });
        let element = match &origin.ty {
            Some(Type::Array { element, .. }) => Some(element.as_ref()),
            _ => None,
        };
        if accept_rust_standard_owned_array_call(
            role,
            origin.array_source_model.unwrap_or(""),
            count(origin.wrappers.len()),
            known_array,
        ) && accept_rust_scalar_vector_copy_receiver(
            origin.ty.as_ref().map_or("", Type::kind),
            element.map_or("", Type::kind),
            origin.proven,
            references,
            origin.array_element_copy_evidence,
            0.0,
        ) {
            return Ok(Some(
                self.build_scalar_vector_copy(receiver, origin, range, path),
            ));
        }
        Err(unsupported(
            "Rust standard owned call",
            "the actual borrowed standard scalar, String, str or direct scalar vector receiver is required",
            range,
        ))
    }

    fn mark_standard_owned_value(
        &mut self,
        receiver: &SExpr,
        mut copied: SourceOrigin,
        range: Option<Span>,
        path: &[String],
        text: bool,
    ) -> SExpr {
        let mut result = receiver.clone();
        result.span = range;
        copied.path = path.to_vec();
        copied.wrappers.clear();
        copied.source_lifetimes.clear();
        copied.copy_evidence = if text { None } else { Some(()) };
        if text {
            copied.ty = Some(STRING);
            copied.text_source_model = Some("owned string");
            copied.source_owner = Some(SourceOwner {
                kind: "library".into(),
                path: vec!["std".into(), "string".into(), "String".into()],
                arguments: Vec::new(),
            });
        }
        self.origins.mark(&result, copied);
        result
    }
}
