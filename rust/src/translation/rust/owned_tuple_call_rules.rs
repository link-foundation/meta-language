// Generated through the checked JavaScript translator. Do not edit.
/// Apply a shared owned tuple initialization call decision.
#[must_use]
pub fn accept_rust_owned_tuple_call_model(model: &str) -> bool {
    model == "closed owned text tuple initialization calls 1"
}

/// Apply a shared owned tuple initialization call decision.
#[must_use]
pub const fn accept_rust_owned_tuple_call_initializer(
    target: bool,
    arguments_match: bool,
    fields_match: bool,
) -> bool {
    (target && arguments_match) && fields_match
}
