// Generated through the checked JavaScript source whitespace decisions. Do not edit.
/// Read the shared Rust source whitespace decision.
#[must_use]
pub const fn accept_rust_source_whitespace(unit: f64) -> bool {
    ((((((((((unit.to_bits() == 9f64.to_bits()) || (unit.to_bits() == 10f64.to_bits()))
        || (unit.to_bits() == 11f64.to_bits()))
        || (unit.to_bits() == 12f64.to_bits()))
        || (unit.to_bits() == 13f64.to_bits()))
        || (unit.to_bits() == 32f64.to_bits()))
        || (unit.to_bits() == 133f64.to_bits()))
        || (unit.to_bits() == 8206f64.to_bits()))
        || (unit.to_bits() == 8207f64.to_bits()))
        || (unit.to_bits() == 8232f64.to_bits()))
        || (unit.to_bits() == 8233f64.to_bits())
}

/// Read the shared Rust source whitespace decision.
#[must_use]
pub const fn accept_rust_source_byte_order_mark(unit: f64, first_position: bool) -> bool {
    first_position && (unit.to_bits() == 65_279f64.to_bits())
}

/// Read the shared Rust source whitespace decision.
#[must_use]
pub fn accept_rust_source_number_digit_boundary(
    floating: bool,
    trailing_separator: bool,
    suffix: &str,
    continued_suffix: bool,
    following_code_unit: f64,
) -> bool {
    crate::translation::lexer::number_rules::accept_rust_number_digit_boundary(
        floating,
        trailing_separator,
        suffix,
        continued_suffix,
        following_code_unit,
    ) || (crate::translation::lexer::source_whitespace::accept_rust_source_whitespace(
        following_code_unit,
    ) && crate::translation::lexer::number_rules::accept_rust_number_digit_boundary(
        floating,
        trailing_separator,
        suffix,
        continued_suffix,
        32f64,
    ))
}
