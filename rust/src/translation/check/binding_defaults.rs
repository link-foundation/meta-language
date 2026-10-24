//! Actual type/IR transport into the source-owned default selection rules.
use super::{BOOL, Checker, Env, Expr, Node, Result, SExpr, Span, Type, type_error, unsupported};
use crate::translation::array_binding_rules::{self as rules, ml_string::Text};
fn none(absence: &str, ty: Type) -> Expr {
    Expr::new(
        Node::None {
            absence: absence.to_owned(),
        },
        ty,
    )
}
fn equal_absence(value: Expr, absence: &str) -> Expr {
    let ty = value.ty.clone();
    Expr::new(
        Node::StrictEqual {
            left: Box::new(value),
            right: Box::new(none(absence, ty)),
            negate: false,
        },
        BOOL,
    )
}
fn element(ty: &Type) -> &Type {
    match ty {
        Type::Option { element, .. } => element,
        _ => ty,
    }
}
fn absence(ty: &Type) -> &str {
    match ty {
        Type::Option { absence, .. } => absence,
        _ => "",
    }
}
impl Checker {
    fn lift_binding_default(
        &mut self,
        value: Expr,
        ty: &Type,
        drop_undefined: bool,
        span: Option<Span>,
    ) -> Result<Expr> {
        let form = rules::read_java_script_binding_default_lift_form(
            &Text::from_text(value.ty.kind()),
            &Text::from_text(if matches!(value.ty, Type::Option { .. }) {
                element(&value.ty).kind()
            } else {
                ""
            }),
            &Text::from_text(absence(&value.ty)),
            drop_undefined,
        )
        .to_string();
        if form == "present" {
            return self.coerce(value, ty, span);
        }
        let name = self.fresh_name("ml_binding_default_value");
        let bound = Expr::var(name.clone(), value.ty.clone());
        let mut body = if form == "undefined" || form == "null" {
            none(&form, ty.clone())
        } else if form == "absences" {
            Expr::new(
                Node::If {
                    cond: Box::new(equal_absence(bound.clone(), "null")),
                    then: Box::new(none("null", ty.clone())),
                    otherwise: Box::new(none("undefined", ty.clone())),
                },
                ty.clone(),
            )
        } else {
            let present = Expr::new(
                Node::UnwrapOption {
                    arg: Box::new(bound.clone()),
                },
                element(&bound.ty).clone(),
            );
            self.coerce(present, ty, span)?
        };
        if form == "optional" {
            for absent in rules::read_java_script_binding_default_lift_absences(
                &Text::from_text(absence(&bound.ty)),
                drop_undefined,
            )
            .into_iter()
            .rev()
            {
                let absent = absent.to_string();
                body = Expr::new(
                    Node::If {
                        cond: Box::new(equal_absence(bound.clone(), &absent)),
                        then: Box::new(none(&absent, ty.clone())),
                        otherwise: Box::new(body),
                    },
                    ty.clone(),
                );
            }
        }
        Ok(Expr::new(
            Node::Let {
                name,
                value: Box::new(value),
                body: Box::new(body),
            },
            ty.clone(),
        ))
    }
    pub(super) fn javascript_binding_default(
        &mut self,
        value: &SExpr,
        fallback: &SExpr,
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        let left = self.expr(value, env, path, None, false)?;
        let form = rules::read_java_script_binding_default_form(
            &Text::from_text(self.language.as_str()),
            &Text::from_text(left.ty.kind()),
            &Text::from_text(absence(&left.ty)),
        )
        .to_string();
        if form == "refuse-language" || form == "defer" {
            return Err(unsupported(
                "array binding default",
                "the actual JavaScript value domain must be known",
                span,
            ));
        }
        let first = element(&left.ty).clone();
        let right = self.expr(
            fallback,
            env,
            path,
            if first == Type::Unit {
                None
            } else {
                Some(&first)
            },
            false,
        )?;
        if form == "identity" {
            return Ok(left);
        }
        let other = element(&right.ty);
        if first != Type::Unit && *other != Type::Unit && !first.same(other) {
            return Err(type_error(
                "array binding default and present element need compatible represented types",
                span,
            ));
        }
        let base = if first == Type::Unit {
            other.clone()
        } else {
            first.clone()
        };
        let output_absence = rules::read_java_script_binding_default_result_absence(
            &Text::from_text(absence(&left.ty)),
            &Text::from_text(right.ty.kind()),
            &Text::from_text(absence(&right.ty)),
        )
        .to_string();
        let ty = if output_absence.is_empty() {
            base
        } else {
            crate::translation::types::option(base, &output_absence)
        };
        let name = self.fresh_name("ml_binding_default_operand");
        let bound = Expr::var(name.clone(), left.ty.clone());
        let fallback = self.lift_binding_default(right, &ty, false, span)?;
        let body = if first == Type::Unit && absence(&left.ty) == "undefined" {
            fallback
        } else {
            let retained = self.lift_binding_default(bound.clone(), &ty, true, span)?;
            Expr::new(
                Node::If {
                    cond: Box::new(equal_absence(bound, "undefined")),
                    then: Box::new(fallback),
                    otherwise: Box::new(retained),
                },
                ty.clone(),
            )
        };
        Ok(Expr::new(
            Node::Let {
                name,
                value: Box::new(left),
                body: Box::new(body),
            },
            ty,
        ))
    }
}
