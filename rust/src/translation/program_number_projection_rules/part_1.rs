// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::*;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_program_derived_number_operand(kind: &str, operation: &str, field: &str) -> bool {
    ((kind == "binary") && ((field == "left") || (field == "right")))
        && (((((operation == "add") || (operation == "sub")) || (operation == "mul"))
            || (operation == "div"))
            || (operation == "rem"))
}
