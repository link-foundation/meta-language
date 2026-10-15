//! Bind actual surface occurrences to ordered local ownership facts.
use super::{
    BOOL, BinaryOp, Environment, LocalExpression, LocalState, LocalValue, Result, RustParser,
    SExpr, SNode, STRING, Span, Type, Value, binding_expression, fail, json, merge_availability,
    owned_local_type, owned_pair, policy,
};
impl LocalState {
    pub(super) fn reference(
        &mut self,
        entry: &mut LocalValue,
        copy: bool,
        span: Option<Span>,
    ) -> Result<LocalExpression> {
        if !entry.available {
            return Err(fail(
                "a moved String or tuple field cannot be read again",
                span,
            ));
        }
        if entry.ty == STRING {
            if copy && !self.copies {
                return Err(fail(
                    "the move profile does not erase a standard String clone",
                    span,
                ));
            }
            self.reads.push(json!({"binding":entry.name,"mode":if copy {"copy"} else {"move"},"type":entry.ty,"span":span,"path":null}));
            if !copy {
                entry.available = false;
            }
        }
        Ok(LocalExpression {
            value: SExpr::new(
                SNode::Name {
                    path: vec![entry.name.clone()],
                },
                span,
            ),
            ty: entry.ty.clone(),
        })
    }
    pub(super) fn expression(
        &mut self,
        parser: &mut RustParser,
        node: SExpr,
        environment: &mut Environment,
        constructor_literal: bool,
    ) -> Result<LocalExpression> {
        self.expression_in_context(parser, node, environment, constructor_literal, false)
    }

    pub(super) fn initializer_expression(
        &mut self,
        parser: &mut RustParser,
        node: SExpr,
        environment: &mut Environment,
    ) -> Result<LocalExpression> {
        self.expression_in_context(parser, node, environment, false, true)
    }

