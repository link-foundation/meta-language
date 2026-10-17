//! Mechanically extracted original expression arms; no semantic replacement.
use super::*;

impl Inference {
    pub(super) fn infer_conditional_expression(&mut self, expr: &SExpr, env: &Env) -> Result<Term> {
        match &expr.node {
            SNode::If {
                cond,
                then,
                otherwise,
            } => {
                let condition = self.expr(cond, env)?;
                self.product_condition(&condition, cond.span.or(expr.span));
                let then = self.expr(then, env)?;
                let otherwise = self.expr(otherwise, env)?;
                Ok(self.join(then, otherwise, expr.span))
            }
            _ => {
                unreachable!("the original dispatcher selects only this helper's retained variants")
            }
        }
    }

    pub(super) fn infer_binding_expression(&mut self, expr: &SExpr, env: &Env) -> Result<Term> {
        match &expr.node {
            SNode::Let {
                name, value, body, ..
            } => {
                let value = self.expr(value, env)?;
                let mut scope = env.clone();
                scope.insert(name.clone(), value);
                self.expr(body, &scope)
            }
            _ => {
                unreachable!("the original dispatcher selects only this helper's retained variants")
            }
        }
    }

    pub(super) fn infer_match_expression(&mut self, expr: &SExpr, env: &Env) -> Result<Term> {
        match &expr.node {
            SNode::Match { scrutinees, rows } => {
                let scrutinees = scrutinees
                    .iter()
                    .map(|scrutinee| self.expr(scrutinee, env))
                    .collect::<Result<Vec<_>>>()?;
                let mut result: Option<Term> = None;
                for row in rows {
                    let place = row.span.or(expr.span);
                    let mut scope = env.clone();
                    for (pattern, scrutinee) in row.patterns.iter().zip(&scrutinees) {
                        self.pattern(pattern, scrutinee, &mut scope, place)?;
                    }
                    let body = self.expr(&row.body, &scope)?;
                    match &result {
                        Some(previous) => result = Some(self.join(body, previous.clone(), place)),
                        None => result = Some(body),
                    }
                }
                Ok(result.unwrap_or_else(|| self.fresh(false)))
            }
            _ => {
                unreachable!("the original dispatcher selects only this helper's retained variants")
            }
        }
    }
}
