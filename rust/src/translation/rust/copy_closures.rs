//! Original types, scopes and expressions into the existing callable nodes.
use super::{
    Result, RustParser, SExpr, SNode, SParam, SourceDeclaration, SourceOrigin, Span, Type,
    unsupported,
};
use crate::translation::rust_contextual_closure_rules as contextual;
use crate::translation::rust_copy_closure_rules as rules;
use crate::translation::surface_reference_facts::create_surface_reference_facts;

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
            && let SourceDeclaration::Function {
                parameters: actual,
                result: actual_result,
                ..
            } = entry.declaration
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
            args.push(self.read_standard_string_view_argument(
                path,
                parameters.get(args.len()),
                result.as_ref(),
            )?);
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
            self.read_copy_closure_with_expected(path, expected, parameters_only)
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
                        let node =
                            self.read_copy_closure_with_expected(path, expected, parameters_only)?;
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
        self.read_copy_closure_with_expected(path, None, false)
    }
    fn read_copy_closure_with_expected(
        &mut self,
        path: &[String],
        expected: Option<&SourceOrigin>,
        parameters_only: bool,
    ) -> Result<SExpr> {
        let start = self.peek();
        self.cursor.eat("move");
        let mut params = Vec::new();
        let mut origins = Vec::new();
        if self.cursor.eat("||").is_none() {
            self.cursor.expect("|", Some("closure"))?;
            while !self.cursor.is("|") {
                let token = self.peek();
                let role = ["mut", "ref", "&", "_"]
                    .into_iter()
                    .find(|value| self.cursor.is(value))
                    .unwrap_or("binding");
                let form = rules::read_rust_copy_closure_parameter_form(role);
                if form == "unsupported" {
                    return Err(unsupported(
                        "Rust closure parameter",
                        "a closure needs a named direct immutable typed parameter",
                        Some(token.span()),
                    ));
                }
                let name = if form == "wildcard" {
                    self.cursor.advance();
                    self.fresh_product_binding()
                } else {
                    let name = self.cursor.identifier(Some("closure parameter"))?.value;
                    self.require_nonconstant_binding(&name, path, Some(token.span()))?;
                    name
                };
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
        for (parameter, origin) in params.iter().zip(origins.iter().cloned()) {
            self.origins.scope.insert(parameter.name.clone(), origin);
        }
        let read: Result<SExpr> = (|| {
            let body = if written || self.cursor.is("{") {
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
                let reason = contextual::read_rust_contextual_copy_closure_header_error(
                    self.ordinary_completion_context(),
                    &names,
                    &copies,
                    ret.is_some(),
                    result_origin
                        .as_ref()
                        .is_some_and(SourceOrigin::direct_copy_value),
                );
                if !reason.is_empty() {
                    return Err(unsupported("Rust Copy closure", reason, Some(start.span())));
                }
            }
            self.origins.argument(
                &self.origins.origin(&body, path),
                result_origin
                    .as_ref()
                    .expect("actual resolved source result"),
                body.span,
            )?;
            Ok(body)
        })();
        self.origins.scope = outer.clone();
        let body = read?;
        let ret = ret.expect("actual resolved source result");
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
        let ty = Type::Function {
            params: params
                .iter()
                .map(|param| param.ty.clone().expect("actual written type"))
                .collect(),
            ret: Box::new(ret.clone()),
        };
        let span = Some(Span::new(
            start.start,
            self.cursor.tokens[self.cursor.index - 1].end,
        ));
        let capture_count = captures.len();
        let node = SExpr::new(
            SNode::Lambda {
                params,
                ret: Some(ret),
                captures,
                assigned_captures: None,
                body: Box::new(body),
            },
            span,
        );
        let mut origin = SourceOrigin::with_type(ty, path, Vec::new());
        origin.copy_closure_identity = span;
        origin.copy_closure_capture_count = Some(capture_count);
        origin.callable_parameters = Some(origins);
        origin.callable_result = result_origin.map(Box::new);
        self.origins.mark(&node, origin);
        Ok(node)
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
