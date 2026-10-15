//! Ordered source statements; expressions still use the production Rust parser.
use super::{
    LocalBindingPattern, LocalPattern, LocalState, Options, Result, RustParser, SExpr, SNode, Span,
    Statement, TokenKind, fail, span,
};
impl RustParser {
    fn owned_local_raw_expression(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let start = self.cursor.index;
        let value = self.expr(path, options)?;
        if self.cursor.tokens[start..self.cursor.index]
            .iter()
            .any(|token| {
                token.kind == TokenKind::Identifier
                    && ["mut", "ref"].contains(&token.value.as_str())
                    || token.kind == TokenKind::Punct && ["&", "&&"].contains(&token.value.as_str())
            })
        {
            return Err(fail(
                "references and mutable expression bindings need a separate ownership interpretation",
                value.span,
            ));
        }
        Ok(value)
    }
    fn owned_local_pattern_name(&mut self) -> Result<LocalPattern> {
        let token = self.peek();
        let mutable = self.cursor.eat("mut").is_some();
        if self.cursor.is("ref") || self.cursor.is("&") {
            return Err(fail(
                "references and mutable expression bindings need a separate ownership interpretation",
                Some(token.span()),
            ));
        }
        let name = self.cursor.identifier(Some("owned local state binding"))?;
        if name.value == "_" || ["true", "false"].contains(&name.value.as_str()) {
            return Err(fail(
                "only direct names and a flat two-field tuple pattern are admitted",
                Some(name.span()),
            ));
        }
        Ok(LocalPattern {
            name: name.value,
            mutable,
            span: Some(Span::new(name.start, name.end)),
        })
    }
    fn owned_local_pattern(&mut self) -> Result<LocalBindingPattern> {
        self.owned_local_pattern_at_depth(0)
    }
    fn owned_local_pattern_at_depth(&mut self, depth: usize) -> Result<LocalBindingPattern> {
        let token = self.peek();
        if depth > 256 {
            return Err(fail(
                "source binding exceeds its structural depth limit",
                Some(token.span()),
            ));
        }
        if self.cursor.eat("(").is_none() {
            return Ok(LocalBindingPattern::Name(self.owned_local_pattern_name()?));
        }
        let first = self.owned_local_pattern_at_depth(depth + 1)?;
        if self.cursor.eat(",").is_none() {
            self.cursor.expect(")", Some("parenthesised binding"))?;
            return Ok(first);
        }
        let mut patterns = vec![first];
        while !self.cursor.is(")") {
            patterns.push(self.owned_local_pattern_at_depth(depth + 1)?);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        let close = self.cursor.expect(")", Some("tuple binding"))?;
        let range = Some(Span::new(token.start, close.end));
        if patterns.len() != 2
            || patterns
                .iter()
                .any(|pattern| !matches!(pattern, LocalBindingPattern::Name(_)))
        {
            return Err(fail(
                "only direct names and a flat two-field tuple pattern are admitted",
                range,
            ));
        }
        let items = patterns
            .into_iter()
            .map(|pattern| {
                let LocalBindingPattern::Name(item) = pattern else {
                    unreachable!()
                };
                item
            })
            .collect();
        Ok(LocalBindingPattern::Tuple { items, span: range })
    }
    pub(super) fn owned_local_syntax_block(
        &mut self,
        path: &[String],
        state: &mut LocalState,
    ) -> Result<Vec<Statement>> {
        self.cursor.expect("{", Some("owned local state block"))?;
        let previous = self.origins.enter(None);
        let result = self.owned_local_statements(path, state);
        self.origins.scope = previous;
        result
    }
    fn owned_local_statements(
        &mut self,
        path: &[String],
        state: &mut LocalState,
    ) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        while !self.cursor.is("}") {
            if self.cursor.is("let") {
                let start = self.cursor.advance();
                let pattern = self.owned_local_pattern()?;
                let ty = if self.cursor.eat(":").is_some() {
                    Some(self.parse_type()?)
                } else {
                    None
                };
                let origin = ty.as_ref().map(|_| self.origins.current_type.clone());
                self.cursor.expect("=", Some("owned local state binding"))?;
                let value = self.owned_local_raw_expression(path, Options::default())?;
                self.cursor.expect(";", Some("owned local state binding"))?;
                match &pattern {
                    LocalBindingPattern::Name(binding) => {
                        self.origins
                            .bind(&binding.name, &value, path, origin.as_ref())?
                    }
                    LocalBindingPattern::Tuple { items, .. } => {
                        let name = state.fresh(self);
                        self.origins.bind(&name, &value, path, origin.as_ref())?;
                        for (index, item) in items.iter().enumerate() {
                            let projection = SExpr::new(
                                SNode::TupleIndex {
                                    object: Box::new(SExpr::new(
                                        SNode::Name {
                                            path: vec![name.clone()],
                                        },
                                        None,
                                    )),
                                    index,
                                },
                                item.span,
                            );
                            self.origins
                                .product_projection(&projection, path, item.span)?;
                            self.origins.bind(&item.name, &projection, path, None)?;
                        }
                    }
                }
                statements.push(Statement::Let {
                    pattern,
                    ty,
                    value,
                    span: span(&start, self.cursor.peek()),
                });
            } else if self.cursor.is("if") {
                let start = self.cursor.advance();
                let condition = self.owned_local_raw_expression(
                    path,
                    Options {
                        no_struct: true,
                        ..Options::default()
                    },
                )?;
                let then = self.owned_local_syntax_block(path, state)?;
                let otherwise = if self.cursor.eat("else").is_some() {
                    self.owned_local_syntax_block(path, state)?
                } else {
                    Vec::new()
                };
                self.cursor.eat(";");
                statements.push(Statement::If {
                    condition,
                    then,
                    otherwise,
                    span: span(&start, self.cursor.peek()),
                });
            } else if self.cursor.is_kind(TokenKind::Identifier) && self.cursor.is_at("=", 1) {
                let target = self.cursor.advance();
                self.cursor.advance();
                let value = self.owned_local_raw_expression(path, Options::default())?;
                self.cursor
                    .expect(";", Some("owned local state replacement"))?;
                self.origins.bind(&target.value, &value, path, None)?;
                let range = span(&target, self.cursor.peek());
                statements.push(Statement::Write {
                    target: target.value,
                    value,
                    span: range,
                });
            } else {
                let value = self.owned_local_raw_expression(path, Options::default())?;
                if self.cursor.eat(";").is_some() {
                    return Err(fail(
                        "expression statements and early completions need their own effect interpretation",
                        value.span,
                    ));
                }
                if !self.cursor.is("}") {
                    return Err(fail(
                        "every block needs one final value after its ordered statements",
                        value.span,
                    ));
                }
                statements.push(Statement::Tail(value));
                break;
            }
        }
        self.cursor.expect("}", Some("owned local state block"))?;
        Ok(statements)
    }
}
