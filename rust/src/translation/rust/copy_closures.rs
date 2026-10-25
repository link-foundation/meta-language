//! Original types, scopes and expressions into the existing callable nodes.
use super::{Result, RustParser, SExpr, SNode, SParam, SourceOrigin, Span, Type, unsupported};
use crate::translation::rust_contextual_closure_rules as contextual;
use crate::translation::rust_copy_closure_rules as rules;
use crate::translation::surface_reference_facts::create_surface_reference_facts;

impl RustParser {
    pub(super) fn read_expected_copy_closure(
        &mut self,
        path: &[String],
        expected: Option<&SourceOrigin>,
    ) -> Result<SExpr> {
        if contextual::accept_rust_contextual_copy_closure(
            expected
                .and_then(|origin| origin.ty.as_ref())
                .map_or("", Type::kind),
            expected.is_some_and(SourceOrigin::direct_copy_value),
            &self.cursor.peek().value,
        ) {
            self.read_copy_closure_with_expected(path, expected)
        } else {
            self.expr(path, super::Options::default())
        }
    }
    pub(super) fn read_copy_closure(&mut self, path: &[String]) -> Result<SExpr> {
        self.read_copy_closure_with_expected(path, None)
    }
    fn read_copy_closure_with_expected(
        &mut self,
        path: &[String],
        expected: Option<&SourceOrigin>,
    ) -> Result<SExpr> {
        let start = self.peek();
        self.cursor.eat("move");
        let mut params = Vec::new();
        let mut origins = Vec::new();
        if self.cursor.eat("||").is_none() {
            self.cursor.expect("|", Some("closure"))?;
            while !self.cursor.is("|") {
                let token = self.peek();
                if ["mut", "ref", "&", "_"]
                    .iter()
                    .any(|value| self.cursor.is(value))
                {
                    return Err(unsupported(
                        "Rust closure parameter",
                        "a closure needs a named direct immutable typed parameter",
                        Some(token.span()),
                    ));
                }
                let name = self.cursor.identifier(Some("closure parameter"))?.value;
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
                    let origin = SourceOrigin::with_type(ty.clone(), path, Vec::new());
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
        } else {
            expected.and_then(|origin| match origin.ty.as_ref() {
                Some(Type::Function { ret, .. }) => Some(ret.as_ref().clone()),
                _ => None,
            })
        };
        let mut result_origin = if written {
            Some(self.origins.current_type.clone())
        } else {
            ret.as_ref()
                .map(|ret| SourceOrigin::with_type(ret.clone(), path, Vec::new()))
        };
        let names = params
            .iter()
            .map(|param| param.name.clone())
            .collect::<Vec<_>>();
        let copies = origins
            .iter()
            .map(SourceOrigin::direct_copy_value)
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
        for (parameter, origin) in params.iter().zip(origins) {
            self.origins.scope.insert(parameter.name.clone(), origin);
        }
        let read: Result<SExpr> = (|| {
            let body = if written || self.cursor.is("{") {
                if let (Some(ret), Some(origin)) = (&ret, &result_origin)
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
        let Some(Type::Function { params, ret }) = origin.ty else {
            unreachable!("actual callable source type");
        };
        for (index, arg) in args.iter().enumerate() {
            if let Some(parameter @ Type::Function { .. }) = params.get(index) {
                let mut expected = SourceOrigin::with_type(parameter.clone(), path, Vec::new());
                expected.copy_evidence = Some(());
                self.origins.argument(
                    &self.origins.origin(arg, path),
                    &expected,
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
        let mut result = SourceOrigin::with_type(*ret, path, Vec::new());
        if pointer {
            result.copy_evidence = Some(());
        }
        self.origins.mark(&node, result);
        Ok(Some(node))
    }
}
