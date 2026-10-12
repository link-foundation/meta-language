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
pub fn read_program_expression_projection(term: &str) -> String {
    if term == "lit" {
        String::from("lit")
    } else if term == "lit:bool" {
        String::from("lit:bool")
    } else if term == "lit:string" {
        String::from("lit:string")
    } else if term == "lit:nat" {
        String::from("lit:nat")
    } else if term == "lit:int" {
        String::from("lit:int")
    } else if term == "lit:fixed" {
        String::from("lit:fixed")
    } else if term == "lit:float" {
        String::from("lit:float")
    } else if term == "unit" {
        String::from("unit")
    } else if term == "var" {
        String::from("var")
    } else if term == "call" {
        String::from("call")
    } else if term == "unary" {
        String::from("unary")
    } else if term == "binary" {
        String::from("binary")
    } else if term == "if" {
        String::from("if")
    } else if term == "let" {
        String::from("let")
    } else if term == "toString" {
        String::from("toString")
    } else if term == "toString:true" {
        String::from("toString:true")
    } else if term == "stringMap" {
        String::from("stringMap")
    } else if term == "stringTest" {
        String::from("stringTest")
    } else if term == "cast" {
        String::from("cast")
    } else if term == "abort" {
        String::from("abort")
    } else if term == "print" {
        String::from("print")
    } else if term == "array" {
        String::from("array")
    } else if term == "append" {
        String::from("append")
    } else if term == "index" {
        String::from("index")
    } else if term == "length" {
        String::from("length")
    } else if term == "length:string" {
        String::from("length:string")
    } else if term == "math" {
        String::from("math")
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

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn accept_program_machine_integer_width(width: &str) -> bool {
    width == "8" || width == "16" || width == "32" || width == "64" || width == "128"
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn accept_program_optional_absence(absence: &str) -> bool {
    absence == "null" || absence == "undefined" || absence == "both"
}
