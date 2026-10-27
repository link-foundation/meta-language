//! Actual source terms transported through the generated formatting plan.
use super::{
    BinaryOp, Options, Result, RustParser, SExpr, SNode, STRING, ShowStyle, SourceOrigin, Span,
    Token, TokenKind, TranslationError, span, unsupported,
};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::rust_formatting_rules as rules;
use crate::translation::rust_named_formatting_rules as named_rules;

fn malformed(place: Option<Span>) -> TranslationError {
    unsupported(
        "Rust formatting facts",
        "the format requires actual literal units and operand positions",
        place,
    )
}
fn text_slice(units: &[u16], first: f64, last: f64, place: Option<Span>) -> Result<String> {
    let first = number_index(first).ok_or_else(|| malformed(place))?;
    let last = number_index(last).ok_or_else(|| malformed(place))?;
    let slice = units.get(first..last).ok_or_else(|| malformed(place))?;
    String::from_utf16(slice).map_err(|_| malformed(place))
}

impl RustParser {
    /// Only formatting macros accept identifier = expression operands.
    pub(super) fn formatting_arguments(
        &mut self,
        token: &Token,
        path: &[String],
    ) -> Result<(Vec<SExpr>, Vec<String>)> {
        let close = match self.cursor.peek().value.as_str() {
            "(" => ")",
            "[" => "]",
            "{" => "}",
            _ => {
                return Err(Self::fail(
                    &format!("expected ( after {}!", token.value),
                    self.cursor.peek(),
                ));
            }
        };
        self.cursor.advance();
        let mut args = Vec::new();
        let mut names = Vec::new();
        while !self.cursor.is(close) {
            let name = if self.cursor.peek().kind == TokenKind::Identifier
                && self.cursor.is_at("=", 1)
            {
                let source = self.cursor.advance();
                self.cursor.advance();
                if args.is_empty() {
                    return Err(TranslationError::syntax(
                        "the format string must be the first positional argument",
                        span(&source, &source),
                    ));
                }
                if source.value == "_"
                    || !crate::translation::lexer::accept_rust_source_identifier_name(&source.value)
                {
                    return Err(TranslationError::syntax(
                        "invalid named format argument",
                        span(&source, &source),
                    ));
                }
                source.value
            } else {
                String::new()
            };
            args.push(self.expr(path, Options::default())?);
            if args.len() > 1 {
                names.push(name);
            }
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor
            .expect(close, Some(&format!("{}!", token.value)))?;
        let error = if names.iter().any(|name| !name.is_empty()) {
            named_rules::read_rust_formatting_argument_name_error(&names)
        } else {
            String::new()
        };
        if !error.is_empty() {
            return Err(TranslationError::syntax(
                error,
                span(token, self.cursor.peek()),
            ));
        }
        Ok((args, names))
    }

    pub(super) fn format(
        &mut self,
        args: Vec<SExpr>,
        token: &Token,
        path: &[String],
        argument_names: Option<Vec<String>>,
    ) -> Result<SExpr> {
        let mut args = args.into_iter();
        let Some(SExpr {
            node: SNode::Str { value: source },
            span: template_span,
            ..
        }) = args.next()
        else {
            return Err(unsupported(
                &format!("{}!", token.value),
                "the format string must be a literal",
                span(token, token),
            ));
        };
        let values = args.collect::<Vec<_>>();
        let units = source.encode_utf16().collect::<Vec<_>>();
        let columns = units
            .iter()
            .map(|unit| f64::from(*unit))
            .collect::<Vec<_>>();
        #[allow(clippy::cast_precision_loss)]
        let count = values.len() as f64;
        let plan = if let Some(names) =
            argument_names.filter(|names| names.iter().any(|name| !name.is_empty()))
        {
            let terms = rules::plan_rust_formatting_field_terms(&columns, count);
            let mut fields = Vec::new();
            if terms.first() == Some(&0.0) {
                for instruction in terms[1..].as_chunks::<3>().0 {
                    if instruction[0] == 2.0 {
                        let source_name =
                            text_slice(&units, instruction[1], instruction[2], template_span)?;
                        if source_name == "_"
                            || !crate::translation::lexer::accept_rust_source_identifier_name(
                                &source_name,
                            )
                        {
                            return Err(unsupported(
                                &format!("format argument {{{source_name}}}"),
                                "outside the portable core",
                                template_span,
                            ));
                        }
                        fields.push(crate::translation::lexer::normalize_unicode_text(
                            &source_name,
                        )?);
                    }
                }
            }
            named_rules::plan_rust_named_formatting_value(&terms, &names, &fields)
        } else {
            rules::plan_rust_formatting_value(&columns, count)
        };
        let Some(code) = plan.first().copied() else {
            return Err(malformed(template_span));
        };
        if code != 0.0 {
            let code = number_index(-code).ok_or_else(|| malformed(template_span))?;
            let spec = if matches!(code, 3 | 6) {
                text_slice(&units, plan[1], plan[2], template_span)?
            } else {
                String::new()
            };
            return Err(match code {
                1 => TranslationError::syntax("unmatched } in format string", template_span),
                2 => TranslationError::syntax("unclosed { in format string", template_span),
                3 => unsupported(
                    &format!("format spec {{{spec}}}"),
                    "only plain {} Display formatting is portable",
                    template_span,
                ),
                5 => TranslationError::syntax(
                    "format string has more {} than arguments",
                    template_span,
                ),
                6 => TranslationError::syntax(
                    format!("format argument {spec} is missing"),
                    template_span,
                ),
                7 => TranslationError::syntax(
                    "format string has fewer {} than arguments",
                    template_span,
                ),
                _ => malformed(template_span),
            });
        }
        if !(plan.len() - 1).is_multiple_of(3) {
            return Err(malformed(template_span));
        }
        let mut captures = Vec::new();
        let mut references = Vec::new();
        for position in rules::plan_rust_formatting_operand_capture(count) {
            let index = number_index(position).ok_or_else(|| malformed(template_span))?;
            let value = values
                .get(index)
                .cloned()
                .ok_or_else(|| malformed(template_span))?;
            self.refuse_shared_value_observation(&value, None, path, value.span)?;
            self.refuse_callable_observation(&value, path, value.span)?;
            let name = self.fresh_product_binding();
            let reference = SExpr::new(
                SNode::Name {
                    path: vec![name.clone()],
                },
                value.span,
            );
            let origin = self.origins.origin(&value, path);
            self.origins.mark(&reference, origin);
            references.push(reference);
            captures.push((name, value));
        }
        let mut pieces = Vec::new();
        for instruction in plan[1..].as_chunks::<3>().0 {
            let code = number_index(instruction[0]).ok_or_else(|| malformed(template_span))?;
            if code == 0 {
                pieces.push(SExpr::new(
                    SNode::Str {
                        value: text_slice(&units, instruction[1], instruction[2], template_span)?,
                    },
                    template_span,
                ));
                continue;
            }
            let value = match code {
                1 => references
                    .get(number_index(instruction[1]).ok_or_else(|| malformed(template_span))?)
                    .cloned()
                    .ok_or_else(|| malformed(template_span))?,
                2 => {
                    let source_name =
                        text_slice(&units, instruction[1], instruction[2], template_span)?;
                    if source_name == "_"
                        || !crate::translation::lexer::accept_rust_source_identifier_name(
                            &source_name,
                        )
                    {
                        return Err(unsupported(
                            &format!("format argument {{{source_name}}}"),
                            "outside the portable core",
                            template_span,
                        ));
                    }
                    let name = crate::translation::lexer::normalize_unicode_text(&source_name)?;
                    let value = self
                        .read_function_reference(std::slice::from_ref(&name), path, template_span)?
                        .unwrap_or_else(|| {
                            SExpr::new(SNode::Name { path: vec![name] }, template_span)
                        });
                    self.refuse_callable_observation(&value, path, value.span)?;
                    self.refuse_shared_value_observation(&value, None, path, value.span)?;
                    value
                }
                _ => return Err(malformed(template_span)),
            };
            let value_span = value.span;
            pieces.push(SExpr::new(
                SNode::Show {
                    arg: Box::new(value),
                    style: ShowStyle::Rust,
                },
                value_span,
            ));
        }
        let mut pieces = pieces.into_iter();
        let mut body = pieces.next().ok_or_else(|| malformed(template_span))?;
        for right in pieces {
            body = SExpr::new(
                SNode::Binary {
                    op: BinaryOp::Concat,
                    left: Box::new(body),
                    right: Box::new(right),
                    rounding: None,
                },
                template_span,
            );
        }
        for (name, value) in captures.into_iter().rev() {
            let place = value.span;
            body = SExpr::new(
                SNode::Let {
                    name,
                    ty: None,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                place,
            );
        }
        self.origins
            .mark(&body, SourceOrigin::with_type(STRING, path, Vec::new()));
        Ok(body)
    }
}
