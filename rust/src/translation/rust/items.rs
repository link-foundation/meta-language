//! Attributes, items, `use` trees, enums, types, functions, blocks and `main`.

use super::{
    ACCEPTED_DERIVES, ACCEPTED_LINTS, BOOL, Binding, ItemsEnd, Language, Options, RESERVED_ITEMS,
    Result, RustParser, SComparison, SCtor, SData, SEffect, SExpr, SField, SFn, SItem, SMain,
    SModule, SNode, SParam, SProgram, SProp, SPropNode, STRING, ShowStyle, Token, TokenKind,
    TranslationError, Type, UNIT, UNSUPPORTED_TYPES, absolute, format, is_if_or_match, prop_of,
    rust_fixed_type, span, unsupported,
};

impl RustParser {
    pub(super) fn file(mut self) -> Result<SProgram> {
        self.inner_attributes()?;
        let items = self.items(&[], ItemsEnd::File)?;
        let Some(main) = self.main.take() else {
            return Err(TranslationError::syntax(
                "a Rust program needs fn main",
                Some(self.cursor.peek().span()),
            ));
        };
        Ok(SProgram {
            language: Language::Rust,
            items,
            main: Some(main),
        })
    }

    pub(super) fn inner_attributes(&mut self) -> Result<()> {
        while self.cursor.is("#") && self.cursor.is_at("!", 1) {
            let start = self.cursor.advance();
            self.cursor.advance();
            self.attribute_body(&start)?;
        }
        Ok(())
    }

    pub(super) fn attributes(&mut self) -> Result<()> {
        while self.cursor.is("#") && self.cursor.is_at("[", 1) {
            let start = self.cursor.advance();
            self.attribute_body(&start)?;
        }
        Ok(())
    }

    /// `#[derive(Clone, …)]` and lint attributes do not change behaviour.
    pub(super) fn attribute_body(&mut self, start: &Token) -> Result<()> {
        let c = &mut self.cursor;
        c.expect("[", Some("attribute"))?;
        let name = c.identifier(Some("attribute"))?.value;
        if name == "derive" {
            c.expect("(", Some("derive"))?;
            while !c.is(")") {
                let derived = c.identifier(Some("derive"))?;
                if !ACCEPTED_DERIVES.contains(&derived.value.as_str()) {
                    return Err(unsupported(
                        &format!("derive({})", derived.value),
                        "only derives that add no behaviour are portable",
                        span(&derived, &derived),
                    ));
                }
                if c.eat(",").is_none() {
                    break;
                }
            }
            c.expect(")", Some("derive"))?;
        } else if ACCEPTED_LINTS.contains(&name.as_str()) {
            let mut depth = 0_i64;
            while !c.at_end() && (depth != 0 || !c.is("]")) {
                if c.is("(") {
                    depth += 1;
                }
                if c.is(")") {
                    depth -= 1;
                }
                c.advance();
            }
        } else {
            return Err(unsupported(
                &format!("#[{name}]"),
                "attributes that change compilation are outside the portable core",
                span(start, c.peek()),
            ));
        }
        c.expect("]", Some("attribute"))?;
        Ok(())
    }

