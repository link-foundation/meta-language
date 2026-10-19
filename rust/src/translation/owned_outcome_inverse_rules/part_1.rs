// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_owned_outcome_target_width(width: f64) -> bool {
    (width == 32f64) || (width == 64f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_owned_outcome_json_kind(
    null_value: bool,
    array_value: bool,
    value_type: &str,
) -> String {
    if null_value {
        String::from("null")
    } else if array_value {
        String::from("array")
    } else if value_type == "object" {
        String::from("object")
    } else if value_type == "number" {
        String::from("number")
    } else if value_type == "boolean" {
        String::from("boolean")
    } else if value_type == "string" {
        String::from("string")
    } else {
        String::from("invalid")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_owned_outcome_json_fields(kind: &str) -> Vec<String> {
    if kind == "object" {
        vec![
            String::from("kind"),
            String::from("keys"),
            String::from("aliasOf"),
        ]
    } else if kind == "array" {
        vec![String::from("kind"), String::from("aliasOf")]
    } else if kind == "number" {
        vec![String::from("kind"), String::from("bits")]
    } else if (kind == "string") || (kind == "boolean") {
        vec![String::from("kind"), String::from("value")]
    } else {
        vec![String::from("kind")]
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_owned_outcome_container(
    source_kind: &str,
    json_kind: &str,
    variant_record: bool,
) -> bool {
    ((((source_kind == "Vec") || (source_kind == "IndexMap")) || (source_kind == "struct_item"))
        || ((source_kind == "enum_item") && variant_record))
        || ((source_kind == "json") && ((json_kind == "array") || (json_kind == "object")))
}
