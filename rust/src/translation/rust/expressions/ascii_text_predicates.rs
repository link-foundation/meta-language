//! Token, primitive source-owner and existing-IR transport for shared ASCII queries.
//! The actual UTF-16 Text ABI is generated from the complete JavaScript closure.
use crate::translation::ascii_text_predicate_rules::{
    accept_rust_ascii_character_owner, ml_array::number_index, ml_string::Text,
    plan_rust_ascii_character_callback, plan_rust_ascii_text_query,
};
use crate::translation::rust::{
    BOOL, BinaryOp, Result, RustParser, SExpr, SNode, SParam, STRING, SourceOrigin, Span, Token,
    TokenKind, Type, unsupported,
};
use crate::translation::rust_literal_match_rules::accept_rust_literal_match_receiver;
use crate::translation::source_diagnostic_attributes::token_kind;

fn position(value: usize) -> f64 {
    serde_json::Number::from(value)
        .as_f64()
        .expect("an actual native token position has a finite binary64 representation")
}

fn read(name: &str, span: Option<Span>) -> SExpr {
    SExpr::new(
        SNode::Name {
            path: vec![name.to_owned()],
        },
        span,
    )
}

fn character(value: f64) -> char {
    let index =
        number_index(value).expect("the checked shared plan returns an integral ASCII scalar");
    char::from(u8::try_from(index).expect("every shared ASCII scalar fits a byte"))
}

impl RustParser {
    fn ascii_source_owner(&self, spelling: &str, path: &[String]) -> (String, bool) {
        let source = vec![spelling.to_owned()];
        let resolved = self.resolve(&source, path);
        let shadowed = self
            .origins
            .lookup_literal_match_owner(&source, path)
            .is_some()
            || self
                .origins
                .lookup_literal_match_owner(&resolved, path)
                .is_some();
        (resolved.join("::"), shadowed)
    }

    fn fresh_ascii_binding(&mut self, prefix: &str) -> String {
        let mut ordinal = 0;
        loop {
            let name = format!("{prefix}{ordinal}");
            ordinal += 1;
            if self.reserved_binding_names.insert(name.clone()) {
                return name;
            }
        }
    }

