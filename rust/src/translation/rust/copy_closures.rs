//! Original types, scopes and expressions into the existing callable nodes.
use super::{Result, RustParser, SExpr, SNode, SParam, SourceOrigin, Span, Type, unsupported};
use crate::translation::rust_copy_closure_rules as rules;
use crate::translation::surface_reference_facts::create_surface_reference_facts;

impl RustParser {
    pub(super) fn read_copy_closure(&mut self, path: &[String]) -> Result<SExpr> {
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
                self.cursor.expect(":", Some("closure parameter"))?;
                let ty = self.parse_type()?;
                origins.push(self.origins.current_type.clone());
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
        if !written {
            return Err(unsupported(
                "Rust Copy closure",
                "the closure result requires its actual explicit type",
                Some(start.span()),
            ));
        }
        let ret = self.parse_type()?;
        let result_origin = self.origins.current_type.clone();
        let names = params
            .iter()
            .map(|param| param.name.clone())
            .collect::<Vec<_>>();
        let copies = origins
            .iter()
            .map(SourceOrigin::direct_copy_value)
            .collect::<Vec<_>>();
        let reason = rules::read_rust_copy_closure_header_error(
            self.ordinary_completion_context(),
            &names,
            &copies,
            written,
            result_origin.direct_copy_value(),
        );
        if !reason.is_empty() {
            return Err(unsupported("Rust Copy closure", reason, Some(start.span())));
        }
        let outer = self.origins.enter(None);
        for (parameter, origin) in params.iter().zip(origins) {
            self.origins.scope.insert(parameter.name.clone(), origin);
        }
        let read: Result<SExpr> = (|| {
            let body = if self.ordinary_completion_selected() {
                self.ordinary_function_completion(path, &ret, &result_origin)?
            } else {
                self.block_with_context(path, true)?
            };
            self.origins
                .argument(&self.origins.origin(&body, path), &result_origin, body.span)?;
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
        let Some(Type::Function { ret, .. }) = origin.ty else {
            unreachable!("actual callable source type");
        };
        for arg in args {
            self.origins.direct(arg, path, arg.span.or(span))?;
        }
        let node = SExpr::new(
            SNode::App {
                func: Box::new(function.clone()),
                args: args.to_vec(),
            },
            span,
        );
        self.origins
            .mark(&node, SourceOrigin::with_type(*ret, path, Vec::new()));
        Ok(Some(node))
    }
}
