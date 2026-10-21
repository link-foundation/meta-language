// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_boolean_declaration_value_context(
    language: &str,
    original_const_proof: bool,
) -> bool {
    ((language == "JavaScript") && !original_const_proof)
        || ((language == "Rust") && original_const_proof)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_boolean_declaration_value_entry(kind: &str, nested: bool, generated: bool) -> bool {
    ((kind == "fn") && !nested) && !generated
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_boolean_declaration_value_selector(
    single_selector: bool,
    owned_declaration: bool,
) -> bool {
    single_selector && owned_declaration
}
