//! Actual target operands passed to the complete shared range constructor.
use super::{Expr, Node, Result, RustEmitter, Type, unsupported};
use crate::translation::array_state_method_rules as rules;

impl RustEmitter<'_> {
    pub(super) fn array_state_method(&mut self, expression: &Expr) -> Result<String> {
        let Node::Method { object, op, args } = &expression.node else {
            unreachable!("a selected array range method")
        };
        self.temporaries += 1;
        let receiver = self.expr(object)?;
        let mut arguments = Vec::new();
        let mut profiles = Vec::new();
        let mut types = Vec::new();
        for argument in args {
            let value = argument.as_ref().ok_or_else(|| {
                unsupported(
                    "shared array range argument",
                    "each actual source argument needs its expression",
                    expression.span,
                )
            })?;
            arguments.push(self.expr(value)?);
            types.push(self.ty(&value.ty)?);
            let (element, absence) = if let Type::Option { element, absence } = &value.ty {
                (element.kind(), absence.as_str())
            } else {
                ("", "")
            };
            profiles.push(rules::read_array_state_method_argument_profile(
                value.ty.kind(),
                element,
                absence,
            ));
        }
        let initial = serde_json::Number::from(self.temporaries)
            .as_f64()
            .expect("a bounded temporary is a Number");
        let suffix =
            rules::read_array_state_method_suffix(initial, &receiver, &arguments).to_string();
        self.temporaries = suffix
            .parse()
            .expect("the actual selected suffix is an integer");
        self.state.encode(
            &rules::read_array_state_method_identity(op),
            &rules::read_array_state_method_statement(),
        );
        Ok(rules::render_array_state_method_expression(
            "Rust", op, &receiver, &arguments, &profiles, &types, &suffix,
        ))
    }
}
