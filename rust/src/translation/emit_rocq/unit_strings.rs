//! Actual Rocq emitter bindings for shared formal unit adapters.
use super::{Expr, Helper, Result, RocqEmitter};
impl crate::translation::formal_unit_strings::Emitter for RocqEmitter<'_> {
    fn unit_unicode_case(&mut self) {
        self.floats();
        self.helpers.insert(Helper::UnicodeCase);
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
        "Rocq"
    }
    fn unit_expr(&mut self, value: &Expr, _depth: usize) -> Result<String> {
        self.expr(value)
    }
    fn unit_text(&mut self, value: &Expr, _depth: usize, console: bool) -> Result<String> {
        self.text_of(value, console)
    }
    fn unit_helper(&mut self, name: &'static str) {
        self.floats();
        self.helpers.insert(match name {
            "stringNumber" => Helper::StringNumber,
            "bigIntegerConversion" => Helper::BigIntegerConversion,
            "utf16Text" => Helper::Utf16Text,
            "floatIndex" => Helper::FloatIndex,
            _ => unreachable!("declared formal UTF16 helper"),
        });
    }
}
