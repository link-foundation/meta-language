// Generated from linked completion policy through the checked JavaScript translator.
// Do not edit by hand.
/// Read the shared closed Rust function completion policy.
#[must_use]
pub fn accept_rust_completion_model(model: &str) -> bool {
    model.is_empty() || model == "closed guarded function returns 1"
}

/// Read the shared closed Rust function completion policy.
#[must_use]
pub fn accept_rust_return_ownership(proven: bool, indirected: bool) -> bool {
    proven && !indirected
}

/// Read the shared closed Rust function completion policy.
#[must_use]
pub fn read_rust_conditional_completion(
    then_kind: &str,
    otherwise_kind: &str,
    has_else: bool,
    continuation: bool,
) -> String {
    if then_kind == "returned" && otherwise_kind.is_empty() && !has_else && continuation {
        String::from("guarded continuation")
    } else if then_kind == "returned" && otherwise_kind == "returned" && has_else && !continuation {
        String::from("returned conditional")
    } else if then_kind == "value" && otherwise_kind == "value" && has_else && !continuation {
        String::from("conditional value")
    } else if then_kind == "returned" && otherwise_kind == "returned" && has_else && continuation {
        String::from("unreachable continuation")
    } else if then_kind == "value" && otherwise_kind == "value" && has_else && continuation {
        String::from("discarded conditional value")
    } else {
        String::from("unsupported completion")
    }
}

/// Read the shared closed Rust function completion policy.
#[must_use]
pub fn accept_rust_completion_panic_arguments(
    model: &str,
    macro_name: &str,
    first_kind: &str,
    first_value: &str,
    second_kind: &str,
    count: &str,
) -> bool {
    model == "closed guarded function returns 1"
        && macro_name == "panic"
        && first_kind == "str"
        && first_value == "{}"
        && second_kind == "str"
        && count == "2"
}
