//! Complete file assembly at the original JavaScript parser boundary.

use super::*;

impl JavaScriptParser {
    pub(in super::super) fn file(mut self) -> Result<SProgram> {
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
        self.global_binding_names.clone_from(&self.scope.tdz);
        self.prescan_product_bindings();
        self.async_names = self.async_declarations(self.cursor.index);
        // Top-level statements that declare or assign variables, lowered together before the next statement that prints, binds a constant or asserts.
        let mut pending = Vec::new();
        while !self.cursor.at_end() {
            let start = self.peek();
            if self.cursor.is("import") {
                self.import_declaration()?;
                continue;
            }
            let exports_async =
                |cursor: &TokenCursor| cursor.is("async") && cursor.is_at("function", 1);
            let exported = self.cursor.is("export");
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
                let function = self.function_declaration(&start, true)?;
                if exported {
                    self.exported_names.insert(function.name.clone());
                }
                items.push(SItem::Fn(function));
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
                let function = self.function_declaration(&start, false)?;
                if exported {
                    self.exported_names.insert(function.name.clone());
                }
                items.push(SItem::Fn(function));
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
                let function = self.const_function(&start)?;
                if exported {
                    self.exported_names.insert(function.name.clone());
                }
                items.push(SItem::Fn(function));
                continue;
            }
            if self.cursor.is("const")
                && self.cursor.is_kind_at(TokenKind::Identifier, 1)
                && self.cursor.is_at("=", 2)
                && self.cursor.is_at("{", 3)
                && !(self.cursor.is_at("$", 4) && self.cursor.is_at(":", 5))
            {
                if super::super::surface_rules::read_java_script_object_statement_form(
                    false,
                    false,
                    !effects.is_empty() || !pending.is_empty(),
                ) == "refuse-order"
                {
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
                let close = self.matching(self.cursor.index + 3);
                let methods = (self.cursor.index + 4..close).any(|index| {
                    let token = &self.cursor.tokens[index];
                    super::super::surface_rules::accept_java_script_namespace_method(
                        if token.kind == TokenKind::Identifier {
                            "identifier"
                        } else {
                            ""
                        },
                        self.cursor
                            .tokens
                            .get(index + 1)
                            .map_or("", |next| next.value.as_str()),
                    )
                });
                if super::super::surface_rules::read_java_script_object_statement_form(
                    false, methods, false,
                ) == "namespace"
                {
                    let module = self.namespace(&start)?;
                    if exported {
                        self.exported_names.insert(module.name.clone());
                    }
                    items.push(SItem::Module(module));
                    continue;
                }
            }
            if self.starts_top_level_imperative() {
                pending.push(self.top_level_statement()?);
                continue;
            }
            self.flush(&mut pending, &mut effects)?;
            effects.extend(self.main_statement()?);
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
        if !self.eliminated_owned_array_append_operations.is_empty() {
            self.reference_semantics = self.reference_semantics_forced
                || super::super::reference_analysis::requires_reference_semantics(
                    &self.cursor.tokens,
                    &self.eliminated_owned_array_append_operations,
                );
        }
        let program = SProgram {
            reference_semantics: self.reference_semantics.then(|| true.into()),
            language: Language::JavaScript,
            items,
            main: Some(SMain {
                effects,
                span: Some(Span::new(0, self.source.len())),
                sequential_async: self.sequential_async,
                source_runtime_worker: None,
            }),
            imports: std::mem::take(&mut self.imports),
            externals: std::mem::take(&mut self.externals),
            source_pure_unit_calls: None,
            source_runtime_shell: None,
        };
        if has_javascript_products(&program, &self.nested_product_bindings) {
            let (mut program, types) = infer_javascript_product_types(program)?;
            validate_javascript_products(
                &mut program,
                &types,
                &self.exported_names,
                &self.nested_product_bindings,
            )?;
            super::super::scalar_loops::finish_scalar_loops(&mut program)?;
            super::super::reference_analysis::select_structural_reference_semantics(&mut program)?;
            Ok(program)
        } else {
            let mut program = infer_javascript_types(program)?;
            super::super::scalar_loops::finish_scalar_loops(&mut program)?;
            super::super::reference_analysis::select_structural_reference_semantics(&mut program)?;
            Ok(program)
        }
    }
}
