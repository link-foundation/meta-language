//! Typed host adapters for the source-generated callable plans.

use super::expectation::ExpectedType;
use super::{
    Checker, Decl, Entry, Env, Expr, Node, Result, SExpr, SNode, Span, Type, coerce, type_error,
};
use crate::translation::callable_source_rules as rules;
use crate::translation::ir::LambdaParam;
use crate::translation::surface_reference_facts::create_surface_reference_facts;
use serde_json::json;

fn number(value: usize) -> f64 {
    serde_json::Number::from(value)
        .as_f64()
        .expect("a host arity has a Number representation")
}

fn index(value: f64) -> usize {
    value
        .to_string()
        .parse()
        .expect("a generated plan selects an existing fact position")
}

fn field(
    facts: &crate::translation::surface_reference_facts::SurfaceReferenceFacts<'_>,
    node: f64,
    name: &str,
) -> f64 {
    let node = index(node);
    let start = index(facts.starts[node]);
    (start..start + index(facts.counts[node]))
        .find(|edge| facts.labels[*edge] == name)
        .map_or(-1.0, |edge| facts.targets[edge])
}

fn callable_type(parameters: &[impl serde::Serialize], ret: &Type) -> Type {
    let value = json!({"parameters":parameters,"ret":ret});
    let facts = create_surface_reference_facts(&value);
    let plan = rules::plan_callable_function_type(
        &facts.starts,
        &facts.counts,
        &facts.labels,
        &facts.targets,
        field(&facts, facts.root, "parameters"),
        field(&facts, facts.root, "ret"),
    );
    let ty = |identity| {
        serde_json::from_value(
            facts
                .value(identity)
                .expect("a function constructor selects an actual type")
                .clone(),
        )
        .expect("the original type remains serializable")
    };
    Type::Function {
        params: plan[1..].iter().map(|identity| ty(*identity)).collect(),
        ret: Box::new(ty(plan[0])),
    }
}

impl Checker {
    pub(super) fn callable_reference(
        &self,
        node: &SExpr,
        segments: &[String],
        path: &[String],
        expected: ExpectedType<'_>,
    ) -> Result<Option<Expr>> {
        let entry = self.lookup(segments, path, node.span)?;
        let function = match entry {
            Some(Entry::Decl(name)) => match self.decls.get(&name) {
                Some(Decl::Fn(function)) => Some((
                    function.full_name.clone(),
                    callable_type(&function.params, &function.ret),
                )),
                _ => None,
            },
            Some(Entry::External(name)) => {
                self.externals.iter().find_map(|external| match external {
                    super::External::Function {
                        name: candidate,
                        params,
                        ret,
                    } if *candidate == name => Some((name.clone(), callable_type(params, ret))),
                    _ => None,
                })
            }
            _ => None,
        };
        let accepted = rules::accept_callable_function_reference(
            node.node.kind(),
            expected.kind(),
            if function.is_some() { "fn" } else { "" },
        );
        if !accepted {
            if matches!(node.node, SNode::FunctionValue { .. }) {
                return Err(type_error(
                    format!("unknown function {}", segments.join(".")),
                    node.span,
                ));
            }
            return Ok(None);
        }
        let (name, ty) =
            function.expect("the generated reference rule selects an actual function declaration");
        Ok(Some(Expr::new(Node::FunctionRef { func: name }, ty)))
    }

    /// Both computed source calls and record methods use this same arity/coercion path.
    pub(super) fn checked_callable_application(
        &mut self,
        function: Expr,
        args: &[SExpr],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        self.checked_callable_application_at(function, args, env, path, span, &[])
    }

