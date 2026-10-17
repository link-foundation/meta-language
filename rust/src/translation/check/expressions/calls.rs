//! Named calls, argument guards and data constructor checking.

use super::super::{
    Checker, Ctor, Decl, Entry, Env, Expr, Flavor, INT, Node, Param, Result, SExpr, SNode, Span,
    Type, coerce, data, type_error, unsupported,
};
use crate::translation::frontend_rules::accept_argument_count;
use crate::translation::parameter_default_rules::{
    read_parameter_default_value_step, require_parameter_default_argument_bindings,
};

struct CheckedCallArgument<'a> {
    parameter: &'a Param,
    value: Expr,
    actual_input: bool,
    fallback_pure: Option<bool>,
    coalesced: bool,
    span: Option<Span>,
}

fn pure_parameter_default(mut value: &Expr) -> bool {
    loop {
        let flavor = match &value.node {
            Node::Cast {
                flavor: Flavor::Exact,
                ..
            } => "exact",
            _ => "",
        };
        match read_parameter_default_value_step(value.node.kind(), flavor).as_str() {
            "arg" => {
                let Node::Cast { arg, .. } = &value.node else {
                    unreachable!("the shared default traversal selects the actual cast child");
                };
                value = arg;
            }
            "value" => {
                let Node::Some { value: child } = &value.node else {
                    unreachable!("the shared default traversal selects the actual present child");
                };
                value = child;
            }
            "pure" => return true,
            _ => return false,
        }
    }
}

impl Checker {
    pub(in crate::translation::check) fn application(
        &mut self,
        head: &SExpr,
        args: &[SExpr],
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        span: Option<Span>,
    ) -> Result<Expr> {
        let segments = match &head.node {
            SNode::Name { path } => path.as_slice(),
            _ => &[],
        };
        let local_kind = env
            .local(head)
            .and_then(|name| env.vars.get(name))
            .map_or("", Type::kind);
        let path_length = serde_json::Number::from(segments.len())
            .as_f64()
            .expect("an arity has a Number representation");
        if crate::translation::callable_source_rules::require_callable_computed_application(
            head.node.kind(),
            path_length,
            local_kind,
        ) {
            let function = self.expr(head, env, path, None, false)?;
            return self.checked_callable_application(function, args, env, path, span);
        }
        let segments = match &head.node {
            SNode::DotCtor { name } => {
                let Some(Type::Data { name: data_name }) = expected else {
                    return Err(type_error(
                        format!("cannot infer the type of .{name}"),
                        span,
                    ));
                };
                let decl = self.data_decl(data_name)?;
                let full_name = decl.full_name.clone();
                let Some(ctor) = decl.ctors.iter().find(|ctor| ctor.name == *name).cloned() else {
                    return Err(type_error(
                        format!("{data_name} has no constructor {name}"),
                        span,
                    ));
                };
                return self.construct(&full_name, &ctor, args, env, path, span);
            }
            SNode::Name { path } => path,
            _ => {
                return Err(unsupported(
                    "higher-order application",
                    "only named functions and constructors can be applied",
                    span,
                ));
            }
        };
        if let Some(name) = env.local(head) {
            return Err(unsupported(
                "higher-order application",
                &format!("local {name} is not a function in the portable core"),
                span,
            ));
        }
        let Some(entry) = self.lookup(segments, path, span)? else {
            return Err(type_error(
                format!("unknown name {}", segments.join(".")),
                span,
            ));
        };
        let decl_name = match entry {
            Entry::Ctor { data, name } => {
                let decl = self.data_decl(&data)?;
                let Some(ctor) = decl.ctors.iter().find(|ctor| ctor.name == name).cloned() else {
                    return Err(type_error(
                        format!("{data} has no constructor {name}"),
                        span,
                    ));
                };
                return self.construct(&data, &ctor, args, env, path, span);
            }
            Entry::External(name) => {
                return self.external_application(&name, segments, args, env, path, span);
            }
            Entry::Decl(name) => name,
        };
        let Some(Decl::Fn(entry)) = self.decls.get(&decl_name) else {
            return Err(type_error(
                format!("{} is not a function", segments.join(".")),
                span,
            ));
        };
        if !self.signatures.contains(&decl_name) {
            return Err(type_error(
                format!("{} is used before its signature is known", entry.full_name),
                span,
            ));
        }
        let (full_name, params, ret, module_path) = (
            entry.full_name.clone(),
            entry.params.clone(),
            entry.ret.clone(),
            entry.module_path.clone(),
        );
        self.checked_call(full_name, &params, ret, &module_path, args, env, path, span)
    }

