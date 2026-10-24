// Generated from complete js/src/translation/javascript-property-call-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_property_operation_kind(operation: &str) -> String {
    if ((operation == "includes") || (operation == "startsWith")) || (operation == "endsWith") {
        String::from("test")
    } else if ((((operation == "trim") || (operation == "trimStart")) || (operation == "trimEnd"))
        || (operation == "toLowerCase"))
        || (operation == "toUpperCase")
    {
        String::from("map")
    } else if operation == "toString" {
        String::from("text")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_resolved_property_call_form(receiver: &str, operation: &str) -> String {
    {
        let kind = self::read_java_script_source_property_operation_kind(operation);
        if (kind).is_empty() {
            String::new()
        } else if receiver == "record" {
            String::from("record")
        } else if kind == "text" {
            String::from("text")
        } else if receiver == "string" {
            kind
        } else {
            String::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_method_inference_step(
    kind: &str,
    parameters: f64,
    arguments_count: f64,
) -> String {
    if kind == "var" {
        String::from("infer")
    } else if (kind == "function")
        && ((parameters).partial_cmp(&(arguments_count)) == Some(std::cmp::Ordering::Equal))
    {
        String::from("apply")
    } else {
        String::from("checked error")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_call_fallback_kind(operation: &str, selected: &str) -> String {
    if operation == "includes" {
        String::from("string")
    } else if operation == "toString" {
        String::from("scalar")
    } else {
        selected.to_owned()
    }
}