    fn checked_callable_application_at(
        &mut self,
        function: Expr,
        args: &[SExpr],
        env: &Env,
        path: &[String],
        span: Option<Span>,
        places: &[Option<Span>],
    ) -> Result<Expr> {
        let (params, ret) = match &function.ty {
            Type::Function { params, ret } => (params.as_slice(), Some(ret.as_ref())),
            _ => (&[][..], None),
        };
        let error = rules::read_callable_application_error(
            function.ty.kind(),
            number(params.len()),
            number(args.len()),
        );
        if !error.is_empty() {
            return Err(type_error(error, span));
        }
        let checked_args = args
            .iter()
            .zip(params)
            .enumerate()
            .map(|(index, (argument, expected))| {
                let checked = self.expr(argument, env, path, Some(expected), false)?;
                coerce(
                    checked,
                    expected,
                    self.language,
                    places.get(index).copied().flatten().or(span),
                    None,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        let ty = ret
            .expect("a generated successful call has an actual Function return slot")
            .clone();
        Ok(Expr::new(
            Node::Apply {
                func: Box::new(function),
                args: checked_args,
            },
            ty,
        ))
    }

    pub(super) fn checked_record_method_application(
        &mut self,
        object: Expr,
        operation: &str,
        args: &[Option<SExpr>],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let fields = match &object.ty {
            Type::Record { fields, .. } => fields.as_slice(),
            _ => &[],
        };
        let names = fields
            .iter()
            .map(|field| field.name.encode_utf16().map(f64::from).collect())
            .collect::<Vec<Vec<f64>>>();
        let operation_units = operation.encode_utf16().map(f64::from).collect::<Vec<_>>();
        let selected = crate::translation::record_value_rules::read_record_field_position(
            &names,
            &operation_units,
        );
        let selected = selected
            .to_string()
            .parse::<usize>()
            .ok()
            .and_then(|index| fields.get(index));
        let (kind, count) = match selected.map(|field| &field.ty) {
            Some(Type::Function { params, .. }) => ("function", params.len()),
            Some(ty) => (ty.kind(), 0),
            None => ("", 0),
        };
        let error = crate::translation::record_value_rules::read_record_method_error(
            kind,
            operation,
            number(count),
            number(args.len()),
        );
        if !error.is_empty() {
            return Err(type_error(error, span));
        }
        let function = Self::record_field(object, operation, span)?
            .expect("a successful generated method plan selects an actual record field");
        let args = args
            .iter()
            .cloned()
            .map(|argument| {
                argument.ok_or_else(|| {
                    type_error("a record method argument has no actual expression", span)
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let places = args
            .iter()
            .map(|argument| argument.span)
            .collect::<Vec<_>>();
        self.checked_callable_application_at(function, &args, env, path, span, &places)
    }

    pub(super) fn callable_expression(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
        expected: ExpectedType<'_>,
    ) -> Result<Expr> {
        match &node.node {
            SNode::FunctionValue { path: segments } => self
                .callable_reference(node, segments, path, expected)?
                .ok_or_else(|| type_error("an actual function reference is required", node.span)),
            SNode::Lambda { params, body, .. } => {
                let value = json!({"parameters":params,"expected":expected.value()});
                let facts = create_surface_reference_facts(&value);
                let plan = rules::plan_callable_parameter_types(
                    &facts.forms,
                    &facts.texts,
                    &facts.starts,
                    &facts.counts,
                    &facts.labels,
                    &facts.targets,
                    field(&facts, facts.root, "parameters"),
                    field(&facts, facts.root, "expected"),
                );
                let mut inner = env.clone();
                let mut parameters = Vec::new();
                for (parameter, selection) in params.iter().zip(plan.as_chunks::<3>().0) {
                    let selected: Option<Type> = facts
                        .value(selection[1])
                        .filter(|value| !value.is_null())
                        .map(|value| {
                            serde_json::from_value(value.clone()).map_err(|error| {
                                type_error(
                                    format!(
                                        "a callable parameter annotation is malformed: {error}"
                                    ),
                                    parameter.span.or(node.span),
                                )
                            })
                        })
                        .transpose()?;
                    let ty = if selection[2].to_bits() == 1.0_f64.to_bits() {
                        selected
                            .map(|ty| self.resolve_type(Some(&ty), path, parameter.span))
                            .transpose()?
                    } else {
                        selected
                    };
                    let ty = ty.ok_or_else(|| {
                        type_error(
                            format!("cannot infer callback parameter {}", parameter.name),
                            parameter.span.or(node.span),
                        )
                    })?;
                    inner.bind_local(&parameter.name, ty.clone());
                    parameters.push(LambdaParam {
                        name: parameter.name.clone(),
                        ty,
                        span: parameter.span,
                        default_value: parameter.default_value.clone(),
                        guard: parameter.guard.clone(),
                        rocq_type: parameter.rocq_type.clone(),
                    });
                }
                let result = expected.result();
                let body = self.expr(body, &inner, path, result, false)?;
                let ty = callable_type(&parameters, &body.ty);
                Ok(Expr::new(
                    Node::Lambda {
                        params: parameters,
                        body: Box::new(body),
                    },
                    ty,
                ))
            }
            SNode::CellAllocate { value: initial, .. } => {
                let value = json!({"node":node,"expected":expected.value()});
                let facts = create_surface_reference_facts(&value);
                let plan = rules::plan_callable_cell_element(
                    &facts.forms,
                    &facts.texts,
                    &facts.starts,
                    &facts.counts,
                    &facts.labels,
                    &facts.targets,
                    field(&facts, facts.root, "node"),
                    field(&facts, facts.root, "expected"),
                );
                let selected = facts
                    .value(plan[0])
                    .filter(|value| !value.is_null())
                    .map(|value| {
                        serde_json::from_value::<Type>(value.clone()).map_err(|error| {
                            type_error(
                                format!("a cell element annotation is malformed: {error}"),
                                node.span,
                            )
                        })
                    })
                    .transpose()?;
                let selected = if plan[1].to_bits() == 1.0_f64.to_bits() {
                    selected
                        .map(|ty| self.resolve_type(Some(&ty), path, node.span))
                        .transpose()?
                } else {
                    selected
                };
                let mut value = self.expr(initial, env, path, selected.as_ref(), false)?;
                let element = selected.unwrap_or_else(|| value.ty.clone());
                value = coerce(value, &element, self.language, node.span, None)?;
                Ok(Expr::new(
                    Node::CellAllocate {
                        value: Box::new(value),
                    },
                    Type::Cell {
                        element: Box::new(element),
                    },
                ))
            }
            SNode::CellRead { cell } => {
                let cell = self.expr(cell, env, path, None, false)?;
                let error = rules::read_callable_cell_error(cell.ty.kind(), "cellRead");
                if !error.is_empty() {
                    return Err(type_error(error, node.span));
                }
                let Type::Cell { element } = &cell.ty else {
                    unreachable!("the generated cell rule validates Cell")
                };
                let ty = element.as_ref().clone();
                Ok(Expr::new(
                    Node::CellRead {
                        cell: Box::new(cell),
                    },
                    ty,
                ))
            }
            SNode::CellWrite { cell, value, body } => {
                let cell = self.expr(cell, env, path, None, false)?;
                let error = rules::read_callable_cell_error(cell.ty.kind(), "cellWrite");
                if !error.is_empty() {
                    return Err(type_error(error, node.span));
                }
                let Type::Cell { element } = &cell.ty else {
                    unreachable!("the generated cell rule validates Cell")
                };
                let value = coerce(
                    self.expr(value, env, path, Some(element), false)?,
                    element,
                    self.language,
                    node.span,
                    None,
                )?;
                let body = self.expr_with_context(body, env, path, expected, false)?;
                let ty = body.ty.clone();
                Ok(Expr::new(
                    Node::CellWrite {
                        cell: Box::new(cell),
                        value: Box::new(value),
                        body: Box::new(body),
                    },
                    ty,
                ))
            }
            _ => unreachable!("the caller selects an actual callable family"),
        }
    }
}
