// Generated from complete js/src/translation/rust-exclusive-source-vector-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_exclusive_source_vector_method_arity(method: &str) -> f64 {
    if (method == "pop") || (method == "clear") {
        0f64
    } else if method == "truncate" {
        1f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_exclusive_source_vector_receiver(
    kind: &str,
    model: &str,
    layers: f64,
    proven: bool,
    owner: &str,
    path: &str,
    arguments_: f64,
) -> bool {
    ((((((kind == "array") && (model == "owned vector")) && (layers == 0f64)) && proven)
        && (owner == "library"))
        && (path == "std::vec::Vec"))
        && (arguments_ == 1f64)
}
