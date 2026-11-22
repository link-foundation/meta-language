//! Actual binders/declarations into shared nominal-copy/absence policies.
use super::{
    Checker, Env, Expr, Language, Node, Result, SExpr, SNode, Span, Type, UnaryOp, type_error,
};
use crate::translation::ir::{Case, Pattern};
use crate::translation::javascript_constructor_presence::{field_value_type, own_value};
use crate::translation::javascript_nominal_spread_rules as rules;
use crate::translation::surface::SRecordEntry;

impl Checker {
    pub(super) fn nominal_spread_branch_environments(
        &self,
        condition: &SExpr,
        env: &Env,
    ) -> (Env, Env) {
        let (mut yes, mut no) = (env.clone(), env.clone());
        if self.language != Language::JavaScript {
            return (yes, no);
        }
        let (guard, negated) = if let SNode::Unary {
            op: UnaryOp::Not,
            arg,
        } = &condition.node
        {
            (arg.as_ref(), true)
        } else {
            (condition, false)
        };
        let SNode::Binary {
            op, left, right, ..
        } = &guard.node
        else {
            return (yes, no);
        };
        let pair = match (&left.node, &right.node) {
            (SNode::Name { path }, SNode::None { absence }) if path.len() == 1 => {
                Some((path[0].as_str(), absence.as_str()))
            }
            (SNode::None { absence }, SNode::Name { path }) if path.len() == 1 => {
                Some((path[0].as_str(), absence.as_str()))
            }
            _ => None,
        };
        let Some((name, absence)) = pair else {
            return (yes, no);
        };
        let arm = rules::read_java_script_nominal_spread_guard_arm("binary", op.name(), absence);
        if arm == 0.0 || !env.has(name) {
            return (yes, no);
        }
        let selected = if (if negated { 3.0 - arm } else { arm }) == 1.0 {
            &mut yes
        } else {
            &mut no
        };
        selected
            .nominal_spread_absences
            .push((name.to_owned(), absence.to_owned()));
        (yes, no)
    }
    pub(super) fn guarded_nominal_optional(&self, node: &SExpr, value: Expr, env: &Env) -> Expr {
        if self.language != Language::JavaScript {
            return value;
        }
        let Some(name) = node.simple_name() else {
            return value;
        };
        let Type::Option { element, absence } = &value.ty else {
            return value;
        };
        let excluded = |absence: &str| {
            env.nominal_spread_absences
                .iter()
                .any(|(binder, kind)| binder == name && kind == absence)
        };
        if if absence == "both" {
            excluded("null") && excluded("undefined")
        } else {
            excluded(absence)
        } {
            let ty = element.as_ref().clone();
            Expr::new(
                Node::UnwrapOption {
                    arg: Box::new(value),
                },
                ty,
            )
        } else {
            value
        }
    }
    pub(super) fn nominal_spread_literal(
        &mut self,
        entries: &[SRecordEntry],
        selected: &Type,
        projection: bool,
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Option<Expr>> {
        let Type::Data { name: owner } = selected else {
            return Ok(None);
        };
        if self.language != Language::JavaScript
            || !entries.first().is_some_and(|entry| entry.spread)
        {
            return Ok(None);
        }
        let declaration = self.data_decl(owner)?.clone();
        let ctor = declaration.ctors.first();
        let kinds = entries
            .iter()
            .map(|entry| if entry.spread { "spread" } else { "field" }.to_owned())
            .collect::<Vec<_>>();
        let count = serde_json::Number::from(declaration.ctors.len())
            .as_f64()
            .expect("actual constructor count is a Number");
        if declaration.ctors.len() > 1 {
            let common = entries.iter().skip(1).all(|entry| {
                declaration.ctors[0]
                    .fields
                    .iter()
                    .find(|field| Some(field.name.as_str()) == entry.name.as_deref())
                    .is_some_and(|field| {
                        declaration.ctors[1..].iter().all(|variant| {
                            variant.fields.iter().any(|candidate| {
                                candidate.name == field.name
                                    && candidate.optional == field.optional
                                    && candidate.ty == field.ty
                            })
                        })
                    })
            });
            let error = rules::read_java_script_nominal_sum_spread_error(count, &kinds, common);
            if !error.is_empty() {
                return Err(type_error(error, span));
            }
        }
        let mut values = Vec::new();
        for entry in entries {
            let declared = if entry.spread {
                None
            } else {
                ctor.and_then(|ctor| {
                    ctor.fields
                        .iter()
                        .find(|field| Some(field.name.as_str()) == entry.name.as_deref())
                })
                .map(|field| {
                    if field.optional {
                        field_value_type(&field.ty).expect("checked field presence")
                    } else {
                        &field.ty
                    }
                })
            };
            let value = self.expr(&entry.value, env, path, declared, false)?;
            values.push(if entry.spread {
                self.guarded_nominal_optional(&entry.value, value, env)
            } else {
                value
            });
        }
        let names = entries
            .iter()
            .map(|entry| {
                if entry.spread {
                    String::new()
                } else {
                    entry.name.clone().unwrap_or_default()
                }
            })
            .collect::<Vec<_>>();
        let owners = entries
            .iter()
            .zip(&values)
            .map(|(entry, value)| {
                if entry.spread {
                    if let Type::Data { name } = &value.ty {
                        name.clone()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            })
            .collect::<Vec<_>>();
        let fields = ctor
            .map(|ctor| {
                ctor.fields
                    .iter()
                    .map(|field| field.name.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let error = rules::read_java_script_nominal_spread_error(
            self.language.as_str(),
            count.min(1.0),
            &fields,
            &kinds,
            &names,
            &owners,
            owner,
            projection,
        );
        if !error.is_empty() {
            return Err(type_error(error, span));
        }
        if declaration.ctors.len() > 1 {
            let mut cases = Vec::with_capacity(declaration.ctors.len());
            for variant in &declaration.ctors {
                let binds = variant
                    .fields
                    .iter()
                    .map(|_| self.fresh_name("ml_nominal_field"))
                    .collect::<Vec<_>>();
                let mut args = variant
                    .fields
                    .iter()
                    .zip(&binds)
                    .map(|(field, name)| Expr::var(name, field.ty.clone()))
                    .collect::<Vec<_>>();
                let mut overrides = Vec::new();
                for (entry, value) in entries.iter().skip(1).zip(values.iter().skip(1)) {
                    let index = variant
                        .fields
                        .iter()
                        .position(|field| Some(field.name.as_str()) == entry.name.as_deref())
                        .expect("the shared rule selects a common declared field");
                    let field = &variant.fields[index];
                    let expected = if field.optional {
                        field_value_type(&field.ty).expect("checked field presence")
                    } else {
                        &field.ty
                    };
                    let name = self.fresh_name("ml_nominal_override");
                    let value = self.coerce(value.clone(), expected, span)?;
                    let variable = Expr::var(&name, expected.clone());
                    args[index] = if field.optional {
                        own_value(variable, &field.ty)
                    } else {
                        variable
                    };
                    overrides.push((name, value));
                }
                let mut body = Expr::new(
                    Node::Ctor {
                        data: declaration.full_name.clone(),
                        ctor: variant.name.clone(),
                        args,
                        frozen: false,
                    },
                    selected.clone(),
                );
                for (name, value) in overrides.into_iter().rev() {
                    body = Expr::new(
                        Node::Let {
                            name,
                            value: Box::new(value),
                            body: Box::new(body),
                        },
                        selected.clone(),
                    );
                }
                cases.push(Case {
                    pattern: Pattern::Ctor {
                        data: declaration.full_name.clone(),
                        ctor: variant.name.clone(),
                        binds: binds.into_iter().map(Some).collect(),
                    },
                    body,
                });
            }
            return Ok(Some(Expr::new(
                Node::Match {
                    scrutinee: Box::new(values.remove(0)),
                    cases,
                },
                selected.clone(),
            )));
        }
        let ctor = ctor.expect("the shared rule selects one actual constructor");
        let mut bindings = Vec::new();
        let mut selected_fields = std::collections::HashMap::<String, Expr>::new();
        for (entry, value) in entries.iter().zip(values) {
            let name = self.fresh_name(if entry.spread {
                "ml_nominal_spread"
            } else {
                "ml_nominal_override"
            });
            let variable = Expr::var(&name, value.ty.clone());
            bindings.push((name, value));
            if entry.spread {
                for field in &ctor.fields {
                    let name = self.fresh_name("ml_nominal_field");
                    let value = self
                        .javascript_data_field_projection(
                            &variable,
                            &field.name,
                            span,
                            field.optional,
                        )?
                        .expect("actual nominal field projection");
                    bindings.push((name.clone(), value));
                    let current = Expr::var(name, field.ty.clone());
                    let value = match (selected_fields.remove(&field.name), &field.ty) {
                        (Some(previous), Type::Option { element, .. }) if field.optional => {
                            let condition =
                                Self::javascript_optional_presence(current.clone(), element, span)?;
                            Expr::new(
                                Node::If {
                                    cond: Box::new(condition),
                                    then: Box::new(current),
                                    otherwise: Box::new(previous),
                                },
                                field.ty.clone(),
                            )
                        }
                        _ => current,
                    };
                    selected_fields.insert(field.name.clone(), value);
                }
            } else {
                let field = entry
                    .name
                    .as_deref()
                    .expect("a checked explicit field has its name");
                let declaration = ctor
                    .fields
                    .iter()
                    .find(|item| item.name == field)
                    .expect("the shared rule selects a declared field");
                let expected = if declaration.optional {
                    field_value_type(&declaration.ty).expect("checked field presence")
                } else {
                    &declaration.ty
                };
                let value = self.coerce(variable, expected, span)?;
                selected_fields.insert(
                    field.to_owned(),
                    if declaration.optional {
                        own_value(value, &declaration.ty)
                    } else {
                        value
                    },
                );
            }
        }
        let args = ctor
            .fields
            .iter()
            .map(|field| {
                selected_fields
                    .remove(&field.name)
                    .expect("complete nominal field snapshot")
            })
            .collect();
        let mut body = Expr::new(
            Node::Ctor {
                data: declaration.full_name,
                ctor: ctor.name.clone(),
                args,
                frozen: false,
            },
            selected.clone(),
        );
        for (name, value) in bindings.into_iter().rev() {
            body = Expr::new(
                Node::Let {
                    name,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                selected.clone(),
            );
        }
        Ok(Some(body))
    }
}
