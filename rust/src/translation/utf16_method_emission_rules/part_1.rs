// Generated from complete js/src/translation/utf16-method-emission-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_utf16_text_method(kind: &str, receiver: &str, operation: &str) -> bool {
    ((kind == "method") && (receiver == "string"))
        && (((operation == "split") || (operation == "replace")) || (operation == "replaceAll"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_utf16_text_method(
    target: &str,
    operation: &str,
    object: &str,
    arguments_text: &str,
) -> String {
    if (operation == "replace") || (operation == "replaceAll") {
        self::render_utf16_text_replacement(
            target,
            object,
            arguments_text,
            (operation == "replaceAll"),
        )
    } else if operation == "split" {
        if target == "JavaScript" {
            {
                let text = String::from(
                    "Array.from({length:ml_text.length},(_,ml_index)=>ml_text.charCodeAt(ml_index))",
                );
                {
                    let separator = String::from(
                        "ml_separator===undefined?[]:Array.from({length:ml_separator.length},(_,ml_index)=>ml_separator.charCodeAt(ml_index))",
                    );
                    format!(
                        "((ml_text,ml_separator,ml_limit)=>ml_utf16_text.split({text}, {separator}, ml_limit, ml_separator===undefined).map(ml_units=>ml_units.map(ml_unit=>String.fromCharCode(ml_unit)).join(\"\")))({}, {})",
                        object.to_owned(),
                        arguments_text.to_owned()
                    )
                }
            }
        } else if target == "Rust" {
            {
                let text_2 = String::from(
                    "ml_text.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()",
                );
                {
                    let separator_2 = String::from(
                        "ml_separator.map(|ml_value|ml_value.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()).unwrap_or_default()",
                    );
                    format!(
                        "(|ml_text:&crate::ml_string::Text,ml_separator:Option<&crate::ml_string::Text>,ml_limit:f64|{{crate::ml_utf16_text::split_utf16_text(&({text_2}), &({separator_2}), ml_limit, ml_separator.is_none()).into_iter().map(|ml_units|crate::ml_string::Text::from_units(ml_units.into_iter().map(|ml_unit|ml_unit as u16).collect())).collect::<Vec<_>>()}})(&({}), {})",
                        object.to_owned(),
                        arguments_text.to_owned()
                    )
                }
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_utf16_text_replacement(
    target: &str,
    object: &str,
    arguments_text: &str,
    all: bool,
) -> String {
    {
        let flag = if all {
            String::from("true")
        } else {
            String::from("false")
        };
        if target == "JavaScript" {
            {
                let text = String::from(
                    "Array.from({length:ml_text.length},(_,ml_index)=>ml_text.charCodeAt(ml_index))",
                );
                {
                    let search = String::from(
                        "Array.from({length:ml_search.length},(_,ml_index)=>ml_search.charCodeAt(ml_index))",
                    );
                    {
                        let replacement = String::from(
                            "Array.from({length:ml_replacement.length},(_,ml_index)=>ml_replacement.charCodeAt(ml_index))",
                        );
                        format!(
                            "((ml_text,ml_search,ml_replacement)=>ml_utf16_text.replace({text}, {search}, {replacement}, {flag}).map(ml_unit=>String.fromCharCode(ml_unit)).join(\"\"))({}, {})",
                            object.to_owned(),
                            arguments_text.to_owned()
                        )
                    }
                }
            }
        } else if target == "Rust" {
            {
                let text_2 = String::from(
                    "ml_text.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()",
                );
                {
                    let search_2 = String::from(
                        "ml_search.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()",
                    );
                    {
                        let replacement_2 = String::from(
                            "ml_replacement.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()",
                        );
                        format!(
                            "(|ml_text:&crate::ml_string::Text,ml_search:&crate::ml_string::Text,ml_replacement:&crate::ml_string::Text|crate::ml_string::Text::from_units(crate::ml_utf16_text::replace_utf16_text(&({text_2}), &({search_2}), &({replacement_2}), {flag}).into_iter().map(|ml_unit|ml_unit as u16).collect()))(&({}), {})",
                            object.to_owned(),
                            arguments_text.to_owned()
                        )
                    }
                }
            }
        } else {
            String::new()
        }
    }
}
