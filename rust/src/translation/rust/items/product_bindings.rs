//! Irrefutable positional bindings lower to ordinary ordered value bindings.

use super::super::{
    Binding, Options, ProductBindingPattern, Result, RustParser, SExpr, SNode, SourceOrigin, Span,
    Type, UNIT, span, type_error, unsupported,
};

impl RustParser {
    pub(in super::super) fn let_statement(&mut self, path: &[String]) -> Result<Vec<Binding>> {
        let start = self.cursor.advance();
        if self.cursor.is("mut") {
            return Err(unsupported(
                "mutable binding",
                "mutation is outside the portable core",
                span(&start, self.cursor.peek()),
            ));
        }
        let pattern = if let Some(pattern) = self.let_else_pattern(path)? {
            ProductBindingPattern::Nominal(pattern)
        } else {
            self.product_binding_pattern()?
        };
        let mut names = std::collections::HashSet::new();
        self.validate_product_binding_names(&pattern, &mut names)?;
        let ty = if self.cursor.eat(":").is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };
        let source_type = ty.as_ref().map(|_| self.origins.current_type.clone());
        if self.cursor.eat("=").is_none() {
            return Err(unsupported(
                "uninitialised binding",
                "every binding needs its value",
                span(&start, self.cursor.peek()),
            ));
        }
        let initializer_start = self.cursor.index;
        let value = self.expr(path, Options::default())?;
        if self.cursor.is("else") {
            let tokens = &self.cursor.tokens[initializer_start..self.cursor.index];
            let kinds = tokens
                .iter()
                .map(|token| {
                    if token.kind == super::super::TokenKind::Punct {
                        "punct".to_owned()
                    } else {
                        "atom".to_owned()
                    }
                })
                .collect::<Vec<_>>();
            let spellings = tokens
                .iter()
                .map(|token| token.value.clone())
                .collect::<Vec<_>>();
            let reason =
                crate::translation::rust_record_rules::read_rust_let_else_initializer_error(
                    &kinds, &spellings,
                );
            if !reason.is_empty() {
                return Err(unsupported(
                    "Rust let-else initializer",
                    &reason,
                    span(&start, self.cursor.peek()),
                ));
            }
        }
        let fallback = if self.cursor.eat("else").is_some() {
            let body = self.block(path)?;
            if !self.origins.origin(&body, path).diverging {
                return Err(unsupported(
                    "Rust let-else fallback",
                    "the fallback needs a represented diverging completion",
                    body.span,
                ));
            }
            Some(body)
        } else {
            None
        };
        self.cursor.expect(";", Some("let"))?;
        let range = span(&start, self.cursor.peek());
        if let ProductBindingPattern::Name(token) = pattern {
            self.origins
                .bind(&token.value, &value, path, source_type.as_ref())?;
            return Ok(vec![Binding {
                name: token.value,
                ty,
                value,
                span: range,
            }]);
        }
        if let Some(fallback) = fallback {
            let ProductBindingPattern::Nominal(pattern) = pattern else {
                return Err(unsupported(
                    "let-else",
                    "refutable bindings are outside the portable core",
                    range,
                ));
            };
            let name = self.fresh_product_binding();
            self.origins
                .bind(&name, &value, path, source_type.as_ref())?;
            let reference = SExpr::new(
                SNode::Name {
                    path: vec![name.clone()],
                },
                range,
            );
            let mut bindings = vec![Binding {
                name,
                ty,
                value,
                span: range,
            }];
            self.lower_nominal_binding(pattern, reference, path, &mut bindings, Some(fallback))?;
            return Ok(bindings);
        }
        self.lower_product_binding(pattern, value, ty, source_type.as_ref(), path, range)
    }

    /// Read irrefutable positional patterns without introducing a pattern core.
    pub(super) fn product_binding_pattern(&mut self) -> Result<ProductBindingPattern> {
        let token = self.peek();
        if self.cursor.eat("(").is_some() {
            if let Some(close) = self.cursor.eat(")") {
                return Ok(ProductBindingPattern::Unit {
                    span: Some(Span::new(token.start, close.end)),
                });
            }
            let mut items = Vec::new();
            let mut rest = None;
            let mut comma = false;
            while !self.cursor.is(")") {
                if self.cursor.eat("..").is_some() {
                    if rest.is_some() {
                        return Err(unsupported(
                            "tuple rest binding",
                            "a tuple binding has at most one rest position",
                            span(&token, self.cursor.peek()),
                        ));
                    }
                    rest = Some(items.len());
                } else {
                    items.push(self.product_binding_pattern()?);
                }
                if self.cursor.eat(",").is_none() {
                    break;
                }
                comma = true;
            }
            let close = self.cursor.expect(")", Some("tuple binding"))?;
            if items.len() == 1 && rest.is_none() && !comma {
                return Ok(items.remove(0));
            }
            return Ok(ProductBindingPattern::Tuple {
                items,
                rest,
                span: Some(Span::new(token.start, close.end)),
            });
        }
        if self.cursor.is("mut") {
            return Err(unsupported(
                "mutable tuple binding",
                "mutation is outside the immutable product model",
                span(&token, &token),
            ));
        }
        if self.cursor.is("ref") || self.cursor.is("&") {
            return Err(unsupported(
                "reference tuple binding",
                "product bindings must retain direct Copy values",
                span(&token, &token),
            ));
        }
        if let Some(pattern) = self.nominal_binding_pattern()? {
            return Ok(ProductBindingPattern::Nominal(pattern));
        }
        let name = self.cursor.identifier(Some("let"))?;
        if !name.raw.starts_with("r#") && (name.value == "true" || name.value == "false") {
            return Err(unsupported(
                "refutable product binding",
                "only immutable names, ignored fields and anonymous tuple products are admitted",
                Some(name.span()),
            ));
        }
        if self.cursor.is("::") || self.cursor.is("(") || self.cursor.is("{") || self.cursor.is("@")
        {
            return Err(unsupported(
                "refutable product binding",
                "only immutable names, ignored fields and anonymous tuple products are admitted",
                span(&token, self.cursor.peek()),
            ));
        }
        Ok(if name.value == "_" {
            ProductBindingPattern::Wildcard(name)
        } else {
            ProductBindingPattern::Name(name)
        })
    }

    pub(in crate::translation::rust) fn validate_product_binding_names(
        &self,
        pattern: &ProductBindingPattern,
        names: &mut std::collections::HashSet<String>,
    ) -> Result<()> {
        match pattern {
            ProductBindingPattern::Name(token) => {
                if !names.insert(token.value.clone()) {
                    return Err(unsupported(
                        "duplicate tuple binding",
                        "each name occurs only once in a destructuring pattern",
                        Some(token.span()),
                    ));
                }
            }
            ProductBindingPattern::Tuple { items, .. } => {
                for item in items {
                    self.validate_product_binding_names(item, names)?;
                }
            }
            ProductBindingPattern::Wildcard(_) | ProductBindingPattern::Unit { .. } => {}
            ProductBindingPattern::Nominal(pattern) => {
                for (name, range) in self.nominal_binding_names(pattern, false)? {
                    if !names.insert(name) {
                        return Err(unsupported(
                            "duplicate tuple binding",
                            "each name occurs only once in a destructuring pattern",
                            range,
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    pub(in super::super) fn fresh_product_binding(&mut self) -> String {
        loop {
            let name = format!("ml_tuple_value{}", self.next_product_binding);
            self.next_product_binding += 1;
            if self.reserved_binding_names.insert(name.clone()) {
                return name;
            }
        }
    }

    pub(super) fn lower_product_binding(
        &mut self,
        pattern: ProductBindingPattern,
        value: SExpr,
        ty: Option<Type>,
        source_type: Option<&SourceOrigin>,
        path: &[String],
        range: Option<Span>,
    ) -> Result<Vec<Binding>> {
        let name = self.fresh_product_binding();
        self.origins.bind(&name, &value, path, source_type)?;
        let origin = self.origins.scope[&name].clone();
        let mut bindings = vec![Binding {
            name: name.clone(),
            ty,
            value,
            span: range,
        }];
        let reference = SExpr::new(SNode::Name { path: vec![name] }, range);
        self.lower_product_pattern(pattern, reference, &origin, path, &mut bindings)?;
        Ok(bindings)
    }

    fn lower_product_pattern(
        &mut self,
        pattern: ProductBindingPattern,
        value: SExpr,
        origin: &SourceOrigin,
        path: &[String],
        bindings: &mut Vec<Binding>,
    ) -> Result<()> {
        match pattern {
            ProductBindingPattern::Nominal(pattern) => {
                self.lower_nominal_binding(pattern, value, path, bindings, None)?;
            }
            ProductBindingPattern::Name(token) => {
                self.origins.bind(&token.value, &value, path, None)?;
                bindings.push(Binding {
                    name: token.value.clone(),
                    ty: None,
                    value,
                    span: Some(token.span()),
                });
            }
            ProductBindingPattern::Wildcard(_) => {}
            ProductBindingPattern::Unit { span } => {
                if !self.origins.collecting
                    && (origin.ty != Some(UNIT) || !origin.direct_copy_value())
                {
                    return Err(unsupported(
                        "unit product binding",
                        "the unit pattern requires a direct unit source value",
                        span,
                    ));
                }
            }
            ProductBindingPattern::Tuple { items, rest, span } => {
                self.origins.require_product_origin(origin, span)?;
                let count = |value| {
                    f64::from(
                        u32::try_from(value)
                            .expect("source tuple width fits the shared array limit"),
                    )
                };
                let width = origin.fields.as_ref().map_or(items.len(), Vec::len);
                let plan = crate::translation::rust_record_rules::plan_tuple_pattern_fields(
                    count(width),
                    count(items.len()),
                    rest.map_or(-1.0, count),
                );
                if !self.origins.collecting && plan.first() != Some(&0.0) {
                    return Err(type_error(
                        "tuple binding has the wrong number of fields",
                        span,
                    ));
                }
                let mut positions = (0..items.len()).collect::<Vec<_>>();
                if !self.origins.collecting {
                    for (position, index) in plan.into_iter().skip(1).enumerate() {
                        if index >= 0.0 {
                            let index = index.to_string().parse::<usize>().map_err(|_| {
                                type_error("tuple binding plan has an invalid source index", span)
                            })?;
                            positions[index] = position;
                        }
                    }
                }
                for (source, item) in items.into_iter().enumerate() {
                    let index = positions[source];
                    let range = item.source_span();
                    let projection = SExpr::new(
                        SNode::TupleIndex {
                            object: Box::new(value.clone()),
                            index,
                        },
                        range,
                    );
                    self.origins.product_projection(&projection, path, range)?;
                    let field = self.origins.origin(&projection, path);
                    self.lower_product_pattern(item, projection, &field, path, bindings)?;
                }
            }
        }
        Ok(())
    }
}
