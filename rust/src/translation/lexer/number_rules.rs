// Generated from the linked Rust numeric policy through JavaScript self-translation. Do not edit.
/// Admit the shared Rust decimal integer suffix separator boundary.
#[must_use]
pub fn accept_rust_number_digit_boundary(
    floating: bool,
    trailing_separator: bool,
    suffix: &str,
    continued_suffix: bool,
    following_code_unit: f64,
) -> bool {
    (floating || !trailing_separator)
        || ((!continued_suffix && (following_code_unit < (128f64)))
            && ((((((((((((suffix == "u8") || (suffix == "i8")) || (suffix == "u16"))
                || (suffix == "i16"))
                || (suffix == "u32"))
                || (suffix == "i32"))
                || (suffix == "u64"))
                || (suffix == "i64"))
                || (suffix == "u128"))
                || (suffix == "i128"))
                || (suffix == "usize"))
                || (suffix == "isize")))
}
