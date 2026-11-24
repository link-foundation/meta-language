//! Actual declaration metadata into source-generated callable initialization modes.
use super::{
    Checker, Env, Expr, INT, Language, Node, Param, Result, SExpr, SNode, Span, Type, unsupported,
};
use crate::translation::ir::LambdaParam;
use crate::translation::named_default_callable_rules as rules;
pub(super) fn reference_plan(language: Language, parameters: &[Param]) -> Vec<f64> {
    let defaults = parameters
        .iter()
        .map(|param| {
            let (kind, absence) =
                param
                    .default_value
                    .as_ref()
                    .map_or(("", ""), |value| match &value.node {
                        SNode::None { absence } => ("none", absence.as_str()),
                        _ => (value.node.kind(), ""),
                    });
            rules::require_named_callable_default_initialization(
                param.default_value.is_some(),
                param.optional,
                kind,
                absence,
            )
        })
        .collect::<Vec<_>>();
    let guards = parameters
        .iter()
        .map(|param| param.guard.is_some())
        .collect::<Vec<_>>();
    rules::plan_named_callable_default_reference(language.as_str(), &defaults, &guards)
}
fn parameter_domain(ty: &Type, mode: f64) -> Result<Type> {
    let selected = if mode >= 2.0 { INT } else { ty.clone() };
    if mode == 0.0 || mode == 2.0 {
        return Ok(selected);
    }
    let absence = match &selected {
        Type::Option { absence, .. } => absence.as_str(),
        _ => "",
    };
    let absence = rules::read_named_callable_default_reference_absence(selected.kind(), absence);
    if absence.is_empty() {
        return Err(unsupported(
            "named callable default domain",
            "an optional interface needs its complete actual absence policy",
            None,
        ));
    }
    Ok(crate::translation::types::option(selected, &absence))
}
impl Checker {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn named_default_callable_reference(
        &mut self,
        full_name: &str,
        parameters: &[Param],
        ret: &Type,
        module_path: &[String],
        plan: &[f64],
        span: Option<Span>,
    ) -> Result<Expr> {
        if plan[0] < 0.0 {
            return Err(unsupported(
                "named callable default depth",
                "the actual parameter list exceeds its complete source interface budget",
                span,
            ));
        }
        let mut params = Vec::new();
        let mut env = Env::default();
        let mut args = Vec::new();
        for (index, param) in parameters.iter().enumerate() {
            let name = self.fresh_name("ml_named_default_argument");
            let ty = parameter_domain(&param.ty, plan[index + 1])?;
            env.bind_local(&name, ty.clone());
            args.push(SExpr::new(
                SNode::Name {
                    path: vec![name.clone()],
                },
                span,
            ));
            params.push(LambdaParam {
                name,
                ty,
                span: None,
                default_value: None,
                guard: None,
                rocq_type: None,
            });
        }
        let body = self.sequential_parameter_call(
            full_name.to_owned(),
            parameters,
            ret.clone(),
            module_path,
            &args,
            &env,
            module_path,
            span,
        )?;
        let ty = Type::Function {
            params: params.iter().map(|param| param.ty.clone()).collect(),
            ret: Box::new(body.ty.clone()),
        };
        Ok(Expr::new(
            Node::Lambda {
                params,
                body: Box::new(body),
            },
            ty,
        ))
    }
}
