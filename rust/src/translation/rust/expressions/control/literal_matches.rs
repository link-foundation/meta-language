//! Token and source-owner transport into the shared literal matches! plan.
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::rust::{
    BOOL, BinaryOp, Options, Result, RustParser, SExpr, SNode, SourceOrigin, Span, Token, Type,
    unsupported,
};
use crate::translation::rust_literal_match_rules::{
    accept_rust_literal_match_receiver, plan_rust_literal_match_pattern,
};
use crate::translation::source_diagnostic_attributes::token_kind;

impl RustParser {
    pub(super) fn literal_match(&mut self, token: &Token, path: &[String]) -> Result<SExpr> {
        let close = match self.cursor.peek().value.as_str() {
            "(" => ")",
            "[" => "]",
            "{" => "}",
            _ => return Err(Self::fail("expected ( after matches!", self.cursor.peek())),
        };
        self.cursor.advance();
        let receiver = self.expr(path, Options::default())?;
        self.cursor.expect(",", Some("matches!"))?;
        let kinds = self
            .cursor
            .tokens
            .iter()
            .map(|item| token_kind(item).to_owned())
            .collect::<Vec<_>>();
        let values = self
            .cursor
            .tokens
            .iter()
            .map(|item| item.value.clone())
            .collect::<Vec<_>>();
        let start = serde_json::Number::from(self.cursor.index)
            .as_f64()
            .expect("a native token position has a finite binary64 representation");
        let plan = plan_rust_literal_match_pattern(&kinds, &values, start, close);
        let Some(end) = plan.first().copied().and_then(number_index) else {
            return Err(unsupported(
                "Rust literal matches!",
                "only a nonempty string-literal alternative list without guards or bindings is represented",
                Some(Span {
                    start: token.start,
                    end: self.cursor.peek().end,
                }),
            ));
        };
        let range = Some(Span {
            start: token.start,
            end: self.cursor.tokens[end - 1].end,
        });
        let origin = self.origins.origin(&receiver, path);
        let spelling = vec!["str".to_owned()];
        let resolved = self.resolve(&spelling, &origin.path);
        let shadowed = self
            .origins
            .lookup_literal_match_owner(&spelling, &origin.path)
            .is_some()
            || self
                .origins
                .lookup_literal_match_owner(&resolved, &origin.path)
                .is_some()
            || resolved.join("::") != "str";
        if (origin.proven || !self.origins.collecting)
            && !accept_rust_literal_match_receiver(
                origin.ty.as_ref().map_or("", Type::kind),
                origin.text_source_model.unwrap_or(""),
                origin.proven,
                &origin.wrappers.join("|"),
                shadowed,
            )
        {
            return Err(unsupported(
                "Rust literal matches! receiver",
                "the scrutinee needs its actual proven unshadowed borrowed str source owner",
                range,
            ));
        }
        self.cursor.index = end;
        let mut count = 0;
        let name = loop {
            let candidate = format!("rust_literal_match_value{count}");
            count += 1;
            if self.reserved_binding_names.insert(candidate.clone()) {
                break candidate;
            }
        };
        let mut alternatives = plan[1..].iter().map(|position| {
            let index = number_index(*position)
                .expect("the shared literal plan retains actual token positions");
            let literal = &self.cursor.tokens[index];
            SExpr::new(
                SNode::Binary {
                    op: BinaryOp::Eq,
                    left: Box::new(SExpr::new(
                        SNode::Name {
                            path: vec![name.clone()],
                        },
                        receiver.span.or(range),
                    )),
                    right: Box::new(SExpr::new(
                        SNode::Str {
                            value: literal.value.clone(),
                        },
                        Some(Span {
                            start: literal.start,
                            end: literal.end,
                        }),
                    )),
                    rounding: None,
                },
                range,
            )
        });
        let first = alternatives
            .next()
            .expect("the shared plan admits a nonempty literal list");
        let body = alternatives.fold(first, |left, right| {
            SExpr::new(
                SNode::Binary {
                    op: BinaryOp::Or,
                    left: Box::new(left),
                    right: Box::new(right),
                    rounding: None,
                },
                range,
            )
        });
        let result = SExpr::new(
            SNode::Let {
                name,
                ty: None,
                value: Box::new(receiver),
                body: Box::new(body),
            },
            range,
        );
        self.origins
            .mark(&result, SourceOrigin::with_type(BOOL, path, Vec::new()));
        Ok(result)
    }
}
