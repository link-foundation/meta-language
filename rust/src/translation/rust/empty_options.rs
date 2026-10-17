//! Actual Rust token and namespace adapters for the closed empty interpretation.
use super::empty_option_rules::{
    accept_rust_empty_option_constructor_owner, accept_rust_empty_option_element,
    accept_rust_empty_option_result_owner,
};
use super::{
    Result, RustParser, SExpr, SFn, SNode, SourceDeclaration, SourceOrigin, Span, Token, Type,
    span, unsupported,
};
use serde_json::{Value, json};

fn source_failure(detail: &str, span: Option<Span>) -> super::TranslationError {
    unsupported("Rust empty option source", detail, span)
}
const fn actual_span(start: &Token, end: &Token) -> Span {
    Span {
        start: start.start,
        end: end.end,
    }
}
impl RustParser {
    fn empty_option_shadowed(&self, path: &[String], construction: bool) -> bool {
        if construction && path.len() == 1 {
            return self.origins.scope.contains_key(&path[0])
                || self.origins.lookup(path, &self.module_path).is_some();
        }
        let resolved = self.resolve(path, &self.module_path);
        self.origins
            .lookup_type_owner(path, &self.module_path)
            .is_some()
            || self
                .origins
                .lookup_type_owner(&resolved, &self.module_path)
                .is_some()
            || path.len() > 1
                && self
                    .origins
                    .lookup_type_owner(&path[..1], &self.module_path)
                    .is_some()
    }
    fn empty_option_owner(&self, path: &[String], construction: bool, span: Span) -> Result<Value> {
        let resolved = self.resolve(path, &self.module_path);
        let shadowed = !self.origins.collecting && self.empty_option_shadowed(path, construction);
        let accepted = if construction {
            accept_rust_empty_option_constructor_owner(&path.join("::"), shadowed)
        } else {
            accept_rust_empty_option_result_owner(&path.join("::"), shadowed)
        };
        if path != resolved.as_slice() || !accepted {
            return Err(source_failure(
                "the actual standard Option or None path and namespace must be unshadowed and unaliased",
                Some(span),
            ));
        }
        let policy = crate::translation::source_empty_options::policy();
        Ok(
            json!({"sourcePath":path,"resolvedPath":resolved,"canonicalPath":policy[if construction{"canonicalConstructor"}else{"canonicalResult"}],"arguments":[]}),
        )
    }
    fn empty_option_argument(&mut self) -> Result<Value> {
        let open = self
            .cursor
            .expect("<", Some("empty option type argument"))?;
        let token = self.cursor.identifier(Some("empty option type argument"))?;
        let actual = self.resolve(std::slice::from_ref(&token.value), &self.module_path);
        if !accept_rust_empty_option_element(
            if token.value == "f64" { "float" } else { "" },
            &actual.join("::"),
            !self.origins.collecting
                && self.empty_option_shadowed(std::slice::from_ref(&token.value), false),
        ) {
            return Err(source_failure(
                "the one explicit element argument needs the direct unshadowed primitive f64 owner",
                Some(actual_span(&open, &token)),
            ));
        }
        self.cursor
            .expect(">", Some("empty option type argument"))?;
        let policy = crate::translation::source_empty_options::policy();
        Ok(
            json!({"type":policy["elementType"],"sourcePath":[token.value],"resolvedPath":actual,"canonicalPath":policy["elementPath"],"arguments":[],"wrappers":[],"lifetimes":[],"span":token.span()}),
        )
    }
    pub(super) fn read_empty_option_result(
        &mut self,
        path: &[String],
        start: &Token,
    ) -> Result<Option<Type>> {
        if self.empty_option_source_model.is_empty()
            || !accept_rust_empty_option_result_owner(&path.join("::"), false)
        {
            return Ok(None);
        }
        let mut owner = self.empty_option_owner(
            path,
            false,
            actual_span(start, &self.cursor.tokens[self.cursor.index - 1]),
        )?;
        owner["arguments"] = json!([self.empty_option_argument()?]);
        let ty = Type::Option {
            element: Box::new(super::FLOAT),
            absence: "undefined".to_owned(),
        };
        let position = json!({"type":ty,"owner":owner,"wrappers":[],"lifetimes":[],"span":actual_span(start,&self.cursor.tokens[self.cursor.index-1])});
        self.empty_option_result = Some(position);
        Ok(Some(self.origins.typed(ty, &self.module_path, Vec::new())))
    }
    pub(super) fn empty_option_function(&mut self, path: &[String]) -> Result<SFn> {
        let start = self.cursor.advance();
        let name = self.cursor.identifier(Some("fn"))?.value;
        if self.cursor.is("<") {
            return Err(source_failure(
                "generic functions and lifetime binders require a separate interpretation",
                span(&start, self.cursor.peek()),
            ));
        }
        self.cursor.expect("(", Some("fn"))?;
        if !self.cursor.is(")") {
            return Err(source_failure(
                "the closed empty interpretation admits zero parameters",
                span(&start, self.cursor.peek()),
            ));
        }
        self.cursor.expect(")", Some("fn"))?;
        if self.cursor.eat("->").is_none() {
            return Err(unsupported(
                "function without a result",
                &format!("{name} returns (); only value-returning functions are portable"),
                span(&start, self.cursor.peek()),
            ));
        }
        let result_token = self.peek();
        self.empty_option_result = None;
        let ret = self.parse_type()?;
        let source_result = self.origins.current_type.clone();
        let Some(result) = self.empty_option_result.take().filter(|result| {
            result["span"]["start"] == json!(result_token.start)
                && source_result.wrappers.is_empty()
        }) else {
            return Err(source_failure(
                "the result needs an exact direct standard Option<f64> owner",
                span(&start, self.cursor.peek()),
            ));
        };
        self.origins.declaration(
            &name,
            path,
            SourceDeclaration::Function {
                parameters: Vec::new(),
                result: source_result.clone(),
            },
        )?;
        if self.cursor.is("where") {
            return Err(unsupported(
                "where clause",
                "trait bounds are outside the portable core",
                span(&start, self.cursor.peek()),
            ));
        }
        let previous = self.origins.enter(Some(std::collections::HashMap::new()));
        self.cursor.expect("{", Some("empty option function"))?;
        let construction_start = self.peek();
        let segments = self.path_segments("empty option constructor")?;
        let mut owner = self.empty_option_owner(
            &segments,
            true,
            actual_span(
                &construction_start,
                &self.cursor.tokens[self.cursor.index - 1],
            ),
        )?;
        self.cursor
            .expect("::", Some("explicit empty option constructor"))?;
        owner["arguments"] = json!([self.empty_option_argument()?]);
        let construction_span = actual_span(
            &construction_start,
            &self.cursor.tokens[self.cursor.index - 1],
        );
        let close = self.cursor.expect("}", Some("empty option function"))?;
        let body = SExpr::new(
            SNode::None {
                absence: "undefined".to_owned(),
            },
            Some(construction_span),
        );
        self.origins.mark(
            &body,
            SourceOrigin::with_type(ret.clone(), path, Vec::new()),
        );
        self.origins
            .argument(&self.origins.origin(&body, path), &source_result, body.span)?;
        self.origins.scope = previous;
        let policy = crate::translation::source_empty_options::policy();
        let proof = json!({"model":self.empty_option_source_model,"language":policy["language"],"sourceName":name,"namespace":{"declaredTypeOwners":[],"externalOptionRoots":policy["externalOptionRoots"],"imports":[],"declarationNames":[]},"parameters":[],"result":result,"construction":{"owner":owner,"span":construction_span,"path":[]}});
        Ok(SFn {
            source_diagnostic_attributes: None,
            semantic_ret: None,
            name,
            params: Vec::new(),
            ret: Some(ret),
            body,
            span: Some(actual_span(&start, &close)),
            generated: false,
            source_signature: None,
            source_loop: None,
            source_primitive_owners: None,
            source_empty_option: Some(Box::new(proof)),
            source_type_namespace: None,
            source_pure_unit: None,
            source_local_state: None,
            source_const_function: None,
        })
    }
}
impl RustParser {
    pub(super) fn empty_option_attribute_guard(&self) -> Result<()> {
        if !self.empty_option_source_model.is_empty() && self.cursor.is("#") {
            return Err(source_failure(
                "attributes require a separate compilation context proof",
                span(self.cursor.peek(), self.cursor.peek()),
            ));
        }
        Ok(())
    }
    pub(super) fn empty_option_item_guard(&self, path: &[String], start: &Token) -> Result<()> {
        if !self.empty_option_source_model.is_empty() && (!path.is_empty() || !self.cursor.is("fn"))
        {
            return Err(source_failure(
                "only flat zero-argument empty leaf functions and an empty main are admitted",
                span(start, self.cursor.peek()),
            ));
        }
        Ok(())
    }
    pub(super) fn empty_option_main_guard(&mut self, start: &Token) -> Result<()> {
        if !self.empty_option_source_model.is_empty() && self.empty_option_main_seen {
            return Err(source_failure(
                "the complete source must have one owned main declaration",
                span(start, self.cursor.peek()),
            ));
        }
        self.empty_option_main_seen = true;
        Ok(())
    }
    pub(super) fn finish_empty_option_source(
        &self,
        items: &mut [super::SItem],
        main: &super::SMain,
    ) -> Result<()> {
        if self.empty_option_source_model.is_empty() {
            return Ok(());
        }
        let forbidden = items.is_empty()
            || !main.effects.is_empty()
            || items
                .iter()
                .any(|item| !matches!(item, super::SItem::Fn(_)));
        if !super::empty_option_rules::accept_rust_empty_option_context("Rust", forbidden) {
            return Err(source_failure(
                "the model requires complete flat zero-argument empty leaf functions and an empty main",
                None,
            ));
        }
        let names = items
            .iter()
            .filter_map(|item| {
                if let super::SItem::Fn(function) = item {
                    Some(function.name.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for item in items {
            if let super::SItem::Fn(function) = item {
                function
                    .source_empty_option
                    .as_mut()
                    .expect("the selected reader retains its owned proof")["namespace"]["declarationNames"] =
                    json!(names);
            }
        }
        Ok(())
    }
}