    /// A call of the function `full_name` with its checked arguments: an
    /// omitted argument takes its parameter's default, checked in `module_path`.
    #[allow(clippy::too_many_arguments)]
    pub(in crate::translation::check) fn checked_call(
        &mut self,
        full_name: String,
        params: &[Param],
        ret: Type,
        module_path: &[String],
        args: &[SExpr],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let absent = SExpr::new(
            SNode::None {
                absence: "undefined".to_owned(),
            },
            None,
        );
        let mut arguments: Vec<(&SExpr, bool)> = args.iter().map(|arg| (arg, false)).collect();
        #[allow(clippy::cast_precision_loss)]
        let argument_count = args.len() as f64;
        if args.len() < params.len()
            && accept_argument_count(
                params
                    .iter()
                    .map(|param| param.default_value.is_some() || param.optional)
                    .collect(),
                argument_count,
            )
        {
            arguments.extend(
                params[args.len()..]
                    .iter()
                    .map(|param| (param.default_value.as_ref().unwrap_or(&absent), true)),
            );
        }
        if arguments.len() != params.len() {
            if args.len() < params.len() {
                return Err(unsupported(
                    "partial application",
                    &format!("{full_name} expects {} arguments", params.len()),
                    span,
                ));
            }
            return Err(type_error(
                format!(
                    "{full_name} expects {} arguments but got {}",
                    params.len(),
                    args.len()
                ),
                span,
            ));
        }
        let mut plans = Vec::new();
        let closed_env = Env::default();
        for (index, ((arg, closed_default), param)) in arguments.iter().zip(params).enumerate() {
            let replaced = param.default_value.is_some()
                && matches!(&arg.node, SNode::None { absence } if absence == "undefined");
            let (arg, closed_default) = if matches!(&arg.node, SNode::None { absence } if absence == "undefined")
            {
                param
                    .default_value
                    .as_ref()
                    .map_or((*arg, *closed_default), |default| (default, true))
            } else {
                (*arg, *closed_default)
            };
            // A guarded argument is an integer the guard checks, wherever in it a negative value arises.
            let expected = if param.guard.is_some() && !matches!(arg.node, SNode::Num { .. }) {
                &INT
            } else {
                &param.ty
            };
            let mut value = self.expr(
                arg,
                if closed_default { &closed_env } else { env },
                if closed_default { module_path } else { path },
                Some(expected),
                false,
            )?;
            let actual_input = index < args.len() && !replaced;
            let mut fallback_pure = replaced.then(|| pure_parameter_default(&value));
            let coalesced = if let Some(default) = &param.default_value
                && matches!(&value.ty, Type::Option { element, absence } if absence == "undefined" && element.same(&param.ty))
            {
                let fallback =
                    self.expr(default, &closed_env, module_path, Some(&param.ty), false)?;
                let fallback = coerce(fallback, &param.ty, self.language, span, None)?;
                fallback_pure = Some(pure_parameter_default(&fallback));
                value = Expr::new(
                    Node::Coalesce {
                        left: Box::new(value),
                        right: Box::new(fallback),
                    },
                    param.ty.clone(),
                );
                true
            } else {
                false
            };
            plans.push(CheckedCallArgument {
                parameter: param,
                value,
                actual_input,
                fallback_pure,
                coalesced,
                span: arg.span.or(span),
            });
        }
        let bind_arguments = plans.iter().enumerate().any(|(index, plan)| {
            plan.fallback_pure.is_some_and(|pure| {
                let index = serde_json::Number::from(index)
                    .as_f64()
                    .expect("a parameter index has a Number representation");
                require_parameter_default_argument_bindings(index, argument_count, pure)
            })
        });
        let mut bindings = Vec::new();
        if bind_arguments {
            for plan in &mut plans {
                if !plan.actual_input {
                    continue;
                }
                let name = self.fresh_name("ml_a");
                let input = if plan.coalesced {
                    let Node::Coalesce { left, .. } = &mut plan.value.node else {
                        unreachable!(
                            "the actual planned optional input retains its coalescing node"
                        );
                    };
                    let variable = Expr::var(name.clone(), left.ty.clone());
                    std::mem::replace(left.as_mut(), variable)
                } else {
                    let variable = Expr::var(name.clone(), plan.value.ty.clone());
                    std::mem::replace(&mut plan.value, variable)
                };
                bindings.push((name, input));
            }
        }
        let mut checked = Vec::new();
        for plan in plans {
            let param = plan.parameter;
            let flavor = param.guard.is_some().then_some(Flavor::Checked);
            let mut value = coerce(plan.value, &param.ty, self.language, plan.span, flavor)?;
            if let (
                Node::Cast {
                    flavor: Flavor::Checked,
                    message,
                    order,
                    ..
                },
                Some(guard),
            ) = (&mut value.node, &param.guard)
            {
                *message = Some(guard.message.clone());
                *order = Some(guard.order);
            }
            checked.push(value);
        }
        let mut call = self.guard_order(full_name, checked, ret);
        for (name, value) in bindings.into_iter().rev() {
            let ty = call.ty.clone();
            call = Expr::new(
                Node::Let {
                    name,
                    value: Box::new(value),
                    body: Box::new(call),
                },
                ty,
            );
        }
        Ok(call)
    }

