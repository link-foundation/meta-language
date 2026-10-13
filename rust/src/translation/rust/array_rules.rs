// Generated from rust-array-rules.js by the checked translator.
// Do not edit by hand.
/// Admit the selected immutable Rust array source model.
#[must_use]
pub fn accept_rust_source_array_owner(owner: &str, shadowed: bool) -> bool {
    !shadowed && (owner == "Vec" || owner == "std::vec::Vec")
}

/// Admit the selected immutable Rust array source model.
#[must_use]
pub fn accept_rust_source_array_element(kind: &str, indirected: bool) -> bool {
    !indirected
        && (kind == "bool"
            || kind == "float"
            || kind == "fixed"
            || kind == "unit"
            || kind == "string")
}

/// Admit the selected immutable Rust array source model.
#[must_use]
pub fn accept_rust_source_array_coercion(actual: &str, expected: &str, borrowed: bool) -> bool {
    actual == expected || (borrowed && actual == "owned vector" && expected == "immutable slice")
}

/// Admit the selected immutable Rust array source model.
#[must_use]
pub fn accept_rust_source_array_result_model(model: &str) -> bool {
    model.is_empty() || model == "immutable fresh scalar vector results 1"
}

/// Admit the selected immutable Rust array source model.
#[must_use]
pub fn accept_rust_source_array_result(model: &str, kind: &str, owned: bool, fresh: bool) -> bool {
    model == "immutable fresh scalar vector results 1"
        && owned
        && fresh
        && (kind == "bool" || kind == "float" || kind == "fixed" || kind == "unit")
}

/// Admit the selected immutable Rust array source model.
#[must_use]
pub fn accept_rust_fresh_array_result_tail(kind: &str, constructor: bool, children: bool) -> bool {
    (kind == "array" && constructor) || ((kind == "let" || kind == "if") && children)
}