    /// Called after the actual .METHOD token, before ordinary method arguments.
    /// A selected invalid profile refuses; unrelated existing syntax returns None.
    pub(super) fn read_ascii_text_predicate(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        // Mechanical dispatch only; all selected syntax/owner decisions remain shared.
        if !["starts_with", "chars"].contains(&method.value.as_str()) {
            return Ok(None);
        }
        let kinds = self
            .cursor
            .tokens
            .iter()
            .map(|token| Text::from_text(token_kind(token)))
            .collect::<Vec<_>>();
        let values = self
            .cursor
            .tokens
            .iter()
            .map(|token| Text::from_text(&token.value))
            .collect::<Vec<_>>();
        let query = plan_rust_ascii_text_query(
            &Text::from_text(&method.value),
            &kinds,
            &values,
            position(self.cursor.index),
        );
        let Some(callback) = query.get(1).copied().and_then(number_index) else {
            return Ok(None);
        };
        self.require_type_namespace_body_method(receiver, &method.value, Some(method.span()))?;
        self.cursor.index = callback;
        let characters = self
            .cursor
            .tokens
            .iter()
            .map(|token| {
                if token.kind == TokenKind::Character {
                    token
                        .value
                        .parse::<f64>()
                        .expect("the canonical character lexer supplies a scalar Number")
                } else {
                    -1.0
                }
            })
            .collect::<Vec<_>>();
        let plan =
            plan_rust_ascii_character_callback(&kinds, &values, &characters, position(callback));
        let Some(end) = plan.first().copied().and_then(number_index) else {
            return Err(unsupported(
                "Rust ASCII character predicate",
                "the selected standard query needs a closed pure ASCII character predicate",
                Some(self.cursor.peek().span()),
            ));
        };
        let range = Some(Span::new(
            receiver.span.map_or(method.start, |span| span.start),
            self.cursor.tokens[end].end,
        ));
        let origin = self.origins.origin(receiver, path);
        let (text_owner, text_shadowed) = self.ascii_source_owner("str", &origin.path);
        if (origin.proven || !self.origins.collecting)
            && !accept_rust_literal_match_receiver(
                origin.ty.as_ref().map_or("", Type::kind),
                origin.text_source_model.unwrap_or(""),
                origin.proven,
                &origin.wrappers.join("|"),
                text_shadowed || text_owner != "str",
            )
        {
            return Err(unsupported(
                "Rust ASCII text receiver",
                "the actual receiver needs its proven unshadowed immutable borrowed str source owner",
                range,
            ));
        }
        let (character_owner, character_shadowed) = self.ascii_source_owner("char", path);
        let annotated = plan.get(1).copied().and_then(number_index).is_some();
        if !accept_rust_ascii_character_owner(
            annotated,
            &Text::from_text(&character_owner),
            character_shadowed,
        ) {
            return Err(unsupported(
                "Rust ASCII character owner",
                "the annotation must denote the actual primitive char",
                range,
            ));
        }
        self.cursor.index = end + 1;
        let name = self.fresh_ascii_binding("ascii_predicate_points");
        let first = query.first().copied().and_then(number_index) == Some(1);
        let (value, body) = if first {
            // A bounded <=128 pure startsWith disjunction. The receiver is
            // bound once and never converted into a full character collection.
            let mut alternatives = plan[2..].iter().map(|scalar| {
                SExpr::new(
                    SNode::StringTest {
                        op: "startsWith".to_owned(),
                        object: Box::new(read(&name, range)),
                        search: Box::new(SExpr::new(
                            SNode::Str {
                                value: character(*scalar).to_string(),
                            },
                            range,
                        )),
                    },
                    range,
                )
            });
            let body = alternatives.next().map_or_else(
                || SExpr::new(SNode::Bool { value: false }, range),
                |first| {
                    alternatives.fold(first, |left, right| {
                        SExpr::new(
                            SNode::Binary {
                                op: BinaryOp::Or,
                                left: Box::new(left),
                                right: Box::new(right),
                                rounding: None,
                            },
                            range,
                        )
                    })
                },
            );
            (receiver.clone(), body)
        } else {
            // Explicit eager StringPoints storage; allocation/failure behavior
            // of lazy source Iterator::all is outside this value interpretation.
            let point = self.fresh_ascii_binding("ascii_predicate_point");
            let set = plan[2..]
                .iter()
                .map(|scalar| character(*scalar))
                .collect::<String>();
            let callback = SExpr::new(
                SNode::Lambda {
                    params: vec![SParam {
                        name: point.clone(),
                        optional: false,
                        default_value: None,
                        ty: Some(STRING),
                        span: range,
                        guard: None,
                        rocq_type: None,
                    }],
                    ret: Some(BOOL),
                    captures: Vec::new(),
                    assigned_captures: None,
                    body: Box::new(SExpr::new(
                        SNode::StringTest {
                            op: "includes".to_owned(),
                            object: Box::new(SExpr::new(SNode::Str { value: set }, range)),
                            search: Box::new(read(&point, range)),
                        },
                        range,
                    )),
                },
                range,
            );
            let body = SExpr::new(
                SNode::Method {
                    object: Box::new(read(&name, range)),
                    op: "every".to_owned(),
                    args: vec![Some(callback)],
                },
                range,
            );
            (
                SExpr::new(
                    SNode::StringPoints {
                        object: Box::new(receiver.clone()),
                    },
                    range,
                ),
                body,
            )
        };
        let result = SExpr::new(
            SNode::Let {
                name,
                ty: None,
                value: Box::new(value),
                body: Box::new(body),
            },
            range,
        );
        self.origins
            .mark(&result, SourceOrigin::with_type(BOOL, path, Vec::new()));
        Ok(Some(result))
    }
}
