// Generated from complete js/src/translation/rust-primitive-expression-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_primitive_boolean_result(
    form: &str,
    operation: &str,
    boolean_operand: bool,
) -> bool {
    ((form == "comparison") || (form == "boolean-operation"))
        || (((form == "primitive-operation") && (operation == "not")) && boolean_operand)
}
