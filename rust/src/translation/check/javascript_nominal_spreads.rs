//! Actual binders/declarations into shared nominal-copy/absence policies.
use super::{
    Checker, Env, Expr, Language, Node, Result, SExpr, SNode, Span, Type, UnaryOp, type_error,
};
use crate::translation::ir::{Case, Pattern};
use crate::translation::javascript_constructor_presence::{field_value_type, own_value};
use crate::translation::javascript_nominal_spread_rules as rules;
use crate::translation::surface::SRecordEntry;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct FieldPath {
    pub(super) root: String,
    fields: Vec<String>,
}

#[derive(Clone, Debug)]
pub(super) struct ConstructorGuard {
    pub(super) path: FieldPath,
    pub(super) owner: String,
    pub(super) names: Vec<String>,
}

impl FieldPath {
    fn read(mut node: &SExpr) -> Option<Self> {
        let mut fields = Vec::new();
        while let SNode::Field { object, field } = &node.node {
            if fields.len() == 256 {
                return None;
            }
            fields.push(field.clone());
            node = object;
        }
        fields.reverse();
        Some(Self {
            root: node.simple_name()?.to_owned(),
            fields,
        })
    }

    fn child(mut self, field: &str) -> Option<Self> {
        if self.fields.len() >= 256 {
            return None;
        }
        self.fields.push(field.to_owned());
        Some(self)
    }
}

pub(super) fn guarded_constructors<'a>(node: &SExpr, env: &'a Env) -> Option<&'a ConstructorGuard> {
    if env.nominal_constructors.is_empty() {
        return None;
    }
    let path = FieldPath::read(node)?;
    env.nominal_constructors
        .iter()
        .find(|guard| guard.path == path)
}

pub(super) fn guards_own_field(object: &SExpr, field: &str, env: &Env) -> bool {
    !env.nominal_own_fields.is_empty()
        && FieldPath::read(object)
            .and_then(|path| path.child(field))
            .is_some_and(|path| env.nominal_own_fields.contains(&path))
}

fn direct_tag_test(condition: &SExpr) -> Option<&crate::translation::surface::STagTest> {
    use crate::translation::surface::SPatternNode;
    let test = condition.tag_test.as_deref()?;
    let SNode::Match { scrutinees, rows } = &condition.node else {
        return None;
    };
    let [source] = scrutinees.as_slice() else {
        return None;
    };
    let [matching, remaining] = rows.as_slice() else {
        return None;
    };
    let [matched] = matching.patterns.as_slice() else {
        return None;
    };
    let [fallback] = remaining.patterns.as_slice() else {
        return None;
    };
    let SPatternNode::Ctor { path, args } = &matched.node else {
        return None;
    };
    (source == &test.object
        && path.last() == Some(&test.tag)
        && args
            .iter()
            .all(|arg| matches!(&arg.node, SPatternNode::Wild))
        && matches!(&fallback.node, SPatternNode::Wild)
        && matches!(&matching.body.node, SNode::Bool { value } if *value != test.negated)
        && matches!(&remaining.body.node, SNode::Bool { value } if *value == test.negated))
    .then_some(test)
}

impl Checker {
    fn immutable_nominal_path(&self, node: &SExpr, env: &Env) -> Option<(FieldPath, Type)> {
        let path = FieldPath::read(node)?;
        let mut ty = env.vars.get(&path.root)?.clone();
        for field in &path.fields {
            let owner = if let Type::Option { element, .. } = &ty {
                element.as_ref()
            } else {
                &ty
            };
            let Type::Data { name } = owner else {
                return None;
            };
            let variants = &self.data_decl(name).ok()?.ctors;
            let declared = variants
                .iter()
                .filter_map(|variant| {
                    variant
                        .fields
                        .iter()
                        .find(|candidate| &candidate.name == field)
                })
                .collect::<Vec<_>>();
            let value = |field: &crate::translation::ir::Field| {
                field_value_type(&field.ty)
                    .filter(|_| field.optional)
                    .unwrap_or(&field.ty)
                    .clone()
            };
            let selected = value(declared.first()?);
            if declared.iter().any(|field| value(field) != selected) {
                return None;
            }
            ty = if declared.len() == variants.len() {
                selected
            } else {
                crate::translation::types::option(selected, "undefined")
            };
        }
        Some((path, ty))
    }

