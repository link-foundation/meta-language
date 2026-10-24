//! Existing free-function reader with owned source-header capture.
use super::super::array_rules::accept_rust_source_array_result;
use super::super::indirection_rules::{
    accept_rust_source_lifetime_binding, accept_rust_source_lifetime_result,
};
use super::super::scalar_parameter_rules::accept_rust_unwritten_scalar_parameter;
use super::super::scalar_parameters::read_rust_scalar_parameter_body;
use super::super::{
    Result, RustParser, SFn, SParam, SourceDeclaration, TokenKind, Type, span, unsupported,
};

impl RustParser {
    pub(in super::super) fn function(&mut self, path: &[String]) -> Result<SFn> {
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
        let function_keyword = self.cursor.advance();
        let const_keyword = self.const_function_keyword.take();
        let start = const_keyword.as_ref().unwrap_or(&function_keyword).clone();
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
            if let Some((parameter, origin)) = self.inherent_receiver(&name, path, params.len())? {
                params.push(parameter);
                source_params.push(origin);
                if self.cursor.eat(",").is_none() {
                    break;
                }
                continue;
            }
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
                optional: false,
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
        let explicit_result = self.cursor.eat("->").is_some();
        let ret = if explicit_result {
            self.parse_type()?
        } else {
            self.origins.typed(Type::Unit, path, Vec::new())
        };
        let mut source_result = self.origins.current_type.clone();
        if !self.shared_array_handle_source_model.is_empty()
            && (!mutable_params.is_empty() || !self.lifetime_bindings.is_empty())
        {
            return Err(unsupported(
                "Rust shared array handle source",
                "mutable parameters and named lifetime binders need another source profile",
                span(&start, self.cursor.peek()),
            ));
        }
        if !self.const_function_model.is_empty()
            && (!mutable_params.is_empty() || !self.lifetime_bindings.is_empty())
        {
            return Err(unsupported(
                "Rust const function interface",
                "mutable parameters and lifetime binders require an independent interpretation",
                None,
            ));
        }
        let scalar_loop = !self.scalar_loop_model.is_empty()
            && self.cursor.is("{")
            && self.cursor.is_at("loop", 1);
        if !self.local_value_state_model.is_empty()
            && (!mutable_params.is_empty() || !self.lifetime_bindings.is_empty())
        {
            return Err(unsupported(
                "Rust owned local state",
                "mutable parameters and lifetimes require a separate interface proof",
                None,
            ));
        }
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
            && !self.shared_array_origin(&source_result)
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
        let const_header_end = self.cursor.peek().start;
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
        let mut source_local_state = None;
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
        } else if !self.local_value_state_model.is_empty() {
            let (body, proof) = self.read_owned_local_state_body(
                path,
                &name,
                &params,
                &ret,
                source_signature.as_ref(),
            )?;
            source_local_state = Some(Box::new(proof));
            body
        } else if self.completion_model.is_empty() {
            if self.ordinary_completion_selected() {
                self.ordinary_function_completion(path, &ret, &source_result)?
            } else {
                self.block_with_context(path, true)?
            }
        } else {
            self.closed_function_completion(path)?
        };
        let source_const_function = const_keyword.as_ref().map(|keyword| {
            Box::new(self.create_const_function_proof(
                &name,
                keyword,
                &function_keyword,
                const_header_end,
            ))
        });
        if let Type::Array { element, .. } = &ret
            && !self.shared_array_origin(&source_result)
        {
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
            source_diagnostic_attributes: None,
            semantic_ret: None,
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
            source_local_state,
            source_const_function,
        })
    }
}
