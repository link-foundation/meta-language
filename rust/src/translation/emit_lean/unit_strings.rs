//! Actual Lean emitter bindings for shared formal unit adapters.
use super::{Expr, LeanEmitter, Result, Type, unsupported};
impl crate::translation::formal_unit_strings::Emitter for LeanEmitter<'_> {
    fn unit_unicode_case(&mut self) {
        self.helpers.insert("unicodeCase");
        self.state.encode(
            "unicode-default-case-16",
            &crate::translation::unicode_case_emission_rules::read_unicode_case_encoding_statement(
            ),
        );
    }
    fn unit_enabled(&self) -> bool {
        self.utf16_strings
    }
    fn unit_target(&self) -> &'static str {
        "Lean"
    }
    fn unit_expr(&mut self, value: &Expr, depth: usize) -> Result<String> {
        self.expr(value, depth)
    }
    fn unit_text(&mut self, value: &Expr, depth: usize, console: bool) -> Result<String> {
        self.text_of(value, depth, console)
    }
    fn unit_helper(&mut self, name: &'static str) {
        self.helpers.insert(name);
    }
}

impl LeanEmitter<'_> {
    pub(super) fn text_of(&mut self, arg: &Expr, depth: usize, console: bool) -> Result<String> {
        if !self.utf16_strings {
            return self.scalar_text_of(arg, depth, console);
        }
        if arg.ty == Type::String {
            return self.expr(arg, depth);
        }
        if arg.ty == Type::Unit {
            return Ok(crate::translation::formal_unit_strings::literal(
                "Lean",
                "undefined",
            ));
        }
        let text = self.scalar_text_of(arg, depth, console)?;
        Ok(
            crate::translation::formal_string_unit_rules::render_formal_string_units_from_scalar(
                "Lean", &text,
            ),
        )
    }

    fn scalar_text_of(&mut self, arg: &Expr, depth: usize, console: bool) -> Result<String> {
        if matches!(arg.ty, Type::Data { .. } | Type::Unit) {
            return Err(unsupported(
                "output of structured values",
                &format!("a {} value has no portable textual form", arg.ty.kind()),
                arg.span,
            ));
        }
        if arg.ty.is_float() {
            self.helpers.insert("jsNumber");
            if console {
                self.helpers.insert("jsConsole");
            }
            let function = if console {
                "ml_js_console"
            } else {
                "ml_js_number"
            };
            return Ok(format!("({function} {})", self.expr(arg, depth)?));
        }
        Ok(format!("(toString {})", self.expr(arg, depth)?))
    }
}