    pub(super) fn items(&mut self, path: &[String], end: ItemsEnd) -> Result<Vec<SItem>> {
        let mut items = Vec::new();
        loop {
            let done = match end {
                ItemsEnd::File => self.cursor.at_end(),
                ItemsEnd::Module => self.cursor.is("}"),
            };
            if done {
                return Ok(items);
            }
            self.attributes()?;
            let start = self.peek();
            self.module_path = path.to_vec();
            self.visibility()?;
            let c = &mut self.cursor;
            if c.is("mod") {
                c.advance();
                let name = c.identifier(Some("mod"))?.value;
                if c.is(";") {
                    return Err(unsupported(
                        "out-of-line module",
                        "the program must be a single file",
                        span(&start, c.peek()),
                    ));
                }
                c.expect("{", Some("mod"))?;
                let module_span = span(&start, c.peek());
                let mut inner = path.to_vec();
                inner.push(name.clone());
                let module_items = self.items(&inner, ItemsEnd::Module)?;
                self.cursor.expect("}", Some("mod"))?;
                items.push(SItem::Module(SModule {
                    name,
                    items: module_items,
                    span: module_span,
                }));
                continue;
            }
            if c.is("use") {
                self.use_declaration(path)?;
                continue;
            }
            if c.is("enum") {
                items.push(self.enum_item()?);
                continue;
            }
            if c.is("fn") || c.is("const") {
                if c.is("fn") && c.is_at("main", 1) && path.is_empty() {
                    self.main = Some(self.main_fn()?);
                    continue;
                }
                let item = if c.is("fn") {
                    self.function(path)?
                } else {
                    self.const_item(path)?
                };
                items.push(SItem::Fn(item));
                continue;
            }
            if start.kind == TokenKind::Identifier && RESERVED_ITEMS.contains(&start.value.as_str())
            {
                return Err(unsupported(
                    &format!("Rust {} item", start.value),
                    &format!("{} items are outside the portable core", start.value),
                    span(&start, &start),
                ));
            }
            if c.at_end() {
                // `path.at(-1)` of the file's empty path reads `undefined`.
                let module = path.last().map_or("undefined", String::as_str);
                return Err(TranslationError::syntax(
                    format!("module {module} is not closed"),
                    Some(start.span()),
                ));
            }
            return Err(Self::fail("expected a Rust item", c.peek()));
        }
    }

    pub(super) fn visibility(&mut self) -> Result<()> {
        let c = &mut self.cursor;
        if c.eat("pub").is_none() {
            return Ok(());
        }
        if c.is("(") {
            c.advance();
            c.eat("in");
            // The end of input stops the scan, so an unclosed `pub(` is reported rather than looping.
            while !c.is(")") && !c.at_end() {
                c.advance();
            }
            c.expect(")", Some("visibility"))?;
        }
        Ok(())
    }

    /// `use super::Tree;`, `use crate::a::{b, c as d};`: each imported name is
    /// an alias for its absolute path inside the importing module.
    pub(super) fn use_declaration(&mut self, path: &[String]) -> Result<()> {
        let start = self.cursor.advance();
        self.aliases.entry(path.join(".")).or_default();
        self.use_tree(&[], &start, path)?;
        self.cursor.expect(";", Some("use"))?;
        Ok(())
    }

