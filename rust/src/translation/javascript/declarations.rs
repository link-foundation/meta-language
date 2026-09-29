//! Files, imports, `JSDoc` typedefs, functions, namespaces and blocks.

use super::infer::infer_javascript_types;
use super::loops::reserved;
use super::{
    array, describe, guarded_parameter, imperative, is_identifier_name, is_js_space, js_trim,
    jsdoc_tags, lower, lower_imperative, non_empty, span, statement_uses, tokenize, type_error,
    unsupported, Assertion, HashSet, JavaScriptParser, JsDoc, Language, Result, SCtor, SData,
    SField, SFn, SItem, SMain, SModule, SParam, SProgram, ScanEnd, Scope, Span, Stmt, Token,
    TokenCursor, TokenKind, TranslationError, Type, BOOL, FLOAT, INT, NAT, ROOT, STRING,
};

/// How a function's body is written: a block, or an arrow's block or expression.
#[derive(Clone, Copy)]
enum Body {
    Block,
    Arrow,
}

impl JavaScriptParser {
    pub(super) fn file(mut self) -> Result<SProgram> {
        let mut items = self.typedefs()?;
        let mut effects = Vec::new();
        if self.cursor.is_kind(TokenKind::String) && self.cursor.peek().value == "use strict" {
            self.cursor.advance();
            self.cursor.eat(";");
        }
        self.scope = Scope {
            locals: HashSet::new(),
            tdz: self.block_declarations(self.cursor.index, ScanEnd::File),
            mutable: HashSet::new(),
        };
        self.async_names = self.async_declarations(self.cursor.index);
        // Top-level statements that declare or assign variables, lowered together
        // before the next statement that prints, binds a constant or asserts.
        let mut pending = Vec::new();
        while !self.cursor.at_end() {
            let start = self.peek();
            if self.cursor.is("import") {
                self.import_declaration()?;
                continue;
            }
            let exports_async =
                |cursor: &TokenCursor| cursor.is("async") && cursor.is_at("function", 1);
            if self.cursor.eat("export").is_some()
                && !self.cursor.is("function")
                && !self.cursor.is("const")
                && !exports_async(&self.cursor)
            {
                return Err(unsupported(
                    &format!("export {}", describe(self.cursor.peek())),
                    "only function and const declarations are exported",
                    Some(self.to_here(&start)),
                ));
            }
            if self.cursor.is("async") && self.cursor.is_at("function", 1) {
                self.cursor.advance();
                items.push(SItem::Fn(self.function_declaration(&start, true)?));
                continue;
            }
            if self.cursor.is("async") {
                return Err(unsupported(
                    "async function",
                    "only async functions declared by name or bound to a top-level constant are portable",
                    Some(self.to_here(&start)),
                ));
            }
            if self.cursor.is("function") {
                items.push(SItem::Fn(self.function_declaration(&start, false)?));
                continue;
            }
            if self.cursor.is("const")
                && self.cursor.is_kind_at(TokenKind::Identifier, 1)
                && self.cursor.is_at("=", 2)
                && self.starts_function(self.cursor.index + 3)
            {
                if !effects.is_empty() || !pending.is_empty() {
                    let name = self.peek_at(1);
                    return Err(unsupported(
                        "function after a top-level statement",
                        &format!(
                            "the statements before const {} could call it before it is initialised; declare every function first",
                            name.value
                        ),
                        Some(span(&start, &name)),
                    ));
                }
                items.push(SItem::Fn(self.const_function(&start)?));
                continue;
            }
            if self.cursor.is("const")
                && self.cursor.is_kind_at(TokenKind::Identifier, 1)
                && self.cursor.is_at("=", 2)
                && self.cursor.is_at("{", 3)
                && !(self.cursor.is_at("$", 4) && self.cursor.is_at(":", 5))
            {
                if !effects.is_empty() || !pending.is_empty() {
                    let name = self.peek_at(1);
                    return Err(unsupported(
                        "namespace after a top-level statement",
                        &format!(
                            "the statements before const {} would run before it is initialised; declare every namespace first",
                            name.value
                        ),
                        Some(span(&start, &name)),
                    ));
                }
                items.push(SItem::Module(self.namespace(&start)?));
                continue;
            }
            if self.starts_top_level_imperative() {
                pending.push(self.top_level_statement()?);
                continue;
            }
            self.flush(&mut pending, &mut effects)?;
            effects.push(self.main_statement()?);
        }
        self.flush(&mut pending, &mut effects)?;
        if let Some((name, place)) = self.unawaited.first() {
            return Err(unsupported(
                &format!("call of async function {name} without await"),
                "the Promise it returns is outside the portable core; await it where it is called",
                Some(*place),
            ));
        }
        items.append(&mut self.generated);
        infer_javascript_types(SProgram {
            language: Language::JavaScript,
            items,
            main: Some(SMain {
                effects,
                span: Some(Span::new(0, self.source.len())),
                sequential_async: self.sequential_async,
            }),
        })
    }

