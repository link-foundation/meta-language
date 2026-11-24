//! Host typing adapter for the shared checked Number interpretation.
use super::{
    Checker, Env, Expr, FLOAT, INT, Node, Result, SArrayItem, Span, Type, type_error, unsupported,
};
use crate::translation::big_integer_bitwise_rules as rules;

impl Checker {
    pub(super) fn check_number_bitwise_operands(
        &mut self,
        operation: &str,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
    ) -> Result<Expr> {
        let count = match args.len() {
            1 => 1.0,
            2 => 2.0,
            _ => 0.0,
        };
        let number_arity =
            crate::translation::number_bitwise_rules::read_java_script_number_bitwise_arity(
                operation,
            );
        let arity = if number_arity > 0.0 {
            number_arity
        } else {
            rules::read_big_integer_bitwise_arity(operation)
        };
        if arity != count || args.iter().any(|arg| arg.spread) {
            return Err(type_error(
                "a Number bitwise operation requires its exact unspread operands",
                span,
            ));
        }
        let checked = args
            .iter()
            .map(|arg| {
                let value = self.expr(&arg.value, env, path, None, false)?;
                self.number_value(value, arg.value.span.or(span))
            })
            .collect::<Result<Vec<_>>>()?;
        let kinds = checked
            .iter()
            .map(|argument| argument.ty.kind().to_owned())
            .collect::<Vec<_>>();
        if let Some(failure) = self.materialize_numeric_failure(operation, &checked, expected) {
            return Ok(failure);
        }
        let domain = rules::read_primitive_bitwise_domain(&kinds);
        let selected = if domain == "BigInt" {
            rules::read_big_integer_bitwise_operation(operation)
        } else {
            String::new()
        };
        let (operation, checked, ty) = if !selected.is_empty() {
            let checked = checked
                .into_iter()
                .map(|argument| self.coerce(argument, &INT, span))
                .collect::<Result<Vec<_>>>()?;
            (selected, checked, INT)
        } else if domain == "Number" && number_arity > 0.0 {
            (operation.to_owned(), checked, FLOAT)
        } else {
            return Err(unsupported(
                "primitive bitwise operand",
                "the actual operands need one complete Number or BigInt domain; implicit coercions and unsigned BigInt shifts retain their own obligations",
                span,
            ));
        };
        Ok(Expr::new(
            Node::Math {
                op: operation,
                args: checked,
            },
            ty,
        ))
    }
}
