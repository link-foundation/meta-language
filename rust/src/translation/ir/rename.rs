//! Capture-avoiding renaming and traversal helpers over checked expressions and propositions.

use super::{
    BTreeSet, Binder, Case, Comparison, Effect, Env, Expr, FnDecl, Hints, Main, Node, Param,
    Pattern, Plan, PlanCase, Prop, Scope, Split, TheoremDecl,
};

/// Renames every binder in an expression; `env` maps source names to target names.
///
/// # Panics
/// On a variable the environment does not bind, which a checked program never has.
pub fn rename_expr(expr: &Expr, env: &Env, scope: &mut Scope<'_>) -> Expr {
    match &expr.node {
        Node::Var { name } => {
            let renamed = env
                .get(name)
                .unwrap_or_else(|| panic!("unbound variable {name}"));
            Expr {
                node: Node::Var {
                    name: renamed.clone(),
                },
                ..expr.clone()
            }
        }
        Node::Let { name, value, body } => {
            let value = rename_expr(value, env, scope);
            let fresh = scope.fresh(name);
            let mut inner = env.clone();
            inner.insert(name.clone(), fresh.clone());
            let body = rename_expr(body, &inner, scope);
            Expr {
                node: Node::Let {
                    name: fresh,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                ..expr.clone()
            }
        }
        Node::Match { scrutinee, cases } => {
            let scrutinee = rename_expr(scrutinee, env, scope);
            let cases = cases
                .iter()
                .map(|kase| {
                    let mut inner = env.clone();
                    let pattern = match &kase.pattern {
                        Pattern::NatSucc { name } => {
                            let fresh = scope.fresh(name);
                            inner.insert(name.clone(), fresh.clone());
                            Pattern::NatSucc { name: fresh }
                        }
                        Pattern::Bind { name } => {
                            let fresh = scope.fresh(name);
                            inner.insert(name.clone(), fresh.clone());
                            Pattern::Bind { name: fresh }
                        }
                        Pattern::Ctor { data, ctor, binds } => Pattern::Ctor {
                            data: data.clone(),
                            ctor: ctor.clone(),
                            binds: binds
                                .iter()
                                .map(|bind| {
                                    bind.as_ref().map(|bind| {
                                        let fresh = scope.fresh(bind);
                                        inner.insert(bind.clone(), fresh.clone());
                                        fresh
                                    })
                                })
                                .collect(),
                        },
                        other => other.clone(),
                    };
                    Case {
                        pattern,
                        body: rename_expr(&kase.body, &inner, scope),
                    }
                })
                .collect();
            Expr {
                node: Node::Match {
                    scrutinee: Box::new(scrutinee),
                    cases,
                },
                ..expr.clone()
            }
        }
        _ => expr.map_children(&mut |child| rename_expr(child, env, scope)),
    }
}

pub fn rename_prop(prop: &Prop, env: &Env, scope: &mut Scope<'_>) -> Prop {
    let both = |left: &Prop, right: &Prop, scope: &mut Scope<'_>| {
        let left = rename_prop(left, env, scope);
        (Box::new(left), Box::new(rename_prop(right, env, scope)))
    };
    match prop {
        Prop::Forall { binders, body } => {
            let mut inner = env.clone();
            let binders = binders
                .iter()
                .map(|binder| {
                    let name = scope.fresh(&binder.name);
                    inner.insert(binder.name.clone(), name.clone());
                    Binder {
                        name,
                        ty: binder.ty.clone(),
                    }
                })
                .collect();
            Prop::Forall {
                binders,
                body: Box::new(rename_prop(body, &inner, scope)),
            }
        }
        Prop::And { left, right } => {
            let (left, right) = both(left, right, scope);
            Prop::And { left, right }
        }
        Prop::Or { left, right } => {
            let (left, right) = both(left, right, scope);
            Prop::Or { left, right }
        }
        Prop::Implies { left, right } => {
            let (left, right) = both(left, right, scope);
            Prop::Implies { left, right }
        }
        Prop::Not { arg } => Prop::Not {
            arg: Box::new(rename_prop(arg, env, scope)),
        },
        Prop::Bool { expr } => Prop::Bool {
            expr: rename_expr(expr, env, scope),
        },
        other => {
            let (op, comparison) = other.comparison().expect("comparison");
            let left = rename_expr(&comparison.left, env, scope);
            let right = rename_expr(&comparison.right, env, scope);
            Prop::compare(
                op,
                Comparison {
                    left,
                    right,
                    domain: comparison.domain.clone(),
                },
            )
        }
    }
}

