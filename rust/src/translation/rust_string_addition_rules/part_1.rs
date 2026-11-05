// Generated from complete js/src/translation/rust-string-addition-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_string_addition_candidate(
    operation: &str,
    ordinary: bool,
    left: &str,
    right: &str,
) -> bool {
    ((operation == "add") && ordinary) && ((left == "string") || (right == "string"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_string_addition_owner(
    side: &str,
    kind: &str,
    path: &str,
    children: f64,
) -> bool {
    (children == 0f64)
        && (((kind == "library") && (path == "std::string::String"))
            || (((side == "right") && (kind == "primitive")) && (path == "str")))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_string_addition_operand(
    side: &str,
    kind: &str,
    model: &str,
    layers: f64,
    proven: bool,
    references: bool,
    known: bool,
) -> bool {
    ((((kind == "string") && proven) && references) && known)
        && ((((side == "left") && (model == "owned string")) && (layers == 0f64))
            || (((side == "right") && ((model == "owned string") || (model == "string slice")))
                && (layers == 1f64)))
}
