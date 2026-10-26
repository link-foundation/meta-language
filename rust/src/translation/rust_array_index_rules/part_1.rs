// Generated from complete js/src/translation/rust-array-index-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_array_index_element(kind: &str) -> bool {
    (((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "unit")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_array_index_receiver_error(
    kind: &str,
    element: &str,
    model: &str,
    proven: bool,
    layers: bool,
    inferred_copy: bool,
) -> String {
    if ((proven && layers) && ((model == "owned vector") || (model == "immutable slice")))
        && (((kind == "array") && self::accept_rust_scalar_array_index_element(element))
            || (((kind).is_empty()) && inferred_copy))
    {
        String::new()
    } else {
        String::from(
            "the receiver needs a proven standard vector or slice of direct scalar Copy elements",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_array_index_operand_error(
    literal: bool,
    kind: &str,
    bits: f64,
    signed: bool,
    owner: &str,
    wrappers: f64,
    proven: bool,
) -> String {
    if literal
        || (((((proven && (kind == "fixed")) && (bits == 64f64)) && !signed) && (owner == "usize"))
            && (wrappers == 0f64))
    {
        String::new()
    } else {
        String::from(
            "the index needs an unsuffixed integer literal or a proven direct usize source owner",
        )
    }
}
