impl JavaScriptParser {
    /// `const Name = { method(…) { … }, Nested: { … } };` is a module.
    pub(super) fn namespace(&mut self, start: &Token) -> Result<SModule> {
        self.cursor.expect("const", Some("namespace"))?;
        let name = self.cursor.identifier(Some("namespace"))?;
        global(&name)?;
        self.lexical.declaration(&name.value, name.span(), 1);
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
            if !accept_object_property_definition(&member.value, false, self.cursor.is(":")) {
                return Err(unsupported(
                    "object prototype definition",
                    "an uncomputed __proto__: entry is prototype-definition syntax rather than an own data property; the portable core does not represent object prototypes",
                    Some(span(&member, &member)),
                ));
            }
            if super::surface_rules::accept_java_script_namespace_method(
                "identifier",
                &self.cursor.peek().value,
            ) {
                self.function_names
                    .insert(format!("{name}.{}", member.value));
                self.owned_array_reader_namespace += 1;
                let function = self.function_rest(&member, &member, false, false);
                self.owned_array_reader_namespace -= 1;
                items.push(SItem::Fn(function?));
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
    pub(super) fn block_declarations(&mut self, from: usize, _end: ScanEnd) -> HashSet<String> {
        self.block_declaration_counts(from).0
    }

    fn block_declaration_counts(
        &mut self,
        from: usize,
    ) -> (HashSet<String>, HashMap<String, usize>) {
        let mut names = HashSet::new();
        let mut counts = HashMap::new();
        let mut collect = |name: String| {
            names.insert(name.clone());
            *counts.entry(name).or_insert(0) += 1;
        };
        let saved = self.cursor.index;
        self.cursor.index = from;
        // Signed Number depth retains the shared parser's malformed-delimiter
        // behavior without unsigned underflow. No token columns are copied.
        let mut depth = 0.0;
        let mut declaration = false;
        loop {
            let token = self.cursor.advance();
            let kind = match token.kind {
                TokenKind::Punct => "punct",
                TokenKind::Identifier => "identifier",
                TokenKind::Eof => "eof",
                _ => "other",
            };
            let step = crate::translation::frontend_rules::ml_array::number_index(
                read_java_script_lexical_declaration_step(kind, &token.value, depth, declaration),
            );
            match step {
                None => break,
                Some(1) => depth += 1.0,
                Some(2) => depth -= 1.0,
                Some(3 | 4) => declaration = true,
                Some(5) => declaration = false,
                _ => {}
            }
            if !matches!(step, Some(3 | 4)) {
                continue;
            }
            let binding_start = self.cursor.index;
            let c = &self.cursor;
            if c.peek().kind == TokenKind::Identifier {
                collect(c.peek().value.clone());
            } else if c.is("{") {
                for token in self.object_binding_names(c.index) {
                    collect(token.value);
                }
            } else if c.is("[") {
                for token in self.array_binding_names(c.index) {
                    collect(token.value);
                }
            }
            self.cursor.index = binding_start;
        }
        self.cursor.index = saved;
        (names, counts)
    }

    pub(super) fn declare_local(&mut self, name: &str, mutable: bool) {
        self.lexical.binding(name, None);
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
        self.with_block_declarations(Self::block_body)
    }

    pub(super) fn with_block_declarations<T>(
        &mut self,
        parse: impl FnOnce(&mut Self) -> Result<T>,
    ) -> Result<T> {
        let mut tdz = self.scope.tdz.clone();
        let (names, counts) = self.block_declaration_counts(self.cursor.index);
        let names = names.into_iter().collect::<Vec<_>>();
        tdz.extend(names.iter().cloned());
        let inner = Scope {
            locals: self.scope.locals.clone(),
            tdz,
            mutable: self.scope.mutable.clone(),
        };
        let outer = std::mem::replace(&mut self.scope, inner);
        let statements = self.with_lexical_scope(&names, 0, |parser| {
            parser.lexical.declaration_counts(counts);
            parse(parser)
        });
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