    fn expression_in_context(
        &mut self,
        parser: &mut RustParser,
        node: SExpr,
        environment: &mut Environment,
        constructor_literal: bool,
        call_initializer: bool,
    ) -> Result<LocalExpression> {
        let span = node.span;
        let copy = span.is_some_and(|range| parser.local_value_copies.contains(&range));
        let value = match node.node {
            SNode::Name { path } => {
                if path.len() != 1 || !environment.contains_key(&path[0]) {
                    return Err(fail(
                        "values require their actual unique local binding owner",
                        span,
                    ));
                }
                let entry = environment.get_mut(&path[0]).unwrap();
                if matches!(entry.ty, Type::Tuple { .. }) {
                    return Err(fail(
                        "whole tuple moves need a separate partial-move proof",
                        span,
                    ));
                }
                return self.reference(entry, copy, span);
            }
            SNode::Str { value } => {
                if !constructor_literal {
                    return Err(fail(
                        "a borrowed string literal is not an owned standard String value",
                        span,
                    ));
                }
                LocalExpression {
                    value: SExpr::new(SNode::Str { value }, span),
                    ty: STRING,
                }
            }
            SNode::Num {
                value,
                ty,
                negative,
                unit,
            } => {
                if !ty
                    .as_ref()
                    .is_some_and(|ty| owned_local_type(ty) && matches!(ty, Type::Fixed { .. }))
                {
                    return Err(fail(
                        "integer literals need their actual signed i32 source domain",
                        span,
                    ));
                }
                LocalExpression {
                    value: SExpr::new(
                        SNode::Num {
                            value,
                            ty: ty.clone(),
                            negative,
                            unit,
                        },
                        span,
                    ),
                    ty: ty.unwrap(),
                }
            }
            SNode::Bool { value } => LocalExpression {
                value: SExpr::new(SNode::Bool { value }, span),
                ty: BOOL,
            },
            SNode::App { func, args } if self.calls => {
                let SNode::Name { path } = &func.node else {
                    return Err(fail(
                        "calls require one complete tuple initializer with an actual unshadowed declaration owner",
                        span,
                    ));
                };
                if !call_initializer || path.len() != 1 || environment.contains_key(&path[0]) {
                    return Err(fail(
                        "calls require one complete tuple initializer with an actual unshadowed declaration owner",
                        span,
                    ));
                }
                let target = parser.origins.lookup(path, &[]).and_then(|entry| {
                    if let super::super::source_origins::SourceDeclaration::Function {
                        parameters,
                        result,
                    } = entry.declaration
                    {
                        Some((parameters, result))
                    } else {
                        None
                    }
                });
                if target.is_none() && parser.owned_tuple_call_discovery.is_none() {
                    return Err(fail(
                        "a tuple initializer needs its actual declared target",
                        span,
                    ));
                }
                if let Some((parameters, result)) = &target {
                    let parameter_pair = Type::Tuple {
                        elements: parameters
                            .iter()
                            .filter_map(|parameter| parameter.ty.clone())
                            .collect(),
                    };
                    if !result.ty.as_ref().is_some_and(owned_pair)
                        || parameters.len() != 2
                        || !owned_pair(&parameter_pair)
                    {
                        return Err(fail(
                            "the initializer target must return owned String/i32 from direct String/i32 parameters",
                            span,
                        ));
                    }
                }
                let values = args
                    .into_iter()
                    .map(|argument| self.expression(parser, argument, environment, false))
                    .collect::<Result<Vec<_>>>()?;
                let argument_pair = Type::Tuple {
                    elements: values.iter().map(|argument| argument.ty.clone()).collect(),
                };
                if !owned_pair(&argument_pair) {
                    return Err(fail(
                        "the initializer arguments must retain ordered owned String/i32 domains",
                        span,
                    ));
                }
                let ty = target
                    .and_then(|(_, result)| result.ty)
                    .unwrap_or(argument_pair);
                LocalExpression {
                    value: SExpr::new(
                        SNode::App {
                            func,
                            args: values.into_iter().map(|argument| argument.value).collect(),
                        },
                        span,
                    ),
                    ty,
                }
            }
            SNode::ToString { arg } => {
                let constructor_span = span
                    .and_then(|range| parser.local_value_constructors.get(&range))
                    .copied();
                if !matches!(arg.node, SNode::Str { .. }) || constructor_span.is_none() {
                    return Err(fail(
                        "only the actual standard String literal constructor is admitted",
                        span,
                    ));
                }
                let mut fact = policy()["constructor"].clone();
                fact["span"] = json!(arg.span);
                fact["callSpan"] = json!(constructor_span);
                fact["path"] = Value::Null;
                self.constructors.push(fact);
                let value = self.expression(parser, *arg, environment, true)?;
                LocalExpression {
                    value: SExpr::new(
                        SNode::ToString {
                            arg: Box::new(value.value),
                        },
                        span,
                    ),
                    ty: STRING,
                }
            }
            SNode::Tuple { items, .. } => {
                let items = items
                    .into_iter()
                    .map(|item| self.expression(parser, item, environment, false))
                    .collect::<Result<Vec<_>>>()?;
                let ty = Type::Tuple {
                    elements: items.iter().map(|item| item.ty.clone()).collect(),
                };
                if !owned_pair(&ty) {
                    return Err(fail(
                        "the selected tuple must contain owned String then signed i32",
                        span,
                    ));
                }
                LocalExpression {
                    value: SExpr::new(
                        SNode::Tuple {
                            items: items.into_iter().map(|item| item.value).collect(),
                            frozen: false,
                            product_model: None,
                        },
                        span,
                    ),
                    ty,
                }
            }
            SNode::TupleIndex { object, index } => {
                let SNode::Name { path } = &object.node else {
                    return Err(fail(
                        "tuple fields need their actual initialized local tuple",
                        span,
                    ));
                };
                if path.len() != 1 || !environment.contains_key(&path[0]) {
                    return Err(fail(
                        "tuple fields need their actual initialized local tuple",
                        span,
                    ));
                }
                let entry = environment.get_mut(&path[0]).unwrap();
                if !owned_pair(&entry.ty) || index > 1 {
                    return Err(fail(
                        "tuple positions must retain the exact owned text/i32 layout",
                        span,
                    ));
                }
                let Type::Tuple { elements } = &entry.ty else {
                    unreachable!()
                };
                let ty = elements[index].clone();
                let name = entry.name.clone();
                if ty == STRING {
                    self.reference(&mut entry.fields[0], copy, span)?;
                }
                LocalExpression {
                    value: SExpr::new(
                        SNode::TupleIndex {
                            object: Box::new(SExpr::new(
                                SNode::Name { path: vec![name] },
                                object.span,
                            )),
                            index,
                        },
                        span,
                    ),
                    ty,
                }
            }
            SNode::Binary {
                op, left, right, ..
            } => {
                if op != BinaryOp::Eq {
                    return Err(fail(
                        "arithmetic, ordered comparison and overflow need separate source proofs",
                        span,
                    ));
                }
                let left = self.expression(parser, *left, environment, false)?;
                let right = self.expression(parser, *right, environment, false)?;
                if left.ty == STRING || left.ty != right.ty {
                    return Err(fail(
                        "equality is confined to actual Boolean or signed i32 values",
                        span,
                    ));
                }
                LocalExpression {
                    value: SExpr::new(
                        SNode::Binary {
                            op,
                            left: Box::new(left.value),
                            right: Box::new(right.value),
                            rounding: None,
                        },
                        span,
                    ),
                    ty: BOOL,
                }
            }
            SNode::Let {
                name, value, body, ..
            } if self.copies => {
                self.name(&name, span)?;
                let value = self.expression(parser, *value, environment, false)?;
                let mut inner = environment.clone();
                inner.insert(
                    name.clone(),
                    LocalValue::new(name.clone(), value.ty.clone(), false),
                );
                let fact_type = match &value.value.node {
                    SNode::Let { ty: Some(ty), .. } => ty,
                    _ => &value.ty,
                };
                self.bind(json!({"name":name,"sourceName":name,"role":if owned_pair(fact_type) {"tuple initializer"} else {"immutable"},"mutable":false,"previous":null,"type":fact_type,"initializer":null,"index":null}));
                let body = self.expression(parser, *body, &mut inner, false)?;
                merge_availability(environment, &inner, &inner);
                binding_expression(name, value, body, span)
            }
            SNode::If {
                cond,
                then,
                otherwise,
            } if self.copies => {
                let cond = self.expression(parser, *cond, environment, false)?;
                if cond.ty != BOOL {
                    return Err(fail("conditions require actual Boolean values", span));
                }
                let mut then_environment = environment.clone();
                let mut else_environment = environment.clone();
                let then = self.expression(parser, *then, &mut then_environment, false)?;
                let otherwise =
                    self.expression(parser, *otherwise, &mut else_environment, false)?;
                if then.ty != otherwise.ty {
                    return Err(fail(
                        "branch results need the same actual value domain",
                        span,
                    ));
                }
                merge_availability(environment, &then_environment, &else_environment);
                LocalExpression {
                    ty: then.ty,
                    value: SExpr::new(
                        SNode::If {
                            cond: Box::new(cond.value),
                            then: Box::new(then.value),
                            otherwise: Box::new(otherwise.value),
                        },
                        span,
                    ),
                }
            }
            other => {
                return Err(fail(
                    &format!(
                        "expression {} has no closed owned-text interpretation",
                        other.kind()
                    ),
                    span,
                ));
            }
        };
        Ok(value)
    }
}
