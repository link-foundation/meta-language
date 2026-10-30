//! Expression macros, conditional values, matches and source patterns.

mod completion_panic;
mod completions;
pub(in crate::translation::rust) mod conditional_chains;
mod conditional_patterns;
mod literal_matches;
mod pattern_matches;

use super::{
    Alias, Options, Result, RustParser, SArrayItem, SExpr, SNode, SPattern, SPatternNode, SRow,
    SourceOrigin, Span, Token, TokenKind, Type, accept_rust_source_array_element, describe,
    is_if_or_match, pattern_value, rust_macro_message, span, unsupported,
};

impl RustParser {
    pub(in super::super) fn expression_macro(
        &mut self,
        token: &Token,
        path: &[String],
    ) -> Result<SExpr> {
        if token.value == "matches" {
            return self.literal_match(token, path);
        }
        let (args, argument_names) = if token.value == "format"
            || crate::translation::rust_panic_value_rules::accept_rust_panic_formatting_macro(
                &token.value,
                self.ordinary_completion_context(),
            ) {
            let (args, names) = self.formatting_arguments(token, path)?;
            (args, Some(names))
        } else {
            (self.macro_arguments(token, path)?, None)
        };
        let range = span(token, self.cursor.peek());
        match token.value.as_str() {
            "vec" => {
                let argument_origins = args
                    .iter()
                    .map(|argument| self.origins.origin(argument, path))
                    .collect::<Vec<_>>();
                let owners = argument_origins
                    .iter()
                    .map(SourceOrigin::pointer_integer_owner)
                    .collect::<Vec<_>>();
                let owner_error = if argument_origins
                    .iter()
                    .any(|origin| origin.integer_source_owner_conflict)
                {
                    crate::translation::rust_pointer_integer_owner_rules::read_rust_pointer_integer_owner_error("usize","u64")
                } else {
                    crate::translation::rust_pointer_integer_owner_rules::read_rust_pointer_integer_owner_join_error(&owners)
                };
                if !owner_error.is_empty() {
                    return Err(unsupported(
                        "Rust integer source owner",
                        &owner_error,
                        range,
                    ));
                }
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
                let owner=crate::translation::rust_pointer_integer_owner_rules::select_rust_pointer_integer_result_owner(&owners);
                let actual_owners = argument_origins
                    .iter()
                    .map(|value| {
                        value
                            .expanded_source_owner
                            .as_ref()
                            .or(value.source_owner.as_ref())
                    })
                    .collect::<Vec<_>>();
                let identities = actual_owners
                    .iter()
                    .map(|value| {
                        value.map_or_else(String::new, |value| {
                            format!("{}:{}", value.kind, value.path.join("::"))
                        })
                    })
                    .collect::<Vec<_>>();
                let admitted = actual_owners.iter().zip(&argument_origins).map(|(owner, value)| {
                    let Some(owner) = owner else { return false; };
                    let Some(ty) = value.ty.as_ref() else { return false; };
                    let (bits, signed) = match ty { Type::Fixed { bits, signed } => (bits.to_string(), *signed), _ => (String::new(), false) };
                    owner.kind == "primitive" && owner.arguments.is_empty() && crate::translation::source_signature_rules::accept_rust_source_signature_primitive(ty.kind(), &owner.path.join("::"), &bits, signed)
                }).collect::<Vec<_>>();
                let owner_index = crate::translation::rust_array_element_owner_rules::select_rust_array_element_owner_index(&identities, &admitted);
                let element_owner = if owner_index >= 0.0 {
                    actual_owners[owner_index as usize].cloned()
                } else if !owner.is_empty() {
                    Some(crate::translation::source_signatures::SourceOwner {
                        kind: "primitive".to_owned(),
                        path: vec![owner],
                        arguments: Vec::new(),
                    })
                } else {
                    None
                };
                if let Some(element_owner) = element_owner {
                    origin.source_owner =
                        Some(crate::translation::source_signatures::SourceOwner {
                            kind: "library".to_owned(),
                            path: vec!["std".to_owned(), "vec".to_owned(), "Vec".to_owned()],
                            arguments: vec![element_owner],
                        });
                }
                origin.array_element_copy_evidence = !args.is_empty()
                    && args
                        .iter()
                        .all(|argument| self.origins.origin(argument, path).direct_copy_value());
                let result = SExpr::new(
                    SNode::Array {
                        items: args
                            .into_iter()
                            .map(|value| SArrayItem::new(value, false))
                            .collect(),
                        element: None,
                    },
                    range,
                );
                self.origins.mark_vector_constructor(&result, origin);
                self.shared_array_literal(&result)?;
                Ok(result)
            }
            "format" => {
                let mut formatted = self.format(args, token, path, argument_names)?;
                let origin = self.origins.origin(&formatted, path);
                formatted.span = range;
                self.origins.mark(&formatted, origin);
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
                let (kind,braces)=args.first().map_or(("",false),|value|(value.node.kind(),matches!(&value.node,SNode::Str{value} if value.contains('{')||value.contains('}'))));
                if crate::translation::rust_panic_value_rules::accept_rust_panic_formatting_value(
                    &token.value,
                    self.ordinary_completion_context(),
                    kind,
                    args.len() as f64,
                    braces,
                ) {
                    let formatted = self.format(args, token, path, argument_names)?;
                    let prefix=crate::translation::rust_panic_value_rules::read_rust_panic_formatting_prefix(&token.value);
                    let value = if prefix.is_empty() {
                        formatted
                    } else {
                        SExpr::new(
                            SNode::Binary {
                                op: super::BinaryOp::Concat,
                                left: Box::new(SExpr::new(SNode::Str { value: prefix }, range)),
                                right: Box::new(formatted),
                                rounding: None,
                            },
                            range,
                        )
                    };
                    let result=SExpr::new(SNode::Math{op:crate::translation::rust_panic_value_rules::read_rust_panic_value_operation(),name:"Rust formatted panic message".to_owned(),args:vec![SArrayItem{value,spread:false}]},range);
                    let mut origin =
                        SourceOrigin::with_type(super::super::super::UNIT, path, Vec::new());
                    origin.diverging = true;
                    self.origins.mark(&result, origin);
                    return Ok(result);
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
        if self.has_conditional_chain() {
            return self.conditional_chain(path, &token);
        }
        if self.cursor.is("let") {
            return self.conditional_pattern_value(path, &token);
        }
        let cond = self.expr(
            path,
            Options {
                statement: false,
                no_struct: true,
            },
        )?;
        self.origins.direct(&cond, path, cond.span)?;
        let ownership_base = self.origins.snapshot_owned_bindings();
        let then = self.block(path)?;
        let ownership_then = self.origins.snapshot_owned_bindings();
        self.origins.restore_owned_bindings(ownership_base);

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
        let ownership_else = self.origins.snapshot_owned_bindings();
        self.origins.join_owned_bindings(
            &ownership_then,
            &ownership_else,
            !self.origins.origin(&then, path).diverging,
            !self.origins.origin(&otherwise, path).diverging,
        );
        self.origins.checked_join(&[
            self.origins.origin(&then, path),
            self.origins.origin(&otherwise, path),
        ])?;
        let result = SExpr::new(
            SNode::If {
                cond: Box::new(cond),
                then: Box::new(then),
                otherwise: Box::new(otherwise),
            },
            span(&token, self.cursor.peek()),
        );
        self.origins.transfer_owned_result(&result, path)?;
        Ok(result)
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
        self.match_from_scrutinee(path, &token, scrutinee)
    }

    pub(in crate::translation::rust) fn match_from_scrutinee(
        &mut self,
        path: &[String],
        token: &Token,
        scrutinee: SExpr,
    ) -> Result<SExpr> {
        self.cursor.expect("{", Some("match"))?;
        let mut rows = Vec::new();
        let ownership_base = self.origins.snapshot_owned_bindings();
        let mut ownership_arms = Vec::new();
        let mut guarded_arms = Vec::new();
        while !self.cursor.is("}") {
            self.origins.restore_owned_bindings(ownership_base.clone());
            let arm_start = self.peek();
            let mut aliases = Vec::new();
            self.cursor.eat("|");
            let mut alternatives = vec![self.pattern(path, &mut aliases)?];
            while self.cursor.eat("|").is_some() {
                alternatives.push(self.pattern(path, &mut aliases)?);
            }
            let scope = self.origins.cases(&alternatives, &scrutinee, path)?;
            let previous = self.origins.enter(Some(scope));
            let parsed = (|| {
                for alias in &aliases {
                    self.origins.bind(&alias.name, &alias.value, path, None)?;
                }
                let ownership_guard = self.origins.snapshot_owned_bindings();
                let guard = if self.cursor.eat("if").is_some() {
                    let guard = self.expr(path, Options::default())?;
                    self.origins.direct(&guard, path, guard.span)?;
                    Some(guard)
                } else {
                    None
                };
                if let Some(guard) = guard.as_ref() {
                    self.origins
                        .check_owned_guard(&ownership_guard, guard.span)?;
                }
                self.cursor.expect("=>", Some("match arm"))?;
                let body = self.expr(path, Options::default())?;
                let origin = self.origins.transfer_owned_result(&body, path)?;
                self.origins.expire_owned_bindings(&ownership_base, &origin);
                ownership_arms.push((self.origins.snapshot_owned_bindings(), !origin.diverging));
                Ok::<_, crate::translation::diagnostics::TranslationError>((body, origin, guard))
            })();
            self.origins.scope = previous;
            let (mut body, body_origin, guard) = parsed?;
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
            guarded_arms.push(conditional_chains::GuardArm {
                patterns: alternatives.clone(),
                body: body.clone(),
                aliases: aliases.clone(),
                guard,
                origin: body_origin.clone(),
                span: span(&arm_start, self.cursor.peek()),
            });
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
                span(token, self.cursor.peek()),
            ));
        }
        let mut ownership_joined = std::collections::HashMap::new();
        let mut ownership_returns = false;
        for (states, returns) in ownership_arms {
            self.origins.join_owned_bindings(
                &ownership_joined,
                &states,
                ownership_returns,
                returns,
            );
            ownership_joined = self.origins.snapshot_owned_bindings();
            ownership_returns |= returns;
        }
        if guarded_arms.iter().any(|arm| arm.guard.is_some()) {
            return self.lower_guarded_match(
                scrutinee,
                guarded_arms,
                path,
                span(token, self.cursor.peek()),
            );
        }
        let origins = rows
            .iter()
            .map(|row| self.origins.origin(&row.body, path))
            .collect::<Vec<_>>();
        let origin = self.origins.checked_join(&origins)?;
        let result = SExpr::new(
            SNode::Match {
                scrutinees: vec![scrutinee],
                rows,
            },
            span(token, self.cursor.peek()),
        );
        self.origins.mark(&result, origin);
        self.origins.transfer_owned_result(&result, path)?;
        Ok(result)
    }

    /// Patterns: `_`, bindings, literals, `Enum::Variant(p, …)`, `&p` and `name @ p`.
    #[allow(clippy::too_many_lines)] // one branch per pattern form, as in rust.js
    fn numeric_pattern(&self, token: &Token, first: &Token, negative: bool) -> Result<SPattern> {
        if self.cursor.is("..") || self.cursor.is("..=") {
            return Err(unsupported(
                "range pattern",
                "ranges are outside the portable pattern language",
                span(first, self.cursor.peek()),
            ));
        }
        let lexical_error=crate::translation::rust_numeric_pattern_rules::read_rust_source_numeric_pattern_lexical_error(&token.suffix,token.floating);
        if !lexical_error.is_empty() {
            return Err(unsupported(
                "Rust numeric pattern",
                &lexical_error,
                span(first, token),
            ));
        }
        let owner =
            crate::translation::rust_numeric_pattern_rules::read_rust_source_numeric_pattern_owner(
                &token.suffix,
                token.floating,
            );
        let node = if owner.is_empty() {
            SPatternNode::NumLit {
                value: token.value.clone(),
                negative,
            }
        } else {
            let ty = if owner == "f64" {
                Some(super::super::FLOAT)
            } else {
                super::super::rust_fixed_type(&owner)
            };
            let Some(ty) = ty else {
                return Err(unsupported(
                    &format!("literal suffix {owner}"),
                    "portable numeric literals are integers and IEEE-754 binary64 values",
                    span(first, token),
                ));
            };
            SPatternNode::TypedNumLit {
                value: token.value.clone(),
                ty,
                owner,
                negative,
            }
        };
        Ok(SPattern {
            node,
            span: span(first, token),
        })
    }

    pub(in super::super) fn pattern(
        &mut self,
        path: &[String],
        aliases: &mut Vec<Alias>,
    ) -> Result<SPattern> {
        self.split_reference_type_token();
        let token = self.peek();
        if self.cursor.eat("&").is_some() {
            let pattern = self.pattern(path, aliases)?;
            self.origins.reference_pattern(&pattern);
            return Ok(pattern);
        }
        if self.cursor.eat("_").is_some() {
            let span = if self.record_pattern_depth == 0 {
                self.origins.wildcard_pattern();
                None
            } else {
                Some(token.span())
            };
            return Ok(SPattern {
                node: SPatternNode::Wild,
                span,
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
            return self.numeric_pattern(&number, &token, true);
        }
        if token.kind == TokenKind::Number {
            self.cursor.advance();
            return self.numeric_pattern(&token, &token, false);
        }
        if crate::translation::rust_byte_pattern_rules::accept_rust_source_byte_pattern_token(
            if token.kind == TokenKind::ByteCharacter {
                "byteChar"
            } else {
                ""
            },
        ) {
            self.cursor.advance();
            return Ok(SPattern {
                node: SPatternNode::ByteLit {
                    value: token.value.clone(),
                },
                span: span(&token, &token),
            });
        }
        if matches!(token.kind, TokenKind::Character | TokenKind::ByteCharacter) {
            return Err(unsupported(
                "character pattern",
                "characters are outside the portable core",
                span(&token, &token),
            ));
        }
        if crate::translation::rust_string_pattern_rules::accept_rust_source_string_pattern_token(
            if token.kind == TokenKind::String {
                "string"
            } else {
                ""
            },
        ) {
            self.cursor.advance();
            return Ok(SPattern {
                node: SPatternNode::StrLit {
                    value: token.value.clone(),
                },
                span: span(&token, &token),
            });
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
        if self.cursor.eat("(").is_some() {
            self.record_pattern_depth += 1;
            let result = (|| -> Result<(Vec<SPattern>, Option<usize>, bool)> {
                let mut args = Vec::new();
                let mut rest = None;
                let mut comma = false;
                while !self.cursor.is(")") {
                    if self.cursor.eat("..").is_some() {
                        if rest.is_some() {
                            return Err(unsupported(
                                "tuple rest pattern",
                                "a tuple pattern has at most one rest position",
                                span(&token, self.cursor.peek()),
                            ));
                        }
                        rest = Some(args.len());
                    } else {
                        args.push(self.pattern(path, aliases)?);
                    }
                    if self.cursor.eat(",").is_none() {
                        break;
                    }
                    comma = true;
                }
                self.cursor.expect(")", Some("tuple pattern"))?;
                Ok((args, rest, comma))
            })();
            self.record_pattern_depth -= 1;
            let (mut args, rest, comma) = result?;
            if args.len() == 1 && rest.is_none() && !comma {
                return Ok(args.remove(0));
            }
            return Ok(SPattern {
                node: SPatternNode::Tuple { args, rest },
                span: span(&token, self.cursor.peek()),
            });
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
        if self
            .origins
            .lookup(&self.source_value_path(&segments, path), path)
            .is_some_and(|entry| {
                matches!(
                    entry.declaration,
                    super::super::SourceDeclaration::Constant { .. }
                )
            })
        {
            return Err(unsupported("Rust constant pattern",crate::translation::rust_source_constant_rules::read_rust_source_constant_pattern_error("const"),span(&token,self.cursor.peek())));
        }

        if self.cursor.eat("@").is_some() {
            if segments.len() != 1 {
                return Err(Self::fail("expected a binding before @", &token));
            }
            self.require_nonconstant_binding(&segments[0], path, Some(token.span()))?;
            let inner = self.pattern(path, aliases)?;
            if self.ordinary_completion_context() {
                return Ok(SPattern {
                    node: SPatternNode::Alias {
                        name: segments[0].clone(),
                        pattern: Box::new(inner),
                    },
                    span: span(&token, self.cursor.peek()),
                });
            }
            let value = pattern_value(&inner, span(&token, self.cursor.peek()))?;
            self.origins.alias_literal_pattern(&inner, &value);
            aliases.push(Alias {
                name: segments[0].clone(),
                value,
            });
            return Ok(inner);
        }
        if self.cursor.is("{") {
            return self.record_pattern(&segments, path, aliases, &token);
        }
        let resolved = self
            .record_value_path(&segments, path, self.cursor.is("("))
            .unwrap_or_else(|| {
                self.resolve(
                    &self.resolve_type_alias_path(&segments, path, "associated value"),
                    path,
                )
            });
        if self.cursor.eat("(").is_some() {
            let mut args = Vec::new();
            let mut rest = None;
            while !self.cursor.is(")") {
                if self.cursor.is("..") {
                    if rest.is_some() {
                        return Err(unsupported(
                            "constructor rest pattern",
                            "a constructor pattern has at most one rest position",
                            span(&token, self.cursor.peek()),
                        ));
                    }
                    rest = Some((args.len(), self.cursor.advance()));
                } else {
                    args.push(self.pattern(path, aliases)?);
                }
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
            self.cursor.expect(")", Some("pattern"))?;
            let args = self.expand_constructor_rest(&resolved, args, rest, path)?;
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
                node: if token.raw.starts_with("r#") {
                    SPatternNode::Bind {
                        name: segments[0].clone(),
                    }
                } else {
                    SPatternNode::BindOrCtor {
                        name: segments[0].clone(),
                    }
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
