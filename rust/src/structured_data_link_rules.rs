// Generated automatically from js/src/structured-data-link-rules.js.
// Emitter styles are recorded in parity/self-translation/structured-data-link-rules-decorators.lino.
// Do not edit by hand.
/// Read a shared structured data graph decision.
#[must_use]
pub fn read_structured_data_kind(term: &str) -> String {
    if term == "structured data" {
        String::from("root")
    } else if term == "object value" {
        String::from("object")
    } else if term == "array value" {
        String::from("array")
    } else if term == "key value pair" {
        String::from("pair")
    } else if term == "string value" {
        String::from("string")
    } else if term == "Unicode code unit string" {
        String::from("units")
    } else if term == "Unicode code unit" {
        String::from("unit")
    } else if term == "number value" {
        String::from("number")
    } else if term == "boolean value" {
        String::from("boolean")
    } else if term == "null value" {
        String::from("null")
    } else {
        String::from("")
    }
}

/// Read a shared structured data graph decision.
#[must_use]
pub fn accept_structured_data_shape(kind: &str, arity: &str, has_definition: bool) -> bool {
    if kind == "root" {
        arity == "1" && has_definition
    } else if kind == "pair" {
        arity == "2" && !has_definition
    } else if kind == "object" || kind == "array" || kind == "units" {
        !has_definition
    } else if kind == "string" {
        arity == "0"
    } else if kind == "number" || kind == "boolean" || kind == "unit" {
        arity == "0" && has_definition
    } else if kind == "null" {
        arity == "0" && !has_definition
    } else {
        false
    }
}
