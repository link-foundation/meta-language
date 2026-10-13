//! Expression macros, conditional values, matches and source patterns.

mod completion_panic;
mod completions;

use super::{
    Alias, Options, Result, RustParser, SArrayItem, SExpr, SNode, SPattern, SPatternNode, SRow,
    SourceOrigin, Span, Token, TokenKind, Type, accept_rust_source_array_element, describe, format,
    is_if_or_match, pattern_value, rust_macro_message, span, unsupported,
};

impl RustParser {
    pub(in super::super) fn expression_macro(
        &mut self,
        token: &Token,
        path: &[String],
    ) -> Result<SExpr> {
        let args = self.macro_arguments(token, path)?;
        let range = span(token, self.cursor.peek());
        match token.value.as_str() {
            "vec" => {
                for argument in &args {
                    let origin = self.origins.origin(argument, path);
                    if !origin.wrappers.is_empty()
                        || origin
                            .ty
                            .as_ref()
                            .is_some_and(|ty| !accept_rust_source_array_element(ty.kind(), false))
                    {
                        return Err(unsupported(
                            "Rust array element",
                            "only direct immutable scalar and owned string elements are admitted",
                            range,
                        ));
                    }
                }
                let element = args
                    .first()
                    .and_then(|argument| self.origins.origin(argument, path).ty);
                let mut origin = element.map_or_else(SourceOrigin::value, |element| {
                    SourceOrigin::with_type(
                        Type::Array {
                            element: Box::new(element),
                            reference: false,
                        },
                        path,
                        Vec::new(),
                    )
                });
                origin.array_source_model = Some("owned vector");
                let result = SExpr::new(
                    SNode::Array {
                        items: args
                            .into_iter()
                            .map(|value| SArrayItem {
                                spread: false,
                                value,
                            })
                            .collect(),
                        element: None,
                    },
                    range,
                );
                self.origins.mark(&result, origin);
                Ok(result)
            }
            "format" => {
                let mut formatted = format(args, token)?;
                formatted.span = range;
                Ok(formatted)
            }
            "panic" | "unreachable" | "unimplemented" | "todo" => {
                if let Some(message) = completion_panic::read_rust_completion_panic_message(
                    &self.completion_model,
                    &token.value,
                    &args,
                ) {
                    return Ok(SExpr::new(SNode::Abort { message }, range));
                }
                if args.len() > 1 {
                    return Err(unsupported(
                        &format!("{}! arguments", token.value),
                        "only a literal message is portable",
                        range,
                    ));
                }
                let text = match args.into_iter().next() {
                    None => None,
                    Some(SExpr {
                        node: SNode::Str { value },
                        span,
                        ..
                    }) => Some(panic_text(&value, span)?),
                    Some(_) => {
                        return Err(unsupported(
                            &format!("{}! message", token.value),
                            "the message must be a literal",
                            range,
                        ));
                    }
                };
                let message = rust_macro_message(&token.value, text.as_deref());
                Ok(SExpr::new(SNode::Abort { message }, range))
            }
            _ => Err(unsupported(
                &format!("{}!", token.value),
                "outside the portable core",
                range,
            )),
        }
    }

    pub(in super::super) fn if_expr(&mut self, path: &[String]) -> Result<SExpr> {
        let token = self.cursor.advance();
        if self.cursor.is("let") {
            return Err(unsupported(
                "if let",
                "write a match",
                span(&token, self.cursor.peek()),
            ));
        }
        let cond = self.expr(
            path,
            Options {
                statement: false,
                no_struct: true,
            },
        )?;
        self.origins.direct(&cond, path, cond.span)?;
        let then = self.block(path)?;
        if self.cursor.eat("else").is_none() {
            return Err(unsupported(
                "if without else",
                "the value of an if must be defined on both branches",
                span(&token, self.cursor.peek()),
            ));
        }
        let otherwise = if self.cursor.is("if") {
            self.if_expr(path)?
        } else {
            self.block(path)?
        };
        Ok(SExpr::new(
            SNode::If {
                cond: Box::new(cond),
                then: Box::new(then),
                otherwise: Box::new(otherwise),
            },
            span(&token, self.cursor.peek()),
        ))
    }

