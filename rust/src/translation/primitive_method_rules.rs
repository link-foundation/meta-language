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
                "crate::ml_string::Text::from_units(vec![{}])",
                value.to_owned()
            )
        } else {
            format!("String::from({})", value.to_owned())
        }
    } else if form == "constant" {
        if units {
            format!("crate::ml_string::Text::from_text({})", value.to_owned())
        } else {
            format!("String::from({})", value.to_owned())
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
            format!("crate::ml_string::Text::from_text(&{})", value.to_owned())
        } else {
            value.to_owned()
        }
    } else if form == "concat" {
        if units {
            format!("{}.concat({})", value.to_owned(), other.to_owned())
        } else {
            format!(
                "format!(\"{{}}{{}}\", {}, {})",
                value.to_owned(),
                other.to_owned()
            )
        }
    } else if units {
        if form == "length" {
            format!("{}.length()", value.to_owned())
        } else if form == "test" {
            {
                let method = if operation == "startsWith" {
                    String::from("starts_with")
                } else if operation == "endsWith" {
                    String::from("ends_with")
                } else {
                    String::from("contains")
                };
                format!("{}.{method}({})", value.to_owned(), other.to_owned())
            }
        } else if form == "map" {
            if (operation == "toLowerCase") || (operation == "toUpperCase") {
                format!(
                    "{}.map_case({})",
                    value.to_owned(),
                    if operation == "toUpperCase" {
                        String::from("true")
                    } else {
                        String::from("false")
                    }
                )
            } else {
                {
                    let start = (operation != "trimEnd") && (operation != "rustTrimEnd");
                    {
                        let end = (operation != "trimStart") && (operation != "rustTrimStart");
                        {
                            let rust = operation.starts_with("rust");
                            format!(
                                "{}.trim({}, {}, {})",
                                value.to_owned(),
                                if start {
                                    String::from("true")
                                } else {
                                    String::from("false")
                                },
                                if end {
                                    String::from("true")
                                } else {
                                    String::from("false")
                                },
                                if rust {
                                    String::from("true")
                                } else {
                                    String::from("false")
                                }
                            )
                        }
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
    if (receiver == "array")
        && (((operation == "slice") || (operation == "join")) || (operation == "at"))
    {
        0f64
    } else if (receiver == "array")
        && (((operation == "includes") || (operation == "indexOf")) || (operation == "lastIndexOf"))
    {
        1f64
    } else if receiver == "string" {
        if ((((operation == "split") || (operation == "slice")) || (operation == "charCodeAt"))
            || (operation == "charAt"))
            || (operation == "at")
        {
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
    } else {
        -1f64
    }
}

#[must_use]
pub fn read_primitive_method_maximum_arguments(receiver: &str, operation: &str) -> f64 {
    if receiver == "array" {
        if (operation == "join") || (operation == "at") {
            1f64
        } else if (((operation == "slice") || (operation == "includes"))
            || (operation == "indexOf"))
            || (operation == "lastIndexOf")
        {
            2f64
        } else {
            -1f64
        }
    } else if receiver == "string" {
        if ((((operation == "repeat") || (operation == "codePointAt"))
            || (operation == "charCodeAt"))
            || (operation == "charAt"))
            || (operation == "at")
        {
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
    if (receiver == "array") && (operation == "at") {
        String::from("number-value")
    } else if (receiver == "array") && (operation == "join") {
        String::from("string")
    } else if (receiver == "array") && (operation == "slice") {
        String::from("float")
    } else if (receiver == "array")
        && (((operation == "includes") || (operation == "indexOf")) || (operation == "lastIndexOf"))
    {
        if index == 0f64 {
            String::from("element")
        } else {
            String::from("float")
        }
    } else if receiver == "string" {
        if ((operation == "charCodeAt") || (operation == "charAt")) || (operation == "at") {
            String::from("number-value")
        } else if (((operation == "replace") || (operation == "replaceAll"))
            || ((index == 0f64)
                && (((operation == "split") || (operation == "indexOf"))
                    || (operation == "lastIndexOf"))))
            || ((index == 1f64) && ((operation == "padStart") || (operation == "padEnd")))
        {
            String::from("string")
        } else {
            String::from("float")
        }
    } else {
        String::new()
    }
}

#[must_use]
pub fn read_primitive_method_default_form(receiver: &str, operation: &str, index: f64) -> String {
    if (receiver == "array") && (operation == "at") {
        String::from("float")
    } else if (receiver == "array") && (operation == "join") {
        String::from("string")
    } else if ((receiver == "array")
        && (((operation == "includes") || (operation == "indexOf"))
            || (operation == "lastIndexOf")))
        && (index == 1f64)
    {
        String::from("float")
    } else if ((receiver == "string") && (operation == "split")) && (index == 0f64) {
        String::from("absent")
    } else if (((((receiver == "string") && (operation == "split")) && (index == 1f64))
        || (((receiver == "string")
            && (((operation == "charCodeAt") || (operation == "charAt")) || (operation == "at")))
            && (index == 0f64)))
        || (((receiver == "string") || (receiver == "array")) && (operation == "slice")))
        || (((receiver == "string") && ((operation == "indexOf") || (operation == "lastIndexOf")))
            && (index == 1f64))
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
    if (receiver == "array") && (operation == "at") {
        String::from("0")
    } else if (receiver == "array") && (operation == "join") {
        String::from(",")
    } else if ((receiver == "array") && ((operation == "includes") || (operation == "indexOf")))
        && (index == 1f64)
    {
        String::from("0")
    } else if ((receiver == "array") && (operation == "lastIndexOf")) && (index == 1f64) {
        String::from("Infinity")
    } else if ((receiver == "string") && (operation == "split")) && (index == 1f64) {
        String::from("4294967295")
    } else if ((receiver == "string")
        && (((operation == "charCodeAt") || (operation == "charAt")) || (operation == "at")))
        && (index == 0f64)
    {
        String::from("0")
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
        String::new()
    }
}

#[must_use]
pub fn read_primitive_method_checked_result_form(receiver: &str, operation: &str) -> String {
    if (receiver == "array") && (operation == "at") {
        String::from("optional-element")
    } else if (receiver == "array") && (operation == "join") {
        String::from("string")
    } else if (receiver == "array") && (operation == "slice") {
        String::from("receiver")
    } else if (receiver == "array") && (operation == "includes") {
        String::from("bool")
    } else if (receiver == "array") && ((operation == "indexOf") || (operation == "lastIndexOf")) {
        String::from("float")
    } else if receiver == "string" {
        if operation == "split" {
            String::from("array-string")
        } else if ((operation == "indexOf") || (operation == "lastIndexOf"))
            || (operation == "charCodeAt")
        {
            String::from("float")
        } else if operation == "codePointAt" {
            String::from("optional-float")
        } else if operation == "at" {
            String::from("optional-string")
        } else {
            String::from("string")
        }
    } else {
        String::new()
    }
}

#[must_use]
pub fn read_primitive_method_argument_obligation(
    receiver: &str,
    operation: &str,
    element: &str,
) -> String {
    if (receiver == "array") && (operation == "join") {
        {
            let form = self::read_primitive_array_join_element_form(element);
            if (form == "refuse") || (form == "optional") {
                String::from(
                    "joining structured values needs their JavaScript string conversion and cycle semantics",
                )
            } else {
                String::new()
            }
        }
    } else if (receiver != "array")
        || (((operation != "includes") && (operation != "indexOf")) && (operation != "lastIndexOf"))
    {
        String::new()
    } else if (((((element == "array") || (element == "record")) || (element == "map"))
        || (element == "set"))
        || (element == "data"))
        || (element == "function")
    {
        String::from("object identity must be represented before searching arrays of objects")
    } else {
        String::new()
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
                "{}{present}{}",
                String::from(
                    "|a, b| match (a, b) { (crate::MlOptional::Value(a), crate::MlOptional::Value(b)) => "
                ),
                String::from(
                    ", (crate::MlOptional::Null, crate::MlOptional::Null) | (crate::MlOptional::Undefined, crate::MlOptional::Undefined) => true, _ => false }"
                )
            )
        } else if (absence == "null") || (absence == "undefined") {
            format!(
                "|a, b| match (a, b) {{ (Some(a), Some(b)) => {present}, (None, None) => true, _ => false }}"
            )
        } else if (operation == "includes") && (element == "float") {
            format!("|a: &f64, b: &f64| {present}")
        } else {
            format!("|a, b| {present}")
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
            "crate::ml_array::{}({}, &{}, {}, {}, {comparator})",
            if operation == "includes" {
                String::from("includes")
            } else {
                String::from("index_of")
            },
            object.to_owned(),
            search.to_owned(),
            from.to_owned(),
            if operation == "lastIndexOf" {
                String::from("true")
            } else {
                String::from("false")
            }
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
        String::new()
    } else if receiver == "array" {
        if (((operation == "slice") || (operation == "includes")) || (operation == "indexOf"))
            || (operation == "lastIndexOf")
        {
            format!(
                "this array operation needs an operational encoding in {}",
                target.to_owned()
            )
        } else {
            String::new()
        }
    } else {
        String::from("this target needs a UTF-16 operational string encoding for this operation")
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
            format!("Some({})", text.to_owned())
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
            "({}).{}({})",
            object.to_owned(),
            operation.to_owned(),
            arguments_text.to_owned()
        )
    } else if ((target == "Rust") && (receiver == "array")) && (operation == "slice") {
        format!(
            "crate::ml_array::slice({}, {})",
            object.to_owned(),
            arguments_text.to_owned()
        )
    } else if (target != "Rust") || (receiver != "string") {
        String::new()
    } else if operation == "indexOf" {
        format!(
            "{}.index_of({})",
            object.to_owned(),
            arguments_text.to_owned()
        )
    } else if operation == "lastIndexOf" {
        format!(
            "{}.last_index_of({})",
            object.to_owned(),
            arguments_text.to_owned()
        )
    } else if operation == "codePointAt" {
        format!(
            "{}.code_point_at({})",
            object.to_owned(),
            arguments_text.to_owned()
        )
    } else if (operation == "replace") || (operation == "replaceAll") {
        format!(
            "{}.replace({}, {})",
            object.to_owned(),
            arguments_text.to_owned(),
            if operation == "replaceAll" {
                String::from("true")
            } else {
                String::from("false")
            }
        )
    } else if (operation == "padStart") || (operation == "padEnd") {
        format!(
            "{}.pad({}, {})",
            object.to_owned(),
            arguments_text.to_owned(),
            if operation == "padStart" {
                String::from("true")
            } else {
                String::from("false")
            }
        )
    } else {
        format!(
            "{}.{}({})",
            object.to_owned(),
            operation.to_owned(),
            arguments_text.to_owned()
        )
    }
}

#[must_use]
pub fn require_primitive_method_utf16(expression: &str, receiver: &str) -> bool {
    (expression == "method") && (receiver == "string")
}

#[must_use]
pub fn read_primitive_method_lowering_form(receiver: &str, operation: &str) -> String {
    if (receiver == "array") && (operation == "at") {
        String::from("relative-element-template")
    } else if (receiver == "string") && (operation == "at") {
        String::from("relative-character-template")
    } else if (receiver == "string") && (operation == "charAt") {
        String::from("character-template")
    } else if (receiver == "string") && (operation == "charCodeAt") {
        String::from("code-unit-template")
    } else {
        String::from("method")
    }
}

#[must_use]
pub fn read_primitive_array_join_element_form(kind: &str) -> String {
    if kind == "string" {
        String::from("string")
    } else if kind == "unit" {
        String::from("empty")
    } else if kind == "option" {
        String::from("optional")
    } else if ((((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "int"))
        || (kind == "nat")
    {
        String::from("text")
    } else {
        String::from("refuse")
    }
}

#[must_use]
pub fn read_primitive_method_argument_obligation_feature(operation: &str) -> String {
    if operation == "join" {
        String::from("array join")
    } else {
        String::from("array identity search")
    }
}

#[must_use]
pub fn read_primitive_array_join_operand_form(
    units: bool,
    string_element: bool,
    operand: &str,
) -> String {
    if units && ((operand == "separator") || string_element) {
        String::from("borrow")
    } else {
        String::from("receiver")
    }
}

#[must_use]
pub fn render_primitive_array_join_element(
    kind: &str,
    absence: &str,
    name: &str,
    present: &str,
    empty: &str,
) -> String {
    {
        let form = self::read_primitive_array_join_element_form(kind);
        if form == "empty" {
            empty.to_owned()
        } else if form == "optional" {
            if absence == "both" {
                format!(
                    "match {}{}{}, _ => {} }}",
                    name.to_owned(),
                    String::from(" { crate::MlOptional::Value(ml_present) => "),
                    present.to_owned(),
                    empty.to_owned()
                )
            } else {
                format!(
                    "match {} {{ Some(ml_present) => {}, None => {} }}",
                    name.to_owned(),
                    present.to_owned(),
                    empty.to_owned()
                )
            }
        } else {
            present.to_owned()
        }
    }
}

#[must_use]
pub fn render_primitive_array_join_expression(
    units: bool,
    string_element: bool,
    object: &str,
    separator: &str,
    element_text: &str,
) -> String {
    if string_element {
        if units {
            format!(
                "crate::ml_string::join({}, {})",
                object.to_owned(),
                separator.to_owned()
            )
        } else {
            format!(
                "{}.join({}.as_str())",
                object.to_owned(),
                separator.to_owned()
            )
        }
    } else {
        {
            let values = format!(
                "{}.iter().cloned().map(|ml_value| {}).collect::<Vec<_>>()",
                object.to_owned(),
                element_text.to_owned()
            );
            if units {
                format!(
                    "crate::ml_string::join(&{values}, {})",
                    separator.to_owned()
                )
            } else {
                format!("({values}).join({}.as_str())", separator.to_owned())
            }
        }
    }
}
