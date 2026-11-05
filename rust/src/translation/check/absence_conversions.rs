//! Actual values and absence policies into shared generated conversion choices.
use super::{BOOL, Expr, Language, Node, Type};
pub(super) fn convert_absence_only_value(
    value: &Expr,
    ty: &Type,
    language: Language,
) -> Option<Expr> {
    let (element, absence) = match &value.ty {
        Type::Option { element, absence } => (element.kind(), absence.as_str()),
        _ => ("", ""),
    };
    let policy = match ty {
        Type::Option { absence, .. } => absence.as_str(),
        _ => "",
    };
    let form =
        crate::translation::absence_conversion_rules::read_java_script_absence_only_conversion(
            language.as_str(),
            value.ty.kind(),
            element,
            absence,
            ty.kind(),
            policy,
        );
    if form.is_empty() {
        return None;
    }
    let name = "ml_absence_conversion_value".to_owned();
    let absent = |absence: &str| {
        Expr::new(
            Node::None {
                absence: absence.to_owned(),
            },
            ty.clone(),
        )
    };
    let body = if form == "undefined" {
        absent("undefined")
    } else {
        let condition = Expr::new(
            Node::StrictEqual {
                left: Box::new(Expr::var(name.clone(), value.ty.clone())),
                right: Box::new(Expr::new(
                    Node::None {
                        absence: "null".to_owned(),
                    },
                    value.ty.clone(),
                )),
                negate: false,
            },
            BOOL,
        );
        Expr::new(
            Node::If {
                cond: Box::new(condition),
                then: Box::new(absent("null")),
                otherwise: Box::new(absent("undefined")),
            },
            ty.clone(),
        )
    };
    Some(Expr::new(
        Node::Let {
            name,
            value: Box::new(value.clone()),
            body: Box::new(body),
        },
        ty.clone(),
    ))
}