    pub(super) fn use_tree(
        &mut self,
        prefix: &[String],
        start: &Token,
        path: &[String],
    ) -> Result<()> {
        if self.cursor.eat("{").is_some() {
            while !self.cursor.is("}") {
                self.use_tree(prefix, start, path)?;
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
            self.cursor.expect("}", Some("use"))?;
            return Ok(());
        }
        let c = &mut self.cursor;
        if c.is("*") {
            return Err(unsupported(
                "glob import",
                "name each imported item explicitly",
                span(start, c.peek()),
            ));
        }
        let segment = c.identifier(Some("use"))?.value;
        let mut full = prefix.to_vec();
        full.push(segment.clone());
        if c.eat("::").is_some() {
            return self.use_tree(&full, start, path);
        }
        if !["crate", "super", "self"].contains(&full[0].as_str()) {
            return Err(unsupported(
                &format!("use {}", full.join("::")),
                "only items of this crate can be imported; the standard library is outside the portable core",
                span(start, c.peek()),
            ));
        }
        let alias = if c.eat("as").is_some() {
            c.identifier(Some("use alias"))?.value
        } else {
            segment
        };
        let target = absolute(&full, path, span(start, c.peek()))?;
        self.aliases
            .entry(path.join("."))
            .or_default()
            .insert(alias, target);
        Ok(())
    }

    /// A `use` alias is visible only in the module that declares it.
    pub(super) fn resolve(&self, segments: &[String], path: &[String]) -> Vec<String> {
        self.aliases
            .get(&path.join("."))
            .and_then(|aliases| aliases.get(&segments[0]))
            .map_or_else(
                || segments.to_vec(),
                |target| target.iter().chain(&segments[1..]).cloned().collect(),
            )
    }

    pub(super) fn enum_item(&mut self) -> Result<SItem> {
        let start = self.cursor.advance();
        let name = self.cursor.identifier(Some("enum"))?.value;
        if self.cursor.is("<") {
            return Err(unsupported(
                "generic enum",
                "type parameters are outside the portable core",
                span(&start, self.cursor.peek()),
            ));
        }
        self.cursor.expect("{", Some("enum"))?;
        let mut ctors = Vec::new();
        while !self.cursor.is("}") {
            self.attributes()?;
            let variant = self.cursor.identifier(Some("variant"))?;
            let mut fields = Vec::new();
            if self.cursor.eat("(").is_some() {
                while !self.cursor.is(")") {
                    fields.push(SField {
                        name: None,
                        ty: self.parse_type()?,
                        rocq_type: None,
                        span: None,
                    });
                    if self.cursor.eat(",").is_none() {
                        break;
                    }
                }
                self.cursor.expect(")", Some("variant"))?;
            } else if self.cursor.is("{") {
                return Err(unsupported(
                    "struct variant",
                    "use a tuple variant; named fields are outside the Rust frontend",
                    span(&variant, self.cursor.peek()),
                ));
            } else if self.cursor.is("=") {
                return Err(unsupported(
                    "explicit discriminant",
                    "discriminant values are outside the portable core",
                    span(&variant, self.cursor.peek()),
                ));
            }
            ctors.push(SCtor {
                name: variant.value,
                fields,
            });
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("}", Some("enum"))?;
        Ok(SItem::Data(SData {
            name,
            ctors,
            span: span(&start, self.cursor.peek()),
            generated: false,
        }))
    }

    /// Types: machine integers, `bool`, `String`, `&str`, `()`, crate types,
    /// and `&T`/`Box<T>`, which denote the value `T` since every portable value
    /// is immutable.
    pub(super) fn parse_type(&mut self) -> Result<Type> {
        let token = self.peek();
        if self.cursor.eat("&").is_some() {
            if self.cursor.is("'") {
                self.cursor.advance();
                let lifetime = self.cursor.identifier(Some("lifetime"))?;
                if lifetime.value != "static" {
                    return Err(unsupported(
                        "lifetime parameter",
                        "only 'static references are portable",
                        span(&token, &lifetime),
                    ));
                }
            }
            if self.cursor.is("mut") {
                return Err(unsupported(
                    "mutable reference",
                    "mutation is outside the portable core",
                    span(&token, self.cursor.peek()),
                ));
            }
            if self.cursor.is("str") {
                self.cursor.advance();
                return Ok(STRING);
            }
            return self.parse_type();
        }
        if self.cursor.eat("(").is_some() {
            if self.cursor.eat(")").is_some() {
                return Ok(UNIT);
            }
            return Err(unsupported(
                "tuple type",
                "tuples are outside the portable core",
                span(&token, self.cursor.peek()),
            ));
        }
        let segments = self.path_segments("type")?;
        let name = segments.join("::");
        if name == "Box" {
            self.cursor.expect("<", Some("Box"))?;
            let inner = self.parse_type()?;
            self.cursor.expect(">", Some("Box"))?;
            return Ok(inner);
        }
        if self.cursor.is("<") {
            return Err(unsupported(
                &format!("type {name}<…>"),
                "generic types are outside the portable core",
                span(&token, self.cursor.peek()),
            ));
        }
        if segments.len() == 1
            && let Some(fixed) = rust_fixed_type(&name)
        {
            return Ok(fixed);
        }
        if name == "bool" {
            return Ok(BOOL);
        }
        if name == "String" {
            return Ok(STRING);
        }
        if UNSUPPORTED_TYPES.contains(&name.as_str()) {
            return Err(unsupported(
                &format!("Rust type {name}"),
                "outside the portable core",
                span(&token, &token),
            ));
        }
        Ok(Type::Named {
            path: self.resolve(&segments, &self.module_path),
            span: span(&token, &token),
        })
    }

    pub(super) fn path_segments(&mut self, context: &str) -> Result<Vec<String>> {
        let c = &mut self.cursor;
        let mut segments = vec![c.identifier(Some(context))?.value];
        while c.is("::") && c.peek_at(1).kind == TokenKind::Identifier {
            c.advance();
            segments.push(c.advance().value);
        }
        Ok(segments)
    }

    pub(super) fn function(&mut self, path: &[String]) -> Result<SFn> {
        let start = self.cursor.advance();
        let name = self.cursor.identifier(Some("fn"))?.value;
        if self.cursor.is("<") {
            return Err(unsupported(
                "generic function",
                "type parameters are outside the portable core",
                span(&start, self.cursor.peek()),
            ));
        }
        self.cursor.expect("(", Some("fn"))?;
        let mut params = Vec::new();
        while !self.cursor.is(")") {
            let param_token = self.peek();
            if self.cursor.is("mut") {
                return Err(unsupported(
                    "mutable parameter",
                    "mutation is outside the portable core",
                    span(&param_token, &param_token),
                ));
            }
            if self.cursor.is("self") || self.cursor.is("&") {
                return Err(unsupported(
                    "method",
                    "methods are outside the Rust frontend; declare a free function",
                    span(&param_token, &param_token),
                ));
            }
            let param_name = self.cursor.identifier(Some("parameter"))?.value;
            self.cursor.expect(":", Some("parameter"))?;
            let ty = self.parse_type()?;
            params.push(SParam {
                default: None,
                name: param_name,
                ty: Some(ty),
                span: span(&param_token, self.cursor.peek()),
                guard: None,
                rocq_type: None,
            });
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect(")", Some("fn"))?;
        if self.cursor.eat("->").is_none() {
            return Err(unsupported(
                "function without a result",
                &format!("{name} returns (); only value-returning functions are portable"),
                span(&start, self.cursor.peek()),
            ));
        }
        let ret = self.parse_type()?;
        if self.cursor.is("where") {
            return Err(unsupported(
                "where clause",
                "trait bounds are outside the portable core",
                span(&start, self.cursor.peek()),
            ));
        }
        let body = self.block(path)?;
        Ok(SFn {
            name,
            params,
            ret: Some(ret),
            body,
            span: span(&start, self.cursor.peek()),
            generated: false,
        })
    }

    /// `const N: T = e;` is a function without parameters.
    pub(super) fn const_item(&mut self, path: &[String]) -> Result<SFn> {
        let start = self.cursor.advance();
        let name = self.cursor.identifier(Some("const"))?.value;
        self.cursor.expect(":", Some("const"))?;
        let ret = self.parse_type()?;
        self.cursor.expect("=", Some("const"))?;
        let body = self.expr(path, Options::default())?;
        self.cursor.expect(";", Some("const"))?;
        Ok(SFn {
            name,
            params: Vec::new(),
            ret: Some(ret),
            body,
            span: span(&start, self.cursor.peek()),
            generated: false,
        })
    }

    /// A block `{ let x = e; …; tail }` is a chain of lets ending in its tail
    /// expression. Statements with effects are not values of the portable core.
    pub(super) fn block(&mut self, path: &[String]) -> Result<SExpr> {
        let open = self.cursor.expect("{", Some("block"))?;
        let mut lets = Vec::new();
        let mut tail: Option<SExpr> = None;
        while !self.cursor.is("}") {
            let token = self.peek();
            if self.cursor.is("let") {
                lets.push(self.let_statement(path)?);
                continue;
            }
            if self.cursor.is("return") {
                return Err(unsupported(
                    "early return",
                    "write the result as the tail expression of the block",
                    span(&token, &token),
                ));
            }
            if token.kind == TokenKind::Identifier
                && ["while", "loop", "for"].contains(&token.value.as_str())
            {
                return Err(unsupported(
                    &format!("{} loop", token.value),
                    "loops over mutable state are outside the portable core; use recursion",
                    span(&token, &token),
                ));
            }
            let expr = self.expr(
                path,
                Options {
                    statement: true,
                    no_struct: false,
                },
            )?;
            if self.cursor.eat(";").is_some() {
                if matches!(expr.node, SNode::Abort { .. }) {
                    tail = Some(expr);
                    continue;
                }
                return Err(unsupported(
                    "expression statement",
                    "statements with effects are outside the portable core in function bodies",
                    expr.span,
                ));
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
        let Some(tail) = tail else {
            return Err(unsupported(
                "block without a value",
                "the block evaluates to (), which is not a portable result",
                span(&open, &close),
            ));
        };
        Ok(lets.into_iter().rev().fold(tail, |body, binding| {
            SExpr::new(
                SNode::Let {
                    name: binding.name,
                    ty: binding.ty,
                    value: Box::new(binding.value),
                    body: Box::new(body),
                },
                binding.span,
            )
        }))
    }

    pub(super) fn let_statement(&mut self, path: &[String]) -> Result<Binding> {
        let start = self.cursor.advance();
        if self.cursor.is("mut") {
            return Err(unsupported(
                "mutable binding",
                "mutation is outside the portable core",
                span(&start, self.cursor.peek()),
            ));
        }
        if self.cursor.is("(") {
            return Err(unsupported(
                "destructuring let",
                "tuples are outside the portable core",
                span(&start, self.cursor.peek()),
            ));
        }
        let name = self.cursor.identifier(Some("let"))?.value;
        let ty = if self.cursor.eat(":").is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };
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
        Ok(Binding {
            name,
            ty,
            value,
            span: span(&start, self.cursor.peek()),
        })
    }

    pub(super) fn main_fn(&mut self) -> Result<SMain> {
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
        self.module_path = Vec::new();
        self.cursor.expect("{", Some("fn main"))?;
        let mut effects = Vec::new();
        while !self.cursor.is("}") {
            let token = self.peek();
            if self.cursor.is("let") {
                let binding = self.let_statement(&[])?;
                effects.push(SEffect::Let {
                    constant: false,
                    name: binding.name,
                    ty: binding.ty,
                    value: binding.value,
                    span: binding.span,
                });
                continue;
            }
            if token.kind == TokenKind::Macro {
                effects.push(self.main_macro()?);
                self.cursor.expect(";", Some(&token.value))?;
                continue;
            }
            return Err(unsupported(
                "main statement",
                "main may only print, bind and assert",
                span(&token, &token),
            ));
        }
        self.cursor.expect("}", Some("fn main"))?;
        Ok(SMain {
            effects,
            span: span(&start, self.cursor.peek()),
            sequential_async: false,
        })
    }

    pub(super) fn main_macro(&mut self) -> Result<SEffect> {
        let token = self.cursor.advance();
        let mut args = self.macro_arguments(&token, &[])?;
        match token.value.as_str() {
            "println" => {
                let expr = if args.is_empty() {
                    SExpr::new(
                        SNode::Str {
                            value: String::new(),
                        },
                        None,
                    )
                } else {
                    format(args, &token)?
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
                let comparison = SComparison {
                    left: args.remove(0),
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
            args.push(self.expr(path, Options::default())?);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor
            .expect(close, Some(&format!("{}!", token.value)))?;
        Ok(args)
    }
}
