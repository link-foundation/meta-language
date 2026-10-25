//! Actual payload-preserving optional value transport through a shared policy rule.
use super::{Checker, Expr, Language, Node, Result, Span, Type};
pub(super) fn widen_optional_policy(
    value: &Expr,
    ty: &Type,
    language: Language,
    span: Option<Span>,
) -> Result<Option<Expr>> {
    let (element, absence) = match &value.ty {
        Type::Option { element, absence } => (Some(element.as_ref()), absence.as_str()),
        _ => (None, ""),
    };
    let (target, policy) = match ty {
        Type::Option { element, absence } => (Some(element.as_ref()), absence.as_str()),
        _ => (None, ""),
    };
    let equal = element
        .zip(target)
        .is_some_and(|(left, right)| left.same(right));
    let selected =
        crate::translation::optional_policy_rules::read_java_script_optional_policy_widening(
            language.as_str(),
            value.ty.kind(),
            absence,
            ty.kind(),
            policy,
            equal,
        );
    if selected.is_empty() {
        return Ok(None);
    }
    let element = element.expect("actual optional source element");
    let name = "ml_optional_policy_value".to_owned();
    let variable = Expr::var(name.clone(), value.ty.clone());
    let condition = Checker::javascript_optional_presence(variable.clone(), element, span)?;
    let present = Expr::new(
        Node::Some {
            value: Box::new(Expr::new(
                Node::UnwrapOption {
                    arg: Box::new(variable),
                },
                element.clone(),
            )),
        },
        ty.clone(),
    );
    let absent = Expr::new(Node::None { absence: selected }, ty.clone());
    let body = Expr::new(
        Node::If {
            cond: Box::new(condition),
            then: Box::new(present),
            otherwise: Box::new(absent),
        },
        ty.clone(),
    );
    Ok(Some(Expr::new(
        Node::Let {
            name,
            value: Box::new(value.clone()),
            body: Box::new(body),
        },
        ty.clone(),
    )))
}
