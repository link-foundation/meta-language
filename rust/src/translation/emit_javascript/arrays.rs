//! JavaScript arrays stay arrays: the portable core never mutates one.

use super::{Expr, Helper, JavaScriptEmitter, Node, Result};

impl JavaScriptEmitter<'_> {
    /// An array literal, an append, an element read or a length.
    pub(super) fn array_expr(&mut self, e: &Expr) -> Result<String> {
        match &e.node {
            Node::Array { items } => {
                self.arrays();
                Ok(format!("[{}]", self.exprs(items)?.join(", ")))
            }
            Node::Append { left, right } => Ok(format!(
                "[...{}, ...{}]",
                self.expr(left)?,
                self.expr(right)?
            )),
            Node::Index { array, index } => {
                self.arrays();
                self.helpers.insert(Helper::At);
                self.state.array_read();
                Ok(format!(
                    "ml_at({}, {})",
                    self.expr(array)?,
                    self.expr(index)?
                ))
            }
            Node::Length { array } => {
                let array = self.expr(array)?;
                Ok(if e.ty.is_float() {
                    format!("{array}.length")
                } else {
                    format!("BigInt({array}.length)")
                })
            }
            _ => unreachable!("not an array expression"),
        }
    }

    fn arrays(&mut self) {
        self.state.encode("arrays", "a JavaScript array stays an array, which the portable core never mutates; a read outside it throws RangeError where JavaScript reads undefined");
    }
}
