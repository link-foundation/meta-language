// Experimental automatic translation; not a native runtime integration.
/// Experimental generated owned literal-edit decision.
#[must_use]
pub fn accept_generated_owned_rust_value_edit_model(model: &str) -> bool {
    model == "closed generated owned Rust literal value edits 1"
}

/// Experimental generated owned literal-edit decision.
#[must_use]
pub fn accept_generated_owned_rust_literal_edit_owner(kind: &str, operation: &str) -> bool {
    ((kind == "string") && (operation == "owned standard String literal construction"))
        || ((kind == "fixed") && (operation == "owned signed i32 literal"))
}

/// Experimental generated owned literal-edit decision.
#[must_use]
pub fn accept_generated_owned_rust_sign_owner(negative: bool, owned: bool) -> bool {
    negative == owned
}