    /// A JavaScript call evaluates every argument, then the callee's guards run
    /// in their order. Checking each argument where it is passed is the same
    /// unless a later argument could print or abort, or the guards run in
    /// another order; then the arguments are bound first and checked after.
    fn guard_order(&mut self, func: String, args: Vec<Expr>, ret: Type) -> Expr {
        let guarded: Vec<(usize, usize)> = args
            .iter()
            .enumerate()
            .filter_map(|(index, arg)| guard_of(arg).map(|(order, _)| (index, order)))
            .collect();
        let Some(&(first, _)) = guarded.first() else {
            return Expr::new(Node::Call { func, args }, ret);
        };
        let in_order = guarded.windows(2).all(|pair| pair[0].1 < pair[1].1);
        let later = args[first + 1..]
            .iter()
            .all(|arg| simple(guard_of(arg).map_or(arg, |(_, inner)| inner)));
        if in_order && later {
            return Expr::new(Node::Call { func, args }, ret);
        }
        let bound: Vec<(String, Expr)> = args
            .iter()
            .map(|arg| {
                let value = guard_of(arg).map_or(arg, |(_, inner)| inner).clone();
                (self.fresh_name("ml_a"), value)
            })
            .collect();
        let mut sorted = guarded;
        sorted.sort_by_key(|&(_, order)| order);
        let checks: Vec<(usize, String, Expr)> = sorted
            .into_iter()
            .map(|(index, _)| {
                let mut value = args[index].clone();
                if let Node::Cast { arg, .. } = &mut value.node {
                    let ty = arg.ty.clone();
                    **arg = Expr::var(bound[index].0.clone(), ty);
                }
                (index, self.fresh_name("ml_a"), value)
            })
            .collect();
        let call_args = args
            .iter()
            .enumerate()
            .map(|(index, arg)| {
                let name = checks
                    .iter()
                    .find(|(candidate, ..)| *candidate == index)
                    .map_or(&bound[index].0, |(_, name, _)| name);
                Expr::var(name.clone(), arg.ty.clone())
            })
            .collect();
        let call = Expr::new(
            Node::Call {
                func,
                args: call_args,
            },
            ret,
        );
        let bindings = bound
            .into_iter()
            .chain(checks.into_iter().map(|(_, name, value)| (name, value)));
        bindings.rev().fold(call, |body, (name, value)| {
            let ty = body.ty.clone();
            Expr::new(
                Node::Let {
                    name,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                ty,
            )
        })
    }

    pub(in crate::translation::check) fn construct(
        &mut self,
        data_name: &str,
        ctor: &Ctor,
        args: &[SExpr],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        if args.len() != ctor.fields.len() {
            return Err(type_error(
                format!(
                    "{data_name}.{} expects {} fields but got {}",
                    ctor.name,
                    ctor.fields.len(),
                    args.len()
                ),
                span,
            ));
        }
        let mut checked = Vec::new();
        for (arg, field) in args.iter().zip(&ctor.fields) {
            let value = self.expr(arg, env, path, Some(&field.ty), false)?;
            checked.push(self.coerce(value, &field.ty, arg.span.or(span))?);
        }
        Ok(Expr::new(
            Node::Ctor {
                data: data_name.to_owned(),
                ctor: ctor.name.clone(),
                args: checked,
                frozen: false,
            },
            data(data_name),
        ))
    }

    pub(in crate::translation::check) fn ctor_object(
        &mut self,
        tag: &str,
        fields: &[(String, SExpr)],
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
    ) -> Result<Expr> {
        let mut candidates = Vec::new();
        for name in &self.order {
            let Some(Decl::Data(entry)) = self.decls.get(name) else {
                continue;
            };
            if let Some(Type::Data { name: expected }) = expected
                && entry.full_name != *expected
            {
                continue;
            }
            if let Some(ctor) = entry.ctors.iter().find(|ctor| ctor.name == tag) {
                candidates.push((entry.full_name.clone(), ctor.clone()));
            }
        }
        if candidates.len() != 1 {
            return Err(type_error(
                if candidates.is_empty() {
                    format!("no data type has tag {tag}")
                } else {
                    format!("tag {tag} is ambiguous")
                },
                span,
            ));
        }
        let (data_name, ctor) = candidates.remove(0);
        let expected_names: Vec<&str> = ctor
            .fields
            .iter()
            .map(|field| field.name.as_str())
            .collect();
        if fields.len() != expected_names.len()
            || fields
                .iter()
                .any(|(name, _)| !expected_names.contains(&name.as_str()))
        {
            return Err(type_error(
                format!("{tag} expects fields {}", expected_names.join(", ")),
                span,
            ));
        }
        let args: Vec<SExpr> = expected_names
            .iter()
            .filter_map(|name| {
                fields
                    .iter()
                    .rev()
                    .find(|(field, _)| field == name)
                    .map(|(_, value)| value.clone())
            })
            .collect();
        self.construct(&data_name, &ctor, &args, env, path, span)
    }
}

/// The order of the guard a checked argument is, and the argument it checks.
fn guard_of(arg: &Expr) -> Option<(usize, &Expr)> {
    match &arg.node {
        Node::Cast {
            arg: inner,
            order: Some(order),
            ..
        } => Some((*order, inner)),
        _ => None,
    }
}

/// A value whose evaluation neither prints nor aborts.
fn simple(e: &Expr) -> bool {
    match &e.node {
        Node::Var { .. } | Node::Lit { .. } => true,
        Node::Cast {
            flavor: Flavor::Exact,
            arg,
            ..
        } => simple(arg),
        _ => false,
    }
}
