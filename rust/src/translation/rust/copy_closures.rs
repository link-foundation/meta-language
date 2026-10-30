//! Original types, scopes and expressions into the existing callable nodes.
use super::{
    ProductBindingPattern, Result, RustParser, SExpr, SNode, SParam, SourceDeclaration,
    SourceOrigin, Span, Type, unsupported,
};
use crate::translation::rust_contextual_closure_rules as contextual;
use crate::translation::rust_copy_closure_rules as rules;
use crate::translation::surface_reference_facts::create_surface_reference_facts;

pub(super) fn result_context_error(node: &SExpr, result: &SourceOrigin) -> String {
    let mut pending = vec![(node, None)];
    let mut scopes: Vec<(&str, &SExpr, Option<usize>)> = Vec::new();
    let mut visited = std::collections::HashSet::new();
    let kind = result.ty.as_ref().map_or("", Type::kind);
    let signed = matches!(result.ty, Some(Type::Fixed { signed: true, .. }));
    while let Some((value, scope)) = pending.pop() {
        if !visited.insert((std::ptr::from_ref(value), scope)) {
            continue;
        }
        let negative = matches!(
            &value.node,
            SNode::Unary {
                op: super::UnaryOp::Neg,
                ..
            }
        );
        let typed = matches!(&value.node, SNode::Num { ty: Some(_), .. });
        let error = rules::read_rust_copy_closure_literal_result_error(
            value.node.kind(),
            negative,
            typed,
            kind,
            signed,
        );
        if !error.is_empty() {
            return error;
        }
        let operation = match &value.node {
            SNode::Unary {
                op: super::UnaryOp::Neg,
                ..
            } => "neg",
            SNode::Binary { op, .. } => op.name(),
            _ => "",
        };
        match rules::read_rust_copy_closure_result_context_role(value.node.kind(), operation)
            .as_str()
        {
            "binding" => {
                if let SNode::Let {
                    name,
                    value: initializer,
                    body,
                    ..
                } = &value.node
                {
                    let frame = scopes.len();
                    scopes.push((name.as_str(), initializer, scope));
                    pending.push((body, Some(frame)))
                } else {
                    unreachable!("actual source binding context")
                }
            }
            "name" => {
                if let SNode::Name { path } = &value.node {
                    let mut frame = scope;
                    while let Some(index) = frame {
                        let (name, initializer, parent) = scopes[index];
                        if rules::test_rust_copy_closure_result_binding(path, name) {
                            pending.push((initializer, parent));
                            break;
                        }
                        frame = parent;
                    }
                } else {
                    unreachable!("actual source name context")
                }
            }
            "branches" => {
                if let SNode::If {
                    then, otherwise, ..
                } = &value.node
                {
                    pending.push((then, scope));
                    pending.push((otherwise, scope))
                } else {
                    unreachable!("actual source branch contexts")
                }
            }
            "rows" => {
                if let SNode::Match { rows, .. } = &value.node {
                    pending.extend(rows.iter().map(|row| (&row.body, scope)))
                } else {
                    unreachable!("actual source row contexts")
                }
            }
            "operand" => {
                if let SNode::Unary { arg, .. } = &value.node {
                    pending.push((arg, scope))
                } else {
                    unreachable!("actual source operand context")
                }
            }
            "operands" => {
                if let SNode::Binary { left, right, .. } = &value.node {
                    pending.push((left, scope));
                    pending.push((right, scope))
                } else {
                    unreachable!("actual source operand contexts")
                }
            }
            "leaf" => {}
            _ => return "unknown shared closure result context role".to_owned(),
        }
    }
    String::new()
}

impl RustParser {
    pub(super) fn copy_pointer_result_block(
        &mut self,
        path: &[String],
        expected: Option<&SourceOrigin>,
    ) -> Result<Option<SExpr>> {
        let role = ["|", "||", "move", "(", "return"]
            .into_iter()
            .find(|value| self.cursor.is_at(value, 1))
            .unwrap_or("");
        if !self.ordinary_completion_context()
            || !contextual::accept_rust_contextual_copy_pointer_result_block(
                expected
                    .and_then(|origin| origin.ty.as_ref())
                    .map_or("", Type::kind),
                expected.is_some_and(SourceOrigin::direct_copy_value),
                self.cursor.is("{"),
                role,
            )
        {
            return Ok(None);
        }
        self.cursor.expect("{", Some("function pointer result"))?;
        let returned = self.cursor.eat("return").is_some();
        let value = self.read_expected_copy_closure(path, expected)?;
        if returned {
            while self.cursor.eat(";").is_some() {}
        }
        self.cursor.expect("}", Some("function pointer result"))?;
        self.origins.argument(
            &self.origins.origin(&value, path),
            expected.expect("actual pointer result"),
            value.span,
        )?;
        Ok(Some(value))
    }

