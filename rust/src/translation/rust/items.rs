//! Attributes, items, `use` trees, enums, types, functions, blocks and `main`.

use super::array_rules::{accept_rust_source_array_owner, accept_rust_source_array_result};
use super::indirection_rules::{
    accept_rust_source_box_owner, accept_rust_source_lifetime_binding,
    accept_rust_source_lifetime_result,
};
use super::read_only_query_rules::accept_rust_read_only_query_owner;
use super::scalar_parameter_rules::accept_rust_unwritten_scalar_parameter;
use super::scalar_parameters::read_rust_scalar_parameter_body;
use super::{
    FLOAT, ItemsEnd, Language, Options, RESERVED_ITEMS, Result, RustParser, SComparison, SCtor,
    SData, SEffect, SExpr, SField, SFn, SItem, SMain, SModule, SNode, SParam, SProgram, SProp,
    SPropNode, ShowStyle, SourceDeclaration, SourceOrigin, Token, TokenKind, TranslationError,
    Type, absolute, format, is_if_or_match, prop_of, span, unsupported,
};

mod attributes;
mod product_bindings;
mod source_headers;
mod types;

impl RustParser {
    pub(super) fn file(&mut self) -> Result<SProgram> {
        self.inner_attributes()?;
        let mut items = self.items(&[], ItemsEnd::File)?;
        let Some(main) = self.main.take() else {
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
        Ok(SProgram {
            language: Language::Rust,
            items,
            main: Some(main),
            imports: Vec::new(),
            externals: Vec::new(),
            source_pure_unit_calls,
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
            let derivations = self.attributes()?;
            let start = self.peek();
            self.module_path = path.to_vec();
            self.pure_unit_item_context(path, &start)?;
            self.visibility()?;
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
            if c.is("fn") || c.is("const") {
                if c.is("fn") && c.is_at("main", 1) && path.is_empty() {
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
            let attributes = self.attributes()?;
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

    pub(super) fn function(&mut self, path: &[String]) -> Result<SFn> {
        if !self.pure_unit_call_model.is_empty() {
            return self.pure_unit_function(path);
        }
        let outer_lifetimes = std::mem::take(&mut self.lifetime_bindings);
        let outer_header = std::mem::replace(&mut self.lifetime_header, true);
        let result = if self.empty_option_source_model.is_empty() {
            self.function_with_lifetimes(path)
        } else {
            self.empty_option_function(path)
        };
        self.lifetime_bindings = outer_lifetimes;
        self.lifetime_header = outer_header;
        result
    }

    fn function_with_lifetimes(&mut self, path: &[String]) -> Result<SFn> {
        let start = self.cursor.advance();
        let name = self.cursor.identifier(Some("fn"))?.value;
        let mut source_lifetime_bindings = Vec::new();
        if self.cursor.is("<") {
            let generic = self.cursor.advance();
            while !self.cursor.is(">") {
                if self.cursor.eat("'").is_none() {
                    return Err(unsupported(
                        "generic function",
                        "type parameters are outside the portable core",
                        span(&start, &generic),
                    ));
                }
                let lifetime = self.cursor.identifier(Some("lifetime"))?;
                if !accept_rust_source_lifetime_binding(
                    &lifetime.value,
                    self.lifetime_bindings.contains(&lifetime.value),
                    self.cursor.is(":"),
                ) {
                    return Err(unsupported(
                        "function lifetime binding",
                        "lifetime names must be unique and unconstrained; bounds need their own linked obligations",
                        span(&lifetime, self.cursor.peek()),
                    ));
                }
                source_lifetime_bindings.push(lifetime.value.clone());
                self.lifetime_bindings.insert(lifetime.value);
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
            self.cursor.expect(">", Some("function lifetimes"))?;
            if self.lifetime_bindings.is_empty() {
                return Err(unsupported(
                    "function lifetime binding",
                    "a lifetime parameter list must contain a named binding",
                    span(&generic, self.cursor.peek()),
                ));
            }
        }
        self.cursor.expect("(", Some("fn"))?;
        let mut params = Vec::new();
        let mut source_params = Vec::new();
        let mut mutable_params = Vec::new();
        while !self.cursor.is(")") {
            let param_token = self.peek();
            let mutable = self.cursor.eat("mut").is_some();
            if self.cursor.is("self") || self.cursor.is("&") {
                return Err(unsupported(
                    "method",
                    "methods are outside the Rust frontend; declare a free function",
                    span(&param_token, &param_token),
                ));
            }
            let param_name = self.cursor.identifier(Some("parameter"))?.value;
            self.cursor.expect(":", Some("parameter"))?;
            let type_token = self.peek();
            let ty = self.parse_type()?;
            if mutable {
                mutable_params.push((
                    param_token.clone(),
                    type_token,
                    self.origins.current_type.clone(),
                    param_name.clone(),
                ));
            }
            source_params.push(self.origins.current_type.clone());
            params.push(SParam {
                name: param_name,
                default_value: None,
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
        let mut source_result = self.origins.current_type.clone();
        let scalar_loop = !self.scalar_loop_model.is_empty()
            && self.cursor.is("{")
            && self.cursor.is_at("loop", 1);
        let mutable_names = mutable_params
            .iter()
            .map(|(_, _, _, name)| name.clone())
            .collect::<Vec<_>>();
        if !self.primitive_owner_model.is_empty()
            && (!mutable_params.is_empty() || !self.lifetime_bindings.is_empty())
        {
            return Err(unsupported(
                "Rust primitive owner context",
                "mutable parameters and named lifetime binders require a separate owned context proof",
                None,
            ));
        }
        if !scalar_loop && !mutable_params.is_empty() {
            let (closed, possible_write) = read_rust_scalar_parameter_body(&self.cursor);
            for (token, type_token, origin, _) in mutable_params {
                let resolved = self.resolve(std::slice::from_ref(&type_token.value), path);
                let shadowed = type_token.kind != TokenKind::Identifier
                    || self.origins.lookup(&resolved, path).is_some();
                if !accept_rust_unwritten_scalar_parameter(
                    origin.ty.as_ref().map_or("", Type::kind),
                    (origin.direct_copy_value(), shadowed, closed, possible_write),
                ) {
                    return Err(unsupported(
                        "mutable parameter",
                        "mutation is outside the portable core",
                        span(&token, &token),
                    ));
                }
            }
        }
        if let Type::Array { element, .. } = &ret
            && !accept_rust_source_array_result(
                &self.origins.array_result_model,
                element.kind(),
                source_result.array_source_model == Some("owned vector")
                    && source_result.wrappers.is_empty(),
                true,
            )
        {
            return Err(unsupported(
                "Rust array result",
                "returning vectors or borrowed slices needs a separate consumption and outlives proof",
                span(&start, self.cursor.peek()),
            ));
        }
        if !accept_rust_source_lifetime_result(
            !self.lifetime_bindings.is_empty(),
            source_result
                .wrappers
                .iter()
                .any(|layer| layer == "reference"),
        ) {
            return Err(unsupported(
                "function lifetime result",
                "reference results with lifetime binders require an explicit return-lifetime instantiation and outlives proof",
                span(&start, self.cursor.peek()),
            ));
        }
        self.origins.declaration(
            &name,
            path,
            SourceDeclaration::Function {
                parameters: source_params.clone(),
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
        let source_signature = self.capture_source_signature(
            &source_params,
            &source_result,
            source_lifetime_bindings,
            scalar_loop,
        )?;
        self.lifetime_header = false;
        let scope = params
            .iter()
            .zip(source_params.iter().cloned())
            .map(|(parameter, origin)| (parameter.name.clone(), origin))
            .collect();
        let previous = self.origins.enter(Some(scope));
        let mut source_loop = None;
        let body = if scalar_loop {
            if !path.is_empty() {
                return Err(unsupported(
                    "Rust scalar loop",
                    "the selected scalar state model requires a flat function",
                    span(&start, self.cursor.peek()),
                ));
            }
            let active = std::mem::replace(&mut self.scalar_loop_active, true);
            let read = self.read_scalar_loop(path, &name, &params, &ret, &mutable_names);
            self.scalar_loop_active = active;
            let (body, proof) = read?;
            source_loop = Some(proof);
            body
        } else if self.completion_model.is_empty() {
            self.block(path)?
        } else {
            self.closed_function_completion(path)?
        };
        if let Type::Array { element, .. } = &ret {
            if !accept_rust_source_array_result(
                &self.origins.array_result_model,
                element.kind(),
                source_result.array_source_model == Some("owned vector")
                    && source_result.wrappers.is_empty(),
                self.origins.fresh_array_result(&body, path),
            ) {
                return Err(unsupported(
                    "fresh Rust array result",
                    "every returning tail must construct a new vector of direct scalar Copy values; vector names, calls, clones and borrowed results need separate consumption proofs",
                    body.span,
                ));
            }
            source_result.fresh_array_value = body.span;
            self.origins.certify_array_result(&name, path, body.span);
            self.origins.mark(&body, source_result.clone());
        }
        self.origins
            .argument(&self.origins.origin(&body, path), &source_result, body.span)?;
        self.origins.scope = previous;
        let source_primitive_owners = (!self.primitive_owner_model.is_empty()).then(|| {
            self.create_primitive_owner_proof(
                &source_params,
                &source_result,
                span(&start, self.cursor.peek()),
            )
        });
        let source_type_namespace = self
            .capture_type_namespace(&source_params, &source_result)
            .map(Box::new);
        Ok(SFn {
            name,
            params,
            ret: Some(ret),
            body,
            span: span(&start, self.cursor.peek()),
            generated: false,
            source_signature: source_signature.map(Box::new),
            source_primitive_owners,
            source_empty_option: None,
            source_type_namespace,
            source_loop: source_loop.map(Box::new),
            source_pure_unit: None,
        })
    }

    /// `const N: T = e;` is a function without parameters.
    pub(super) fn const_item(&mut self, path: &[String]) -> Result<SFn> {
        self.require_type_namespace_without_constant()?;
        let start = self.cursor.advance();
        let name = self.cursor.identifier(Some("const"))?.value;
        self.cursor.expect(":", Some("const"))?;
        let ret = self.parse_type()?;
        let source_result = self.origins.current_type.clone();
        if matches!(ret, Type::Array { .. }) {
            return Err(unsupported(
                "Rust array constant",
                "vector allocation and slice constants require a separate constant evaluation model",
                span(&start, self.cursor.peek()),
            ));
        }
        self.origins.declaration(
            &name,
            path,
            SourceDeclaration::Function {
                parameters: Vec::new(),
                result: source_result.clone(),
            },
        )?;
        self.cursor.expect("=", Some("const"))?;
        let body = self.expr(path, Options::default())?;
        self.origins
            .argument(&self.origins.origin(&body, path), &source_result, body.span)?;
        self.cursor.expect(";", Some("const"))?;
        Ok(SFn {
            name,
            params: Vec::new(),
            ret: Some(ret),
            body,
            span: span(&start, self.cursor.peek()),
            generated: false,
            source_signature: None,
            source_primitive_owners: None,
            source_empty_option: None,
            source_type_namespace: None,
            source_loop: None,
            source_pure_unit: None,
        })
    }

    /// A block `{ let x = e; …; tail }` is a chain of lets ending in its tail
    /// expression. Statements with effects are not values of the portable core.
    pub(super) fn block(&mut self, path: &[String]) -> Result<SExpr> {
        let open = self.cursor.expect("{", Some("block"))?;
        let previous = self.origins.enter(None);
        let mut lets = Vec::new();
        let mut tail: Option<SExpr> = None;
        while !self.cursor.is("}") {
            let token = self.peek();
            if self.cursor.is("let") {
                lets.extend(self.let_statement(path)?);
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
        let origin = self.origins.origin(&tail, path);
        let result = lets.into_iter().rev().fold(tail, |body, binding| {
            SExpr::new(
                SNode::Let {
                    name: binding.name,
                    ty: binding.ty,
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
        self.module_path = Vec::new();
        let previous = self.origins.enter(Some(std::collections::HashMap::new()));
        self.cursor.expect("{", Some("fn main"))?;
        let mut effects = Vec::new();
        while !self.cursor.is("}") {
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
            return Err(unsupported(
                "main statement",
                "main may only print, bind and assert",
                span(&token, &token),
            ));
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