    /// `import assert from 'node:assert/strict'` is the only portable import.
    pub(super) fn import_declaration(&mut self) -> Result<()> {
        let start = self.cursor.advance();
        let local;
        let mut strict = false;
        if self.cursor.eat("{").is_some() {
            let imported = self.cursor.identifier(Some("import"))?;
            if imported.value != "strict" || self.cursor.eat("as").is_none() {
                return Err(unsupported(
                    &format!("import {{ {} }}", imported.value),
                    "import the assertion module as a whole, e.g. import assert from 'node:assert/strict'",
                    Some(self.to_here(&start)),
                ));
            }
            local = self.cursor.identifier(Some("import"))?;
            self.cursor.expect("}", Some("import"))?;
            strict = true;
        } else if self.cursor.is("*") {
            return Err(unsupported(
                "namespace import",
                "only node:assert can be imported",
                Some(self.to_here(&start)),
            ));
        } else {
            local = self.cursor.identifier(Some("import"))?;
        }
        self.cursor.expect("from", Some("import"))?;
        let module = self.cursor.advance();
        if module.kind != TokenKind::String {
            return Err(Self::fail("expected a module name", &module));
        }
        self.cursor.eat(";");
        if module.value == "node:assert/strict" || module.value == "assert/strict" {
            strict = true;
        } else if module.value != "node:assert" && module.value != "assert" {
            return Err(unsupported(
                &format!("import from '{}'", module.value),
                "modules other than node:assert are outside the portable core",
                Some(span(&start, &module)),
            ));
        }
        if self.assertion.is_some() {
            return Err(unsupported(
                "second assertion import",
                "import node:assert once",
                Some(span(&start, &module)),
            ));
        }
        self.scope.tdz.remove(&local.value);
        self.assertion = Some(Assertion {
            name: local.value,
            strict,
        });
        Ok(())
    }

    /// `@typedef {{ $: 'leaf' } | { $: 'node', left: Tree, value: bigint }} Tree`
    /// declares a data type: each alternative is a constructor named by its
    /// `$` tag, and its other properties are the constructor's fields.
    pub(super) fn typedefs(&mut self) -> Result<Vec<SItem>> {
        let mut items = Vec::new();
        for comment in self.docs.clone() {
            for tag in jsdoc_tags(&comment)? {
                if tag.tag != "typedef" {
                    continue;
                }
                let range = Span::new(comment.start, comment.end);
                let (Some(text), Some(name)) = (non_empty(tag.ty), tag.name) else {
                    return Err(TranslationError::syntax(
                        "@typedef needs a type and a name",
                        Some(range),
                    ));
                };
                let ctors = self.typedef_ctors(&text, &name, range)?;
                if self.data_types.iter().any(|data| data.name == name) {
                    return Err(type_error(
                        format!("duplicate @typedef {name}"),
                        Some(range),
                    ));
                }
                self.data_types.push(SData {
                    name: name.clone(),
                    ctors: ctors.clone(),
                    span: None,
                    generated: false,
                });
                items.push(SItem::Data(SData {
                    name,
                    ctors,
                    span: Some(range),
                    generated: false,
                }));
            }
        }
        Ok(items)
    }

