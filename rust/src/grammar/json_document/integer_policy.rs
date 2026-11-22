// Generated from js/src/grammar-importers/json-schema-integer-policy.js.
pub(super) fn exclusive_integer_boundary(previous: f64, next: f64) -> f64 {
    if previous == 1.0 && next != 1.0 {
        -1.0
    } else if previous != 1.0 && next == 1.0 {
        1.0
    } else {
        0.0
    }
}
