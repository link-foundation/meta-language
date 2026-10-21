//! Complete existing methods, mechanically relocated without behavior changes.
use super::{
    BinaryOp, External, FLOAT, GLOBALS, JavaScriptParser, Language, ROOT, Result, SArrayItem,
    SExpr, SNode, ShowStyle, Token, TokenCursor, TokenKind, node, number_literal, span, tokenize,
    unsupported,
};

impl JavaScriptParser {
    pub(in super::super) fn primary(&mut self) -> Result<SExpr> {
        let token = self.peek();
        match token.kind {
            TokenKind::Number => {
                self.cursor.advance();
                return Ok(number_literal(&token));
            }
            TokenKind::String => {
                self.cursor.advance();
                return Ok(node(
                    SNode::Str {
                        value: token.value.clone(),
                    },
                    span(&token, &token),
                ));
            }
            TokenKind::Template => {
                self.cursor.advance();
                return self.template(&token);
            }
            TokenKind::Identifier => return self.identifier(),
            _ => {}
        }
        let here = Some(span(&token, &token));
        if self.cursor.is("(") {
            if self.starts_function(self.cursor.index) {
                return self.read_closure(false);
            }
            self.cursor.advance();
            let mut inner = self.expr()?;
            self.cursor.expect(")", Some("parenthesised expression"))?;
            inner.parenthesized = Some(
                crate::translation::surface_raw_metadata_rules::accept_surface_raw_metadata_field(
                    "expression",
                    "parenthesized",
                )
                .into(),
            );
            inner.span = Some(self.to_here(&token));
            return Ok(inner);
        }
        if self.cursor.is("{") {
            return self.object_literal();
        }
        if self.cursor.is("[") {
            return self.array_literal();
        }
        if token.kind == TokenKind::RegularExpression || self.cursor.is("/") {
            return Err(unsupported(
                "regular expression",
                "outside the portable core",
                here,
            ));
        }
        Err(self.fail_here("expected an expression"))
    }

