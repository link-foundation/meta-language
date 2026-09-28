//! Files, imports, `JSDoc` typedefs, functions, namespaces and blocks.

use super::{
    describe, guarded_parameter, is_identifier_name, is_js_space, js_trim, jsdoc_tags, lower,
    non_empty, span, tokenize, type_error, unsupported, Assertion, HashSet, JavaScriptParser,
    JsDoc, Language, Result, SCtor, SData, SField, SFn, SItem, SMain, SModule, SParam, SProgram,
    ScanEnd, Scope, Span, Stmt, Token, TokenCursor, TokenKind, TranslationError, Type, BOOL, FLOAT,
    INT, NAT, ROOT, STRING,
};

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
        };
        while !self.cursor.at_end() {
            let start = self.peek();
            if self.cursor.is("import") {
                self.import_declaration()?;
                continue;
            }
            if self.cursor.eat("export").is_some()
                && !self.cursor.is("function")
                && !self.cursor.is("const")
            {
                return Err(unsupported(
                    &format!("export {}", describe(self.cursor.peek())),
                    "only function and const declarations are exported",
                    Some(self.to_here(&start)),
                ));
            }
            if self.cursor.is("async") {
                return Err(unsupported(
                    "async function",
                    "asynchronous code is outside the portable core",
                    Some(self.to_here(&start)),
                ));
            }
            if self.cursor.is("function") {
                items.push(SItem::Fn(self.function_declaration(&start)?));
                continue;
            }
            if self.cursor.is("const")
                && self.cursor.is_kind_at(TokenKind::Identifier, 1)
                && self.cursor.is_at("=", 2)
                && self.cursor.is_at("{", 3)
                && !(self.cursor.is_at("$", 4) && self.cursor.is_at(":", 5))
            {
                if !effects.is_empty() {
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
            effects.push(self.main_statement()?);
        }
        Ok(SProgram {
            language: Language::JavaScript,
            items,
            main: Some(SMain {
                effects,
                span: Some(Span::new(0, self.source.len())),
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
                });
                items.push(SItem::Data(SData {
                    name,
                    ctors,
                    span: Some(range),
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
                    let ty = c.identifier(Some("field type"))?;
                    if !c.is(",") && !c.is(";") && !c.is("}") {
                        return Err(unsupported(
                            &format!("field type of {}", key.value),
                            "field types are bigint, boolean, string or a @typedef name",
                            Some(range),
                        ));
                    }
                    fields.push(SField {
                        name: Some(key.value),
                        ty: self.ty(&ty.value, range)?,
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
        let name = js_trim(text);
        match name {
            "bigint" => return Ok(INT),
            "boolean" => return Ok(BOOL),
            "string" => return Ok(STRING),
            "number" => return Ok(FLOAT),
            _ => {}
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
                "portable types are number, bigint, boolean, string and @typedef data types",
                Some(range),
            ));
        }
        Ok(Type::Named {
            path: vec![ROOT.to_owned(), name.to_owned()],
            span: Some(range),
        })
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

    pub(super) fn function_declaration(&mut self, start: &Token) -> Result<SFn> {
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
        self.function_rest(start, &name)
    }

    /// Parameters and body of a function or method; `doc_token` is where its `JSDoc` ends.
    pub(super) fn function_rest(&mut self, doc_token: &Token, name_token: &Token) -> Result<SFn> {
        let doc = self.jsdoc_for(doc_token)?;
        let name = name_token.value.clone();
        self.cursor.expect("(", Some(&name))?;
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
        self.cursor.expect(")", Some(&name))?;
        let place = self.to_here(doc_token);
        let Some(doc) = doc else {
            return Err(unsupported(
                &format!("untyped function {name}"),
                "declare its types with JSDoc @param and @returns tags",
                Some(place),
            ));
        };
        let mut params = Vec::new();
        for token in &tokens {
            let Some((_, text)) = doc.params.iter().find(|(param, _)| *param == token.value) else {
                return Err(unsupported(
                    &format!("untyped parameter {}", token.value),
                    "give every parameter a JSDoc @param type",
                    Some(span(token, token)),
                ));
            };
            params.push(SParam {
                name: token.value.clone(),
                ty: Some(self.ty(text, doc.range)?),
                span: Some(span(token, token)),
                guard: None,
                rocq_type: None,
            });
        }
        for (documented, _) in &doc.params {
            if !params.iter().any(|param| param.name == *documented) {
                return Err(type_error(
                    format!("@param {documented} is not a parameter of {name}"),
                    Some(doc.range),
                ));
            }
        }
        let Some(returns) = non_empty(doc.returns) else {
            return Err(unsupported(
                &format!("function {name} without @returns"),
                "declare the result type with a JSDoc @returns tag",
                Some(place),
            ));
        };
        let ret = self.ty(&returns, doc.range)?;
        let outer = std::mem::replace(
            &mut self.scope,
            Scope {
                locals: params.iter().map(|param| param.name.clone()).collect(),
                tdz: HashSet::new(),
            },
        );
        let statements = self.block_statements();
        self.scope = outer;
        let statements = statements?;
        // `if (n < 0n) throw …` as a leading statement makes `n` a natural number.
        let mut index = 0;
        while index < statements.len() {
            let Some(param) = guarded_parameter(&statements[index], &params) else {
                break;
            };
            params[param].ty = Some(NAT);
            params[param].guard = Some(true);
            index += 1;
        }
        let body = lower(&statements[index..], self.to_here(name_token))?;
        Ok(SFn {
            name,
            params,
            ret: Some(ret),
            body,
            span: Some(self.to_here(doc_token)),
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
                items.push(SItem::Fn(self.function_rest(&member, &member)?));
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

    /// Names a block declares with `const`, which are in their temporal dead zone until declared.
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
            } else if depth == 0 && token.kind == TokenKind::Identifier && token.value == "const" {
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

    pub(super) fn declare_local(&mut self, name: &str) {
        self.scope.locals.insert(name.to_owned());
        self.scope.tdz.remove(name);
    }

    pub(super) fn block_statements(&mut self) -> Result<Vec<Stmt>> {
        self.cursor.expect("{", Some("block"))?;
        let mut tdz = self.scope.tdz.clone();
        tdz.extend(self.block_declarations(self.cursor.index, ScanEnd::Brace));
        let inner = Scope {
            locals: self.scope.locals.clone(),
            tdz,
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
