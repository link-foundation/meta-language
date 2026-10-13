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
        String::new()
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

/// Read a shared structured data graph decision.
#[must_use]
pub fn accept_structured_data_record_field(label: &str, value: &str) -> bool {
    if label == "t" {
        value == "semantic"
    } else if label == "n" {
        value == "1"
    } else if label == "term" {
        value == "structured%20data%20record"
    } else if label == "def" {
        value == "1"
    } else if label == "lang" {
        value == "meta-language%20structured%20data"
    } else {
        false
    }
}

/// Read a shared structured data graph decision.
#[must_use]
pub fn accept_structured_data_record_shape(reference_count: &str, metadata_count: &str) -> bool {
    reference_count == "1" && metadata_count == "1"
}
