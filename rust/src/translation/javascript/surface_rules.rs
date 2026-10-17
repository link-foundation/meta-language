// Generated from js/src/translation/javascript-surface-rules.js by
// js/scripts/generate-javascript-surface-rules.mjs. Do not edit by hand.
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
pub fn read_java_script_includes_receiver_form(receiver: &str, force: bool) -> String {
    if receiver == "array" {
        String::from("array")
    } else if (receiver == "var") && !force {
        String::from("defer")
    } else {
        String::from("string")
    }
}

#[must_use]
pub fn accept_java_script_function_start(
    kind: &str,
    value: &str,
    following: &str,
    after_group: &str,
) -> bool {
    ((kind == "identifier") && ((value == "function") || (following == "=>")))
        || (((kind == "punct") && (value == "(")) && (after_group == "=>"))
}

#[must_use]
pub fn accept_java_script_namespace_method(kind: &str, following: &str) -> bool {
    (kind == "identifier") && (following == "(")
}

#[must_use]
pub fn read_java_script_collection_construction(name: &str, extra_argument: bool) -> String {
    if extra_argument {
        String::new()
    } else if name == "Set" {
        String::from("set")
    } else if name == "Map" {
        String::from("map")
    } else {
        String::new()
    }
}

#[must_use]
pub fn read_java_script_construction_obligation() -> String {
    String::from("the constructor needs a registered portable type and construction rule")
}

#[must_use]
pub fn read_java_script_documentation_type_form(tagged: bool) -> String {
    if tagged {
        String::from("tagged")
    } else {
        String::from("alias")
    }
}

#[must_use]
pub fn read_java_script_value_case_form(
    kind: &str,
    value: &str,
    suffix: &str,
    negative: bool,
) -> String {
    if kind == "number" {
        if suffix == "n" {
            String::from("numLit")
        } else {
            String::from("value")
        }
    } else if (!negative && ((kind == "identifier") || (kind == "punct")))
        && ((value == "true") || (value == "false"))
    {
        String::from("boolLit")
    } else if !negative && ((kind == "string") || (kind == "identifier")) {
        String::from("value")
    } else {
        String::new()
    }
}

#[must_use]
pub const fn accept_java_script_value_switch(tagged: bool, value_case: bool) -> bool {
    !tagged && value_case
}

#[must_use]
pub fn read_java_script_value_switch_instruction(
    imperative: bool,
    step: f64,
    count: f64,
) -> String {
    if step == 0f64 {
        String::from("capture")
    } else if step == 1f64 {
        String::from("fallback")
    } else if !imperative && (step == 2f64) {
        String::from("reachability")
    } else {
        {
            let start = if imperative { 2f64 } else { 3f64 };
            if step >= (start + (count * 3f64)) {
                String::from("bind")
            } else {
                {
                    let phase = (step - start) % 3f64;
                    if phase == 0f64 {
                        String::from("test")
                    } else if phase == 1f64 {
                        String::from("body")
                    } else {
                        String::from("select")
                    }
                }
            }
        }
    }
}

#[must_use]
pub fn read_java_script_value_switch_position(imperative: bool, step: f64, count: f64) -> f64 {
    {
        let start = if imperative { 2f64 } else { 3f64 };
        {
            let offset = step - start;
            ((count - 1f64) - ((offset - (offset % 3f64)) / 3f64))
        }
    }
}

#[must_use]
pub fn read_java_script_object_statement_form(
    tagged: bool,
    methods: bool,
    preceding_statement: bool,
) -> String {
    if tagged {
        String::from("value")
    } else if preceding_statement {
        String::from("refuse-order")
    } else if methods {
        String::from("namespace")
    } else {
        String::from("value")
    }
}

#[must_use]
pub fn read_java_script_object_property_form(
    name: &str,
    method: bool,
    colon: bool,
    shorthand: bool,
) -> String {
    if name == "$" {
        String::from("tag")
    } else if method {
        String::from("method")
    } else if colon {
        String::from("field")
    } else if shorthand {
        String::from("shorthand")
    } else {
        String::from("missing-colon")
    }
}

#[must_use]
pub fn read_java_script_object_value_form(tagged: bool, duplicate: bool, spread: bool) -> String {
    if tagged {
        if duplicate {
            String::from("duplicate")
        } else if spread {
            String::from("spread")
        } else {
            String::from("constructor")
        }
    } else {
        String::from("record")
    }
}

