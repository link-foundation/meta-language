//! Existing numeric constraints, separated from the shared inference state.
use super::{
    Arithmetic, BOOL, BinaryOp, Env, Equality, Inference, Plus, Result, SExpr, STRING, Span, Term,
};
use crate::translation::javascript_comparison_rules::read_java_script_comparison_inference_step;

impl Inference {
    pub(super) fn binary(
        &mut self,
        op: BinaryOp,
        left: &SExpr,
        right: &SExpr,
        place: Option<Span>,
        env: &Env,
    ) -> Result<Term> {
        let left_term = self.expr(left, env)?;
        let right_term = self.expr(right, env)?;
        match op {
            BinaryOp::And | BinaryOp::Or => {
                self.product_condition(&left_term, left.span.or(place));
                Ok(self.join(left_term, right_term, place))
            }
            BinaryOp::Eq | BinaryOp::Ne => {
                self.equalities.push(Equality {
                    left: left_term,
                    right: right_term,
                    place,
                });
                Ok(Term::Known(BOOL))
            }
            BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
                self.equalities.push(Equality {
                    left: left_term,
                    right: right_term,
                    place,
                });
                Ok(Term::Known(BOOL))
            }
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Rem => {
                let result = self.fresh(false);
                self.arithmetic.push(Arithmetic {
                    operation: op.name().to_owned(),
                    left: left_term,
                    right: Some(right_term),
                    result: result.clone(),
                    place,
                });
                Ok(result)
            }
            BinaryOp::Concat => Ok(Term::Known(STRING)),
            BinaryOp::Plus => {
                let result = self.fresh(false);
                self.plus.push(Plus {
                    left: left_term,
                    right: right_term,
                    result: result.clone(),
                    place,
                });
                Ok(result)
            }
        }
    }

    pub(super) fn solve_arithmetic(&mut self, force: bool) -> Result<bool> {
        if force {
            let first = self.arithmetic.remove(0);
            self.arithmetic_step(&first, true)?;
            return Ok(true);
        }
        let mut progress = false;
        let mut waiting = Vec::new();
        for constraint in std::mem::take(&mut self.arithmetic) {
            if self.arithmetic_step(&constraint, false)? {
                progress = true;
            } else {
                waiting.push(constraint);
            }
        }
        self.arithmetic = waiting;
        Ok(progress)
    }

    pub(super) fn solve_equalities(&mut self, force: bool) -> Result<bool> {
        if force {
            let first = self.equalities.remove(0);
            return self.equality_step(&first, true);
        }
        let mut progress = false;
        let mut waiting = Vec::new();
        for constraint in std::mem::take(&mut self.equalities) {
            if self.equality_step(&constraint, false)? {
                progress = true;
            } else {
                waiting.push(constraint);
            }
        }
        self.equalities = waiting;
        Ok(progress)
    }

    fn equality_step(&mut self, constraint: &Equality, force: bool) -> Result<bool> {
        let left = self.resolve(&constraint.left);
        let right = self.resolve(&constraint.right);
        let step = read_java_script_comparison_inference_step(
            super::optional_values::kind(&left),
            super::optional_values::kind(&right),
            force,
        );
        if step == "defer" {
            return Ok(false);
        }
        if step == "unify" {
            self.unify(&constraint.left, &constraint.right, constraint.place)?;
        }
        Ok(true)
    }

    fn arithmetic_step(&mut self, constraint: &Arithmetic, force: bool) -> Result<bool> {
        let left = self.numeric_value(&constraint.left);
        let right = constraint
            .right
            .as_ref()
            .map_or_else(|| left.clone(), |right| self.numeric_value(right));
        if !force
            && (matches!(self.resolve(&left), Term::Var(_))
                || matches!(self.resolve(&right), Term::Var(_)))
        {
            return Ok(false);
        }
        if !crate::translation::source_exception_rules::read_java_script_numeric_operation_failure(
            &constraint.operation,
            super::optional_values::kind(&self.resolve(&left)),
            super::optional_values::kind(&self.resolve(&right)),
        )
        .is_empty()
        {
            return Ok(true);
        }
        self.unify(&left, &right, constraint.place)?;
        self.unify(&constraint.result, &left, constraint.place)?;
        Ok(true)
    }
}