    /// The alternatives of a `@typedef` union type.
    pub(super) fn typedef_ctors(&self, text: &str, name: &str, range: Span) -> Result<Vec<SCtor>> {
        let mut c = TokenCursor::new(
            tokenize(text, Language::JavaScript)?.tokens,
            Language::JavaScript,
        );
        let mut ctors: Vec<SCtor> = Vec::new();
        loop {
            if !c.is("{") {
                return Err(unsupported(
                    &format!("@typedef {name}"),
                    "a portable data type is a union of { $: 'tag', … } object types",
                    Some(range),
                ));
            }
            c.advance();
            let mut tag = None;
            let mut fields = Vec::new();
            while !c.is("}") {
                let key = c.identifier(Some("object type"))?;
                c.expect(":", Some("object type"))?;
                if key.value == "$" {
                    let value = c.advance();
                    if value.kind != TokenKind::String {
                        return Err(unsupported(
                            &format!("@typedef {name}"),
                            "the $ tag must be a string literal type",
                            Some(range),
                        ));
                    }
                    tag = Some(value.value);
                } else {
                    let mut ty = c.identifier(Some("field type"))?.value;
                    if c.is("<") {
                        // Array<T>, whose closing brackets may be read as one >> token.
                        let mut depth = 0i32;
                        loop {
                            let token = c.advance();
                            for char in token.value.chars() {
                                depth += match char {
                                    '<' => 1,
                                    '>' => -1,
                                    _ => 0,
                                };
                            }
                            ty.push_str(&token.value);
                            if depth <= 0 || c.at_end() {
                                break;
                            }
                        }
                    }
                    while c.is("[") {
                        c.advance();
                        c.expect("]", Some("field type"))?;
                        ty.push_str("[]");
                    }
                    if !c.is(",") && !c.is(";") && !c.is("}") {
                        return Err(unsupported(
                            &format!("field type of {}", key.value),
                            "field types are number, bigint, boolean, string, a @typedef name or an array T[] of one",
                            Some(range),
                        ));
                    }
                    fields.push(SField {
                        name: Some(key.value),
                        ty: self.ty(&ty, range)?,
                        rocq_type: None,
                        span: None,
                    });
                }
                if c.eat(",").is_none() && c.eat(";").is_none() {
                    break;
                }
            }
            c.expect("}", Some("object type"))?;
            let Some(tag) = tag else {
                return Err(unsupported(
                    &format!("@typedef {name}"),
                    "every alternative needs a $ tag",
                    Some(range),
                ));
            };
            if ctors.iter().any(|ctor| ctor.name == tag) {
                return Err(type_error(
                    format!("duplicate tag {tag} in {name}"),
                    Some(range),
                ));
            }
            ctors.push(SCtor { name: tag, fields });
            if c.eat("|").is_none() {
                break;
            }
        }
        if !c.at_end() {
            return Err(unsupported(
                &format!("@typedef {name}"),
                "a portable data type is a union of object types",
                Some(range),
            ));
        }
        Ok(ctors)
    }

    #[allow(clippy::unused_self)] // a method, as in the JavaScript frontend
    pub(super) fn ty(&self, text: &str, range: Span) -> Result<Type> {
        jsdoc_type(text, range)
    }

    /// The `JSDoc` block directly before a declaration.
    pub(super) fn jsdoc_for(&self, token: &Token) -> Result<Option<JsDoc>> {
        let doc = self.docs.iter().rev().find(|comment| {
            comment.end <= token.start
                && self.source.units()[comment.end..token.start]
                    .iter()
                    .all(|&unit| is_js_space(unit))
        });
        let Some(doc) = doc else {
            return Ok(None);
        };
        let range = Span::new(doc.start, doc.end);
        let mut params: Vec<(String, String)> = Vec::new();
        let mut returns = None;
        for tag in jsdoc_tags(doc)? {
            if tag.tag == "param" {
                let (Some(ty), Some(name)) = (non_empty(tag.ty), tag.name) else {
                    return Err(TranslationError::syntax(
                        "@param needs a type and a name",
                        Some(range),
                    ));
                };
                if params.iter().any(|(param, _)| *param == name) {
                    return Err(type_error(format!("duplicate @param {name}"), Some(range)));
                }
                params.push((name, ty));
            } else if tag.tag == "returns" || tag.tag == "return" {
                returns = tag.ty;
            }
        }
        Ok(Some(JsDoc {
            params,
            returns,
            range,
        }))
    }

