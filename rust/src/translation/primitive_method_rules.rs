// Generated from js/src/translation/primitive-method-rules.js. Do not edit.
// JavaScript Number equality and separate arithmetic rounding are preserved.
// Array index casts retain the existing checked runtime guards.
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons,
    clippy::float_cmp,
    clippy::suboptimal_flops,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

#[must_use]
pub fn render_primitive_string_representation(
    units: bool,
    form: &str,
    value: &str,
    other: &str,
    operation: &str,
) -> String {
    if form == "type" {
        if units {
            String::from("crate::ml_string::Text")
        } else {
            String::from("String")
        }
    } else if form == "literal" {
        if units {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("crate::ml_string::Text::from_units(vec![")),
                    value.to_owned()
                )),
                (String::from("])"))
            )
        } else {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("String::from(")), value.to_owned())),
                (String::from(")"))
            )
        }
    } else if form == "constant" {
        if units {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("crate::ml_string::Text::from_text(")),
                    value.to_owned()
                )),
                (String::from(")"))
            )
        } else {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("String::from(")), value.to_owned())),
                (String::from(")"))
            )
        }
    } else if form == "examples" {
        if units {
            String::from(
                "vec![crate::ml_string::Text::from_text(\"\"), crate::ml_string::Text::from_text(\"a\"), crate::ml_string::Text::from_text(\"ab\")]",
            )
        } else {
            String::from("vec![String::new(), String::from(\"a\"), String::from(\"ab\")]")
        }
    } else if form == "text" {
        if units {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("crate::ml_string::Text::from_text(&")),
                    value.to_owned()
                )),
                (String::from(")"))
            )
        } else {
            value.to_owned()
        }
    } else if form == "concat" {
        if units {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", value.to_owned(), (String::from(".concat(")))),
                    other.to_owned()
                )),
                (String::from(")"))
            )
        } else {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from("format!(\"{}{}\", ")),
                            value.to_owned()
                        )),
                        (String::from(", "))
                    )),
                    other.to_owned()
                )),
                (String::from(")"))
            )
        }
    } else if units {
        if form == "length" {
            format!("{}{}", value.to_owned(), (String::from(".length()")))
        } else if form == "test" {
            {
                let method = if operation == "startsWith" {
                    String::from("starts_with")
                } else if operation == "endsWith" {
                    String::from("ends_with")
                } else {
                    String::from("contains")
                };
                format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (format!("{}{}", value.to_owned(), (String::from(".")))),
                                method
                            )),
                            (String::from("("))
                        )),
                        other.to_owned()
                    )),
                    (String::from(")"))
                )
            }
        } else if form == "map" {
            if (operation == "toLowerCase") || (operation == "toUpperCase") {
                format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", value.to_owned(), (String::from(".map_case(")))),
                        (if operation == "toUpperCase" {
                            String::from("true")
                        } else {
                            String::from("false")
                        })
                    )),
                    (String::from(")"))
                )
            } else {
                {
                    let start = (operation != "trimEnd") && (operation != "rustTrimEnd");
                    {
                        let end = (operation != "trimStart") && (operation != "rustTrimStart");
                        {
                            let rust = operation.starts_with("rust");
                            format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (format!(
                                            "{}{}",
                                            (format!(
                                                "{}{}",
                                                (format!(
                                                    "{}{}",
                                                    (format!(
                                                        "{}{}",
                                                        value.to_owned(),
                                                        (String::from(".trim("))
                                                    )),
                                                    (if start {
                                                        String::from("true")
                                                    } else {
                                                        String::from("false")
                                                    })
                                                )),
                                                (String::from(", "))
                                            )),
                                            (if end {
                                                String::from("true")
                                            } else {
                                                String::from("false")
                                            })
                                        )),
                                        (String::from(", "))
                                    )),
                                    (if rust {
                                        String::from("true")
                                    } else {
                                        String::from("false")
                                    })
                                )),
                                (String::from(")"))
                            )
                        }
                    }
                }
            }
        } else {
            String::from("")
        }
    } else {
        String::from("")
    }
}

