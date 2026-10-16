//! Existing numeric constraints, separated from the shared inference state.
use super::{Arithmetic, BOOL, BinaryOp, Env, Inference, Plus, Result, SExpr, STRING, Span, Term};

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
            BinaryOp::Eq
            | BinaryOp::Ne
            | BinaryOp::Lt
            | BinaryOp::Le
            | BinaryOp::Gt
            | BinaryOp::Ge => {
                self.unify(&left_term, &right_term, place)?;
                Ok(Term::Known(BOOL))
            }
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Rem => {
                let result = self.fresh(false);
                self.arithmetic.push(Arithmetic {
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
        self.unify(&left, &right, constraint.place)?;
        self.unify(&constraint.result, &left, constraint.place)?;
        Ok(true)
    }
}
