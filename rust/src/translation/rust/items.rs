//! Attributes, items, `use` trees, enums, types, functions, blocks and `main`.

use super::array_rules::accept_rust_source_array_owner;
use super::indirection_rules::accept_rust_source_box_owner;
use super::read_only_query_rules::accept_rust_read_only_query_owner;
use super::{
    Binding, FLOAT, ItemsEnd, Language, Options, RESERVED_ITEMS, Result, RustParser, SComparison,
    SCtor, SData, SEffect, SExpr, SField, SItem, SMain, SModule, SNode, SProgram, SProp, SPropNode,
    ShowStyle, SourceDeclaration, SourceOrigin, Token, TokenKind, TranslationError, Type, absolute,
    is_if_or_match, prop_of, span, unsupported,
};

mod attributes;
mod constants;
mod functions;
mod product_bindings;
mod source_headers;
mod types;

impl RustParser {
    pub(super) fn file(&mut self) -> Result<SProgram> {
        self.inner_attributes()?;
        let mut items = self.items(&[], ItemsEnd::File)?;
        self.install_ordinary_completion_declarations(&mut items)?;
        let main = self.main.take();
        if !self.shared_array_handle_source_model.is_empty() {
            self.finish_shared_array_source(&items, main.as_ref())?;
            return Ok(SProgram {
                reference_semantics: Some(true.into()),
                language: Language::Rust,
                items,
                main,
                imports: Vec::new(),
                externals: Vec::new(),
                source_pure_unit_calls: None,
                source_runtime_shell: None,
            });
        }
        let Some(main) = main else {
            return Err(TranslationError::syntax(
                "a Rust program needs fn main",
                Some(self.cursor.peek().span()),
            ));
        };
        if !self.primitive_owner_model.is_empty()
            && (items.iter().any(|item| !matches!(item, SItem::Fn(_))) || !main.effects.is_empty())
        {
            return Err(unsupported(
                "Rust primitive owner profile",
                "only flat primitive leaf functions and an empty main are admitted",
                None,
            ));
        }
        self.finish_empty_option_source(&mut items, &main)?;
        let source_pure_unit_calls = self.finish_pure_unit_source(&items, &main)?;
        self.require_owned_local_program(&items, &main)?;
        self.require_const_function_program(&items, &main)?;
        Ok(SProgram {
            reference_semantics: None,
            language: Language::Rust,
            items,
            main: Some(main),
            imports: Vec::new(),
            externals: Vec::new(),
            source_pure_unit_calls,
            source_runtime_shell: None,
        })
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
            let first_attribute_index = self.cursor.index;
            let mut diagnostic_attributes = Vec::new();
            let derivations = self.attributes(Some(&mut diagnostic_attributes))?;
            let start = self.peek();
            self.module_path = path.to_vec();
            self.pure_unit_item_context(path, &start)?;
            self.const_function_visibility_guard()?;
            self.visibility()?;
            self.const_function_item_guard(path, &start)?;
            if !(diagnostic_attributes.is_empty()
                || self.cursor.is("const") && self.cursor.is_at("fn", 1))
            {
                return Err(unsupported(
                    "Rust const diagnostic attribute attachment",
                    "the supported diagnostic prefix must belong to an actual const function",
                    span(
                        &self.cursor.tokens[first_attribute_index],
                        self.cursor.peek(),
                    ),
                ));
            }
            if !self.const_function_model.is_empty()
                && self.cursor.is("const")
                && self.cursor.is_at("fn", 1)
            {
                let const_keyword_index = self.cursor.index;
                self.const_function_keyword = Some(self.cursor.advance());
                let mut function = self.function(path)?;
                if !diagnostic_attributes.is_empty() {
                    function.source_diagnostic_attributes =
                        Some(Box::new(self.create_diagnostic_attribute_facet(
                            first_attribute_index,
                            const_keyword_index,
                            &function,
                            path,
                        )?));
                }
                items.push(SItem::Fn(function));
                continue;
            }
            self.empty_option_item_guard(path, &start)?;
            self.require_unit_clone_attached_enum(&derivations, &start)?;
            self.require_type_namespace_without_externals(&start)?;
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
                self.inner_attributes()?;
                self.origins
                    .declaration(&name, path, SourceDeclaration::Module)?;
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
            if path.is_empty()
                && c.is("extern")
                && c.is_at("crate", 1)
                && c.is_at("alloc", 2)
                && c.is_at(";", 3)
            {
                if !self.local_value_state_model.is_empty() {
                    return Err(unsupported(
                        "Rust owned local state",
                        "external crate declarations require a separate namespace interpretation",
                        None,
                    ));
                }
                if !self.crate_declarations.insert("alloc".to_owned()) {
                    return Err(unsupported(
                        "extern crate alloc",
                        "each source crate declaration must be unique in its module",
                        span(&start, &start),
                    ));
                }
                for _ in 0..4 {
                    c.advance();
                }
                self.origins
                    .declaration("alloc", path, SourceDeclaration::ExternalCrate)?;
                continue;
            }
            if c.is("use") {
                self.use_declaration(path)?;
                continue;
            }
            if c.is("enum") {
                items.push(self.enum_item(path, &derivations)?);
                continue;
            }
            if c.is("struct") {
                items.push(SItem::Data(self.record_item(path)?));
                continue;
            }
            if c.is("impl") {
                items.push(SItem::Module(self.inherent_item(path)?));
                continue;
            }
            if c.is("fn") || c.is("const") {
                if c.is("fn") && c.is_at("main", 1) && path.is_empty() {
                    if !self.const_function_model.is_empty() && self.empty_option_main_seen {
                        return Err(unsupported(
                            "Rust const function source",
                            "the complete source must have one ordinary main declaration",
                            span(&start, self.cursor.peek()),
                        ));
                    }
                    if !self.pure_unit_call_model.is_empty() && self.empty_option_main_seen {
                        return Err(unsupported(
                            "Rust pure Unit-call source",
                            "the complete source must have one owned main declaration",
                            span(&start, self.cursor.peek()),
                        ));
                    }
                    self.empty_option_main_guard(&start)?;
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
        if !self.local_value_state_model.is_empty() {
            return Err(unsupported(
                "Rust owned local state",
                "imports require a separate actual owner and namespace interpretation",
                None,
            ));
        }
        self.require_type_namespace_without_imports()?;
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
        let standard_box = full.len() > 1 && accept_rust_source_box_owner(&full.join("::"), false);
        let standard_vector =
            full.len() > 1 && accept_rust_source_array_owner(&full.join("::"), false);
        let standard_string =
            accept_rust_read_only_query_owner("string empty", "library", &full.join("::"));
        let query_span = span(start, c.peek());
        let standard_primitive = self
            .read_standard_primitive_position(&full, path)?
            .is_some();
        let c = &mut self.cursor;
        if !self.primitive_owner_model.is_empty() && !standard_primitive {
            return Err(unsupported(
                "Rust primitive owner import",
                "only the two actual standard primitive reexports can be imported",
                query_span,
            ));
        }
        if !["crate", "super", "self"].contains(&full[0].as_str())
            && !standard_box
            && !standard_vector
            && !standard_string
            && !standard_primitive
        {
            return Err(unsupported(
                &format!("use {}", full.join("::")),
                "only items of this crate can be imported; the standard library is outside the portable core",
                span(start, c.peek()),
            ));
        }
        if standard_box {
            self.origins
                .box_owner(&full, &full, path, span(start, c.peek()))?;
        }
        if standard_vector {
            self.origins
                .vector_owner(&full, &full, path, span(start, c.peek()))?;
        }
        let alias = if c.eat("as").is_some() {
            c.identifier(Some("use alias"))?.value
        } else {
            segment
        };
        if !self.primitive_owner_model.is_empty()
            && (!super::primitive_owners::accept_import_name(&alias)
                || !self.primitive_import_names.insert(alias.clone()))
        {
            return Err(unsupported(
                "Rust primitive owner alias",
                "each import needs one distinct ASCII identifier outside the standard crate namespace",
                span(start, c.peek()),
            ));
        }
        let target = if standard_box || standard_vector || standard_string || standard_primitive {
            full
        } else {
            absolute(&full, path, span(start, c.peek()))?
        };
        if standard_primitive
            && !self
                .primitive_imports
                .iter()
                .any(|record| record["name"].as_str() == Some(alias.as_str()))
        {
            self.primitive_imports
                .push(serde_json::json!({"name":alias,"target":target}));
        }
        self.aliases
            .entry(path.join("."))
            .or_default()
            .insert(alias, target);
        Ok(())
    }

    /// A `use` alias is visible only in the module that declares it.
    pub(super) fn resolve(&self, segments: &[String], path: &[String]) -> Vec<String> {
        if let Some(inherent) = self.inherent_path(segments, path) {
            return inherent;
        }
        self.aliases
            .get(&path.join("."))
            .and_then(|aliases| aliases.get(&segments[0]))
            .map_or_else(
                || segments.to_vec(),
                |target| target.iter().chain(&segments[1..]).cloned().collect(),
            )
    }

    pub(super) fn enum_item(
        &mut self,
        path: &[String],
        derivations: &[serde_json::Value],
    ) -> Result<SItem> {
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
        let owner = path
            .iter()
            .chain(std::iter::once(&name))
            .cloned()
            .collect::<Vec<_>>()
            .join(".");
        self.origins
            .declaration(&name, path, SourceDeclaration::Data)?;
        while !self.cursor.is("}") {
            let attributes = self.attributes(None)?;
            self.require_unit_clone_attached_variant(&attributes)?;
            let variant = self.cursor.identifier(Some("variant"))?;
            let mut fields = Vec::new();
            let mut source_fields = Vec::new();
            if self.cursor.eat("(").is_some() {
                while !self.cursor.is(")") {
                    fields.push(SField {
                        name: None,
                        ty: self.parse_type()?,
                        rocq_type: None,
                        span: None,
                    });
                    source_fields.push(self.origins.current_type.clone());
                    if self.cursor.eat(",").is_none() {
                        break;
                    }
                }
                self.cursor.expect(")", Some("variant"))?;
            } else if self.cursor.is("{") {
                let (named_fields, named_origins) =
                    self.named_variant_fields(path, &name, &variant)?;
                fields.extend(named_fields);
                source_fields.extend(named_origins);
            } else if self.cursor.is("=") {
                return Err(unsupported(
                    "explicit discriminant",
                    "discriminant values are outside the portable core",
                    span(&variant, self.cursor.peek()),
                ));
            }
            let mut constructor_path = path.to_vec();
            constructor_path.push(name.clone());
            let mut type_path = vec!["crate".to_owned()];
            type_path.extend(constructor_path.iter().cloned());
            self.origins.declaration(
                &variant.value,
                &constructor_path,
                SourceDeclaration::Constructor {
                    owner: owner.clone(),
                    fields: source_fields,
                    result: SourceOrigin::with_type(
                        Type::Named {
                            path: type_path,
                            span: None,
                        },
                        &[],
                        Vec::new(),
                    ),
                },
            )?;
            ctors.push(SCtor {
                name: variant.value,
                fields,
            });
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("}", Some("enum"))?;
        let mut declaration = SData {
            name,
            ctors,
            span: span(&start, self.cursor.peek()),
            generated: false,
            source_unit_clone_derivation: None,
        };
        if !self.type_namespace_model.is_empty() {
            let proof =
                crate::translation::source_unit_clone_derivations::capture_unit_clone_derivation(
                    derivations,
                    &declaration,
                    path,
                )?;
            self.origins.unit_clone_facts.insert(
                owner,
                (
                    proof.clone(),
                    declaration
                        .ctors
                        .iter()
                        .all(|constructor| constructor.fields.is_empty()),
                ),
            );
            declaration.source_unit_clone_derivation = proof.map(Box::new);
        }
        Ok(SItem::Data(declaration))
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

    /// A block `{ let x = e; …; tail }` is a chain of lets ending in its tail
    /// expression. Statements with effects are not values of the portable core.
    pub(super) fn block(&mut self, path: &[String]) -> Result<SExpr> {
        self.block_with_context(path, false)
    }
    pub(super) fn is_terminal_return(&self) -> bool {
        let mut count = 64usize;
        loop {
            let values = self
                .cursor
                .tokens
                .iter()
                .skip(self.cursor.index)
                .take(count)
                .map(|token| {
                    if token.kind == TokenKind::Punct {
                        token.value.clone()
                    } else {
                        String::new()
                    }
                })
                .collect::<Vec<_>>();
            let boundary =
                crate::translation::rust_unit_value_rules::find_rust_terminal_return_boundary(
                    &values, 0.0,
                );
            if boundary != -2.0 {
                return boundary >= 0.0;
            }
            if count >= self.cursor.tokens.len() - self.cursor.index {
                return false;
            }
            count = count.saturating_mul(2);
        }
    }
    pub(super) fn block_with_context(
        &mut self,
        path: &[String],
        function_body: bool,
    ) -> Result<SExpr> {
        let open = self.cursor.expect("{", Some("block"))?;
        let previous = self.origins.enter(None);
        let mut lets = Vec::new();
        let ordinary = self.ordinary_completion_context();
        let mut tail: Option<SExpr> = None;
        while !self.cursor.is("}") {
            if self.cursor.eat(";").is_some() {
                continue;
            }
            let token = self.peek();
            if tail
                .as_ref()
                .is_some_and(|value| matches!(value.node, SNode::Abort { .. }))
            {
                return Err(unsupported(
                    "unreachable continuation",
                    "statements after an abort are not removed by this model",
                    span(&token, &token),
                ));
            }
            if self.cursor.is("let") {
                lets.extend(self.let_statement(path)?);
                continue;
            }
            if token.kind == TokenKind::Macro
                && crate::translation::rust_unit_value_rules::read_rust_function_macro_disposition(
                    &token.value,
                ) == "print"
            {
                let printed = self.function_print_value(path)?;
                let place = printed.span;
                if self.cursor.eat(";").is_some() {
                    lets.push(Binding {
                        name: self.fresh_product_binding(),
                        ty: None,
                        value: printed,
                        span: place,
                    });
                    continue;
                }
                if !self.cursor.is("}") {
                    return Err(Self::fail("expected ; or }", self.cursor.peek()));
                }
                tail = Some(printed);
                break;
            }
            if function_body
                && self.cursor.is("return")
                && (self.cursor.is_at(";", 1)
                    || self.cursor.is_at("}", 1)
                    || self.is_terminal_return())
            {
                self.cursor.advance();
                tail = Some(if self.cursor.is(";") || self.cursor.is("}") {
                    SExpr::new(SNode::Unit, span(&token, &token))
                } else {
                    self.expr(path, Options::default())?
                });
                while self.cursor.eat(";").is_some() {}
                if !self.cursor.is("}") {
                    return Err(unsupported(
                        "unreachable continuation",
                        "statements after a completed return are not removed by this model",
                        span(&token, self.cursor.peek()),
                    ));
                }
                break;
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
                let origin = self.origins.origin(&expr, path);
                let disposition = (if ordinary {
                    crate::translation::rust_unit_value_rules::read_rust_ordinary_statement_disposition
                } else {
                    crate::translation::rust_unit_value_rules::read_rust_unit_statement_disposition
                })(
                    expr.node.kind(),
                    self.origins.collecting,
                    origin.proven,
                    origin.ty.as_ref().map_or("", Type::kind),
                    !origin.wrappers.is_empty(),
                );
                if disposition == "abort" {
                    tail = Some(expr);
                    continue;
                }
                if disposition == "bind" {
                    lets.push(Binding {
                        name: self.fresh_product_binding(),
                        ty: None,
                        span: expr.span,
                        value: expr,
                    });
                    continue;
                }
                return Err(unsupported(
                    "expression statement",
                    "statements with effects are outside the portable core in function bodies",
                    expr.span,
                ));
            }
            if ordinary {
                let origin = self.origins.origin(&expr, path);
                let implicit=crate::translation::rust_unit_value_rules::read_rust_implicit_block_statement_disposition(expr.node.kind(),expr.block,!self.cursor.is("}"),self.origins.collecting,origin.proven,origin.ty.as_ref().map_or("",Type::kind),!origin.wrappers.is_empty());
                if implicit == "bind" {
                    lets.push(Binding {
                        name: self.fresh_product_binding(),
                        ty: Some(super::UNIT),
                        span: expr.span,
                        value: expr,
                    });
                    continue;
                }
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
        let tail = tail.unwrap_or_else(|| SExpr::new(SNode::Unit, span(&open, &close)));
        let origin = self.origins.origin(&tail, path);
        let result = lets.into_iter().rev().fold(tail, |body, binding| {
            SExpr::new(
                SNode::Let {
                    name: binding.name,
                    ty: binding.ty.map(Into::into),
                    value: Box::new(binding.value),
                    body: Box::new(body),
                },
                binding.span,
            )
        });
        self.origins.mark(&result, origin);
        self.origins.scope = previous;
        Ok(result)
    }

    pub(super) fn main_fn(&mut self) -> Result<SMain> {
        if !self.pure_unit_call_model.is_empty() {
            return self.pure_unit_main();
        }
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
        if self.header_discovery {
            let close = self.declaration_body_boundary()?;
            return Ok(SMain {
                effects: Vec::new(),
                span: Some(super::Span::new(start.start, close.end)),
                sequential_async: false,
                source_runtime_worker: None,
            });
        }
        self.module_path = Vec::new();
        let previous = self.origins.enter(Some(std::collections::HashMap::new()));
        self.cursor.expect("{", Some("fn main"))?;
        let mut effects = Vec::new();
        while !self.cursor.is("}") {
            if self.cursor.eat(";").is_some() {
                continue;
            }
            let token = self.peek();
            if self.cursor.is("let") {
                for binding in self.let_statement(&[])? {
                    effects.push(SEffect::Let {
                        name: binding.name,
                        ty: binding.ty,
                        value: binding.value,
                        constant: false,
                        span: binding.span,

                        source_pure_unit_call: None,
                    });
                }
                continue;
            }
            if token.kind == TokenKind::Macro {
                effects.push(self.main_macro()?);
                self.cursor.expect(";", Some(&token.value))?;
                continue;
            }
            let value = self.expr(
                &[],
                Options {
                    statement: true,
                    no_struct: false,
                },
            )?;
            let origin = self.origins.origin(&value, &[]);
            if crate::translation::rust_unit_value_rules::read_rust_unit_statement_disposition(
                value.node.kind(),
                self.origins.collecting,
                origin.proven,
                origin.ty.as_ref().map_or("", Type::kind),
                !origin.wrappers.is_empty(),
            ) != "bind"
            {
                return Err(unsupported(
                    "main statement",
                    "main may only print, bind, assert and evaluate direct Unit calls",
                    span(&token, &token),
                ));
            }
            if self.cursor.eat(";").is_none() && !self.cursor.is("}") {
                return Err(Self::fail("expected ; or }", self.cursor.peek()));
            }
            effects.push(SEffect::Let {
                name: self.fresh_product_binding(),
                ty: None,
                span: value.span,
                value,
                constant: false,
                source_pure_unit_call: None,
            });
        }
        let close = self.cursor.expect("}", Some("fn main"))?;
        self.origins.scope = previous;
        Ok(SMain {
            effects,
            span: if self.empty_option_source_model.is_empty() {
                span(&start, self.cursor.peek())
            } else {
                Some(super::Span {
                    start: start.start,
                    end: close.end,
                })
            },
            sequential_async: false,
            source_runtime_worker: None,
        })
    }

    pub(super) fn main_macro(&mut self) -> Result<SEffect> {
        self.main_macro_with_path(&[])
    }
    pub(super) fn function_print_value(&mut self, path: &[String]) -> Result<SExpr> {
        let SEffect::Print {
            expr,
            style,
            span: place,
        } = self.main_macro_with_path(path)?
        else {
            unreachable!("source print macro disposition")
        };
        let unit = SExpr::new(SNode::Unit, place);
        let origin = self.origins.origin(&unit, path);
        let printed = SExpr::new(
            SNode::Print {
                expr: Box::new(expr),
                style,
                body: Box::new(unit),
            },
            place,
        );
        self.origins.mark(&printed, origin);
        Ok(printed)
    }
    fn main_macro_with_path(&mut self, path: &[String]) -> Result<SEffect> {
        let token = self.cursor.advance();
        let mut args = self.macro_arguments(&token, path)?;
        match token.value.as_str() {
            "println" => {
                for value in args.iter().skip(1) {
                    self.refuse_shared_value_observation(value, None, path, value.span)?;
                }
                let expr = if args.is_empty() {
                    SExpr::new(
                        SNode::Str {
                            value: String::new(),
                        },
                        None,
                    )
                } else {
                    self.format(args, &token, path)?
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