#[must_use]
pub fn read_primitive_copy_slice_form(kind: &str) -> String {
    if (kind == "array") || (kind == "string") {
        String::from("identity")
    } else if kind == "record" {
        String::from("method")
    } else {
        String::from("refuse")
    }
}

#[must_use]
pub fn read_primitive_method_minimum_arguments(receiver: &str, operation: &str) -> f64 {
    if (receiver == "array") && (operation == "slice") {
        0f64
    } else if (receiver == "array")
        && (((operation == "includes") || (operation == "indexOf")) || (operation == "lastIndexOf"))
    {
        1f64
    } else if receiver != "string" {
        -1f64
    } else if (operation == "split") || (operation == "slice") {
        0f64
    } else if (((((operation == "indexOf") || (operation == "lastIndexOf"))
        || (operation == "repeat"))
        || (operation == "padStart"))
        || (operation == "padEnd"))
        || (operation == "codePointAt")
    {
        1f64
    } else if (operation == "replace") || (operation == "replaceAll") {
        2f64
    } else {
        -1f64
    }
}

#[must_use]
pub fn read_primitive_method_maximum_arguments(receiver: &str, operation: &str) -> f64 {
    if (receiver == "array") && (operation == "slice") {
        2f64
    } else if (receiver == "array")
        && (((operation == "includes") || (operation == "indexOf")) || (operation == "lastIndexOf"))
    {
        2f64
    } else if receiver != "string" {
        -1f64
    } else if (operation == "repeat") || (operation == "codePointAt") {
        1f64
    } else if (((((((operation == "split") || (operation == "slice"))
        || (operation == "indexOf"))
        || (operation == "lastIndexOf"))
        || (operation == "replace"))
        || (operation == "replaceAll"))
        || (operation == "padStart"))
        || (operation == "padEnd")
    {
        2f64
    } else {
        -1f64
    }
}

#[must_use]
pub fn read_primitive_method_argument_count(receiver: &str, operation: &str) -> f64 {
    self::read_primitive_method_maximum_arguments(receiver, operation)
}

#[must_use]
pub fn read_primitive_method_checked_argument_kind(
    receiver: &str,
    operation: &str,
    index: f64,
) -> String {
    if (receiver == "array") && (operation == "slice") {
        String::from("float")
    } else if (receiver == "array")
        && (((operation == "includes") || (operation == "indexOf")) || (operation == "lastIndexOf"))
    {
        if index == 0f64 {
            String::from("element")
        } else {
            String::from("float")
        }
    } else if receiver != "string" {
        String::from("")
    } else if (operation == "replace") || (operation == "replaceAll") {
        String::from("string")
    } else if (index == 0f64)
        && (((operation == "split") || (operation == "indexOf")) || (operation == "lastIndexOf"))
    {
        String::from("string")
    } else if (index == 1f64) && ((operation == "padStart") || (operation == "padEnd")) {
        String::from("string")
    } else {
        String::from("float")
    }
}

#[must_use]
pub fn read_primitive_method_default_form(receiver: &str, operation: &str, index: f64) -> String {
    if ((receiver == "array")
        && (((operation == "includes") || (operation == "indexOf"))
            || (operation == "lastIndexOf")))
        && (index == 1f64)
    {
        String::from("float")
    } else if ((receiver == "string") && (operation == "split")) && (index == 0f64) {
        String::from("absent")
    } else if ((receiver == "string") && (operation == "split")) && (index == 1f64) {
        String::from("float")
    } else if ((receiver == "string") || (receiver == "array")) && (operation == "slice") {
        String::from("float")
    } else if ((receiver == "string") && ((operation == "indexOf") || (operation == "lastIndexOf")))
        && (index == 1f64)
    {
        String::from("float")
    } else if ((receiver == "string") && ((operation == "padStart") || (operation == "padEnd")))
        && (index == 1f64)
    {
        String::from("string")
    } else {
        String::from("required")
    }
}

