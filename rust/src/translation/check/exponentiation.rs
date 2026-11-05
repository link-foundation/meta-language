//! Actual source ToNumeric and typed result transport for exponentiation.
use super::{
    Checker, Env, Expr, FLOAT, INT, Language, Node, Result, SArrayItem, Span, Type, type_error,
    unsupported,
};

impl Checker {
    pub(super) fn javascript_exponentiation(
        &mut self,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
    ) -> Result<Expr> {
        if self.language != Language::JavaScript
            || args.len() != 2
            || args.iter().any(|argument| argument.spread)
        {
            return Err(type_error(
                "JavaScript exponentiation needs two actual source operands",
                span,
            ));
        }
        let values = args
            .iter()
            .map(|argument| self.expr(&argument.value, env, path, None, false))
            .collect::<Result<Vec<_>>>()?;
        let args = values
            .into_iter()
            .map(|value| self.number_value(value, span))
            .collect::<Result<Vec<_>>>()?;
        if let Some(failure) = self.materialize_numeric_failure("exponentiation", &args, expected) {
            return Ok(failure);
        }
        if args.iter().all(|argument| argument.ty == FLOAT) {
            return Ok(Expr::new(
                Node::Math {
                    op: "powerJavaScriptNumberValue".to_owned(),
                    args,
                },
                FLOAT,
            ));
        }
        if !args
            .iter()
            .all(|argument| matches!(argument.ty, Type::Int | Type::Nat))
        {
            return Err(unsupported(
                "exponentiation primitive conversion",
                "the actual primitive operations must be represented",
                span,
            ));
        }
        let args = args
            .into_iter()
            .map(|argument| self.coerce(argument, &INT, span))
            .collect::<Result<Vec<_>>>()?;
        Ok(self.big_integer_arithmetic_result("powerJavaScriptBigIntegerResult",args,"1","0",crate::translation::big_integer_conversion_rules::read_java_script_big_integer_exponent_message()))
    }
}