    pub(super) fn function_declaration(&mut self, start: &Token, is_async: bool) -> Result<SFn> {
        self.cursor
            .expect("function", Some("function declaration"))?;
        if self.cursor.is("*") {
            return Err(unsupported(
                "generator function",
                "generators are outside the portable core",
                Some(self.to_here(start)),
            ));
        }
        let name = self.cursor.identifier(Some("function"))?;
        self.function_rest(start, &name, is_async)
    }

    /// Parameters and body of a function or method; `doc_token` is where its `JSDoc` ends.
    pub(super) fn function_rest(
        &mut self,
        doc_token: &Token,
        name_token: &Token,
        is_async: bool,
    ) -> Result<SFn> {
        let doc = self.jsdoc_for(doc_token)?;
        self.cursor.expect("(", Some(&name_token.value))?;
        let tokens = self.parameters(&name_token.value)?;
        self.function_body(doc_token, name_token, doc, &tokens, Body::Block, is_async)
    }

    /// `const name = (…) => …` or `const name = function (…) { … }` before any
    /// top-level statement: a function, as nothing can call it before it is
    /// initialised. An arrow's expression body is the value it returns.
    pub(super) fn const_function(&mut self, start: &Token) -> Result<SFn> {
        let doc = self.jsdoc_for(start)?;
        self.cursor.expect("const", Some("function"))?;
        let name_token = self.cursor.identifier(Some("function"))?;
        self.cursor.expect("=", Some("function"))?;
        self.scope.tdz.remove(&name_token.value);
        let is_async = self.cursor.is("async") && !self.cursor.is_at("=>", 1);
        if is_async {
            self.cursor.advance();
        }
        let function = if self.cursor.eat("function").is_some() {
            if self.cursor.is("*") {
                return Err(unsupported(
                    "generator function",
                    "generators are outside the portable core",
                    Some(self.to_here(start)),
                ));
            }
            if self.cursor.is_kind(TokenKind::Identifier) {
                let inner = self.peek();
                if inner.value != name_token.value {
                    return Err(unsupported(
                        &format!("function expression {}", inner.value),
                        &format!(
                            "its own name is visible only inside it; call it {}",
                            name_token.value
                        ),
                        Some(span(&inner, &inner)),
                    ));
                }
                self.cursor.advance();
            }
            self.cursor.expect("(", Some(&name_token.value))?;
            let tokens = self.parameters(&name_token.value)?;
            self.function_body(start, &name_token, doc, &tokens, Body::Block, is_async)?
        } else {
            let tokens = if self.cursor.eat("(").is_some() {
                self.parameters(&name_token.value)?
            } else {
                vec![self.cursor.identifier(Some("parameter"))?]
            };
            self.cursor.expect("=>", Some(&name_token.value))?;
            self.function_body(start, &name_token, doc, &tokens, Body::Arrow, is_async)?
        };
        self.cursor.eat(";");
        Ok(function)
    }

    /// Names of the top-level async functions, which every call must await.
    fn async_declarations(&self, index: usize) -> HashSet<String> {
        let tokens = &self.cursor.tokens;
        let value = |at: usize| tokens.get(at).map(|token| token.value.as_str());
        let identifier = |at: usize| {
            tokens
                .get(at)
                .filter(|token| token.kind == TokenKind::Identifier)
        };
        let mut names = HashSet::new();
        let mut depth = 0_i64;
        for (at, token) in tokens.iter().enumerate().skip(index) {
            if token.kind == TokenKind::Punct {
                match token.value.as_str() {
                    "(" | "[" | "{" => depth += 1,
                    ")" | "]" | "}" => depth -= 1,
                    _ => {}
                }
            }
            if depth != 0 || token.kind != TokenKind::Identifier {
                continue;
            }
            if token.value == "async" && value(at + 1) == Some("function") {
                if let Some(name) = identifier(at + 2) {
                    names.insert(name.value.clone());
                }
            }
            if token.value == "const"
                && value(at + 2) == Some("=")
                && value(at + 3) == Some("async")
                && value(at + 4) != Some("=>")
                && self.starts_function(at + 3)
            {
                if let Some(name) = identifier(at + 1) {
                    names.insert(name.value.clone());
                }
            }
        }
        names
    }