#[must_use]
pub fn read_primitive_method_default_value(receiver: &str, operation: &str, index: f64) -> String {
    if ((receiver == "array") && ((operation == "includes") || (operation == "indexOf")))
        && (index == 1f64)
    {
        String::from("0")
    } else if ((receiver == "array") && (operation == "lastIndexOf")) && (index == 1f64) {
        String::from("Infinity")
    } else if ((receiver == "string") && (operation == "split")) && (index == 1f64) {
        String::from("4294967295")
    } else if ((receiver == "string") || (receiver == "array")) && (operation == "slice") {
        if index == 0f64 {
            String::from("0")
        } else {
            String::from("Infinity")
        }
    } else if ((receiver == "string") && (operation == "indexOf")) && (index == 1f64) {
        String::from("0")
    } else if ((receiver == "string") && (operation == "lastIndexOf")) && (index == 1f64) {
        String::from("Infinity")
    } else if ((receiver == "string") && ((operation == "padStart") || (operation == "padEnd")))
        && (index == 1f64)
    {
        String::from(" ")
    } else {
        String::from("")
    }
}

#[must_use]
pub fn read_primitive_method_checked_result_form(receiver: &str, operation: &str) -> String {
    if (receiver == "array") && (operation == "slice") {
        String::from("receiver")
    } else if (receiver == "array") && (operation == "includes") {
        String::from("bool")
    } else if (receiver == "array") && ((operation == "indexOf") || (operation == "lastIndexOf")) {
        String::from("float")
    } else if receiver != "string" {
        String::from("")
    } else if operation == "split" {
        String::from("array-string")
    } else if (operation == "indexOf") || (operation == "lastIndexOf") {
        String::from("float")
    } else if operation == "codePointAt" {
        String::from("optional-float")
    } else {
        String::from("string")
    }
}

#[must_use]
pub fn read_primitive_method_argument_obligation(
    receiver: &str,
    operation: &str,
    element: &str,
) -> String {
    if (receiver != "array")
        || (((operation != "includes") && (operation != "indexOf")) && (operation != "lastIndexOf"))
    {
        String::from("")
    } else if (((((element == "array") || (element == "record")) || (element == "map"))
        || (element == "set"))
        || (element == "data"))
        || (element == "function")
    {
        String::from("object identity must be represented before searching arrays of objects")
    } else {
        String::from("")
    }
}

#[must_use]
pub fn render_primitive_array_search_comparator(
    operation: &str,
    element: &str,
    absence: &str,
) -> String {
    {
        let present = if (operation == "includes") && (element == "float") {
            String::from("a == b || (a.is_nan() && b.is_nan())")
        } else {
            String::from("a == b")
        };
        if absence == "both" {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from(
                        "|a, b| match (a, b) { (crate::MlOptional::Value(a), crate::MlOptional::Value(b)) => "
                    )),
                    present
                )),
                (String::from(
                    ", (crate::MlOptional::Null, crate::MlOptional::Null) | (crate::MlOptional::Undefined, crate::MlOptional::Undefined) => true, _ => false }"
                ))
            )
        } else if (absence == "null") || (absence == "undefined") {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("|a, b| match (a, b) { (Some(a), Some(b)) => ")),
                    present
                )),
                (String::from(", (None, None) => true, _ => false }"))
            )
        } else if (operation == "includes") && (element == "float") {
            format!("{}{}", (String::from("|a: &f64, b: &f64| ")), present)
        } else {
            format!("{}{}", (String::from("|a, b| ")), present)
        }
    }
}

#[must_use]
pub fn render_primitive_array_search_expression(
    operation: &str,
    object: &str,
    search: &str,
    from: &str,
    element: &str,
    absence: &str,
) -> String {
    {
        let comparator =
            self::render_primitive_array_search_comparator(operation, element, absence);
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (format!(
                                            "{}{}",
                                            (format!(
                                                "{}{}",
                                                (format!(
                                                    "{}{}",
                                                    (format!(
                                                        "{}{}",
                                                        (String::from("crate::ml_array::")),
                                                        (if operation == "includes" {
                                                            String::from("includes")
                                                        } else {
                                                            String::from("index_of")
                                                        })
                                                    )),
                                                    (String::from("("))
                                                )),
                                                object.to_owned()
                                            )),
                                            (String::from(", &"))
                                        )),
                                        search.to_owned()
                                    )),
                                    (String::from(", "))
                                )),
                                from.to_owned()
                            )),
                            (String::from(", "))
                        )),
                        (if operation == "lastIndexOf" {
                            String::from("true")
                        } else {
                            String::from("false")
                        })
                    )),
                    (String::from(", "))
                )),
                comparator
            )),
            (String::from(")"))
        )
    }
}

