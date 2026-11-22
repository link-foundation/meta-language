// Generated from complete js/src/translation/rust-owned-standard-sum-predicate-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_standard_sum_predicate_result_error(role: &str, identity: bool) -> String {
    if ((role == "copy") || (role == "owned")) && identity {
        String::new()
    } else {
        String::from(
            "an eager default or first callback must establish its actual complete Copy or supported owned result type",
        )
    }
}
