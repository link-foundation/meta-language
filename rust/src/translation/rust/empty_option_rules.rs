// Generated through the checked JavaScript translator. Do not edit.
/// Read a shared empty primitive option admission decision.
#[must_use]
pub fn accept_rust_empty_option_source_model(model: &str) -> bool {
    model.is_empty() || model == "closed empty primitive Rust option 1"
}

/// Read a shared empty primitive option admission decision.
#[must_use]
pub fn accept_rust_empty_option_result_owner(path: &str, shadowed: bool) -> bool {
    !shadowed
        && ((path == "Option" || path == "std::option::Option") || path == "core::option::Option")
}

/// Read a shared empty primitive option admission decision.
#[must_use]
pub fn accept_rust_empty_option_constructor_owner(path: &str, shadowed: bool) -> bool {
    !shadowed
        && (((path == "None" || path == "Option::None") || path == "std::option::Option::None")
            || path == "core::option::Option::None")
}

/// Read a shared empty primitive option admission decision.
#[must_use]
pub fn accept_rust_empty_option_element(kind: &str, owner: &str, indirect: bool) -> bool {
    (kind == "float" && owner == "f64") && !indirect
}

/// Read a shared empty primitive option admission decision.
#[must_use]
pub fn accept_rust_empty_option_context(language: &str, forbidden: bool) -> bool {
    language == "Rust" && !forbidden
}

/// Read a shared empty primitive option admission decision.
#[must_use]
pub fn accept_rust_empty_option_function(kind: &str, invalid: bool) -> bool {
    kind == "fn" && !invalid
}

/// Read a shared empty primitive option admission decision.
#[must_use]
pub fn accept_rust_empty_option_expression(kind: &str, absence: &str) -> bool {
    kind == "none" && absence == "undefined"
}