    pub(super) fn contextual_call_arguments(
        &mut self,
        path: &[String],
        callee: &SExpr,
    ) -> Result<Vec<SExpr>> {
        let mut parameters = None;
        let mut result = None;
        if let SNode::Name { path: names } = &callee.node
            && !(names.len() == 1 && self.origins.scope.contains_key(&names[0]))
            && let Some(entry) = self
                .origins
                .lookup(&self.source_value_path(names, path), path)
            && let (SourceDeclaration::Function {
                parameters: actual,
                result: actual_result,
                ..
            }
            | SourceDeclaration::Constructor {
                fields: actual,
                result: actual_result,
                ..
            }) = entry.declaration
        {
            parameters = Some(actual);
            result = Some(actual_result);
        }
        let parameters = parameters.unwrap_or_else(|| {
            let origin = self.origins.origin(callee, path);
            if result.is_none() {
                result = origin.callable_result.as_deref().cloned();
            }
            if let Some(parameters) = origin.callable_parameters {
                return parameters;
            }
            match origin.ty {
                Some(Type::Function { params, .. }) => params
                    .into_iter()
                    .map(|ty| {
                        let function = matches!(ty, Type::Function { .. });
                        let mut value = SourceOrigin::with_type(ty, path, Vec::new());
                        if function {
                            value.copy_evidence = Some(());
                        }
                        value
                    })
                    .collect(),
                _ => Vec::new(),
            }
        });
        let mut args = Vec::new();
        while !self.cursor.is(")") {
            let argument = self.read_standard_string_view_argument(
                path,
                parameters.get(args.len()),
                result.as_ref(),
            )?;
            self.origins
                .prepare_owned_argument(&argument, parameters.get(args.len()), path)?;
            args.push(argument);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        Ok(args)
    }

    pub(super) fn read_expected_copy_closure(
        &mut self,
        path: &[String],
        expected: Option<&SourceOrigin>,
    ) -> Result<SExpr> {
        self.read_expected_copy_closure_result_mode(path, expected, false)
    }
    pub(super) fn read_expected_copy_closure_result_mode(
        &mut self,
        path: &[String],
        expected: Option<&SourceOrigin>,
        parameters_only: bool,
    ) -> Result<SExpr> {
        self.read_expected_copy_closure_result_options(path, expected, parameters_only, false)
    }
    pub(super) fn read_expected_copy_closure_deferred_result(
        &mut self,
        path: &[String],
        expected: Option<&SourceOrigin>,
    ) -> Result<SExpr> {
        self.read_expected_copy_closure_result_options(path, expected, true, true)
    }
    fn read_expected_copy_closure_result_options(
        &mut self,
        path: &[String],
        expected: Option<&SourceOrigin>,
        parameters_only: bool,
        defer_result: bool,
    ) -> Result<SExpr> {
        if let Some(value) = self.read_primitive_function_value(path, expected)? {
            return Ok(value);
        }
        let role = ["|", "||", "move"]
            .into_iter()
            .find(|value| self.cursor.is(value))
            .unwrap_or("");
        if contextual::accept_rust_contextual_copy_closure(
            expected
                .and_then(|origin| origin.ty.as_ref())
                .map_or("", Type::kind),
            expected.is_some_and(SourceOrigin::direct_copy_value),
            role,
        ) {
            self.read_copy_closure_with_expected(path, expected, parameters_only, defer_result)
        } else {
            if expected.is_some_and(|origin| {
                origin.direct_copy_value() && matches!(origin.ty, Some(Type::Function { .. }))
            }) && self.cursor.is("(")
            {
                let mut count = 64usize;
                loop {
                    let length = count.min(self.cursor.tokens.len() - self.cursor.index);
                    let syntax = (0..length)
                        .map(|index| {
                            ["(", "|", "||", "move"]
                                .into_iter()
                                .find(|value| self.cursor.is_at(value, index))
                                .unwrap_or("")
                                .to_owned()
                        })
                        .collect::<Vec<_>>();
                    let depth = contextual::find_rust_contextual_closure_parenthesis_count(&syntax);
                    if depth > 0.0 {
                        let depth =
                            crate::translation::frontend_rules::ml_array::number_index(depth)
                                .expect("actual source parenthesis count");
                        self.cursor.index += depth;
                        let node = self.read_copy_closure_with_expected(
                            path,
                            expected,
                            parameters_only,
                            defer_result,
                        )?;
                        for _ in 0..depth {
                            self.cursor.expect(")", Some("contextual closure"))?;
                        }
                        return Ok(node);
                    }
                    if depth != -2.0 || length < count {
                        break;
                    }
                    count = count.checked_mul(2).expect("actual source token window");
                }
            }
            self.expr(path, super::Options::default())
        }
    }
    pub(super) fn read_copy_closure(&mut self, path: &[String]) -> Result<SExpr> {
        self.read_copy_closure_with_expected(path, None, false, false)
    }
    fn read_copy_closure_with_expected(
        &mut self,
        path: &[String],
        expected: Option<&SourceOrigin>,
        parameters_only: bool,
        defer_result: bool,
    ) -> Result<SExpr> {
        let start = self.peek();
        self.cursor.eat("move");
        let mut params = Vec::new();
        let mut origins = Vec::new();
        let mut bindings = Vec::new();
        let mut patterns = Vec::new();
        let mut pattern_names = Vec::new();
        if self.cursor.eat("||").is_none() {
            self.cursor.expect("|", Some("closure"))?;
            while !self.cursor.is("|") {
                let token = self.peek();
                let mut reference_depth = 0usize;
                loop {
                    let depth =
                        rules::read_rust_copy_closure_reference_pattern_depth(&self.peek().value);
                    if depth == 0.0 {
                        break;
                    }
                    reference_depth += depth as usize;
                    self.cursor.advance();
                }
                let nominal = self.nominal_binding_pattern()?;
                let role = ["mut", "ref", "&", "_", "("]
                    .into_iter()
                    .find(|value| self.cursor.is(value))
                    .unwrap_or("binding");
                let form = if nominal.is_some() {
                    "product".to_owned()
                } else {
                    rules::read_rust_copy_closure_parameter_form(role)
                };
                if form == "unsupported" {
                    return Err(unsupported(
                        "Rust closure parameter",
                        "a closure needs a named direct immutable typed parameter",
                        Some(token.span()),
                    ));
                }
                let mut pattern = None;
                let name = if form == "product" {
                    pattern = Some(match nominal {
                        Some(value) => ProductBindingPattern::Nominal(value),
                        None => self.product_binding_pattern()?,
                    });
                    self.fresh_product_binding()
                } else if form == "wildcard" {
                    self.cursor.advance();
                    self.fresh_product_binding()
                } else {
                    let name = self.cursor.identifier(Some("closure parameter"))?.value;
                    self.require_nonconstant_binding(&name, path, Some(token.span()))?;
                    name
                };
                let mut source_names = Vec::new();
                if let Some(value) = &pattern {
                    let mut pending = vec![value];
                    while let Some(current) = pending.pop() {
                        match current {
                            ProductBindingPattern::Name(token) => {
                                source_names.push((token.value.clone(), Some(token.span())))
                            }
                            ProductBindingPattern::Tuple { items, .. } => {
                                pending.extend(items.iter())
                            }
                            ProductBindingPattern::Nominal(value) => {
                                source_names.extend(self.nominal_binding_names(value, false)?)
                            }
                            ProductBindingPattern::Wildcard(_)
                            | ProductBindingPattern::Unit { .. } => {}
                        }
                    }
                } else {
                    source_names.push((name.clone(), Some(token.span())));
                }
                for (source_name, range) in source_names {
                    self.require_nonconstant_binding(&source_name, path, range)?;
                    pattern_names.push(source_name);
                }
                let binding_error =
                    rules::read_rust_copy_closure_pattern_binding_error(&pattern_names);
                if !binding_error.is_empty() {
                    return Err(unsupported(
                        "Rust Copy closure",
                        binding_error,
                        Some(token.span()),
                    ));
                }
                patterns.push(pattern);
                let (ty, origin) = if self.cursor.eat(":").is_some() {
                    let ty = self.parse_type()?;
                    (ty, self.origins.current_type.clone())
                } else if let Some(Type::Function { params, .. }) =
                    expected.and_then(|origin| origin.ty.as_ref())
                {
                    let Some(ty) = params.get(origins.len()).cloned() else {
                        self.cursor.expect(":", Some("closure parameter"))?;
                        unreachable!("missing actual source type");
                    };
                    let origin = expected
                        .and_then(|origin| origin.callable_parameters.as_ref())
                        .and_then(|parameters| parameters.get(origins.len()))
                        .cloned()
                        .unwrap_or_else(|| SourceOrigin::with_type(ty.clone(), path, Vec::new()));
                    (ty, origin)
                } else {
                    self.cursor.expect(":", Some("closure parameter"))?;
                    unreachable!("missing actual source type");
                };
                let reference_error = rules::read_rust_copy_closure_reference_pattern_error(
                    reference_depth as f64,
                    origin.wrappers.len() as f64,
                    origin.wrappers.iter().all(|layer| layer == "reference"),
                    origin.copy_closure_parameter(),
                );
                if !reference_error.is_empty() {
                    return Err(unsupported(
                        "Rust closure parameter",
                        reference_error,
                        Some(token.span()),
                    ));
                }
                let mut binding = origin.clone();
                binding.wrappers = binding.wrappers[reference_depth..].to_vec();
                binding.source_lifetimes = binding
                    .source_lifetimes
                    .iter()
                    .skip(reference_depth)
                    .cloned()
                    .collect();
                bindings.push(binding);
                origins.push(origin);
                params.push(SParam {
                    name,
                    optional: false,
                    default_value: None,
                    ty: Some(ty),
                    span: Some(Span::new(
                        token.start,
                        self.cursor.tokens[self.cursor.index - 1].end,
                    )),
                    guard: None,
                    rocq_type: None,
                });
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
            self.cursor.expect("|", Some("closure"))?;
        }
        let written = self.cursor.eat("->").is_some();
        let mut ret = if written {
            Some(self.parse_type()?)
        } else if crate::translation::rust_copy_closure_parameter_rules::read_rust_copy_closure_result_context(written,parameters_only)=="inferred" {
            None
        } else {
            expected.and_then(|origin| match origin.ty.as_ref() {
                Some(Type::Function { ret, .. }) => Some(ret.as_ref().clone()),
                _ => None,
            })
        };
        let mut result_origin = if written {
            Some(self.origins.current_type.clone())
        } else {
            ret.as_ref().map(|ret| {
                expected
                    .and_then(|origin| origin.callable_result.as_deref())
                    .cloned()
                    .unwrap_or_else(|| SourceOrigin::with_type(ret.clone(), path, Vec::new()))
            })
        };
        let names = params
            .iter()
            .map(|param| param.name.clone())
            .collect::<Vec<_>>();
        let copies = origins
            .iter()
            .map(SourceOrigin::copy_closure_parameter)
            .collect::<Vec<_>>();
        let reason = if written {
            rules::read_rust_copy_closure_header_error(
                self.ordinary_completion_context(),
                &names,
                &copies,
                written,
                result_origin
                    .as_ref()
                    .is_some_and(SourceOrigin::direct_copy_value),
            )
        } else {
            String::new()
        };
        if !reason.is_empty() {
            return Err(unsupported("Rust Copy closure", reason, Some(start.span())));
        }
        let outer = self.origins.enter(None);
        for (parameter, origin) in params.iter().zip(bindings.iter().cloned()) {
            self.origins.scope.insert(parameter.name.clone(), origin);
        }
        let read: Result<SExpr> = (|| {
            let mut projections = Vec::new();
            for (index, pattern) in patterns.into_iter().enumerate() {
                if let Some(pattern) = pattern {
                    let parameter = &params[index];
                    let reference = SExpr::new(
                        SNode::Name {
                            path: vec![parameter.name.clone()],
                        },
                        parameter.span,
                    );
                    projections.extend(self.lower_product_binding(
                        pattern,
                        reference,
                        parameter.ty.clone(),
                        None,
                        path,
                        parameter.span,
                    )?);
                }
            }
            let mut body = if written || self.cursor.is("{") {
                if let Some(body) = self.copy_pointer_result_block(path, result_origin.as_ref())? {
                    body
                } else if let (Some(ret), Some(origin)) = (&ret, &result_origin)
                    && self.ordinary_completion_selected()
                {
                    self.ordinary_function_completion(path, ret, origin)?
                } else {
                    self.block_with_context(path, true)?
                }
            } else {
                self.expr(path, super::Options::default())?
            };
            if ret.is_none() {
                let origin = self.origins.origin(&body, path);
                ret = origin.ty.clone();
                result_origin = Some(origin);
            }
            if !written {
                let deferred = rules::test_rust_copy_closure_deferred_result(
                    defer_result,
                    written,
                    ret.is_some(),
                );
                let reason = if deferred {
                    rules::read_rust_copy_closure_parameter_header_error(
                        self.ordinary_completion_context(),
                        &names,
                        &copies,
                    )
                } else {
                    contextual::read_rust_contextual_copy_closure_header_error(
                        self.ordinary_completion_context(),
                        &names,
                        &copies,
                        ret.is_some(),
                        result_origin
                            .as_ref()
                            .is_some_and(SourceOrigin::direct_copy_value),
                    )
                };
                if !reason.is_empty() {
                    return Err(unsupported("Rust Copy closure", reason, Some(start.span())));
                }
            }
            if ret.is_some() {
                let error = result_context_error(
                    &body,
                    result_origin
                        .as_ref()
                        .expect("actual resolved result context"),
                );
                if !error.is_empty() {
                    return Err(unsupported("Rust Copy closure", &error, body.span));
                }
                self.origins.argument(
                    &self.origins.origin(&body, path),
                    result_origin
                        .as_ref()
                        .expect("actual resolved source result"),
                    body.span,
                )?;
            }
            let body_origin = self.origins.origin(&body, path);
            for projection in projections.into_iter().rev() {
                body = SExpr::new(
                    SNode::Let {
                        name: projection.name,
                        ty: projection.ty.map(Into::into),
                        value: Box::new(projection.value),
                        body: Box::new(body),
                    },
                    projection.span,
                );
            }
            self.origins.mark(&body, body_origin);
            Ok(body)
        })();
        self.origins.scope = outer.clone();
        let body = read?;
        let statements = serde_json::json!([{"s":"return", "expr":&body}]);
        let facts = create_surface_reference_facts(&statements);
        let plan = crate::translation::callable_source_rules::plan_callable_statement_captures(
            &facts.forms,
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            facts.root,
            &names,
            &outer.keys().cloned().collect::<Vec<_>>(),
        );
        let count = crate::translation::frontend_rules::ml_array::number_index(plan[0])
            .expect("actual assigned capture count");
        let captures = plan[1 + count..]
            .iter()
            .map(|id| {
                facts
                    .value(*id)
                    .and_then(serde_json::Value::as_str)
                    .expect("actual capture name")
                    .to_owned()
            })
            .collect::<Vec<_>>();
        let copies = captures
            .iter()
            .map(|name| outer.get(name).is_some_and(SourceOrigin::direct_copy_value))
            .collect::<Vec<_>>();
        let capture_error =
            rules::read_rust_copy_closure_capture_error(self.origins.collecting, &copies, plan[0]);
        if !capture_error.is_empty() {
            return Err(unsupported(
                "Rust Copy closure",
                capture_error,
                Some(start.span()),
            ));
        }
        let span = Some(Span::new(
            start.start,
            self.cursor.tokens[self.cursor.index - 1].end,
        ));
        let capture_count = captures.len();
        let node = SExpr::new(
            SNode::Lambda {
                params,
                ret: ret.clone(),
                captures,
                assigned_captures: None,
                body: Box::new(body),
            },
            span,
        );
        let mut origin = if let Some(ret) = ret {
            SourceOrigin::with_type(
                Type::Function {
                    params: origins
                        .iter()
                        .map(|value| value.ty.clone().expect("actual parameter type"))
                        .collect(),
                    ret: Box::new(ret),
                },
                path,
                Vec::new(),
            )
        } else {
            let mut value = SourceOrigin::unknown();
            value.path = path.to_vec();
            value.uncertain = true;
            value
        };
        origin.copy_closure_identity = span;
        origin.copy_closure_capture_count = Some(capture_count);
        origin.callable_parameters = Some(origins);
        origin.callable_result = result_origin.map(Box::new);
        self.origins.mark(&node, origin);
        Ok(node)
    }

    pub(super) fn finish_deferred_copy_closure(
        &mut self,
        node: SExpr,
        result: &SourceOrigin,
        path: &[String],
    ) -> Result<SExpr> {
        let prior = self.origins.origin(&node, path);
        let pending = matches!(&node.node, SNode::Lambda { ret: None, .. })
            && prior.ty.is_none()
            && prior.copy_closure_identity.is_some()
            && prior.copy_closure_identity == node.span;
        let error = rules::read_rust_copy_closure_deferred_result_error(
            if matches!(&node.node, SNode::Lambda { .. }) {
                "lambda"
            } else {
                ""
            },
            pending,
            result.proven && result.ty.is_some(),
            result.direct_copy_value(),
        );
        if !error.is_empty() {
            return Err(unsupported("Rust Copy closure", &error, node.span));
        }
        let SNode::Lambda { params, body, .. } = &node.node else {
            unreachable!("shared deferred closure source contract")
        };
        let context_error = result_context_error(body, result);
        if !context_error.is_empty() {
            return Err(unsupported("Rust Copy closure", &context_error, body.span));
        }
        self.origins
            .argument(&self.origins.origin(body, path), result, body.span)?;
        let ret = result.ty.clone().expect("actual inferred result context");
        let ty = Type::Function {
            params: params
                .iter()
                .map(|value| value.ty.clone().expect("actual parameter type"))
                .collect(),
            ret: Box::new(ret.clone()),
        };
        let mut finished = node.clone();
        if let SNode::Lambda { ret: written, .. } = &mut finished.node {
            *written = Some(ret);
        }
        let mut origin = SourceOrigin::with_type(ty, path, Vec::new());
        origin.copy_closure_identity = prior.copy_closure_identity;
        origin.copy_closure_capture_count = prior.copy_closure_capture_count;
        origin.callable_parameters = prior.callable_parameters;
        origin.callable_result = Some(Box::new(result.clone()));
        self.origins.mark(&finished, origin);
        Ok(finished)
    }

    pub(super) fn read_copy_closure_call(
        &mut self,
        function: &SExpr,
        args: &[SExpr],
        path: &[String],
        span: Option<Span>,
    ) -> Result<Option<SExpr>> {
        let origin = self.origins.origin(function, path);
        if !rules::accept_rust_copy_closure_invocation(origin.ty.as_ref().map_or("", Type::kind)) {
            return Ok(None);
        }
        let Some(Type::Function { params, ret }) = origin.ty.clone() else {
            unreachable!("actual callable source type");
        };
        for (index, arg) in args.iter().enumerate() {
            if let Some(parameter @ Type::Function { .. }) = params.get(index) {
                let mut expected = origin
                    .callable_parameters
                    .as_ref()
                    .and_then(|parameters| parameters.get(index))
                    .cloned()
                    .unwrap_or_else(|| {
                        SourceOrigin::with_type(parameter.clone(), path, Vec::new())
                    });
                expected.copy_evidence = Some(());
                self.origins.argument(
                    &self.origins.origin(arg, path),
                    &expected,
                    arg.span.or(span),
                )?;
            } else if let Some(expected) = origin
                .callable_parameters
                .as_ref()
                .and_then(|parameters| parameters.get(index))
            {
                self.origins.argument(
                    &self.origins.origin(arg, path),
                    expected,
                    arg.span.or(span),
                )?;
            } else {
                self.origins.direct(arg, path, arg.span.or(span))?;
            }
        }
        let node = SExpr::new(
            SNode::App {
                func: Box::new(function.clone()),
                args: args.to_vec(),
            },
            span,
        );
        let pointer = matches!(*ret, Type::Function { .. });
        let mut result = origin
            .callable_result
            .as_deref()
            .cloned()
            .unwrap_or_else(|| SourceOrigin::with_type(*ret, path, Vec::new()));
        if pointer {
            result.copy_evidence = Some(());
        }
        self.origins.mark(&node, result);
        Ok(Some(node))
    }
}
