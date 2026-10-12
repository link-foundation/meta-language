// Generated from the shared typed-program-links schema through
// js/scripts/generate-typed-program-link-rules.mjs. Do not edit by hand.
/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn read_program_container_kind(term: &str) -> String {
    if term == "program record" {
        String::from("record")
    } else if term == "ordered program list" {
        String::from("array")
    } else if term == "program mapping" {
        String::from("map")
    } else {
        String::from("")
    }
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn read_program_scalar_kind(term: &str) -> String {
    if term == "string value" {
        String::from("string")
    } else if term == "number value" {
        String::from("number")
    } else if term == "boolean value" {
        String::from("boolean")
    } else if term == "integer value" {
        String::from("bigint")
    } else if term == "null value" {
        String::from("null")
    } else if term == "undefined value" {
        String::from("undefined")
    } else {
        String::from("")
    }
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn accept_program_reference_field(label: &str) -> bool {
    label == "binding reference" || label == "declaration reference"
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn is_unsafe_program_field(name: &str) -> bool {
    name == "__proto__" || name == "constructor" || name == "prototype"
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn accept_program_schema_root(term: &str, revision: &str, arity: &str) -> bool {
    term == "typed program" && revision == "1" && arity == "1"
}