    pub(in super::super) fn match_expr(&mut self, path: &[String]) -> Result<SExpr> {
        let token = self.cursor.advance();
        let scrutinee = self.expr(
            path,
            Options {
                statement: false,
                no_struct: true,
            },
        )?;
        self.cursor.expect("{", Some("match"))?;
        let mut rows = Vec::new();
        while !self.cursor.is("}") {
            let arm_start = self.peek();
            let mut aliases = Vec::new();
            self.cursor.eat("|");
            let mut alternatives = vec![self.pattern(path, &mut aliases)?];
            while self.cursor.eat("|").is_some() {
                alternatives.push(self.pattern(path, &mut aliases)?);
            }
            if self.cursor.is("if") {
                return Err(unsupported(
                    "match guard",
                    "guards are outside the portable pattern language",
                    span(&arm_start, self.cursor.peek()),
                ));
            }
            self.cursor.expect("=>", Some("match arm"))?;
            let scope = self.origins.cases(&alternatives, &scrutinee, path)?;
            let previous = self.origins.enter(Some(scope));
            for alias in &aliases {
                self.origins.bind(&alias.name, &alias.value, path, None)?;
            }
            let mut body = self.expr(path, Options::default())?;
            let body_origin = self.origins.origin(&body, path);
            self.origins.scope = previous;
            if body.block || is_if_or_match(&body) {
                self.cursor.eat(",");
            } else if !self.cursor.is("}") {
                self.cursor.expect(",", Some("match arm"))?;
            }
            if alternatives.len() > 1 && !aliases.is_empty() {
                return Err(unsupported(
                    "binding in an or-pattern",
                    "bind the value in each alternative separately",
                    span(&arm_start, self.cursor.peek()),
                ));
            }
            for alias in aliases.into_iter().rev() {
                let body_span = body.span;
                body = SExpr::new(
                    SNode::Let {
                        name: alias.name,
                        ty: None,
                        value: Box::new(alias.value),
                        body: Box::new(body),
                    },
                    body_span,
                );
            }
            self.origins.mark(&body, body_origin);
            // `A | B => e` is one row per alternative with the same body.
            let row_span = span(&arm_start, self.cursor.peek());
            for pattern in alternatives {
                rows.push(SRow {
                    patterns: vec![pattern],
                    body: body.clone(),
                    span: row_span,
                });
            }
        }
        self.cursor.expect("}", Some("match"))?;
        if rows.is_empty() {
            return Err(unsupported(
                "empty match",
                "uninhabited types are outside the portable core",
                span(&token, self.cursor.peek()),
            ));
        }
        let origins = rows
            .iter()
            .map(|row| self.origins.origin(&row.body, path))
            .collect::<Vec<_>>();
        let origin = self.origins.join(&origins);
        let result = SExpr::new(
            SNode::Match {
                scrutinees: vec![scrutinee],
                rows,
            },
            span(&token, self.cursor.peek()),
        );
        self.origins.mark(&result, origin);
        Ok(result)
    }

