// Generated from complete js/src/translation/rust-function-qualifier-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_const_function_prefix(
    ordinary: bool,
    predicate: bool,
    constant: bool,
    function_keyword: bool,
) -> bool {
    ((ordinary || predicate) && constant) && function_keyword
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn require_rust_const_predicate_proof(constant: bool, predicate: bool) -> bool {
    constant && predicate
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_const_function_location(module_depth: f64, inherent: bool, name: &str) -> bool {
    ((module_depth > 0f64) || inherent) || (name != "main")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_associated_constant_item(
    ordinary: bool,
    constant: bool,
    next_function: bool,
) -> bool {
    (ordinary && constant) && !next_function
}
