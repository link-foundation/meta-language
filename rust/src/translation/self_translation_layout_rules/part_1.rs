// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub const fn require_self_translation_layout_envelope(
    ml_boolean_parameters: (bool, bool, bool, bool),
) -> bool {
    let (carriage, leading, trailing, gaps) = ml_boolean_parameters;
    ((carriage || leading) || trailing) || gaps
}
