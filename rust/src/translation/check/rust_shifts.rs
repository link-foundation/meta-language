//! Source lowering retains the total result and explicit overflow continuation.
use super::{Checker, Env, Expr, Node, Result, SArrayItem, Span, Type, type_error};
use crate::translation::{Language, rust_primitive_bitwise_rules, rust_shift_rules as rules};

impl Checker {
    pub(super) fn rust_shift(
        &mut self,
        operation: &str,
        arguments: &[SArrayItem],
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        span: Option<Span>,
    ) -> Result<Expr> {
        if self.language != Language::Rust
            || arguments.len() != 2
            || arguments.iter().any(|argument| argument.spread)
        {
            return Err(type_error(
                "a Rust shift requires two actual unspread integer operands",
                span,
            ));
        }
        let target = expected.filter(|ty| matches!(ty, Type::Fixed { .. }));
        let mut left = self.expr(&arguments[0].value, env, path, target, true)?;
        let mut count = self.expr(&arguments[1].value, env, path, None, true)?;
        if left.ty.is_literal() {
            let ty = target.cloned().unwrap_or_else(|| self.default_number());
            left = self.expr(&arguments[0].value, env, path, Some(&ty), false)?;
        }
        if count.ty.is_literal() {
            count = self.expr(
                &arguments[1].value,
                env,
                path,
                Some(&self.default_number()),
                false,
            )?;
        }
        let bits = |ty: &Type| {
            if let Type::Fixed { bits, .. } = ty {
                Some(f64::from(*bits))
            } else {
                None
            }
        };
        if !bits(&left.ty).is_some_and(|bits| {
            rust_primitive_bitwise_rules::accept_rust_primitive_bitwise_type("fixed", bits)
        }) || !bits(&count.ty).is_some_and(|bits| {
            rust_primitive_bitwise_rules::accept_rust_primitive_bitwise_type("fixed", bits)
        }) {
            return Err(type_error(
                "a Rust shift requires actual fixed integer owners",
                span,
            ));
        }
        let element = left.ty.clone();
        let ty = crate::translation::types::option(element.clone(), "undefined".to_owned());
        let value = Expr::new(
            Node::Math {
                op: rules::read_rust_shift_checked_operation(operation),
                args: vec![left, count],
            },
            ty,
        );
        if !rules::accept_rust_shift_source_operation(operation) {
            return Ok(value);
        }
        Ok(Expr::new(
            Node::Coalesce {
                left: Box::new(value),
                right: Box::new(Expr::new(
                    Node::Abort {
                        message: rules::read_rust_shift_overflow_message(operation),
                    },
                    element.clone(),
                )),
            },
            element,
        ))
    }
}
