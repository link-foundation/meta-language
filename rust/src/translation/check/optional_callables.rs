//! Actual typed arguments and explicit present wrappers from shared source plans.
use super::{Checker, Expr, Language, Node, Result, SExpr, SNode, Span, Type};
use crate::translation::ir::LambdaParam;
use crate::translation::optional_callable_rules as rules;
fn argument_plan(language: Language, parameters: &[Type], provided: usize) -> Vec<f64> {
    let kinds = parameters
        .iter()
        .map(|param| param.kind().to_owned())
        .collect::<Vec<_>>();
    let absences = parameters
        .iter()
        .map(|param| match param {
            Type::Option { absence, .. } => absence.clone(),
            _ => String::new(),
        })
        .collect::<Vec<_>>();
    rules::plan_optional_callable_arguments(
        language.as_str(),
        &kinds,
        &absences,
        serde_json::Number::from(provided)
            .as_f64()
            .expect("actual argument count"),
    )
}
pub(super) fn argument_count(language: Language, parameters: &[Type], provided: usize) -> usize {
    if argument_plan(language, parameters, provided).is_empty() {
        provided
    } else {
        parameters.len()
    }
}
pub(super) fn complete_arguments(
    language: Language,
    parameters: &[Type],
    args: &[SExpr],
    span: Option<Span>,
) -> Vec<SExpr> {
    let plan = argument_plan(language, parameters, args.len());
    if plan.is_empty() || args.len() == parameters.len() {
        return args.to_vec();
    }
    plan[1..]
        .iter()
        .map(|slot| {
            if *slot < 0.0 {
                SExpr::new(
                    SNode::None {
                        absence: "undefined".to_owned(),
                    },
                    span,
                )
            } else {
                args[crate::translation::frontend_rules::ml_array::number_index(*slot)
                    .expect("actual argument slot")]
                .clone()
            }
        })
        .collect()
}
impl Checker {
    pub(super) fn optional_callback_bridge(
        &mut self,
        value: Expr,
        supplied: &[Type],
        span: Option<Span>,
    ) -> Result<Expr> {
        let Type::Function {
            params: actual,
            ret,
        } = &value.ty
        else {
            return Ok(value);
        };
        let exact = actual
            .iter()
            .enumerate()
            .map(|(index, ty)| supplied.get(index).is_some_and(|other| ty.same(other)))
            .collect::<Vec<_>>();
        let lifted=actual.iter().enumerate().map(|(index,ty)|matches!(ty,Type::Option{element,..} if supplied.get(index).is_some_and(|other|element.same(other)))).collect::<Vec<_>>();
        let count = serde_json::Number::from(supplied.len())
            .as_f64()
            .expect("actual callback argument count");
        let plan = rules::plan_optional_callable_parameter_bridge(
            self.language.as_str(),
            count,
            &exact,
            &lifted,
        );
        if plan.is_empty() {
            return Ok(value);
        }
        let name = self.fresh_name("ml_optional_callback");
        let bound = Expr::var(name.clone(), value.ty.clone());
        let mut parameters = Vec::new();
        let mut args = Vec::new();
        for (index, ty) in actual.iter().enumerate() {
            let variable = self.fresh_name("ml_optional_callback_argument");
            let argument = Expr::var(variable.clone(), supplied[index].clone());
            args.push(if plan[index + 1] == 0.0 {
                argument
            } else {
                self.coerce(argument, ty, span)?
            });
            parameters.push(LambdaParam {
                name: variable,
                ty: supplied[index].clone(),
                span: None,
                default_value: None,
                guard: None,
                rocq_type: None,
            });
        }
        let ty = Type::Function {
            params: parameters.iter().map(|param| param.ty.clone()).collect(),
            ret: ret.clone(),
        };
        let body = Expr::new(
            Node::Lambda {
                params: parameters,
                body: Box::new(Expr::new(
                    Node::Apply {
                        func: Box::new(bound),
                        args,
                    },
                    ret.as_ref().clone(),
                )),
            },
            ty.clone(),
        );
        Ok(Expr::new(
            Node::Let {
                name,
                value: Box::new(value),
                body: Box::new(body),
            },
            ty,
        ))
    }
}
