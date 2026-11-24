impl RustParser {
    /// A block `{ let x = e; …; tail }` is a chain of lets ending in its tail
    /// expression. Statements with effects are not values of the portable core.
    pub(super) fn block(&mut self, path: &[String]) -> Result<SExpr> {
        self.block_with_context(path, false)
    }
    pub(super) fn is_terminal_return(&self) -> bool {
        let mut count = 64usize;
        loop {
            let values = self
                .cursor
                .tokens
                .iter()
                .skip(self.cursor.index)
                .take(count)
                .map(|token| {
                    if token.kind == TokenKind::Punct {
                        token.value.clone()
                    } else {
                        String::new()
                    }
                })
                .collect::<Vec<_>>();
            let boundary =
                crate::translation::rust_unit_value_rules::find_rust_terminal_return_boundary(
                    &values, 0.0,
                );
            if boundary != -2.0 {
                return boundary >= 0.0;
            }
            if count >= self.cursor.tokens.len() - self.cursor.index {
                return false;
            }
            count = count.saturating_mul(2);
        }
    }
    pub(super) fn block_with_context(
        &mut self,
        path: &[String],
        function_body: bool,
    ) -> Result<SExpr> {
        let frame = self.begin_owned_record_state_block(function_body);
        let result = self.read_record_state_block(path, function_body);
        self.end_owned_record_state_block(frame)?;
        result
    }

    pub(super) fn read_record_state_block(
        &mut self,
        path: &[String],
        function_body: bool,
    ) -> Result<SExpr> {
        let open = self.cursor.expect("{", Some("block"))?;
        let ownership_before = self.origins.snapshot_owned_bindings();
        let previous = self.origins.enter(None);
        let mut lets = Vec::new();
        let ordinary = self.ordinary_completion_context();
        let mut tail: Option<SExpr> = None;
        while !self.cursor.is("}") {
            if self.cursor.eat(";").is_some() {
                continue;
            }
            let token = self.peek();
            if tail
                .as_ref()
                .is_some_and(|value| super::expressions::source_completion_kind(value) == "abort")
            {
                return Err(unsupported(
                    "unreachable continuation",
                    "statements after an abort are not removed by this model",
                    span(&token, &token),
                ));
            }
            if self.cursor.is("let") {
                lets.extend(self.let_statement(path)?);
                continue;
            }
            if token.kind == TokenKind::Macro
                && crate::translation::rust_unit_value_rules::read_rust_function_macro_disposition(
                    &token.value,
                ) == "print"
            {
                let printed = self.function_print_value(path)?;
                let place = printed.span;
                if self.cursor.eat(";").is_some() {
                    lets.push(Binding {
                        name: self.fresh_product_binding(),
                        ty: None,
                        value: printed,
                        span: place,
                    });
                    continue;
                }
                if !self.cursor.is("}") {
                    return Err(Self::fail("expected ; or }", self.cursor.peek()));
                }
                tail = Some(printed);
                break;
            }
            if function_body
                && self.cursor.is("return")
                && (self.cursor.is_at(";", 1)
                    || self.cursor.is_at("}", 1)
                    || self.is_terminal_return())
            {
                self.cursor.advance();
                tail = Some(if self.cursor.is(";") || self.cursor.is("}") {
                    SExpr::new(SNode::Unit, span(&token, &token))
                } else {
                    self.expr(path, Options::default())?
                });
                while self.cursor.eat(";").is_some() {}
                if !self.cursor.is("}") {
                    return Err(unsupported(
                        "unreachable continuation",
                        "statements after a completed return are not removed by this model",
                        span(&token, self.cursor.peek()),
                    ));
                }
                break;
            }
            if self.cursor.is("return") {
                return Err(unsupported(
                    "early return",
                    "write the result as the tail expression of the block",
                    span(&token, &token),
                ));
            }
            let temporary_frame = self.origins.owned_temporary_frame();
            let finite_loop = if ordinary {
                self.read_owned_record_finite_loop(path)?
            } else {
                None
            };
            if finite_loop.is_none()
                && token.kind == TokenKind::Identifier
                && ["while", "loop", "for"].contains(&token.value.as_str())
            {
                return Err(unsupported(
                    &format!("{} loop", token.value),
                    "loops over mutable state are outside the portable core; use recursion",
                    span(&token, &token),
                ));
            }
            let expr = if let Some(loop_expression) = finite_loop {
                loop_expression
            } else if let Some(statement) = self.read_owned_record_state_statement(path)? {
                lets.extend(statement.bindings);
                statement.value
            } else {
                self.expr(
                    path,
                    Options {
                        statement: true,
                        no_struct: false,
                    },
                )?
            };
            if self.cursor.eat(";").is_some() {
                self.origins
                    .finish_owned_expression(&expr, path, &temporary_frame, false);
                let origin = self.origins.origin(&expr, path);
                let disposition = (if ordinary {
                    crate::translation::rust_unit_value_rules::read_rust_ordinary_statement_disposition
                } else {
                    crate::translation::rust_unit_value_rules::read_rust_unit_statement_disposition
                })(
                    &super::expressions::source_completion_kind(&expr),
                    self.origins.collecting,
                    origin.proven,
                    origin.ty.as_ref().map_or("", Type::kind),
                    !origin.wrappers.is_empty(),
                );
                if disposition == "abort" {
                    tail = Some(expr);
                    continue;
                }
                if disposition == "bind" || self.is_collection_mutation_statement(&expr) {
                    self.origins.consume_owned_binding(&origin, expr.span)?;
                    lets.push(Binding {
                        name: self.fresh_product_binding(),
                        ty: None,
                        span: expr.span,
                        value: expr,
                    });
                    continue;
                }
                return Err(unsupported(
                    "expression statement",
                    "statements with effects are outside the portable core in function bodies",
                    expr.span,
                ));
            }
            if ordinary {
                let origin = self.origins.origin(&expr, path);
                let implicit=crate::translation::rust_unit_value_rules::read_rust_implicit_block_statement_disposition(expr.node.kind(),expr.block,!self.cursor.is("}"),self.origins.collecting,origin.proven,origin.ty.as_ref().map_or("",Type::kind),!origin.wrappers.is_empty());
                if implicit == "bind" {
                    lets.push(Binding {
                        name: self.fresh_product_binding(),
                        ty: Some(super::UNIT),
                        span: expr.span,
                        value: expr,
                    });
                    continue;
                }
            }
            let ends_block_statement = is_if_or_match(&expr);
            let expr_span = expr.span;
            tail = Some(expr);
            if !self.cursor.is("}") {
                if ends_block_statement {
                    return Err(unsupported(
                        "expression statement",
                        "a block value must be its last expression",
                        expr_span,
                    ));
                }
                return Err(Self::fail("expected }", self.cursor.peek()));
            }
        }
        let close = self.cursor.expect("}", Some("block"))?;
        let tail = tail.unwrap_or_else(|| SExpr::new(SNode::Unit, span(&open, &close)));
        let origin = self.origins.transfer_owned_result(&tail, path)?;
        let result = lets.into_iter().rev().fold(tail, |body, binding| {
            SExpr::new(
                SNode::Let {
                    name: binding.name,
                    ty: binding.ty.map(Into::into),
                    value: Box::new(binding.value),
                    body: Box::new(body),
                },
                binding.span,
            )
        });
        self.origins.mark(&result, origin);
        let mut origin = self.origins.transfer_owned_result(&result, path)?;
        self.origins
            .expire_owned_bindings(&ownership_before, &origin)?;
        // Borrowing a block borrows the result value. References contained in
        // that result retain their original lifetime evidence.
        origin.source_lexical_binding_identity = None;
        if origin.wrappers.is_empty() && !matches!(origin.ty, Some(Type::Array { .. })) {
            origin.source_place_binding_identity = None;
            origin.source_place_loan_binding_identities.clear();
        }
        self.origins.mark(&result, origin);
        self.origins.scope = previous;
        Ok(result)
    }

    pub(super) fn main_fn(&mut self) -> Result<SMain> {
        if !self.pure_unit_call_model.is_empty() {
            return self.pure_unit_main();
        }
        let frame = self.begin_owned_record_state_block(true);
        let result = self.read_record_state_main();
        self.end_owned_record_state_block(frame)?;
        result
    }

    fn read_record_state_main(&mut self) -> Result<SMain> {
        let start = self.cursor.advance();
        self.cursor.expect("main", Some("fn main"))?;
        self.cursor.expect("(", Some("fn main"))?;
        self.cursor.expect(")", Some("fn main"))?;
        if self.cursor.is("->") {
            return Err(unsupported(
                "main with a result",
                "main must return ()",
                span(&start, self.cursor.peek()),
            ));
        }
        if self.header_discovery {
            let close = self.declaration_body_boundary()?;
            return Ok(SMain {
                effects: Vec::new(),
                span: Some(super::Span::new(start.start, close.end)),
                sequential_async: false,
                source_runtime_worker: None,
            });
        }
        self.module_path = Vec::new();
        let previous = self.origins.enter(Some(std::collections::HashMap::new()));
        self.cursor.expect("{", Some("fn main"))?;
        let mut effects = Vec::new();
        while !self.cursor.is("}") {
            if self.cursor.eat(";").is_some() {
                continue;
            }
            let token = self.peek();
            if self.cursor.is("let") {
                for binding in self.let_statement(&[])? {
                    effects.push(SEffect::Let {
                        name: binding.name,
                        ty: binding.ty,
                        value: binding.value,
                        constant: false,
                        span: binding.span,

                        source_pure_unit_call: None,
                    });
                }
                continue;
            }
            if token.kind == TokenKind::Macro {
                effects.push(self.main_macro()?);
                self.cursor.expect(";", Some(&token.value))?;
                continue;
            }
            let finite_loop = if self.ordinary_completion_context() {
                self.read_owned_record_finite_loop(&[])?
            } else {
                None
            };
            let state_statement = if finite_loop.is_some() {
                None
            } else {
                self.read_owned_record_state_statement(&[])?
            };
            let is_state_statement = finite_loop.is_some() || state_statement.is_some();
            let value = if let Some(loop_expression) = finite_loop {
                loop_expression
            } else if let Some(statement) = state_statement {
                for binding in statement.bindings {
                    effects.push(SEffect::Let {
                        name: binding.name,
                        ty: binding.ty,
                        value: binding.value,
                        span: binding.span,
                        constant: false,
                        source_pure_unit_call: None,
                    });
                }
                statement.value
            } else {
                self.expr(
                    &[],
                    Options {
                        statement: true,
                        no_struct: false,
                    },
                )?
            };
            let origin = self.origins.origin(&value, &[]);
            if crate::translation::rust_unit_value_rules::read_rust_unit_statement_disposition(
                value.node.kind(),
                self.origins.collecting,
                origin.proven,
                origin.ty.as_ref().map_or("", Type::kind),
                !origin.wrappers.is_empty(),
            ) != "bind"
                && !(self.cursor.is(";") && self.is_collection_mutation_statement(&value))
            {
                return Err(unsupported(
                    "main statement",
                    "main may only print, bind, assert and evaluate direct Unit calls",
                    span(&token, &token),
                ));
            }
            if self.cursor.eat(";").is_none()
                && !self.cursor.is("}")
                && !(is_state_statement && value.block)
            {
                return Err(Self::fail("expected ; or }", self.cursor.peek()));
            }
            effects.push(SEffect::Let {
                name: self.fresh_product_binding(),
                ty: None,
                span: value.span,
                value,
                constant: false,
                source_pure_unit_call: None,
            });
        }
        let close = self.cursor.expect("}", Some("fn main"))?;
        self.origins.scope = previous;
        Ok(SMain {
            effects,
            span: if self.empty_option_source_model.is_empty() {
                span(&start, self.cursor.peek())
            } else {
                Some(super::Span {
                    start: start.start,
                    end: close.end,
                })
            },
            sequential_async: false,
            source_runtime_worker: None,
        })
    }

    pub(super) fn main_macro(&mut self) -> Result<SEffect> {
        self.main_macro_with_path(&[])
    }
    pub(super) fn function_print_value(&mut self, path: &[String]) -> Result<SExpr> {
        let SEffect::Print {
            expr,
            style,
            span: place,
        } = self.main_macro_with_path(path)?
        else {
            unreachable!("source print macro disposition")
        };
        let unit = SExpr::new(SNode::Unit, place);
        let origin = self.origins.origin(&unit, path);
        let printed = SExpr::new(
            SNode::Print {
                expr: Box::new(expr),
                style,
                body: Box::new(unit),
            },
            place,
        );
        self.origins.mark(&printed, origin);
        Ok(printed)
    }
    fn main_macro_with_path(&mut self, path: &[String]) -> Result<SEffect> {
        let token = self.cursor.advance();
        let (mut args, argument_names) = if token.value == "println" {
            let (args, names) = self.formatting_arguments(&token, path)?;
            (args, Some(names))
        } else {
            (self.macro_arguments(&token, path)?, None)
        };
        if ["println", "assert", "assert_eq", "assert_ne"].contains(&token.value.as_str()) {
            for argument in &args {
                self.guard_borrowed_vector_observation(
                    argument,
                    path,
                    &token.value,
                    argument.span,
                )?;
            }
        }
        match token.value.as_str() {
            "println" => {
                for value in args.iter().skip(1) {
                    self.refuse_collection_observation(value, path, "formatting", false)?;
                    self.refuse_shared_value_observation(value, None, path, value.span)?;
                }
                let expr = if args.is_empty() {
                    SExpr::new(
                        SNode::Str {
                            value: String::new(),
                        },
                        None,
                    )
                } else {
                    self.format(args, &token, path, argument_names)?
                };
                Ok(SEffect::Print {
                    expr,
                    style: ShowStyle::Rust,
                    span: span(&token, self.cursor.peek()),
                })
            }
            "assert" => {
                if args.len() != 1 {
                    return Err(unsupported(
                        "assert! message",
                        "custom assertion messages are not kept",
                        span(&token, self.cursor.peek()),
                    ));
                }
                Ok(SEffect::Assert {
                    prop: prop_of(args.remove(0)),
                    span: span(&token, self.cursor.peek()),
                })
            }
            "assert_eq" | "assert_ne" => {
                if args.len() != 2 {
                    return Err(unsupported(
                        &format!("{}! message", token.value),
                        "custom assertion messages are not kept",
                        span(&token, self.cursor.peek()),
                    ));
                }
                let right = args.remove(1);
                let left = args.remove(0);
                let operation = if token.value == "assert_eq" {
                    super::BinaryOp::Eq
                } else {
                    super::BinaryOp::Ne
                };
                let location = span(&token, self.cursor.peek());
                if let Some(value) = self.standard_collection_comparison(
                    &left, &right, operation, false, path, location,
                )? {
                    return Ok(SEffect::Assert {
                        prop: prop_of(value),
                        span: location,
                    });
                }
                self.refuse_collection_observation(&left, path, "assertion comparison", false)?;
                self.refuse_collection_observation(&right, path, "assertion comparison", false)?;
                let comparison = SComparison {
                    left,
                    right,
                    reference: false,
                    same_value: false,
                };
                let node = if token.value == "assert_eq" {
                    SPropNode::Eq(comparison)
                } else {
                    SPropNode::Ne(comparison)
                };
                Ok(SEffect::Assert {
                    prop: SProp { node, span: None },
                    span: span(&token, self.cursor.peek()),
                })
            }
            _ => Err(unsupported(
                &format!("{}!", token.value),
                "main may only use println!, assert!, assert_eq! and assert_ne!",
                span(&token, &token),
            )),
        }
    }

    /// Macro arguments, parsed as expressions; the first one of a format macro is its literal.
    pub(super) fn macro_arguments(&mut self, token: &Token, path: &[String]) -> Result<Vec<SExpr>> {
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
        while !self.cursor.is(close) {
            let value = self.expr(path, Options::default())?;
            if crate::translation::rust_owned_binding_rules::accept_rust_owned_macro_argument_transfer(&token.value){let origin=self.origins.origin(&value,path);self.origins.consume_owned_binding(&origin,value.span)?;}
            args.push(value);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor
            .expect(close, Some(&format!("{}!", token.value)))?;
        Ok(args)
    }
}
