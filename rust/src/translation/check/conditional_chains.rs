//! Actual typed continuation operands into existing product and optional nodes.
use super::{Checker, Env, Expr, Language, Node, Result, SArrayItem, Span, Type, type_error};
use crate::translation::conditional_chain_rules as rules;
fn packet(element: Type) -> Type {
    Type::Option {
        element: Box::new(Type::Tuple {
            elements: vec![element],
        }),
        absence: "undefined".to_owned(),
    }
}
fn element(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Option { element, absence } if absence == "undefined" => match element.as_ref() {
            Type::Tuple { elements } if elements.len() == 1 => elements.first(),
            _ => None,
        },
        _ => None,
    }
}
fn product(value: Expr) -> Expr {
    let ty = Type::Tuple {
        elements: vec![value.ty.clone()],
    };
    Expr::new(
        Node::Tuple {
            items: vec![value],
            frozen: true,
            product_model: None,
        },
        ty,
    )
}
impl Checker {
    pub(super) fn conditional_chain_value(
        &mut self,
        op: &str,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
    ) -> Result<Expr> {
        let result = op == rules::read_conditional_chain_operation("result");
        let success = op == rules::read_conditional_chain_operation("success");
        if self.language != Language::Rust
            || args.len()
                != if result {
                    2
                } else if success {
                    1
                } else {
                    0
                }
            || args.iter().any(|arg| arg.spread)
        {
            return Err(type_error(
                "a conditional continuation needs its complete actual operands",
                span,
            ));
        }
        if result {
            let context = expected.cloned().map(packet);
            let left = self.expr(&args[0].value, env, path, context.as_ref(), false)?;
            let selected = element(&left.ty).cloned().ok_or_else(|| {
                type_error(
                    "a conditional continuation needs a single-field success packet",
                    span,
                )
            })?;
            let right = self.expr(&args[1].value, env, path, Some(&selected), false)?;
            let fallback = product(self.coerce(right, &selected, span)?);
            let ty = fallback.ty.clone();
            let object = Expr::new(
                Node::Coalesce {
                    left: Box::new(left),
                    right: Box::new(fallback),
                },
                ty,
            );
            return Ok(Expr::new(
                Node::TupleIndex {
                    object: Box::new(object),
                    index: 0,
                },
                selected,
            )
            .with_span(span));
        }
        let selected = expected.and_then(element);
        if success {
            let raw = self.expr(&args[0].value, env, path, selected, false)?;
            let value = if let Some(selected) = selected {
                self.coerce(raw, selected, span)?
            } else {
                raw
            };
            let ty = packet(value.ty.clone());
            return Ok(Expr::new(
                Node::Some {
                    value: Box::new(product(value)),
                },
                ty,
            )
            .with_span(span));
        }
        if selected.is_none() {
            return Err(type_error(
                "a conditional failure needs its actual success result context",
                span,
            ));
        }
        Ok(Expr::new(
            Node::None {
                absence: "undefined".to_owned(),
            },
            expected.expect("the actual selected packet").clone(),
        )
        .with_span(span))
    }
}
