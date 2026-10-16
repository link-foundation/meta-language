// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::*;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_partial_callable_result_inference(kind: &str, return_present: bool) -> bool {
    (kind == "function") && !return_present
}
