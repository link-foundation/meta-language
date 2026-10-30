//! Actual primitive operand owners for source-generated Rust bit rules.
use super::{Checker, Env, Expr, Node, Result, SArrayItem, Span, type_error};
use crate::translation::types::Type;
use crate::translation::{Language, rust_primitive_bitwise_rules as rules};

impl Checker {
    pub(super) fn rust_primitive_bitwise(
        &mut self,
        operation: &str,
        arguments: &[SArrayItem],
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        span: Option<Span>,
    ) -> Result<Expr> {
        let count = serde_json::Number::from(arguments.len())
            .as_f64()
            .expect("actual arity is a Number");
        if self.language != Language::Rust
            || count != rules::read_rust_primitive_bitwise_arity(operation)
            || arguments.iter().any(|argument| argument.spread)
        {
            return Err(type_error(
                "a Rust bit operation requires its actual unspread primitive operands",
                span,
            ));
        }
        let mut left = self.expr(&arguments[0].value, env, path, expected, true)?;
        let mut right = if arguments.len() == 2 {
            Some(self.expr(
                &arguments[1].value,
                env,
                path,
                if left.ty.is_literal() {
                    expected
                } else {
                    Some(&left.ty)
                },
                true,
            )?)
        } else {
            None
        };
        if left.ty.is_literal() {
            let target = right
                .as_ref()
                .filter(|right| !right.ty.is_literal())
                .map(|right| &right.ty)
                .or(expected)
                .cloned()
                .unwrap_or_else(|| self.default_number());
            left = self.expr(&arguments[0].value, env, path, Some(&target), false)?;
        }
        if right.as_ref().is_some_and(|right| right.ty.is_literal()) {
            right = Some(self.expr(&arguments[1].value, env, path, Some(&left.ty), false)?);
        }
        let bits = match left.ty {
            Type::Fixed { bits, .. } => f64::from(bits),
            _ => 0.0,
        };
        if !rules::accept_rust_primitive_bitwise_type(left.ty.kind(), bits)
            || right.as_ref().is_some_and(|right| right.ty != left.ty)
        {
            return Err(type_error(
                "a Rust bit operation requires matching actual Boolean or fixed integer types",
                span,
            ));
        }
        let ty = left.ty.clone();
        let mut args = vec![left];
        args.extend(right);
        Ok(Expr::new(
            Node::Math {
                op: operation.to_owned(),
                args,
            },
            ty,
        ))
    }
}
