// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_program_string_representation_flag(kind: &str, present: bool) -> bool {
    !present || (kind == "boolean")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn select_program_string_representation(
    program: bool,
    explicit: bool,
    required: bool,
) -> bool {
    (program || explicit) || required
}
