// Generated from linked policy through the checked JavaScript translator.
// Do not edit by hand.
/// Read the shared Rust binary64 predicate admission.
#[must_use]
pub fn accept_rust_float_predicate_method(method: &str) -> bool {
    method == "is_nan" || method == "is_finite" || method == "is_infinite"
}

/// Read the shared Rust binary64 operation admission.
#[must_use]
pub fn accept_rust_float_operation_method(method: &str) -> bool {
    method == "abs" || method == "floor" || method == "ceil" || method == "trunc"
}

/// Read the shared Rust binary64 predicate admission.
#[must_use]
pub fn accept_rust_float_predicate_receiver(
    kind: &str,
    model: &str,
    proven: bool,
    indirected: bool,
    shadowed: bool,
    arguments_count: &str,
) -> bool {
    proven
        && !indirected
        && !shadowed
        && kind == "float"
        && model == "binary64 scalar"
        && arguments_count == "0"
}