    /// Patterns: `_`, bindings, literals, `Enum::Variant(p, …)`, `&p` and `name @ p`.
    #[allow(clippy::too_many_lines)] // one branch per pattern form, as in rust.js
    pub(in super::super) fn pattern(
        &mut self,
        path: &[String],
        aliases: &mut Vec<Alias>,
    ) -> Result<SPattern> {
        let token = self.peek();
        if self.cursor.eat("&").is_some() {
            let pattern = self.pattern(path, aliases)?;
            self.origins.reference_pattern(&pattern);
            return Ok(pattern);
        }
        if self.cursor.eat("_").is_some() {
            self.origins.wildcard_pattern();
            return Ok(SPattern {
                node: SPatternNode::Wild,
                span: None,
            });
        }
        if self.cursor.is("ref") || self.cursor.is("mut") {
            return Err(unsupported(
                &format!("{} binding", token.value),
                "bindings are by value in the portable core",
                span(&token, &token),
            ));
        }
        if self.cursor.is("-") && self.cursor.is_kind_at(TokenKind::Number, 1) {
            self.cursor.advance();
            let number = self.cursor.advance();
            return Ok(SPattern {
                node: SPatternNode::NumLit {
                    value: number.value.clone(),
                    negative: true,
                },
                span: span(&token, &number),
            });
        }
        if token.kind == TokenKind::Number {
            self.cursor.advance();
            if self.cursor.is("..") || self.cursor.is("..=") {
                return Err(unsupported(
                    "range pattern",
                    "ranges are outside the portable pattern language",
                    span(&token, self.cursor.peek()),
                ));
            }
            return Ok(SPattern {
                node: SPatternNode::NumLit {
                    value: token.value.clone(),
                    negative: false,
                },
                span: span(&token, &token),
            });
        }
        if token.kind == TokenKind::String {
            return Err(unsupported(
                "string pattern",
                "match on &str is outside the Rust frontend",
                span(&token, &token),
            ));
        }
        if self.cursor.is("true") || self.cursor.is("false") {
            self.cursor.advance();
            return Ok(SPattern {
                node: SPatternNode::BoolLit {
                    value: token.value == "true",
                    negative: false,
                },
                span: span(&token, &token),
            });
        }
        if self.cursor.is("(") {
            return Err(unsupported(
                "tuple match pattern",
                "tuple match coverage is not represented; use an immutable destructuring let",
                span(&token, &token),
            ));
        }
        if token.kind != TokenKind::Identifier {
            return Err(unsupported(
                "pattern",
                &format!(
                    "{} is outside the portable pattern language",
                    describe(&token)
                ),
                span(&token, &token),
            ));
        }
        let segments = self.path_segments("pattern")?;
        if self.cursor.eat("@").is_some() {
            if segments.len() != 1 {
                return Err(Self::fail("expected a binding before @", &token));
            }
            let inner = self.pattern(path, aliases)?;
            let value = pattern_value(&inner, span(&token, self.cursor.peek()))?;
            aliases.push(Alias {
                name: segments[0].clone(),
                value,
            });
            return Ok(inner);
        }
        if self.cursor.is("{") {
            return Err(unsupported(
                "struct pattern",
                "named fields are outside the Rust frontend",
                span(&token, self.cursor.peek()),
            ));
        }
        let resolved = self.resolve(&segments, path);
        if self.cursor.eat("(").is_some() {
            let mut args = Vec::new();
            while !self.cursor.is(")") {
                if self.cursor.is("..") {
                    let rest = self.cursor.peek();
                    return Err(unsupported(
                        "rest pattern",
                        "name every field",
                        span(rest, rest),
                    ));
                }
                args.push(self.pattern(path, aliases)?);
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
            self.cursor.expect(")", Some("pattern"))?;
            return Ok(SPattern {
                node: SPatternNode::Ctor {
                    path: resolved,
                    args,
                },
                span: span(&token, self.cursor.peek()),
            });
        }
        if segments.len() == 1 && resolved.len() == 1 {
            return Ok(SPattern {
                node: SPatternNode::BindOrCtor {
                    name: segments[0].clone(),
                },
                span: span(&token, &token),
            });
        }
        Ok(SPattern {
            node: SPatternNode::Ctor {
                path: resolved,
                args: Vec::new(),
            },
            span: span(&token, &token),
        })
    }
}

/// The text a `panic!` message literal prints: `{{` and `}}` are braces; a lone brace captures a variable.
fn panic_text(template: &str, span: Option<Span>) -> Result<String> {
    let mut text = String::new();
    let mut chars = template.chars().peekable();
    while let Some(char) = chars.next() {
        if (char == '{' || char == '}') && chars.next_if_eq(&char).is_none() {
            return Err(unsupported(
                "formatted panic message",
                "only a literal message is portable",
                span,
            ));
        }
        text.push(char);
    }
    Ok(text)
}
