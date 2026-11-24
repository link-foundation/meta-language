// Generated from complete js/src/translation/unicode-case-emission-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_unicode_case_mapping_operation(operation: &str) -> bool {
    (operation == "toLowerCase") || (operation == "toUpperCase")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_unicode_case_encoding_statement() -> String {
    String::from(
        "Unicode 16.0.0 full locale-insensitive case mappings, contextual final sigma and preserved isolated UTF16 units; alternate Unicode/locale profiles require their own declared tables",
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_unicode_case_column_mapping(target: &str, operation: &str, value: &str) -> String {
    if (self::accept_unicode_case_mapping_operation(operation)) {
        {
            let upper = if operation == "toUpperCase" {
                String::from("true")
            } else {
                String::from("false")
            };
            if target == "JavaScript" {
                format!("ml_unicode_case.mapCase({}, {upper})", value.to_owned())
            } else if target == "Rust" {
                format!(
                    "crate::ml_unicode_case::map_unicode_case_utf16_text(&({}), {upper})",
                    value.to_owned()
                )
            } else if (target == "Lean") || (target == "Rocq") {
                format!("(MLUnicodeCase.mapCase {} {upper})", value.to_owned())
            } else {
                String::new()
            }
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_unicode_case_string_map(
    target: &str,
    operation: &str,
    units: bool,
    value: &str,
) -> String {
    if (self::accept_unicode_case_mapping_operation(operation)) {
        if target == "JavaScript" {
            {
                let input = String::from(
                    "Array.from({length:ml_text.length}, (_,ml_index)=>ml_text.charCodeAt(ml_index))",
                );
                format!(
                    "((ml_text)=>{}.map(ml_unit=>String.fromCharCode(ml_unit)).join(\"\"))({})",
                    self::render_unicode_case_column_mapping(target, operation, &(input)),
                    value.to_owned()
                )
            }
        } else if target == "Rust" {
            {
                let input_2 = if units {
                    format!(
                        "({}).0.iter().map(|ml_unit| f64::from(*ml_unit)).collect::<Vec<f64>>()",
                        value.to_owned()
                    )
                } else {
                    format!(
                        "({}).encode_utf16().map(f64::from).collect::<Vec<f64>>()",
                        value.to_owned()
                    )
                };
                {
                    let result = format!(
                        "{}.into_iter().map(|ml_unit| ml_unit as u16).collect::<Vec<u16>>()",
                        self::render_unicode_case_column_mapping(target, operation, &(input_2))
                    );
                    if units {
                        format!("crate::ml_string::Text::from_units({result})")
                    } else {
                        format!(
                            "String::from_utf16(&({result})).expect(\"Unicode case conversion preserves scalar string input\")"
                        )
                    }
                }
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}