#[must_use]
pub fn read_java_script_method_receiver_kind(operation: &str) -> String {
    if (((((((((((((((((operation == "map") || (operation == "flatMap"))
        || (operation == "filter"))
        || (operation == "find"))
        || (operation == "findIndex"))
        || (operation == "some"))
        || (operation == "every"))
        || (operation == "reduce"))
        || (operation == "join"))
        || (operation == "push"))
        || (operation == "pop"))
        || (operation == "shift"))
        || (operation == "unshift"))
        || (operation == "splice"))
        || (operation == "reverse"))
        || (operation == "sort"))
        || (operation == "fill"))
        || (operation == "copyWithin")
    {
        String::from("array")
    } else if ((((((((((((((((operation == "split") || (operation == "trim"))
        || (operation == "trimStart"))
        || (operation == "trimEnd"))
        || (operation == "toLowerCase"))
        || (operation == "toUpperCase"))
        || (operation == "startsWith"))
        || (operation == "endsWith"))
        || (operation == "charAt"))
        || (operation == "charCodeAt"))
        || (operation == "codePointAt"))
        || (operation == "substring"))
        || (operation == "repeat"))
        || (operation == "padStart"))
        || (operation == "padEnd"))
        || (operation == "replace"))
        || (operation == "replaceAll")
    {
        String::from("string")
    } else {
        String::new()
    }
}

#[must_use]
pub fn read_java_script_array_method_surface_form(operation: &str, count: f64) -> String {
    if operation == "slice" {
        if count == 0f64 {
            String::from("copySlice")
        } else {
            String::from("method")
        }
    } else {
        String::from("array")
    }
}

#[must_use]
pub fn read_java_script_method_argument_kind(
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
        } else if index == 1f64 {
            String::from("float")
        } else {
            String::new()
        }
    } else if receiver == "string" {
        if (operation == "charCodeAt") && (index == 0f64) {
            String::from("number-value")
        } else if (index == 0f64)
            && (((((((operation == "slice") || (operation == "substring"))
                || (operation == "repeat"))
                || (operation == "charAt"))
                || (operation == "codePointAt"))
                || (operation == "padStart"))
                || (operation == "padEnd"))
        {
            String::from("float")
        } else if (index == 0f64)
            && ((((((operation == "includes") || (operation == "startsWith"))
                || (operation == "endsWith"))
                || (operation == "indexOf"))
                || (operation == "lastIndexOf"))
                || (operation == "split"))
        {
            String::from("string")
        } else if (index == 1f64)
            && (((((((((operation == "slice") || (operation == "substring"))
                || (operation == "includes"))
                || (operation == "startsWith"))
                || (operation == "endsWith"))
                || (operation == "indexOf"))
                || (operation == "lastIndexOf"))
                || (operation == "charCodeAt"))
                || (operation == "split"))
        {
            String::from("float")
        } else if (index == 1f64) && ((operation == "padStart") || (operation == "padEnd")) {
            String::from("string")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

#[must_use]
pub fn read_java_script_string_method_result_form(operation: &str) -> String {
    if ((operation == "includes") || (operation == "startsWith")) || (operation == "endsWith") {
        String::from("bool")
    } else if ((operation == "indexOf") || (operation == "lastIndexOf"))
        || (operation == "charCodeAt")
    {
        String::from("float")
    } else if operation == "codePointAt" {
        String::from("optional-float")
    } else if operation == "split" {
        String::from("array-string")
    } else {
        String::from("string")
    }
}

#[must_use]
pub const fn accept_java_script_terminal_tag_guard(
    positive: bool,
    alternate: bool,
    terminal: bool,
    same_binding: bool,
    duplicate: bool,
) -> bool {
    (((positive && !alternate) && terminal) && same_binding) && !duplicate
}

#[must_use]
pub const fn accept_java_script_tag_guard_sequence(
    multiple: bool,
    terminal_remainder: bool,
) -> bool {
    multiple && terminal_remainder
}

#[must_use]
pub fn read_java_script_equality_inference_step(
    operation: &str,
    left: &str,
    right: &str,
    force: bool,
) -> String {
    if (operation != "eq") && (operation != "ne") {
        String::from("unify")
    } else if (left == "var") || (right == "var") {
        if force {
            String::from("unify")
        } else {
            String::from("defer")
        }
    } else if (left == "option") || (right == "option") {
        String::from("independent")
    } else {
        String::from("unify")
    }
}
