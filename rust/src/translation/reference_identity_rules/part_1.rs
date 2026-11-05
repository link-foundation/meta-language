// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_reference_identity_comparison(
    operation: &str,
    left_kind: &str,
    right_kind: &str,
    left_reference: bool,
    right_reference: bool,
    same_domain: bool,
) -> bool {
    ((((((operation == "eq") || (operation == "ne")) && left_reference) && right_reference)
        && same_domain)
        && (left_kind == right_kind))
        && ((((left_kind == "array") || (left_kind == "record")) || (left_kind == "map"))
            || (left_kind == "set"))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
#[expect(
    clippy::fn_params_excessive_bools,
    reason = "independent checked source facts retain their ordered Boolean interface"
)]
pub fn read_reference_identity_comparison_error(
    operation: &str,
    left_kind: &str,
    right_kind: &str,
    left_reference: bool,
    right_reference: bool,
    same_domain: bool,
    domain_present: bool,
    domain_reference: bool,
    same_operation_domain: bool,
) -> String {
    if ((!left_reference && !right_reference) && !domain_reference)
        || (((self::accept_reference_identity_comparison(
            operation,
            left_kind,
            right_kind,
            left_reference,
            right_reference,
            same_domain,
        ) && domain_present)
            && domain_reference)
            && same_operation_domain)
    {
        String::new()
    } else {
        String::from(
            "a linked reference comparison requires matching structural reference operands, an explicit identical reference domain, and equality or inequality",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_reference_identity_comparison(operation: &str, left: &str, right: &str) -> String {
    if operation == "eq" {
        format!(
            "std::rc::Rc::ptr_eq({}, {})",
            left.to_owned(),
            right.to_owned()
        )
    } else if operation == "ne" {
        format!(
            "(!std::rc::Rc::ptr_eq({}, {}))",
            left.to_owned(),
            right.to_owned()
        )
    } else {
        String::new()
    }
}
