// Generated through the checked JavaScript translator. Do not edit.
/// Apply the shared resolved type namespace policy.
#[must_use]
pub fn accept_rust_type_namespace_model(model: &str) -> bool {
    (model.is_empty()) || model == "resolved Rust type namespace 1"
}

/// Apply the shared resolved type namespace policy.
#[must_use]
pub fn accept_rust_type_namespace_declaration(kind: &str) -> bool {
    kind == "data"
}

/// Apply the shared resolved type namespace policy.
#[must_use]
pub const fn accept_rust_type_namespace_fields(invalid_fields: bool) -> bool {
    !invalid_fields
}

/// Apply the shared resolved type namespace policy.
#[must_use]
pub const fn accept_rust_type_namespace_anchor(above_crate: bool) -> bool {
    !above_crate
}

/// Apply the shared resolved type namespace policy.
#[must_use]
pub fn accept_rust_type_namespace_position(
    kind: &str,
    invalid_wrappers: bool,
    borrowed_result: bool,
) -> bool {
    (kind == "primitive" || kind == "declaration") && !invalid_wrappers && !borrowed_result
}
