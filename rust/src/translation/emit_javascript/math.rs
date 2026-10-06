//! `Math` and `Number` functions stay the JavaScript built-ins.

use super::{Expr, JavaScriptEmitter, Node, Result};

impl JavaScriptEmitter<'_> {
    pub(super) fn math(&mut self, e: &Expr) -> Result<String> {
        let Node::Math { op, args } = &e.node else {
            unreachable!("not a Math expression")
        };
        let args = self.exprs(args)?;
        if op == "maxOf" || op == "minOf" {
            return Ok(format!("Math.{}(...{})", &op[..3], args[0]));
        }
        let namespace = if e.ty == super::Type::Bool {
            "Number"
        } else {
            "Math"
        };
        Ok(format!("{namespace}.{op}({})", args.join(", ")))
    }
}