    /// Whether the tokens from `index` are a function expression or an arrow
    /// function, either maybe async.
    pub(super) fn starts_function(&self, index: usize) -> bool {
        let tokens = &self.cursor.tokens;
        let Some(mut token) = tokens.get(index) else {
            return false;
        };
        let mut index = index;
        if token.kind == TokenKind::Identifier
            && token.value == "async"
            && tokens.get(index + 1).map(|next| next.value.as_str()) != Some("=>")
        {
            index += 1;
            let Some(next) = tokens.get(index) else {
                return false;
            };
            token = next;
        }
        let arrow = |at: usize| tokens.get(at).is_some_and(|token| token.value == "=>");
        match token.kind {
            TokenKind::Identifier => token.value == "function" || arrow(index + 1),
            TokenKind::Punct => token.value == "(" && arrow(self.matching(index) + 1),
            _ => false,
        }
    }

    /// A parameter list after its `(`, through its `)`.
    fn parameters(&mut self, name: &str) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        while !self.cursor.is(")") {
            let token = self.peek();
            if self.cursor.is("...") {
                return Err(unsupported(
                    "rest parameter",
                    "functions take a fixed number of arguments",
                    Some(span(&token, &token)),
                ));
            }
            if self.cursor.is("{") || self.cursor.is("[") {
                return Err(unsupported(
                    "destructured parameter",
                    "name each parameter",
                    Some(span(&token, &token)),
                ));
            }
            let param = self.cursor.identifier(Some("parameter"))?;
            if self.cursor.is("=") {
                return Err(unsupported(
                    "default parameter",
                    "every argument must be passed",
                    Some(self.to_here(&param)),
                ));
            }
            tokens.push(param);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect(")", Some(name))?;
        Ok(tokens)
    }

    /// A function from its parameters; `body` says how its body is written.
    /// An async function is the function its body computes, and its `JSDoc`
    /// may declare the result as `Promise<T>`.
    fn function_body(
        &mut self,
        doc_token: &Token,
        name_token: &Token,
        doc: Option<JsDoc>,
        tokens: &[Token],
        body: Body,
        is_async: bool,
    ) -> Result<SFn> {
        let name = name_token.value.clone();
        if is_async {
            self.sequential_async = true;
        }
        // A type JSDoc does not declare is inferred once the whole program is read.
        let mut params = Vec::new();
        for token in tokens {
            reserved(token)?;
            let text = doc.as_ref().and_then(|doc| {
                doc.params
                    .iter()
                    .find(|(param, _)| *param == token.value)
                    .map(|(_, text)| (text.clone(), doc.range))
            });
            params.push(SParam {
                name: token.value.clone(),
                ty: text
                    .map(|(text, range)| self.ty(&text, range))
                    .transpose()?,
                span: Some(span(token, token)),
                guard: None,
                rocq_type: None,
            });
        }
        if let Some(doc) = &doc {
            for (documented, _) in &doc.params {
                if !params.iter().any(|param| param.name == *documented) {
                    return Err(type_error(
                        format!("@param {documented} is not a parameter of {name}"),
                        Some(doc.range),
                    ));
                }
            }
        }
        let ret =
            match doc.and_then(|doc| non_empty(doc.returns).map(|returns| (returns, doc.range))) {
                Some((returns, range)) => {
                    let promised = if is_async { promised(&returns) } else { None };
                    Some(self.ty(promised.unwrap_or(&returns), range)?)
                }
                None => None,
            };
        let names: HashSet<String> = params.iter().map(|param| param.name.clone()).collect();
        let outer = std::mem::replace(
            &mut self.scope,
            Scope {
                locals: names.clone(),
                tdz: HashSet::new(),
                mutable: names,
            },
        );
        let outer_async = std::mem::replace(&mut self.in_async, is_async);
        let outer_jumps = std::mem::take(&mut self.jumps);
        let outer_top_level = std::mem::replace(&mut self.top_level, false);
        let statements = match body {
            Body::Arrow if !self.cursor.is("{") => {
                let token = self.peek();
                let expr = if is_async {
                    self.awaited(Self::expr)
                } else {
                    self.expr()
                };
                expr.map(|expr| {
                    vec![Stmt::Return {
                        expr,
                        span: self.to_here(&token),
                    }]
                })
            }
            _ => self.block_statements(),
        };
        self.scope = outer;
        self.in_async = outer_async;
        self.jumps = outer_jumps;
        self.top_level = outer_top_level;
        let statements = statements?;
        // `if (n < 0n) throw …` as a leading statement makes `n` a natural number,
        // unless the body assigns it another value.
        let assigned = statement_uses(&statements).assigned;
        let mut index = 0;
        while index < statements.len() {
            let Some(param) = guarded_parameter(&statements[index], &params) else {
                break;
            };
            if assigned.contains(&params[param].name) {
                break;
            }
            params[param].ty = Some(NAT);
            params[param].guard = Some(true);
            index += 1;
        }
        let place = self.to_here(name_token);
        let tail = &statements[index..];
        let body = if imperative(tail) {
            lower_imperative(self, &name, &params, tail, place)?
        } else {
            lower(tail, place)?
        };
        Ok(SFn {
            name,
            params,
            ret,
            body,
            span: Some(self.to_here(doc_token)),
            generated: false,
        })
    }

