//! Once-evaluated tuple slots and explicit ordered continuation versions.
use super::{
    BOOL, Environment, LocalBindingPattern, LocalExpression, LocalState, LocalValue, Result,
    RustParser, SExpr, SNode, STRING, Statement, Type, binding_expression, fail, json, owned_pair,
    reserved, tuple_call_policy,
};
impl LocalState {
    pub(super) fn sequence(
        &mut self,
        parser: &mut RustParser,
        statements: &[Statement],
        environment: &mut Environment,
        tail_slot: Option<&str>,
    ) -> Result<LocalExpression> {
        let Some((statement, rest)) = statements.split_first() else {
            let Some(slot) = tail_slot else {
                return Err(fail("a function needs its final String value", None));
            };
            return self.reference(
                environment.get_mut(slot).ok_or_else(|| {
                    fail(
                        "only the actual tuple-owned mutable String slot can be replaced",
                        None,
                    )
                })?,
                false,
                None,
            );
        };
        match statement {
            Statement::Tail(value) => {
                if !rest.is_empty() || tail_slot.is_some() {
                    return Err(fail(
                        "statement branches cannot escape their ordered local state continuation",
                        value.span,
                    ));
                }
                self.expression(parser, value.clone(), environment, false)
            }
            Statement::Let {
                pattern,
                ty,
                value,
                span,
            } => {
                let value = if self.calls && matches!(pattern, LocalBindingPattern::Tuple { .. }) {
                    self.initializer_expression(parser, value.clone(), environment)?
                } else {
                    self.expression(parser, value.clone(), environment, false)?
                };
                if ty.as_ref().is_some_and(|ty| *ty != value.ty) {
                    return Err(fail(
                        "the actual local annotation differs from its value domain",
                        *span,
                    ));
                }
                if let LocalBindingPattern::Name(binding) = pattern {
                    if binding.mutable {
                        return Err(fail(
                            "the move profile requires one owned tuple-declared mutable String slot",
                            binding.span,
                        ));
                    }
                    self.name(&binding.name, binding.span)?;
                    environment.insert(
                        binding.name.clone(),
                        LocalValue::new(binding.name.clone(), value.ty.clone(), false),
                    );
                    self.bind(json!({"name":binding.name,"sourceName":binding.name,"role":"immutable","mutable":false,"previous":null,"type":value.ty,"initializer":null,"index":null}));
                    let body = self.sequence(parser, rest, environment, tail_slot)?;
                    return Ok(binding_expression(binding.name.clone(), value, body, *span));
                }
                let LocalBindingPattern::Tuple {
                    items,
                    span: pattern_span,
                } = pattern
                else {
                    unreachable!()
                };
                if !owned_pair(&value.ty)
                    || items.len() != 2
                    || !items[0].mutable
                    || items[1].mutable
                    || self.mutable_slot.is_some()
                {
                    return Err(fail(
                        "one fresh String/i32 unpacking owns exactly one mutable String slot",
                        *pattern_span,
                    ));
                }
                let tuple_name = self.fresh(parser);
                let fields: Vec<_> = items.iter().map(|item| item.name.clone()).collect();
                if fields[0] == fields[1]
                    || fields
                        .iter()
                        .any(|name| self.names.contains(name) || reserved(name))
                {
                    return Err(fail(
                        "tuple fields require distinct unshadowed local owners",
                        *pattern_span,
                    ));
                }
                self.names.extend(fields.iter().cloned());
                self.mutable_slot = Some(fields[0].clone());
                let target = if self.calls {
                    let SNode::App { func, .. } = &value.value.node else {
                        return Err(fail(
                            "the selected call model requires an actual declared tuple initializer",
                            *span,
                        ));
                    };
                    let SNode::Name { path } = &func.node else {
                        unreachable!()
                    };
                    Some(path[0].clone())
                } else {
                    None
                };
                self.bind(json!({"name":tuple_name,"sourceName":tuple_name,"role":"tuple initializer","mutable":false,"previous":null,"type":value.ty,"initializer":null,"index":null}));
                let mut initializer = json!({"binding":tuple_name,"fields":fields,"path":null});
                if let Some(target) = target {
                    initializer["target"] = json!(target);
                    initializer["fieldModes"] = tuple_call_policy()["fieldModes"].clone();
                }
                self.initializers.push(initializer);
                let Type::Tuple { elements } = &value.ty else {
                    unreachable!()
                };
                for (index, item) in items.iter().enumerate() {
                    environment.insert(
                        item.name.clone(),
                        LocalValue::new(item.name.clone(), elements[index].clone(), index == 0),
                    );
                    self.bind(json!({"name":item.name,"sourceName":item.name,"role":"tuple field","mutable":index==0,"previous":null,"type":elements[index],"initializer":tuple_name,"index":index}));
                }
                self.reads.push(json!({"binding":format!("{tuple_name}.0"),"mode":"move","type":STRING,"span":items[0].span,"path":null}));
                let mut body = self.sequence(parser, rest, environment, tail_slot)?;
                for index in (0..2).rev() {
                    let item = &items[index];
                    let projection = LocalExpression {
                        value: SExpr::new(
                            SNode::TupleIndex {
                                object: Box::new(SExpr::new(
                                    SNode::Name {
                                        path: vec![tuple_name.clone()],
                                    },
                                    None,
                                )),
                                index,
                            },
                            item.span,
                        ),
                        ty: elements[index].clone(),
                    };
                    body = binding_expression(item.name.clone(), projection, body, item.span);
                }
                Ok(binding_expression(tuple_name, value, body, *span))
            }
            Statement::Write {
                target,
                value,
                span,
            } => {
                let Some(slot) = environment.get(target) else {
                    return Err(fail(
                        "only the actual tuple-owned mutable String slot can be replaced",
                        *span,
                    ));
                };
                if !slot.mutable || slot.ty != STRING {
                    return Err(fail(
                        "only the actual tuple-owned mutable String slot can be replaced",
                        *span,
                    ));
                }
                let value = self.expression(parser, value.clone(), environment, false)?;
                if value.ty != STRING {
                    return Err(fail(
                        "replacement changes the actual String value domain",
                        *span,
                    ));
                }
                let slot = environment.get_mut(target).unwrap();
                let previous = slot.name.clone();
                let name = self.fresh(parser);
                slot.name.clone_from(&name);
                slot.available = true;
                self.writes += 1;
                self.bind(json!({"name":name,"sourceName":target,"role":"replacement","mutable":false,"previous":previous,"type":STRING,"initializer":null,"index":null}));
                let body = self.sequence(parser, rest, environment, tail_slot)?;
                Ok(binding_expression(name, value, body, *span))
            }
            Statement::If {
                condition,
                then,
                otherwise,
                span,
            } => {
                let Some(mutable_slot) = self.mutable_slot.clone() else {
                    return Err(fail(
                        "an ordered conditional must belong to an initialized mutable String slot",
                        *span,
                    ));
                };
                let cond = self.expression(parser, condition.clone(), environment, false)?;
                if cond.ty != BOOL {
                    return Err(fail("conditions require actual Boolean values", *span));
                }
                let previous = environment[&mutable_slot].name.clone();
                let then =
                    self.sequence(parser, then, &mut environment.clone(), Some(&mutable_slot))?;
                let otherwise = self.sequence(
                    parser,
                    otherwise,
                    &mut environment.clone(),
                    Some(&mutable_slot),
                )?;
                let name = self.fresh(parser);
                let slot = environment.get_mut(&mutable_slot).unwrap();
                slot.name.clone_from(&name);
                slot.available = true;
                self.bind(json!({"name":name,"sourceName":mutable_slot,"role":"join","mutable":false,"previous":previous,"type":STRING,"initializer":null,"index":null}));
                let value = LocalExpression {
                    value: SExpr::new(
                        SNode::If {
                            cond: Box::new(cond.value),
                            then: Box::new(then.value),
                            otherwise: Box::new(otherwise.value),
                        },
                        *span,
                    ),
                    ty: STRING,
                };
                let body = self.sequence(parser, rest, environment, tail_slot)?;
                Ok(binding_expression(name, value, body, *span))
            }
        }
    }
}
