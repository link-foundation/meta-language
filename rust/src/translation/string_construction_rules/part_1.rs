// Generated from complete js/src/translation/string-construction-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_construction_operation(name: &str) -> String {
    if name == "String" {
        String::from("convertJavaScriptPrimitiveText")
    } else if name == "String.fromCharCode" {
        String::from("constructJavaScriptStringFromCharacterCodes")
    } else if name == "String.fromCodePoint" {
        String::from("constructJavaScriptStringFromCodePoints")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_construction_form(operation: &str) -> String {
    if operation == "convertJavaScriptCharacterCodesToString" {
        String::from("characters")
    } else if operation == "convertJavaScriptCodePointUnitsResult" {
        String::from("points")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_java_script_string_construction(kind: &str, operation: &str) -> bool {
    (kind == "math") && (!(self::read_java_script_string_construction_form(operation)).is_empty())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_string_construction(
    target: &str,
    operation: &str,
    values: &str,
) -> String {
    {
        let form = self::read_java_script_string_construction_form(operation);
        if (form).is_empty() {
            String::new()
        } else if target == "JavaScript" {
            if form == "characters" {
                format!(
                    "ml_utf16_text.characterUnits({}).map(ml_unit=>String.fromCharCode(ml_unit)).join(\"\")",
                    values.to_owned()
                )
            } else {
                format!("ml_utf16_text.codePointUnits({})", values.to_owned())
            }
        } else if target == "Rust" {
            if form == "characters" {
                format!(
                    "crate::ml_string::Text::from_units(crate::ml_utf16_text::character_units_utf16_text(&{}).into_iter().map(|ml_unit| ml_unit as u16).collect())",
                    values.to_owned()
                )
            } else {
                format!(
                    "crate::ml_utf16_text::code_point_units_utf16_text(&{})",
                    values.to_owned()
                )
            }
        } else if (target == "Lean") || (target == "Rocq") {
            format!(
                "(MLUtf16Text.{} {})",
                if form == "characters" {
                    String::from("characterUnits")
                } else {
                    String::from("codePointUnits")
                },
                values.to_owned()
            )
        } else {
            String::new()
        }
    }
}
