// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_array_owner(owner: &str, shadowed: bool) -> bool {
    !shadowed && ((owner == "Vec") || (owner == "std::vec::Vec"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_array_element(kind: &str, indirected: bool) -> bool {
    !indirected
        && (((((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "unit"))
            || (kind == "string"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_array_coercion(actual: &str, expected: &str, borrowed: bool) -> bool {
    (actual == expected)
        || ((borrowed && (actual == "owned vector")) && (expected == "immutable slice"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_array_result_model(model: &str) -> bool {
    ((model).is_empty()) || (model == "immutable fresh scalar vector results 1")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_array_result(model: &str, kind: &str, owned: bool, fresh: bool) -> bool {
    (((model == "immutable fresh scalar vector results 1") && owned) && fresh)
        && ((((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "unit"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_fresh_array_result_tail(kind: &str, constructor: bool, children: bool) -> bool {
    ((kind == "array") && constructor) || (((kind == "let") || (kind == "if")) && children)
}

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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_shared_array_handle_source_model(model: &str) -> bool {
    ((model).is_empty()) || (model == "standard Rust shared array identity handles 1")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_shared_array_handle_source_size(units: f64) -> bool {
    ((0f64..=65_536f64).contains(&units)) && ((units % 1f64) == 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_shared_array_handle_layer(role: &str, owner: &str, shadowed: bool) -> bool {
    !shadowed
        && (((role == "handle") && (owner == "std::rc::Rc"))
            || ((role == "cell") && (owner == "std::cell::RefCell")))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_shared_array_handle_payload(kind: &str, indirected: bool) -> bool {
    self::accept_rust_source_array_element(kind, indirected)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_shared_array_handle_transfer(
    actual: &str,
    expected: &str,
    actual_layers: f64,
    expected_layers: f64,
) -> bool {
    (((actual == "standard shared array handle") && (expected == actual))
        && (actual_layers == 0f64))
        && (expected_layers == 0f64)
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
pub fn read_rust_shared_array_handle_source_form(
    operation: &str,
    arity: f64,
    owner: bool,
    left_handle: bool,
    right_handle: bool,
    borrowed: bool,
    same_domain: bool,
    fresh_literal: bool,
) -> String {
    if owner {
        if ((operation == "allocate") && (arity == 1f64)) && fresh_literal {
            String::from("allocation")
        } else if (((operation == "clone") && (arity == 0f64)) && left_handle) && !borrowed {
            String::from("alias")
        } else if (((operation == "pointer equality") && (arity == 2f64)) && borrowed)
            && self::accept_reference_identity_comparison(
                "eq",
                "array",
                "array",
                left_handle,
                right_handle,
                same_domain,
            )
        {
            String::from("identity equality")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_shared_array_handle_value_observation(
    left_handle: bool,
    right_handle: bool,
) -> bool {
    !left_handle && !right_handle
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_shared_array_handle_comparison_projection(
    language: &str,
    operation: &str,
    kind: &str,
    reference: bool,
) -> String {
    if (((language == "Rust") && (kind == "array")) && reference)
        && ((operation == "eq") || (operation == "ne"))
    {
        String::from("referenceIdentity")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_shared_array_handle_source_interpretation(model: &str) -> bool {
    self::accept_rust_shared_array_handle_source_model(model)
        || (model == "standard Rust shared array identity handles 2")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_shared_array_handle_borrowed_clone(
    model: &str,
    arity: f64,
    owner: bool,
    handle: bool,
    borrowed: bool,
) -> bool {
    ((((model == "standard Rust shared array identity handles 2") && (arity == 1f64)) && owner)
        && handle)
        && borrowed
}
