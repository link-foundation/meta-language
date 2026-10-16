// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_const_branch_model(model: &str) -> bool {
    model == "closed Boolean Rust const decision trees 1"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_const_branch_type(kind: &str) -> bool {
    kind == "bool"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_const_branch_kind(kind: &str) -> bool {
    ((((kind == "lit") || (kind == "var")) || (kind == "unary")) || (kind == "binary"))
        || (kind == "if")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_const_branch_operation(kind: &str, operation: &str) -> bool {
    ((kind == "unary") && (operation == "not"))
        || ((kind == "binary") && ((operation == "and") || (operation == "or")))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_const_branch_conditional(
    result: &str,
    condition: &str,
    consequent: &str,
    alternative: &str,
) -> bool {
    (((result == "bool") && (condition == "bool")) && (consequent == "bool"))
        && (alternative == "bool")
}
