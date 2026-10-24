//! Actual numeric/string transport to automatically translated BigInt rules.
use super::big_integer_conversion_rules as rules;

/// The exact integer representation emitted from the shared definition.
pub use rules::ml::Big as JavaScriptBigInteger;

/// A source conversion error retaining every actual UTF-16 message unit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaScriptBigIntegerConversionError {
    /// ECMAScript error constructor identity selected by shared rules.
    pub class_name: String,
    /// Exact source message, including isolated surrogates.
    pub message_units: Vec<u16>,
}

impl std::fmt::Display for JavaScriptBigIntegerConversionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&String::from_utf16_lossy(&self.message_units))
    }
}
impl std::error::Error for JavaScriptBigIntegerConversionError {}

fn conversion_error(form: &str, actual: &[u16]) -> JavaScriptBigIntegerConversionError {
    let pieces = rules::read_java_script_big_integer_conversion_message_pieces(form);
    let mut message_units = pieces[0].encode_utf16().collect::<Vec<_>>();
    message_units.extend_from_slice(actual);
    message_units.extend(pieces[1].encode_utf16());
    JavaScriptBigIntegerConversionError {
        class_name: rules::read_java_script_big_integer_conversion_error_class(form),
        message_units,
    }
}

/// Convert StringIntegerLiteral without losing any UTF-16 input unit.
///
/// # Errors
/// Returns the actual source SyntaxError for invalid integer syntax.
pub fn convert_javascript_string_big_integer_units(
    units: &[u16],
) -> Result<JavaScriptBigInteger, JavaScriptBigIntegerConversionError> {
    let values = units.iter().copied().map(f64::from).collect::<Vec<_>>();
    let mut result = rules::convert_java_script_string_big_integer_units(&values);
    if result.len() != 1 {
        return Err(conversion_error("string", units));
    }
    result
        .pop()
        .ok_or_else(|| conversion_error("string", units))
}

/// Convert Unicode text to its exact integer.
///
/// # Errors
/// Returns the actual source SyntaxError for invalid integer syntax.
pub fn convert_javascript_string_big_integer(
    text: &str,
) -> Result<JavaScriptBigInteger, JavaScriptBigIntegerConversionError> {
    convert_javascript_string_big_integer_units(&text.encode_utf16().collect::<Vec<_>>())
}

/// Convert every integral binary64 value to its exact BigInt value.
///
/// # Errors
/// Returns the actual source RangeError for non-integral or non-finite values.
pub fn convert_javascript_number_big_integer(
    value: f64,
) -> Result<JavaScriptBigInteger, JavaScriptBigIntegerConversionError> {
    let mut result = rules::convert_java_script_number_big_integer(value);
    if result.len() != 1 {
        return Err(conversion_error(
            "number",
            &super::js_number::js_number(value)
                .encode_utf16()
                .collect::<Vec<_>>(),
        ));
    }
    result.pop().ok_or_else(|| {
        conversion_error(
            "number",
            &super::js_number::js_number(value)
                .encode_utf16()
                .collect::<Vec<_>>(),
        )
    })
}

/// Convert a Boolean through the same complete shared definition.
#[must_use]
pub fn convert_javascript_boolean_big_integer(value: bool) -> JavaScriptBigInteger {
    rules::convert_java_script_boolean_big_integer(value)
}
