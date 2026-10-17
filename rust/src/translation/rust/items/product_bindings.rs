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
        let pattern = self.product_binding_pattern()?;
        let mut names = std::collections::HashSet::new();
        Self::validate_product_binding_names(&pattern, &mut names)?;
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
        let value = self.expr(path, Options::default())?;
        if self.cursor.is("else") {
            return Err(unsupported(
                "let-else",
                "refutable bindings are outside the portable core",
                span(&start, self.cursor.peek()),
            ));
        }
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
            let first = self.product_binding_pattern()?;
            if self.cursor.eat(",").is_none() {
                self.cursor.expect(")", Some("parenthesised binding"))?;
                return Ok(first);
            }
            let mut items = vec![first];
            while !self.cursor.is(")") {
                items.push(self.product_binding_pattern()?);
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
            let close = self.cursor.expect(")", Some("tuple binding"))?;
            return Ok(ProductBindingPattern::Tuple {
                items,
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
        let name = self.cursor.identifier(Some("let"))?;
        if name.value == "true" || name.value == "false" {
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

    pub(super) fn validate_product_binding_names(
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
                    Self::validate_product_binding_names(item, names)?;
                }
            }
            ProductBindingPattern::Wildcard(_) | ProductBindingPattern::Unit { .. } => {}
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
            ProductBindingPattern::Tuple { items, span } => {
                self.origins.require_product_origin(origin, span)?;
                if !self.origins.collecting
                    && origin
                        .fields
                        .as_ref()
                        .is_none_or(|fields| fields.len() != items.len())
                {
                    return Err(type_error(
                        "tuple binding has the wrong number of fields",
                        span,
                    ));
                }
                for (index, item) in items.into_iter().enumerate() {
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
