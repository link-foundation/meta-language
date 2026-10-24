// Generated from complete js/src/translation/javascript-own-property-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_own_property_builtin_operation(name: &str) -> String {
    if name == "Object.hasOwn" {
        String::from("testJavaScriptOwnedProperty")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_own_property_receiver_form(kind: &str, element: &str) -> String {
    if (kind == "unit") || ((kind == "option") && (element == "unit")) {
        String::from("absent")
    } else if kind == "option" {
        String::from("guarded")
    } else if kind == "record" {
        String::from("record")
    } else if ((kind == "array") || (kind == "tuple")) || (kind == "string") {
        String::from("indexed")
    } else if (((((kind == "bool") || (kind == "float")) || (kind == "int")) || (kind == "nat"))
        || (kind == "map"))
        || (kind == "set")
    {
        String::from("empty")
    } else {
        String::from("refuse")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_object_coercion_error() -> String {
    String::from("Cannot convert undefined or null to object")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_property_index_limit() -> String {
    String::from("4294967295")
}
