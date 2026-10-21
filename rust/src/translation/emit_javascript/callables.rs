//! Existing callable renderer adapters, separated to bound the emitter root.

use super::{Expr, JavaScriptEmitter, Node, Result};

impl JavaScriptEmitter<'_> {
    pub(super) fn callable_expression(&mut self, expression: &Expr) -> Result<String> {
        match &expression.node {
            Node::FunctionRef { func } => Ok(
                crate::translation::callable_source_rules::render_callable_function_reference(
                    "JavaScript",
                    &(if self.program.external(func).is_some() {
                        func.clone()
                    } else {
                        self.state.reference(func, ".")
                    }),
                ),
            ),
            Node::Apply { func, args } => {
                let function = self.expr(func)?;
                let arguments = self.exprs(args)?;
                Ok(
                    crate::translation::callable_source_rules::render_callable_application(
                        "JavaScript",
                        &function,
                        &arguments,
                    ),
                )
            }
            Node::Lambda { params, body } => {
                let body = self.expr(body)?;
                Ok(
                    crate::translation::callable_source_rules::render_callable_lambda(
                        "JavaScript",
                        &(params
                            .iter()
                            .map(|parameter| parameter.name.clone())
                            .collect::<Vec<_>>()),
                        &[],
                        "",
                        &body,
                        &[],
                    ),
                )
            }
            Node::CellAllocate { value } => {
                let value = self.expr(value)?;
                Ok(
                    crate::translation::callable_source_rules::render_callable_cell_operation(
                        "JavaScript",
                        "cellAllocate",
                        "",
                        &value,
                        "",
                        "",
                        "",
                    ),
                )
            }
            Node::CellRead { cell } => {
                let cell = self.expr(cell)?;
                Ok(
                    crate::translation::callable_source_rules::render_callable_cell_operation(
                        "JavaScript",
                        "cellRead",
                        &cell,
                        "",
                        "",
                        "",
                        "",
                    ),
                )
            }
            Node::CellWrite { cell, value, body } => {
                let cell = self.expr(cell)?;
                let value = self.expr(value)?;
                let body = self.expr(body)?;
                Ok(
                    crate::translation::callable_source_rules::render_callable_cell_operation(
                        "JavaScript",
                        "cellWrite",
                        &cell,
                        &value,
                        &body,
                        "",
                        "",
                    ),
                )
            }
            _ => unreachable!("the caller selects only callable and scalar-cell variants"),
        }
    }
}