    /// `[a, ...xs, b]`: ordered values and the elements of each supported iterable.
    fn array_literal(&mut self) -> Result<SExpr> {
        let start = self.cursor.advance();
        let mut items = Vec::new();
        while !self.cursor.is("]") {
            if self.cursor.is(",") {
                let token = self.peek();
                return Err(unsupported(
                    "array hole",
                    "a hole is an element that reads undefined, which is not a portable value",
                    Some(span(&token, &token)),
                ));
            }
            let spread = self.cursor.eat("...").is_some();
            let value = self.expr()?;
            let place = value.span;
            let value = if spread {
                SExpr::new(
                    SNode::ForOfIterable {
                        object: Box::new(value),
                        ty: None,
                    },
                    place,
                )
            } else {
                value
            };
            items.push(SArrayItem::new(value, spread));
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("]", Some("array literal"))?;
        Ok(node(
            SNode::Array {
                items,
                element: None,
            },
            self.to_here(&start),
        ))
    }

    pub(in super::super) fn matching(&self, index: usize) -> usize {
        let tokens = &self.cursor.tokens;
        let mut depth = 0_i64;
        for (at, token) in tokens.iter().enumerate().skip(index) {
            if token.kind != TokenKind::Punct {
                continue;
            }
            if ["(", "[", "{"].contains(&token.value.as_str()) {
                depth += 1;
            }
            if [")", "]", "}"].contains(&token.value.as_str()) {
                depth -= 1;
                if depth == 0 {
                    return at;
                }
            }
        }
        tokens.len() - 1
    }

    /// Template literals concatenate their text with `String(value)` of each substitution.
    pub(in super::super) fn template(&mut self, token: &Token) -> Result<SExpr> {
        let whole = span(token, token);
        let mut pieces = Vec::new();
        for part in &token.parts {
            if !part.text.is_empty() {
                pieces.push(node(
                    SNode::Str {
                        value: part.text.clone(),
                    },
                    whole,
                ));
            }
            if let Some(expression) = &part.expression {
                let value = self.subexpression(&expression.source, expression.offset)?;
                let place = value.span;
                pieces.push(SExpr::new(
                    SNode::Show {
                        arg: Box::new(value),
                        style: ShowStyle::JsTemplate,
                    },
                    place,
                ));
            }
        }
        let mut pieces = pieces.into_iter();
        let Some(first) = pieces.next() else {
            return Ok(node(
                SNode::Str {
                    value: String::new(),
                },
                whole,
            ));
        };
        Ok(pieces.fold(first, |left, right| {
            node(
                SNode::Binary {
                    op: BinaryOp::Concat,
                    left: Box::new(left),
                    right: Box::new(right),
                    rounding: None,
                },
                whole,
            )
        }))
    }

    pub(in super::super) fn subexpression(&mut self, source: &str, offset: usize) -> Result<SExpr> {
        let mut tokens = tokenize(source, Language::JavaScript)?.tokens;
        for token in &mut tokens {
            token.start += offset;
            token.end += offset;
        }
        let outer = std::mem::replace(
            &mut self.cursor,
            TokenCursor::new(tokens, Language::JavaScript),
        );
        let expr = self.substitution();
        self.cursor = outer;
        expr
    }

    pub(in super::super) fn substitution(&mut self) -> Result<SExpr> {
        let expr = self.expr()?;
        if !self.cursor.at_end() {
            return Err(self.fail_here("expected } after the template substitution"));
        }
        Ok(expr)
    }

    pub(in super::super) fn identifier(&mut self) -> Result<SExpr> {
        let token = self.peek();
        let here = Some(span(&token, &token));
        if token.value == "true" || token.value == "false" {
            self.cursor.advance();
            return Ok(node(
                SNode::Bool {
                    value: token.value == "true",
                },
                span(&token, &token),
            ));
        }
        self.lexical.read(&token, false, token.span());
        if ["NaN", "Infinity", "undefined"].contains(&token.value.as_str())
            && (self.scope.locals.contains(&token.value) || self.scope.tdz.contains(&token.value))
        {
            self.cursor.advance();
            return self.reference(&token);
        }
        if ["NaN", "Infinity", "undefined"].contains(&token.value.as_str()) {
            if let Some(value) = self.literal_bindings.get(&token.value) {
                let mut value = value.clone();
                value.span = Some(span(&token, &token));
                self.cursor.advance();
                return Ok(value);
            }
            if self.global_binding_names.contains(&token.value)
                && !self.function_names.contains(&token.value)
                && !self
                    .externals
                    .iter()
                    .any(|external| external.name() == token.value)
            {
                return Err(unsupported(
                    &format!("global binding {}", token.value),
                    "a delayed function read needs an owned module binding; a mutable module binding requires module-capture representation, and neither can be replaced with the built-in constant",
                    Some(span(&token, &token)),
                ));
            }
        }
        let declared = self.function_names.contains(&token.value)
            || self
                .externals
                .iter()
                .any(|external| external.name() == token.value);
        if !declared && (token.value == "NaN" || token.value == "Infinity") {
            if !self.has_standard_builtin_owner(&token) {
                return Err(unsupported(
                    "unproved standard global owner",
                    "the original module scope does not prove this special global binding",
                    Some(token.span()),
                ));
            }
            self.cursor.advance();
            return Ok(node(
                SNode::Num {
                    value: token.value.clone(),
                    ty: Some(FLOAT),
                    negative: false,
                    unit: false,
                },
                span(&token, &token),
            ));
        }
        if token.value == "undefined" && !declared && !self.has_standard_builtin_owner(&token) {
            return Err(unsupported(
                "unproved standard global owner",
                "the original module scope does not prove this special global binding",
                Some(token.span()),
            ));
        }
        let absent = crate::translation::optional_value_rules::read_optional_source_literal(
            &token.value,
            declared,
        );
        if !absent.is_empty() {
            self.cursor.advance();
            return Ok(node(SNode::None { absence: absent }, span(&token, &token)));
        }
        if token.value == "new" {
            self.cursor.advance();
            let constructor = self.cursor.identifier(Some("constructor"))?;
            self.lexical.read(&constructor, false, constructor.span());
            let arguments = if self.cursor.is("(") {
                self.arguments(&constructor.value)?
            } else {
                Vec::new()
            };
            let collection = super::super::surface_rules::read_java_script_collection_construction(
                &constructor.value,
                arguments.len() > 1,
            );
            if collection.is_empty() {
                return Err(unsupported(
                    &format!("new {}", constructor.value),
                    &super::super::surface_rules::read_java_script_construction_obligation(),
                    Some(self.to_here(&token)),
                ));
            }
            if !self.has_standard_builtin_owner(&constructor) {
                return Err(unsupported(
                    "shadowed standard collection owner",
                    "a local, module or supplied context constructor cannot be replaced with the standard Map or Set constructor",
                    Some(constructor.span()),
                ));
            }
            let place = self.to_here(&token);
            let items = arguments.into_iter().next().unwrap_or_else(|| {
                node(
                    SNode::Array {
                        items: Vec::new(),
                        element: None,
                    },
                    place,
                )
            });
            return match collection.as_str() {
                "set" => Ok(node(
                    SNode::Set {
                        items: Box::new(items),
                        ty: None,
                    },
                    place,
                )),
                "map" => Ok(node(
                    SNode::Map {
                        entries: Box::new(items),
                        ty: None,
                    },
                    place,
                )),
                _ => Err(unsupported(
                    "new expression",
                    "outside the portable core",
                    here,
                )),
            };
        }
        if token.value == "function" {
            return self.read_closure(false);
        }
        let expressions = ["this", "super", "new", "class", "import", "arguments"];
        if expressions.contains(&token.value.as_str()) {
            return Err(unsupported(
                &format!("{} expression", token.value),
                "outside the portable core",
                here,
            ));
        }
        self.cursor.advance();
        if self.scope.locals.contains(&token.value) || self.scope.tdz.contains(&token.value) {
            return self.reference(&token);
        }
        if let Some(value) = self.literal_bindings.get(&token.value) {
            let mut value = value.clone();
            value.span = Some(span(&token, &token));
            return Ok(value);
        }
        if self
            .assertion
            .as_ref()
            .is_some_and(|assertion| assertion.name == token.value)
        {
            return Err(unsupported(
                "assertion in an expression",
                "assertions are top-level statements",
                here,
            ));
        }
        // A constant of another item of the module is read by name.
        if !self.cursor.is("(")
            && self.externals.iter().any(|external| {
                matches!(external, External::Constant { name, .. } if *name == token.value)
            })
        {
            return Ok(node(
                SNode::Name {
                    path: vec![ROOT.to_owned(), token.value.clone()],
                },
                span(&token, &token),
            ));
        }
        // A global: a function, or a namespace path to a method, which must be called.
        let mut segments = vec![token.value.clone()];
        while self.cursor.is(".") && self.cursor.is_kind_at(TokenKind::Identifier, 1) {
            self.cursor.advance();
            segments.push(self.cursor.advance().value);
        }
        let name = segments.join(".");
        let place = self.to_here(&token);
        if let Some(math) = self.math(&token, &name)? {
            return Ok(math);
        }
        if !self.cursor.is("(") {
            let constructor =
                crate::translation::callable_source_rules::read_callable_surface_constructor(
                    "name",
                    self.function_names.contains(&name),
                );
            if constructor == "functionValue" {
                let mut path = vec![ROOT.to_owned()];
                path.extend(segments);
                return Ok(node(SNode::FunctionValue { path }, place));
            }
            return Err(unsupported(
                &format!("function value {name}"),
                "functions and namespaces are only portable when a function is called",
                Some(place),
            ));
        }
        let mut args = self.arguments(&name)?;
        let called = self.to_here(&token);
        if name == "Array.from" && self.has_standard_builtin_owner(&token) {
            return Ok(node(SNode::ArrayFrom { args, ty: None }, called));
        }
        let array_form = crate::translation::frontend_rules::read_array_method_form(&name);
        if (array_form == "copy-from" || array_form == "construct")
            && self.has_standard_builtin_owner(&token)
        {
            return Self::array_method_expression(&array_form, args, None, called);
        }
        if name == "String" && args.len() == 1 && self.has_standard_builtin_owner(&token) {
            let arg = args.remove(0);
            return Ok(node(SNode::ToString { arg: Box::new(arg) }, called));
        }
        if name == "Object.freeze" && args.len() == 1 && self.has_standard_builtin_owner(&token) {
            return self.freeze_object_argument(args.remove(0), called);
        }
        if (GLOBALS.contains(&segments[0].as_str()) || name == "String")
            && !self.function_names.contains(&segments[0])
            && !self.global_binding_names.contains(&segments[0])
            && !self.literal_bindings.contains_key(&segments[0])
            && !self
                .externals
                .iter()
                .any(|external| external.name() == segments[0].as_str())
        {
            return Err(unsupported(
                &name,
                "the JavaScript standard library is outside the portable core",
                Some(called),
            ));
        }
        if segments.len() == 1 && self.async_names.contains(&name) {
            self.unawaited.push((name, called));
        }
        let mut path = vec![ROOT.to_owned()];
        path.extend(segments);
        if args.is_empty() {
            return Ok(node(SNode::Name { path }, called));
        }
        Ok(node(
            SNode::App {
                func: Box::new(node(SNode::Name { path }, place)),
                args,
            },
            called,
        ))
    }

    /// A local variable; reading one before its declaration throws in JavaScript.
    pub(in super::super) fn reference(&mut self, token: &Token) -> Result<SExpr> {
        let here = span(token, token);
        self.lexical.read(token, false, here);
        let temporal = self.scope.tdz.contains(&token.value);
        if temporal {
            self.lexical.validate_temporal(token)?;
        }
        let reason = crate::translation::callable_source_rules::read_callable_reference_obligation(
            temporal,
            self.scope.locals.contains(&token.value),
        );
        if !reason.is_empty() {
            let feature =
                crate::translation::callable_source_rules::read_callable_reference_construct(
                    &token.value,
                    temporal,
                );
            return Err(unsupported(&feature, &reason, Some(here)));
        }
        Ok(node(
            SNode::Name {
                path: vec![token.value.clone()],
            },
            here,
        ))
    }
}
