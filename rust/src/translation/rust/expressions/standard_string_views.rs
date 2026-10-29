//! Actual source tokens and owners into the shared immediate-borrow decisions.
use super::{
    Options, Result, RustParser, SExpr, SNode, STRING, SourceOrigin, Span, TokenKind, Type,
    type_error, unsupported,
};
use crate::translation::rust_standard_string_view_rules as rules;
use crate::translation::source_signatures::SourceOwner;

#[derive(Default)]
pub(in super::super) struct StandardStringBorrowContext {
    enabled: bool,
    layers: usize,
    sites: Vec<Span>,
}
fn count(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}
fn known_result(origin: &SourceOrigin) -> bool {
    if !origin.proven || !origin.wrappers.is_empty() {
        return false;
    }
    let Some(owner) = origin.source_owner.as_ref() else {
        return matches!(origin.ty, Some(Type::Unit));
    };
    if !owner.arguments.is_empty() {
        return false;
    }
    if matches!(origin.ty, Some(Type::Unit)) {
        return owner.kind == "unit" && owner.path.is_empty();
    }
    if matches!(origin.ty, Some(Type::String)) {
        return owner.kind == "library"
            && owner.path.join("::") == "std::string::String"
            && origin.text_source_model == Some("owned string");
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
    fn standard_string_view_path(
        &self,
        path: &[String],
        prefix: bool,
    ) -> Option<(Vec<String>, usize)> {
        let mut offset = 0;
        if prefix {
            while ["&", "&&", "("]
                .iter()
                .any(|token| self.cursor.is_at(token, offset))
            {
                offset += 1;
                if offset > 256 {
                    return None;
                }
            }
        }
        offset += usize::from(self.cursor.is_at("::", offset));
        if self.cursor.peek_at(offset).value != "std" {
            return None;
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
        rules::accept_rust_standard_string_view_path(
            &segments.join("::"),
            self.ordinary_completion_context(),
            shadowed,
        )
        .then_some((segments, offset))
    }
    fn standard_string_view_context(
        &self,
        path: &[String],
        parameter: Option<&SourceOrigin>,
        result: Option<&SourceOrigin>,
    ) -> StandardStringBorrowContext {
        let known_slice = parameter.is_some_and(|origin| {
            origin.proven
                && origin.text_source_model == Some("string slice")
                && origin.wrappers.iter().all(|layer| layer == "reference")
                && origin.source_owner.as_ref().is_some_and(|owner| {
                    owner.kind == "primitive"
                        && owner.path.join("::") == "str"
                        && owner.arguments.is_empty()
                })
        });
        let layers = parameter.map_or(0, |origin| origin.wrappers.len());
        let enabled = rules::accept_rust_standard_string_view_consumer(
            parameter
                .and_then(|origin| origin.ty.as_ref())
                .map_or("", Type::kind),
            count(layers),
            known_slice,
            parameter.is_some_and(|origin| {
                origin
                    .source_lifetimes
                    .iter()
                    .any(|lifetime| lifetime.as_deref() == Some("static"))
            }),
            result
                .and_then(|origin| origin.ty.as_ref())
                .map_or("", Type::kind),
            result.map_or(-1.0, |origin| count(origin.wrappers.len())),
            result.is_some_and(known_result),
        ) && self.standard_string_view_path(path, true).is_some();
        StandardStringBorrowContext {
            enabled,
            layers,
            sites: Vec::new(),
        }
    }
    pub(in super::super) fn read_standard_string_view_argument(
        &mut self,
        path: &[String],
        parameter: Option<&SourceOrigin>,
        result: Option<&SourceOrigin>,
    ) -> Result<SExpr> {
        let selected = self.standard_string_view_context(path, parameter, result);
        let previous = std::mem::replace(&mut self.standard_string_borrow, selected);
        let parsed = self.read_expected_copy_closure(path, parameter);
        let context = std::mem::replace(&mut self.standard_string_borrow, previous);
        let value = parsed?;
        if context.sites.is_empty() {
            return Ok(value);
        }
        let origin = self.origins.origin(&value, path);
        if !rules::accept_rust_standard_string_view_argument(
            count(context.sites.len()),
            value.span == context.sites.first().copied(),
            origin.ty.as_ref().map_or("", Type::kind),
            count(origin.wrappers.len()),
            count(context.layers),
            origin.wrappers.iter().all(|layer| layer == "reference"),
        ) {
            return Err(unsupported(
                "Rust standard string view",
                "the complete direct borrowed argument must reach its checked consumer",
                value.span,
            ));
        }
        Ok(value)
    }
    pub(in super::super) fn read_standard_string_view(
        &mut self,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        let Some((_segments, offset)) = self.standard_string_view_path(path, false) else {
            return Ok(None);
        };
        let start = self.peek();
        let head = Some(Span {
            start: start.start,
            end: self.cursor.peek_at(offset).end,
        });
        if !self.cursor.is_at("(", offset) {
            return Err(unsupported(
                "Rust standard string view",
                "a standard string view function value needs its own callable interpretation",
                head,
            ));
        }
        if !self.standard_string_borrow.enabled {
            return Err(unsupported(
                "Rust standard string view",
                "the view needs an immediate checked nonstatic string argument consumer",
                head,
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
        let close = self.cursor.expect(")", Some("standard string view"))?;
        let range = Some(Span {
            start: start.start,
            end: close.end,
        });
        if args.len() != 1 {
            return Err(type_error(
                "a standard string view needs one actual borrowed String argument",
                range,
            ));
        }
        self.project_standard_string_view(&args[0], range, path)
            .map(Some)
    }
    fn project_standard_string_view(
        &mut self,
        receiver: &SExpr,
        range: Option<Span>,
        path: &[String],
    ) -> Result<SExpr> {
        let mut origin = self.origins.origin(receiver, path);
        let owner = origin
            .source_owner
            .clone()
            .unwrap_or_else(|| self.named_source_type_owner(&["String".to_owned()], Vec::new()));
        let known = owner.kind == "library"
            && owner.path.join("::") == "std::string::String"
            && owner.arguments.is_empty();
        if !rules::accept_rust_standard_string_view_receiver(
            origin.ty.as_ref().map_or("", Type::kind),
            origin.text_source_model.unwrap_or(""),
            count(origin.wrappers.len()),
            origin.proven,
            origin.wrappers.iter().all(|layer| layer == "reference"),
            known,
            1.0,
        ) {
            return Err(unsupported(
                "Rust standard string view",
                "the receiver needs its actual standard borrowed String owner",
                range,
            ));
        }
        if !rules::accept_rust_standard_string_view_layers(count(
            self.standard_string_borrow.sites.len().saturating_add(1),
        )) {
            return Err(unsupported(
                "Rust standard string view",
                "the string view argument exceeds its source site budget",
                range,
            ));
        }
        self.standard_string_borrow
            .sites
            .push(range.expect("actual qualified source call span"));
        let mut value = receiver.clone();
        value.span = range;
        origin.ty = Some(STRING);
        origin.path = path.to_vec();
        origin.text_source_model = Some("string slice");
        origin.wrappers = vec!["reference".to_owned()];
        origin.source_lifetimes = vec![origin.source_lifetimes.first().cloned().flatten()];
        origin.source_owner = Some(SourceOwner {
            kind: "primitive".to_owned(),
            path: vec!["str".to_owned()],
            arguments: Vec::new(),
        });
        origin.copy_evidence = None;
        self.origins.mark(&value, origin);
        Ok(value)
    }
}
