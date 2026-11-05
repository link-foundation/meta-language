//! Actual callee and source values into the source-generated capture order.
use super::{Checker, Env, Expr, INT, Node, Param, Result, SExpr, SNode, Span, Type};
use crate::translation::excess_argument_rules as rules;
pub(super) fn plan(language: &str, parameters: usize, provided: usize) -> Vec<f64> {
    rules::plan_java_script_excess_arguments(
        language,
        serde_json::Number::from(parameters)
            .as_f64()
            .expect("actual parameters"),
        serde_json::Number::from(provided)
            .as_f64()
            .expect("actual arguments"),
    )
}
fn slot(value: f64) -> usize {
    crate::translation::frontend_rules::ml_array::number_index(value)
        .expect("an actual argument index")
}
fn wrap(bindings: Vec<(String, Expr)>, mut body: Expr) -> Expr {
    for (name, value) in bindings.into_iter().rev() {
        let ty = body.ty.clone();
        body = Expr::new(
            Node::Let {
                name,
                value: Box::new(value),
                body: Box::new(body),
            },
            ty,
        );
    }
    body
}
impl Checker {
    pub(super) fn excess_callable_arguments(
        &mut self,
        function: &Expr,
        args: &[SExpr],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Option<Expr>> {
        let Type::Function { params, ret } = &function.ty else {
            return Ok(None);
        };
        if self.checked_projection {
            return Ok(None);
        }
        let plan = plan(self.language.as_str(), params.len(), args.len());
        if plan.is_empty() {
            return Ok(None);
        }
        let callee = self.fresh_name("ml_excess_argument_callee");
        let mut bindings = vec![(callee.clone(), function.clone())];
        for index in &plan[1..] {
            let index = slot(*index);
            let value = self.expr(&args[index], env, path, params.get(index), false)?;
            bindings.push((self.fresh_name("ml_excess_argument_value"), value));
        }
        let mut arguments = Vec::new();
        for (index, (name, value)) in bindings[1..1 + slot(plan[0])].iter().enumerate() {
            arguments.push(self.coerce(
                Expr::var(name.clone(), value.ty.clone()),
                &params[index],
                args[index].span.or(span),
            )?);
        }
        let body = Expr::new(
            Node::Apply {
                func: Box::new(Expr::var(callee, function.ty.clone())),
                args: arguments,
            },
            ret.as_ref().clone(),
        );
        Ok(Some(wrap(bindings, body)))
    }
    #[allow(clippy::too_many_arguments)]
    pub(super) fn excess_named_arguments(
        &mut self,
        full_name: &str,
        params: &[Param],
        ret: &Type,
        module_path: &[String],
        args: &[SExpr],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Option<Expr>> {
        if self.checked_projection {
            return Ok(None);
        }
        let plan = plan(self.language.as_str(), params.len(), args.len());
        if plan.is_empty() {
            return Ok(None);
        }
        let mut bindings = Vec::new();
        let mut scope = env.clone();
        let mut selected = Vec::new();
        for index in &plan[1..] {
            let index = slot(*index);
            let arg = &args[index];
            let expected = params.get(index).map(|param| {
                if param.guard.is_some() && !matches!(arg.node, SNode::Num { .. }) {
                    &INT
                } else {
                    &param.ty
                }
            });
            let value = self.expr(arg, env, path, expected, false)?;
            let name = self.fresh_name("ml_excess_argument_value");
            scope.bind_local(&name, value.ty.clone());
            if index < slot(plan[0]) {
                selected.push(
                    if matches!(&arg.node,SNode::None{absence} if absence=="undefined") {
                        arg.clone()
                    } else {
                        SExpr::new(
                            SNode::Name {
                                path: vec![name.clone()],
                            },
                            arg.span.or(span),
                        )
                    },
                );
            }
            bindings.push((name, value));
        }
        let body = self.checked_call(
            full_name.to_owned(),
            params,
            ret.clone(),
            module_path,
            &selected,
            &scope,
            path,
            span,
        )?;
        Ok(Some(wrap(bindings, body)))
    }
}
