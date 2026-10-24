//! Actual typed representation adapters for the shared numeric conversion code.
use super::{Expr, Node, Prelude, Result, RustEmitter, Type, unsupported};
use crate::translation::string_number_rules as rules;

impl RustEmitter<'_> {
    pub(super) fn javascript_number_conversion(&mut self, expression: &Expr) -> Result<String> {
        let Node::Math { op, args } = &expression.node else {
            unreachable!("checked conversion operation")
        };
        let argument = &args[0];
        let value = self.expr(argument)?;
        let text = self.render_number_conversion(&argument.ty, op, &value, 0)?;
        self.state.encode(
            "javascript-number-conversion",
            &rules::read_java_script_number_conversion_statement(),
        );
        Ok(text)
    }

    fn render_number_conversion(
        &mut self,
        ty: &Type,
        operation: &str,
        value: &str,
        depth: usize,
    ) -> Result<String> {
        let form = rules::read_java_script_number_conversion_form(ty.kind());
        if form == "option" {
            let Type::Option { element, absence } = ty else {
                unreachable!("actual optional type")
            };
            if absence == "both" {
                self.preludes.insert(Prelude::Optional);
            }
            let name = format!("ml_numeric_present{depth}");
            let present = self.render_number_conversion(element, operation, &name, depth + 1)?;
            return Ok(
                crate::translation::optional_value_rules::render_optional_match(
                    "Rust", absence, value, &name, &present, "0.0f64", "f64::NAN",
                ),
            );
        }
        if form == "string"
            || ((form == "int" || form == "nat") && operation == "convertJavaScriptNumber")
        {
            self.preludes.insert(Prelude::Big);
            self.preludes.insert(Prelude::StringNumber);
        }
        let text = rules::render_java_script_number_scalar_conversion(
            "Rust",
            operation,
            &form,
            value,
            self.utf16_strings,
        );
        if text.is_empty() {
            return Err(unsupported(
                "numeric conversion operand",
                "a checked scalar conversion needs its complete shared operation",
                None,
            ));
        }
        Ok(text)
    }
}