    pub(super) fn apply_javascript_optional_guard(
        &self,
        condition: &SExpr,
        env: &mut Env,
        outcome: bool,
        depth: usize,
    ) {
        if self.language != Language::JavaScript || depth >= 256 {
            return;
        }
        if let SNode::Unary {
            op: UnaryOp::Not,
            arg,
        } = &condition.node
        {
            self.apply_javascript_optional_guard(arg, env, !outcome, depth + 1);
            return;
        }
        if outcome
            && let SNode::Math { op, args, .. } = &condition.node
            && op == "testJavaScriptOwnedProperty"
            && let [object, key] = args.as_slice()
            && !object.spread
            && !key.spread
            && let SNode::Str { value } = &key.value.node
            && let Some((path, Type::Data { .. })) = self.immutable_nominal_path(&object.value, env)
            && let Some(path) = path.child(value)
        {
            if !env.nominal_own_fields.contains(&path) {
                env.nominal_own_fields.push(path);
            }
            return;
        }
        if let Some(test) = direct_tag_test(condition) {
            let Some((path, ty)) = self.immutable_nominal_path(&test.object, env) else {
                return;
            };
            let ty = if let Type::Option { element, .. } = ty {
                *element
            } else {
                ty
            };
            let Type::Data { name: owner } = ty else {
                return;
            };
            let Ok(entry) = self.data_decl(&owner) else {
                return;
            };
            if !entry.ctors.iter().any(|variant| variant.name == test.tag) {
                return;
            }
            let previous = env
                .nominal_constructors
                .iter()
                .position(|guard| guard.path == path);
            let mut names = previous.map_or_else(
                || {
                    entry
                        .ctors
                        .iter()
                        .map(|variant| variant.name.clone())
                        .collect::<Vec<_>>()
                },
                |index| env.nominal_constructors.remove(index).names,
            );
            names.retain(|name| (*name == test.tag) == (outcome != test.negated));
            env.nominal_constructors
                .push(ConstructorGuard { path, owner, names });
            return;
        }
        let SNode::Binary {
            op, left, right, ..
        } = &condition.node
        else {
            return;
        };
        if matches!(op, BinaryOp::And | BinaryOp::Or) {
            if (*op == BinaryOp::And) == outcome {
                self.apply_javascript_optional_guard(left, env, outcome, depth + 1);
                self.apply_javascript_optional_guard(right, env, outcome, depth + 1);
            } else {
                let mut first = env.clone();
                let mut second = env.clone();
                self.apply_javascript_optional_guard(left, &mut first, outcome, depth + 1);
                self.apply_javascript_optional_guard(right, &mut second, outcome, depth + 1);
                env.nominal_spread_absences = first
                    .nominal_spread_absences
                    .into_iter()
                    .filter(|fact| second.nominal_spread_absences.contains(fact))
                    .collect();
                env.nominal_own_fields = first
                    .nominal_own_fields
                    .into_iter()
                    .filter(|fact| second.nominal_own_fields.contains(fact))
                    .collect();
                env.nominal_constructors = first
                    .nominal_constructors
                    .into_iter()
                    .filter_map(|mut guard| {
                        let other = second
                            .nominal_constructors
                            .iter()
                            .find(|other| other.path == guard.path && other.owner == guard.owner)?;
                        for name in &other.names {
                            if !guard.names.contains(name) {
                                guard.names.push(name.clone());
                            }
                        }
                        Some(guard)
                    })
                    .collect();
            }
            return;
        }
        let absence = |node: &SExpr| match &node.node {
            SNode::None { absence } => Some(absence.clone()),
            SNode::Unit => Some("undefined".to_owned()),
            _ => None,
        };
        let (source, absence) = if let Some(absence) = absence(right) {
            (left, absence)
        } else if let Some(absence) = absence(left) {
            (right, absence)
        } else {
            return;
        };
        let arm = rules::read_java_script_nominal_spread_guard_arm("binary", op.name(), &absence);
        if arm != if outcome { 1.0 } else { 2.0 } {
            return;
        }
        let Some((path, _)) = self.immutable_nominal_path(source, env) else {
            return;
        };
        if !env
            .nominal_spread_absences
            .iter()
            .any(|fact| fact.0 == path && fact.1 == absence)
        {
            env.nominal_spread_absences.push((path, absence));
        }
    }

    pub(super) fn guarded_nominal_optional(&self, node: &SExpr, value: Expr, env: &Env) -> Expr {
        self.project_guarded_nominal_optional(FieldPath::read(node), value, env)
    }

    pub(super) fn guarded_nominal_field_optional(
        &self,
        object: &SExpr,
        field: &str,
        value: Expr,
        env: &Env,
    ) -> Expr {
        let path = FieldPath::read(object)
            .filter(|path| path.fields.len() < 256)
            .map(|mut path| {
                path.fields.push(field.to_owned());
                path
            });
        self.project_guarded_nominal_optional(path, value, env)
    }

    fn project_guarded_nominal_optional(
        &self,
        path: Option<FieldPath>,
        value: Expr,
        env: &Env,
    ) -> Expr {
        if self.language != Language::JavaScript {
            return value;
        }
        let Type::Option { element, absence } = &value.ty else {
            return value;
        };
        let Some(path) = path else { return value };
        let excluded = |absence: &str| {
            env.nominal_spread_absences
                .iter()
                .any(|(field, kind)| field == &path && kind == absence)
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
