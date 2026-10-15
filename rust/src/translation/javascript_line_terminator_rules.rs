// Generated through JavaScript self-translation from the linked line-terminator policy. Do not edit.
/// Recognize the four ECMAScript line-terminator code units.
#[must_use]
pub const fn accept_java_script_line_terminator(unit: f64) -> bool {
    (((unit.to_bits() == 10f64.to_bits()) || (unit.to_bits() == 13f64.to_bits()))
        || (unit.to_bits() == 8232f64.to_bits()))
        || (unit.to_bits() == 8233f64.to_bits())
}
