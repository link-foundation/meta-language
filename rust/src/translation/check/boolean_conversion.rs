//! Actual argument/IR transport into the existing shared Truthy operation.
use super::{
    BOOL, Checker, Env, Expr, Language, LitValue, Node, Result, SArrayItem, Span, unsupported,
};
impl Checker {
    pub(super) fn javascript_boolean_builtin(
        &mut self,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        if self.language != Language::JavaScript || args.iter().any(|argument| argument.spread) {
            return Err(unsupported(
                "Boolean arguments",
                "the complete actual iterator effects must be represented",
                span,
            ));
        }
        let values = args
            .iter()
            .map(|argument| self.expr(&argument.value, env, path, None, false))
            .collect::<Result<Vec<_>>>()?;
        let names = values
            .iter()
            .map(|_| self.fresh_name("ml_boolean_argument"))
            .collect::<Vec<_>>();
        let mut body = if let Some(first) = values.first() {
            self.boolean_value(Expr::var(names[0].clone(), first.ty.clone()), span)?
        } else {
            Expr::lit(BOOL, LitValue::Bool(false))
        };
        for index in (0..values.len()).rev() {
            body = Expr::new(
                Node::Let {
                    name: names[index].clone(),
                    value: Box::new(values[index].clone()),
                    body: Box::new(body),
                },
                BOOL,
            );
        }
        Ok(body)
    }
}
