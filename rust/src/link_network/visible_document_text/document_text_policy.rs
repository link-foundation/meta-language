// Generated from js/src/document-text-policy.js.
pub(super) fn document_scalar_is_whitespace(value: f64) -> bool {
    value == 9.0 || value == 10.0 || value == 12.0 || value == 13.0 || value == 32.0
}

pub(super) fn document_scalar_is_valid(value: f64) -> bool {
    value > 0.0 && value <= 1_114_111.0 && !(55_296.0..=57_343.0).contains(&value)
}
