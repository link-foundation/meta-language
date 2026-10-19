//! Source adapters for the linked standard readonly query decisions.
//! Mirrors rust-read-only-queries.js; value typing and evaluation use existing IR.

use super::super::read_only_query_rules::{
    accept_rust_inclusive_range_operand, accept_rust_read_only_query_owner,
    accept_rust_string_empty_receiver,
};
use super::super::{
    BOOL, BinaryOp, FLOAT, Options, Result, RustParser, SExpr, SNode, STRING, SourceOrigin, Span,
    Token, type_error, unsupported,
};

impl RustParser {
    pub(super) fn read_inclusive_range_query(
        &mut self,
        start: &Token,
        path: &[String],
        lower: SExpr,
    ) -> Result<SExpr> {
        self.cursor.advance();
        let upper = self.expr(path, Options::default())?;
        self.cursor.expect(")", Some("inclusive range"))?;
        self.cursor.expect(".", Some("inclusive range query"))?;
        let method = self.cursor.identifier(Some("inclusive range query"))?;
        if method.value != "contains" {
            return Err(unsupported(
                "inclusive range query",
                "only standard primitive f64 contains is portable",
                Some(method.span()),
            ));
        }
        self.cursor.expect("(", Some("inclusive range query"))?;
        let argument = self.expr(path, Options::default())?;
        let close = self.cursor.expect(")", Some("inclusive range query"))?;
        let range = Some(Span::new(start.start, close.end));
        for (value, borrowed) in [(&lower, false), (&upper, false), (&argument, true)] {
            let origin = self.origins.origin(value, path);
            let (kind, owner_path) = origin.source_owner.as_ref().map_or_else(
                || ("primitive", "f64".to_owned()),
                |owner| (owner.kind.as_str(), owner.path.join("::")),
            );
            let primitive = vec!["f64".to_owned()];
            let resolved = self.resolve(&primitive, &origin.path);
            let shadowed = self.origins.lookup(&primitive, &origin.path).is_some()
                || self.origins.lookup(&resolved, &origin.path).is_some();
            let layer = match origin.wrappers.as_slice() {
                [] => "value",
                [layer] => layer.as_str(),
                _ => "multiple",
            };
            if !self.origins.collecting
                && (!accept_rust_read_only_query_owner("inclusive range", kind, &owner_path)
                    || !accept_rust_inclusive_range_operand(
                        origin.ty.as_ref().map_or("", super::super::Type::kind),
                        origin.float_source_model.unwrap_or(""),
                        origin.proven,
                        layer,
                        borrowed,
                        shadowed,
                    ))
            {
                return Err(unsupported(
                    "inclusive range query",
                    "endpoints need direct primitive f64 values and contains needs one immutable f64 reference",
                    value.span.or(range),
                ));
            }
        }
        let values = [lower, upper, argument];
        let names = values.each_ref().map(|_| self.fresh_product_binding());
        let read = |index: usize| {
            SExpr::new(
                SNode::Name {
                    path: vec![names[index].clone()],
                },
                values[index].span,
            )
        };
        let comparison = |left, right| {
            let mut expression = super::super::binary(BinaryOp::Le, left, right, start);
            expression.span = range;
            expression
        };
        let mut result = SExpr::new(
            SNode::Binary {
                op: BinaryOp::And,
                left: Box::new(comparison(read(0), read(2))),
                right: Box::new(comparison(read(2), read(1))),
                rounding: None,
            },
            range,
        );
        for (value, name) in values.into_iter().zip(names).rev() {
            result = SExpr::new(
                SNode::Let {
                    name,
                    ty: Some(FLOAT.into()),
                    value: Box::new(value),
                    body: Box::new(result),
                },
                range,
            );
        }
        self.origins
            .mark(&result, SourceOrigin::with_type(BOOL, path, Vec::new()));
        Ok(result)
    }

    pub(super) fn read_string_empty_query(
        &mut self,
        receiver: &SExpr,
        arguments_count: usize,
        path: &[String],
        range: Option<Span>,
    ) -> Result<Option<SExpr>> {
        let origin = self.origins.origin(receiver, path);
        if origin.array_source_model.is_some() {
            return Ok(None);
        }
        if arguments_count != 0 {
            return Err(type_error(
                format!("is_empty expects 0 arguments but got {arguments_count}"),
                range,
            ));
        }
        let standard = origin.source_owner.as_ref().is_some_and(|owner| {
            accept_rust_read_only_query_owner("string empty", &owner.kind, &owner.path.join("::"))
        });
        if !self.origins.collecting
            && !accept_rust_string_empty_receiver(
                origin.ty.as_ref().map_or("", super::super::Type::kind),
                origin.text_source_model.unwrap_or(""),
                origin.proven,
                origin.wrappers.iter().all(|layer| layer == "reference"),
                standard,
            )
        {
            return Err(unsupported(
                "Rust string empty query",
                "the receiver needs a proven standard str or String owner with immutable source layers",
                range,
            ));
        }
        let result = SExpr::new(
            SNode::Binary {
                op: BinaryOp::Eq,
                left: Box::new(receiver.clone()),
                right: Box::new(SExpr::new(
                    SNode::Str {
                        value: String::new(),
                    },
                    range,
                )),
                rounding: None,
            },
            range,
        );
        self.origins
            .mark(&result, SourceOrigin::with_type(BOOL, path, Vec::new()));
        Ok(Some(result))
    }

    pub(super) fn read_empty_string_constructor(
        &mut self,
        segments: &[String],
        arguments_count: usize,
        path: &[String],
        range: Option<Span>,
    ) -> Result<Option<SExpr>> {
        if segments.len() < 2 || segments.last().is_none_or(|name| name != "new") {
            return Ok(None);
        }
        let owner = self.named_source_type_owner(&segments[..segments.len() - 1], Vec::new());
        if owner.kind != "library"
            || !accept_rust_read_only_query_owner(
                "string empty",
                &owner.kind,
                &owner.path.join("::"),
            )
        {
            return Ok(None);
        }
        if arguments_count != 0 {
            return Err(type_error(
                format!("String::new expects 0 arguments but got {arguments_count}"),
                range,
            ));
        }
        let result = SExpr::new(
            SNode::Str {
                value: String::new(),
            },
            range,
        );
        let mut origin = SourceOrigin::with_type(STRING, path, Vec::new());
        origin.source_owner = Some(owner);
        self.origins.mark(&result, origin);
        Ok(Some(result))
    }
}
