//! Transport actual string units to automatically translated definitions.

/// Convert every actual UTF-16 unit, including isolated surrogate code units.
#[must_use]
pub fn convert_javascript_string_number_units(units: &[u16]) -> f64 {
    let values = units.iter().copied().map(f64::from).collect::<Vec<_>>();
    super::string_number_rules::convert_java_script_string_number_units(&values)
}

/// Convert Unicode text using ECMAScript StringNumericLiteral semantics.
#[must_use]
pub fn convert_javascript_string_number(text: &str) -> f64 {
    let values = text.encode_utf16().map(f64::from).collect::<Vec<_>>();
    super::string_number_rules::convert_java_script_string_number_units(&values)
}