#[must_use]
pub fn read_primitive_method_rendering_refusal(
    target: &str,
    receiver: &str,
    operation: &str,
) -> String {
    if (target != "Lean") && (target != "Rocq") {
        String::from("")
    } else if receiver != "array" {
        String::from("this target needs a UTF-16 operational string encoding for this operation")
    } else if (((operation == "slice") || (operation == "includes")) || (operation == "indexOf"))
        || (operation == "lastIndexOf")
    {
        format!(
            "{}{}",
            (String::from("this array operation needs an operational encoding in ")),
            target.to_owned()
        )
    } else {
        String::from("")
    }
}

#[must_use]
pub fn render_primitive_method_argument(
    target: &str,
    receiver: &str,
    operation: &str,
    index: f64,
    absent: bool,
    text: &str,
) -> String {
    if (target == "JavaScript") && absent {
        String::from("undefined")
    } else if (((target == "Rust") && (receiver == "string")) && (operation == "split"))
        && (index == 0f64)
    {
        if absent {
            String::from("None")
        } else {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("Some(")), text.to_owned())),
                (String::from(")"))
            )
        }
    } else {
        text.to_owned()
    }
}

#[must_use]
pub fn render_primitive_method_expression(
    target: &str,
    receiver: &str,
    operation: &str,
    object: &str,
    arguments_text: &str,
) -> String {
    if target == "JavaScript" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!("{}{}", (String::from("(")), object.to_owned())),
                            (String::from(")."))
                        )),
                        operation.to_owned()
                    )),
                    (String::from("("))
                )),
                arguments_text.to_owned()
            )),
            (String::from(")"))
        )
    } else if ((target == "Rust") && (receiver == "array")) && (operation == "slice") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (String::from("crate::ml_array::slice(")),
                        object.to_owned()
                    )),
                    (String::from(", "))
                )),
                arguments_text.to_owned()
            )),
            (String::from(")"))
        )
    } else if (target != "Rust") || (receiver != "string") {
        String::from("")
    } else if operation == "indexOf" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!("{}{}", object.to_owned(), (String::from(".index_of(")))),
                arguments_text.to_owned()
            )),
            (String::from(")"))
        )
    } else if operation == "lastIndexOf" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!("{}{}", object.to_owned(), (String::from(".last_index_of(")))),
                arguments_text.to_owned()
            )),
            (String::from(")"))
        )
    } else if operation == "codePointAt" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!("{}{}", object.to_owned(), (String::from(".code_point_at(")))),
                arguments_text.to_owned()
            )),
            (String::from(")"))
        )
    } else if (operation == "replace") || (operation == "replaceAll") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", object.to_owned(), (String::from(".replace(")))),
                        arguments_text.to_owned()
                    )),
                    (String::from(", "))
                )),
                (if operation == "replaceAll" {
                    String::from("true")
                } else {
                    String::from("false")
                })
            )),
            (String::from(")"))
        )
    } else if (operation == "padStart") || (operation == "padEnd") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", object.to_owned(), (String::from(".pad(")))),
                        arguments_text.to_owned()
                    )),
                    (String::from(", "))
                )),
                (if operation == "padStart" {
                    String::from("true")
                } else {
                    String::from("false")
                })
            )),
            (String::from(")"))
        )
    } else {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", object.to_owned(), (String::from(".")))),
                        operation.to_owned()
                    )),
                    (String::from("("))
                )),
                arguments_text.to_owned()
            )),
            (String::from(")"))
        )
    }
}

#[must_use]
pub fn require_primitive_method_utf16(expression: &str, receiver: &str) -> bool {
    (expression == "method") && (receiver == "string")
}
