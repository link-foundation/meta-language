//! Existing typed callable rendering, separated to bound the emitter root.

use super::{Expr, LeanEmitter, Node, Result};

impl LeanEmitter<'_> {
    pub(super) fn callable_expression(
        &mut self,
        expression: &Expr,
        depth: usize,
    ) -> Result<String> {
        Ok(match &expression.node {
            Node::FunctionRef { func } => {
                crate::translation::callable_source_rules::render_callable_function_reference(
                    "Lean",
                    &(self.state.reference(func, ".")),
                )
            }
            Node::Apply { func, args } => {
                let function = self.expr(func, depth)?;
                let arguments = args
                    .iter()
                    .map(|argument| self.expr(argument, depth))
                    .collect::<Result<Vec<_>>>()?;
                crate::translation::callable_source_rules::render_callable_application(
                    "Lean", &function, &arguments,
                )
            }
            Node::Lambda { params, body } => {
                let types = params
                    .iter()
                    .map(|parameter| self.ty(&parameter.ty))
                    .collect::<Result<Vec<_>>>()?;
                let body = self.expr(body, depth)?;
                crate::translation::callable_source_rules::render_callable_lambda(
                    "Lean",
                    &(params
                        .iter()
                        .map(|parameter| parameter.name.clone())
                        .collect::<Vec<_>>()),
                    &types,
                    "",
                    &body,
                    &[],
                )
            }
            _ => unreachable!("the caller selects an actual callable expression"),
        })
    }
}
