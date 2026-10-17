// Generated from the linked Rust readonly query policy through the checked translator.
// Do not edit by hand.
/// Admit the corresponding standard Rust readonly source query.
#[must_use]
pub fn accept_rust_read_only_query_owner(query: &str, kind: &str, path: &str) -> bool {
    if query == "inclusive range" {
        kind == "primitive" && path == "f64"
    } else if query == "string empty" {
        kind == "primitive" && path == "str" || kind == "library" && path == "std::string::String"
    } else {
        false
    }
}

/// Admit the corresponding standard Rust readonly source query.
#[must_use]
pub fn accept_rust_inclusive_range_operand(
    kind: &str,
    model: &str,
    proven: bool,
    layer: &str,
    borrowed: bool,
    shadowed: bool,
) -> bool {
    proven
        && !shadowed
        && kind == "float"
        && model == "binary64 scalar"
        && ((!borrowed && layer == "value") || (borrowed && layer == "reference"))
}

/// Admit the corresponding standard Rust readonly source query.
#[must_use]
pub fn accept_rust_string_empty_receiver(
    kind: &str,
    model: &str,
    proven: bool,
    immutable: bool,
    owner: bool,
) -> bool {
    proven
        && immutable
        && owner
        && kind == "string"
        && (model == "owned string" || model == "string slice")
}
