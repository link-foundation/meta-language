// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub const fn require_self_translation_layout_envelope(
    carriage: bool,
    leading: bool,
    trailing: bool,
    gaps: bool,
) -> bool {
    ((carriage || leading) || trailing) || gaps
}
