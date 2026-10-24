//! Actual source arguments constrained by the shared intrinsic signatures.
use super::{BOOL, Env, Equality, FLOAT, INT, Inference, Result, STRING, Span, Term};
use crate::translation::big_integer_conversion_rules as rules;
use crate::translation::surface::SArrayItem;

impl Inference {
    pub(super) fn loose_comparison_term(
        &mut self,
        args: &[SArrayItem],
        env: &Env,
        span: Option<Span>,
    ) -> Result<Term> {
        let values = args
            .iter()
            .map(|argument| self.expr(&argument.value, env))
            .collect::<Result<Vec<_>>>()?;
        if values.len() == 2 {
            self.equalities.push(Equality {
                left: values[0].clone(),
                right: values[1].clone(),
                place: span,
            });
        }
        Ok(Term::Known(BOOL))
    }
    pub(super) fn numeric_intrinsic_term(
        &mut self,
        operation: &str,
        args: &[SArrayItem],
        env: &Env,
        span: Option<Span>,
    ) -> Result<Term> {
        let kinds = rules::read_java_script_big_integer_intrinsic_operand_kinds(operation);
        for (index, argument) in args.iter().enumerate() {
            let value = self.expr(&argument.value, env)?;
            if let Some(kind) = kinds.get(index) {
                let ty = match kind.as_str() {
                    "float" => FLOAT,
                    "int" => INT,
                    "bool" => BOOL,
                    "string" => STRING,
                    _ => unreachable!("registered intrinsic input"),
                };
                self.unify(&value, &Term::Known(ty), span)?;
            }
        }
        if operation == "convertJavaScriptBigInteger" {
            return Ok(Term::Known(INT));
        }
        Ok(
            match rules::read_java_script_big_integer_intrinsic_result_kind(operation).as_str() {
                "array-integer" => Term::Array(Box::new(Term::Known(INT))),
                "float" => Term::Known(FLOAT),
                "int" => Term::Known(INT),
                _ => unreachable!("registered intrinsic result"),
            },
        )
    }
}
