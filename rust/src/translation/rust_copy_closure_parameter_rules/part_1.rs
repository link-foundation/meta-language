// Generated from complete js/src/translation/rust-copy-closure-parameter-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_copy_closure_parameter_source(
    known: bool,
    base_copy: bool,
    layers: f64,
    reference: bool,
    primitive_owner: bool,
) -> bool {
    (known && base_copy)
        && ((layers == 0f64) || (((layers == 1f64) && reference) && primitive_owner))
}
