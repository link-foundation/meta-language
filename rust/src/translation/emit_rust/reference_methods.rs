//! Construct target source through the same automatically translated rules.
use super::{Expr, Node, Result, RustEmitter, Type, unsupported};
use crate::translation::reference_method_rules as rules;

impl RustEmitter<'_> {
    pub(super) fn reference_method(&mut self, expression: &Expr) -> Result<String> {
        let Node::Method { object, op, args } = &expression.node else {
            unreachable!("selected reference method");
        };
        self.temporaries += 1;
        let suffix = self.temporaries.to_string();
        let receiver = self.expr(object)?;
        let mut arguments = Vec::new();
        for argument in args {
            let value = argument.as_ref().ok_or_else(|| {
                unsupported(
                    "shared collection method argument",
                    "every source argument needs its actual expression",
                    expression.span,
                )
            })?;
            arguments.push(self.expr(value)?);
        }
        let key = match &object.ty {
            Type::Map { key, .. }
            | Type::Array { element: key, .. }
            | Type::Set { element: key, .. } => key.as_ref(),
            _ => unreachable!("the checked shared receiver has a selected key domain"),
        };
        self.state.encode(
            &rules::read_reference_method_identity(object.ty.kind(), op),
            &rules::read_reference_method_statement(),
        );
        Ok(rules::render_reference_method_expression(
            "Rust",
            object.ty.kind(),
            op,
            &receiver,
            &arguments,
            key.is_float(),
            &suffix,
        ))
    }
}
