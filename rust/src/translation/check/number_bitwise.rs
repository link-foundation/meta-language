//! Host typing adapter for the shared checked Number interpretation.
use super::{Checker, Env, Expr, FLOAT, Node, Result, SArrayItem, Span, type_error, unsupported};

impl Checker {
    pub(super) fn check_number_bitwise_operands(
        &mut self,
        operation: &str,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        let count = match args.len() {
            1 => 1.0,
            2 => 2.0,
            _ => 0.0,
        };
        if !crate::translation::number_bitwise_rules::accept_java_script_number_bitwise_operand_count(operation, count) || args.iter().any(|arg| arg.spread) {
            return Err(type_error("a Number bitwise operation requires its exact unspread operands", span));
        }
        let checked = args.iter().map(|arg| {
            let value = self.expr(&arg.value, env, path, Some(&FLOAT), false)?;
            if !value.ty.is_float() {
                return Err(unsupported("Number bitwise operand", "this interpretation requires Number operands; other primitive conversions and BigInt bitwise operations retain their own obligations", arg.value.span.or(span)));
            }
            Ok(value)
        }).collect::<Result<Vec<_>>>()?;
        Ok(Expr::new(
            Node::Math {
                op: operation.to_owned(),
                args: checked,
            },
            FLOAT,
        ))
    }
}