/// A function's parameters and body with target local names.
pub fn rename_function(
    entry: &FnDecl,
    ident: &dyn Fn(&str) -> String,
    reserved: &BTreeSet<String>,
) -> (Vec<Param>, Expr) {
    let mut scope = Scope::new(ident, reserved);
    let mut env = Env::new();
    let params = entry
        .params
        .iter()
        .map(|param| {
            let name = scope.fresh(&param.name);
            env.insert(param.name.clone(), name.clone());
            Param {
                name,
                ..param.clone()
            }
        })
        .collect();
    let body = rename_expr(&entry.body, &env, &mut scope);
    (params, body)
}

/// A theorem's binders, proposition and proof plan with target local names.
pub fn rename_theorem(
    entry: &TheoremDecl,
    ident: &dyn Fn(&str) -> String,
    reserved: &BTreeSet<String>,
) -> (Vec<Binder>, Prop, Plan) {
    let mut scope = Scope::new(ident, reserved);
    let mut env = Env::new();
    let binders = entry
        .binders
        .iter()
        .map(|binder| {
            let name = scope.fresh(&binder.name);
            env.insert(binder.name.clone(), name.clone());
            Binder {
                name,
                ty: binder.ty.clone(),
            }
        })
        .collect();
    let prop = rename_prop(&entry.prop, &env, &mut scope.child());
    let plan = rename_plan(&entry.proof.plan, &env, &scope);
    (binders, prop, plan)
}

pub(super) fn rename_plan(plan: &Plan, env: &Env, scope: &Scope<'_>) -> Plan {
    let split = match plan {
        Plan::Close { hints } => {
            return Plan::Close {
                hints: Hints {
                    hyps: hints
                        .hyps
                        .iter()
                        .map(|name| env.get(name).unwrap_or(name).clone())
                        .collect(),
                    ..hints.clone()
                },
            };
        }
        Plan::Induction(split) | Plan::Cases(split) => split,
    };
    let renamed = Split {
        // A split variable is always a theorem binder, so the environment names it.
        variable: env.get(&split.variable).cloned().unwrap_or_default(),
        ty: split.ty.clone(),
        cases: split
            .cases
            .iter()
            .map(|kase| {
                let mut inner = env.clone();
                let mut case_scope = scope.child();
                let mut rename = |name: &String| {
                    let fresh = case_scope.fresh(name);
                    inner.insert(name.clone(), fresh.clone());
                    fresh
                };
                let fields = kase.fields.iter().map(&mut rename).collect();
                let ihs = kase.ihs.iter().map(&mut rename).collect();
                PlanCase {
                    ctor: kase.ctor.clone(),
                    fields,
                    ihs,
                    recursive: kase.recursive.clone(),
                    plan: rename_plan(&kase.plan, &inner, &case_scope),
                }
            })
            .collect(),
    };
    match plan {
        Plan::Induction(_) => Plan::Induction(renamed),
        _ => Plan::Cases(renamed),
    }
}

/// The main effects with target local names.
pub fn rename_main(
    main: &Main,
    ident: &dyn Fn(&str) -> String,
    reserved: &BTreeSet<String>,
) -> Vec<Effect> {
    let mut scope = Scope::new(ident, reserved);
    let mut env = Env::new();
    main.effects
        .iter()
        .map(|effect| match effect {
            Effect::Let { name, value, span } => {
                let value = rename_expr(value, &env, &mut scope);
                let fresh = scope.fresh(name);
                env.insert(name.clone(), fresh.clone());
                Effect::Let {
                    name: fresh,
                    value,
                    span: *span,
                }
            }
            Effect::Print { expr, span } => Effect::Print {
                expr: rename_expr(expr, &env, &mut scope),
                span: *span,
            },
            Effect::Assert { prop, span } => Effect::Assert {
                prop: rename_prop(prop, &env, &mut scope),
                span: *span,
            },
        })
        .collect()
}

/// True when any expression of the given kind occurs in the tree.
#[must_use]
pub fn contains_kind(expr: &Expr, kind: &str) -> bool {
    expr.kind() == kind
        || expr
            .children()
            .into_iter()
            .any(|child| contains_kind(child, kind))
}

/// True when any expression of the given kind occurs in the proposition.
#[must_use]
pub fn prop_contains_kind(prop: &Prop, kind: &str) -> bool {
    prop_exprs(prop)
        .into_iter()
        .any(|expr| contains_kind(expr, kind))
}

/// The expressions of a proposition, left to right.
#[must_use]
pub fn prop_exprs(prop: &Prop) -> Vec<&Expr> {
    match prop {
        Prop::Forall { body, .. } => prop_exprs(body),
        Prop::And { left, right } | Prop::Or { left, right } | Prop::Implies { left, right } => {
            let mut exprs = prop_exprs(left);
            exprs.extend(prop_exprs(right));
            exprs
        }
        Prop::Not { arg } => prop_exprs(arg),
        Prop::Bool { expr } => vec![expr],
        other => {
            let (_, comparison) = other.comparison().expect("comparison");
            vec![&comparison.left, &comparison.right]
        }
    }
}