    /// `const Name = { method(…) { … }, Nested: { … } };` is a module.
    pub(super) fn namespace(&mut self, start: &Token) -> Result<SModule> {
        self.cursor.expect("const", Some("namespace"))?;
        let name = self.cursor.identifier(Some("namespace"))?;
        self.cursor.expect("=", Some("namespace"))?;
        self.scope.tdz.remove(&name.value);
        let module = self.namespace_body(name.value, start)?;
        self.cursor.eat(";");
        Ok(module)
    }

    pub(super) fn namespace_body(&mut self, name: String, start: &Token) -> Result<SModule> {
        self.cursor.expect("{", Some("namespace"))?;
        let mut items = Vec::new();
        while !self.cursor.is("}") {
            let member = self.peek();
            if member.kind != TokenKind::Identifier {
                return Err(unsupported(
                    &format!("namespace member {}", describe(&member)),
                    "members are methods or nested namespaces with identifier names",
                    Some(span(&member, &member)),
                ));
            }
            if ["get", "set", "async", "static"].contains(&member.value.as_str())
                && self.cursor.is_kind_at(TokenKind::Identifier, 1)
            {
                return Err(unsupported(
                    &format!("{} member", member.value),
                    "accessors and asynchronous methods are outside the portable core",
                    Some(span(&member, self.cursor.peek_at(1))),
                ));
            }
            self.cursor.advance();
            if self.cursor.is("(") {
                items.push(SItem::Fn(self.function_rest(&member, &member, false)?));
            } else if self.cursor.eat(":").is_some() {
                if self.cursor.is("{") {
                    items.push(SItem::Module(
                        self.namespace_body(member.value.clone(), &member)?,
                    ));
                } else {
                    return Err(unsupported(
                        &format!("namespace property {}", member.value),
                        "namespaces hold methods, name(…) { … }, and nested namespaces only",
                        Some(self.to_here(&member)),
                    ));
                }
            } else {
                return Err(self.fail_here(&format!("expected ( or : after {}", member.value)));
            }
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("}", Some("namespace"))?;
        Ok(SModule {
            name,
            items,
            span: Some(self.to_here(start)),
        })
    }

    /// Names a block declares with `const` or `let`, which are in their temporal dead zone until declared.
    pub(super) fn block_declarations(&mut self, from: usize, end: ScanEnd) -> HashSet<String> {
        let mut names = HashSet::new();
        let saved = self.cursor.index;
        self.cursor.index = from;
        let mut depth = 0_i64;
        loop {
            let done = match end {
                ScanEnd::File => self.cursor.at_end(),
                ScanEnd::Brace => self.cursor.is("}"),
            };
            if self.cursor.at_end() || (depth == 0 && done) {
                break;
            }
            let token = self.cursor.advance();
            let punct = token.kind == TokenKind::Punct;
            if punct && ["{", "(", "["].contains(&token.value.as_str()) {
                depth += 1;
            } else if punct && ["}", ")", "]"].contains(&token.value.as_str()) {
                depth -= 1;
            } else if depth == 0
                && token.kind == TokenKind::Identifier
                && (token.value == "const" || token.value == "let")
            {
                let c = &self.cursor;
                if c.peek().kind == TokenKind::Identifier {
                    names.insert(c.peek().value.clone());
                } else if c.is("{") {
                    let mut offset = 1;
                    while !c.is_at("}", offset) && !c.is_kind_at(TokenKind::Eof, offset) {
                        if c.peek_at(offset).kind == TokenKind::Identifier
                            && (c.is_at(",", offset + 1) || c.is_at("}", offset + 1))
                        {
                            names.insert(c.peek_at(offset).value.clone());
                        }
                        offset += 1;
                    }
                }
            }
        }
        self.cursor.index = saved;
        names
    }

    pub(super) fn declare_local(&mut self, name: &str, mutable: bool) {
        self.scope.locals.insert(name.to_owned());
        self.scope.tdz.remove(name);
        if mutable {
            self.scope.mutable.insert(name.to_owned());
        } else {
            self.scope.mutable.remove(name);
        }
    }

    pub(super) fn block_statements(&mut self) -> Result<Vec<Stmt>> {
        self.cursor.expect("{", Some("block"))?;
        let mut tdz = self.scope.tdz.clone();
        tdz.extend(self.block_declarations(self.cursor.index, ScanEnd::Brace));
        let inner = Scope {
            locals: self.scope.locals.clone(),
            tdz,
            mutable: self.scope.mutable.clone(),
        };
        let outer = std::mem::replace(&mut self.scope, inner);
        let statements = self.block_body();
        self.scope = outer;
        statements
    }

    pub(super) fn block_body(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.cursor.is("}") {
            if self.cursor.at_end() {
                return Err(self.fail_here("expected }"));
            }
            statements.push(self.statement()?);
        }
        self.cursor.expect("}", Some("block"))?;
        Ok(statements)
    }
}

/// `T` of an async function's `@returns {Promise<T>}`.
fn promised(text: &str) -> Option<&str> {
    js_trim(text)
        .strip_prefix("Promise")
        .map(js_trim)?
        .strip_prefix('<')?
        .strip_suffix('>')
}

/// The portable type a `JSDoc` type names.
fn jsdoc_type(text: &str, range: Span) -> Result<Type> {
    let name = js_trim(text);
    match name {
        "bigint" => return Ok(INT),
        "boolean" => return Ok(BOOL),
        "string" => return Ok(STRING),
        "number" => return Ok(FLOAT),
        _ => {}
    }
    if let Some(element) = name.strip_suffix("[]") {
        return Ok(array(jsdoc_type(element, range)?));
    }
    let generic = name
        .strip_prefix("Array<")
        .or_else(|| name.strip_prefix("ReadonlyArray<"))
        .and_then(|rest| rest.strip_suffix('>'))
        .filter(|element| !element.is_empty());
    if let Some(element) = generic {
        return Ok(array(jsdoc_type(element, range)?));
    }
    let reserved = [
        "object",
        "any",
        "unknown",
        "void",
        "undefined",
        "null",
        "Object",
        "Function",
        "Array",
        "symbol",
    ];
    if !is_identifier_name(name) || reserved.contains(&name) {
        return Err(unsupported(
            &format!("JSDoc type {{{name}}}"),
            "portable types are number, bigint, boolean, string, arrays T[] of them and @typedef data types",
            Some(range),
        ));
    }
    Ok(Type::Named {
        path: vec![ROOT.to_owned(), name.to_owned()],
        span: Some(range),
    })
}
