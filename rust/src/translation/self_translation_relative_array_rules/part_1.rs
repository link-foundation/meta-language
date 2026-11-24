// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_self_translation_relative_array_call(
    receiver_operator: &str,
    member: &str,
    call_operator: &str,
) -> bool {
    ((receiver_operator == ".") && (member == "at")) && (call_operator == "(")
}
