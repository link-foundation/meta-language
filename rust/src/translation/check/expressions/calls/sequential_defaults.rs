//! Actual parameter declarations and typed values into the shared order plan.
use super::{
    Checker, Env, Expr, Flavor, INT, Node, Param, Result, SExpr, SNode, Span, Type, coerce,
    type_error, unsupported,
};
use crate::translation::Language;
use crate::translation::frontend_rules::{accept_argument_count, ml_array::number_index};
use crate::translation::sequential_parameter_default_rules as rules;
use crate::translation::surface_reference_facts::create_surface_reference_facts;
pub(super) fn requires_sequential_defaults(params: &[Param]) -> bool {
    let defaults = params
        .iter()
        .map(|param| &param.default_value)
        .collect::<Vec<_>>();
    let value = serde_json::to_value(defaults).expect("actual default source transport");
    let facts = create_surface_reference_facts(&value);
    let names = params
        .iter()
        .map(|param| param.name.clone())
        .collect::<Vec<_>>();
    let references = rules::contains_parameter_default_name_reference(
        &facts.forms,
        &facts.texts,
        &facts.starts,
        &facts.counts,
        &facts.labels,
        &facts.targets,
        &names,
    );
    rules::require_sequential_parameter_initialization(
        &params
            .iter()
            .map(|param| param.default_value.is_some() || param.optional)
            .collect::<Vec<_>>(),
        references,
    )
}
impl Checker {
    pub(super) fn sequential_parameter_call(
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
        let count = serde_json::Number::from(args.len())
            .as_f64()
            .expect("actual argument count");
        if args.len() > params.len() {
            return Err(type_error(
                format!(
                    "{full_name} expects {} arguments but got {}",
                    params.len(),
                    args.len()
                ),
                span,
            ));
        }
        if !accept_argument_count(
            params
                .iter()
                .map(|param| {
                    crate::translation::check::optional_callables::may_omit_parameter(
                        self.language,
                        param,
                    )
                })
                .collect(),
            count,
        ) {
            return Err(unsupported(
                "partial application",
                &format!("{full_name} expects {} arguments", params.len()),
                span,
            ));
        }
        let length = serde_json::Number::from(params.len())
            .as_f64()
            .expect("actual parameter count");
        let plan = rules::plan_sequential_parameter_initialization(length, count);
        if plan.is_empty() {
            return Err(unsupported(
                "parameter initialization depth",
                "the actual parameter list exceeds its shared exact source budget",
                span,
            ));
        }
        let mut inputs = std::collections::BTreeMap::new();
        let mut bindings = Vec::new();
        let mut initialization = Env::default();
        let mut values = Vec::new();
        let absent = SExpr::new(
            SNode::None {
                absence: "undefined".to_owned(),
            },
            span,
        );
        for pair in plan.chunks_exact(2) {
            let slot = number_index(pair[1]).expect("actual source slot");
            let param = &params[slot];
            if pair[0] == 0.0 {
                let expected =
                    if param.guard.is_some() && !matches!(args[slot].node, SNode::Num { .. }) {
                        &INT
                    } else {
                        &param.ty
                    };
                let value = self.expr(&args[slot], env, path, Some(expected), false)?;
                let name = self.fresh_name("ml_parameter_argument");
                let variable = Expr::var(name.clone(), value.ty.clone());
                bindings.push((name, value));
                inputs.insert(slot, variable);
                continue;
            }
            let input = inputs.remove(&slot);
            let fallback = param.default_value.as_ref().or(
                crate::translation::check::optional_callables::may_omit_parameter(
                    self.language,
                    param,
                )
                .then_some(&absent),
            );
            let mut value = match (input, fallback) {
                (Some(input), Some(fallback)) => self.javascript_binding_default_value(
                    input,
                    fallback,
                    span,
                    &initialization,
                    module_path,
                )?,
                (Some(input), None) => input,
                (None, Some(fallback)) => self.expr(
                    fallback,
                    &initialization,
                    module_path,
                    Some(if param.guard.is_some() {
                        &INT
                    } else {
                        &param.ty
                    }),
                    false,
                )?,
                (None, None) => {
                    return Err(unsupported(
                        "partial application",
                        &format!("{full_name} expects {} arguments", params.len()),
                        span,
                    ));
                }
            };
            if param.guard.is_none() {
                value = self.coerce(value, &param.ty, span)?;
            }
            initialization.bind_local(&param.name, value.ty.clone());
            let variable = Expr::var(param.name.clone(), value.ty.clone());
            bindings.push((param.name.clone(), value));
            let mut selected = coerce(
                variable,
                &param.ty,
                Language::JavaScript,
                span,
                param.guard.as_ref().map(|_| Flavor::Checked),
            )?;
            if let (
                Node::Cast {
                    flavor: Flavor::Checked,
                    message,
                    order,
                    ..
                },
                Some(guard),
            ) = (&mut selected.node, &param.guard)
            {
                *message = Some(guard.message.clone());
                *order = Some(guard.order);
            }
            values.push(selected);
        }
        let mut result = self.guard_order(full_name, values, ret);
        for (name, value) in bindings.into_iter().rev() {
            let ty = result.ty.clone();
            result = Expr::new(
                Node::Let {
                    name,
                    value: Box::new(value),
                    body: Box::new(result),
                },
                ty,
            );
        }
        Ok(result)
    }
}
