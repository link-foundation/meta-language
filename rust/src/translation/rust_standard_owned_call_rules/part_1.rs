// Generated from complete js/src/translation/rust-standard-owned-call-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_owned_call_role(path: &str, ordinary: bool, shadowed: bool) -> String {
    if !ordinary || shadowed {
        String::new()
    } else if path == "std::clone::Clone::clone" {
        String::from("clone")
    } else if path == "std::borrow::ToOwned::to_owned" {
        String::from("to_owned")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_owned_text_call(
    role: &str,
    kind: &str,
    model: &str,
    layers: f64,
    proven: bool,
    references: bool,
    known: bool,
) -> bool {
    (((((kind == "string") && (layers == 1f64)) && proven) && references) && known)
        && (((role == "clone") && (model == "owned string"))
            || ((role == "to_owned") && ((model == "owned string") || (model == "string slice"))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_owned_array_call(
    role: &str,
    model: &str,
    layers: f64,
    known: bool,
) -> bool {
    ((layers == 1f64) && known)
        && (((model == "owned vector") && ((role == "clone") || (role == "to_owned")))
            || ((model == "immutable slice") && (role == "to_owned")))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_owned_scalar_call(
    role: &str,
    kind: &str,
    layers: f64,
    proven: bool,
    references: bool,
    known: bool,
) -> bool {
    ((((((role == "clone") || (role == "to_owned"))
        && ((((kind == "fixed") || (kind == "float")) || (kind == "bool")) || (kind == "unit")))
        && (layers == 1f64))
        && proven)
        && references)
        && known
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_standard_string_owned_method_result(
    method: &str,
    kind: &str,
    model: &str,
    layers: f64,
    proven: bool,
    references: bool,
    owner: &str,
    path: &str,
    children: f64,
) -> bool {
    ((((kind == "string") && proven) && references) && (children == 0f64))
        && ((((((method == "to_owned") && (model == "string slice")) && (layers == 1f64))
            && (owner == "primitive"))
            && (path == "str"))
            || ((((((method == "clone") || (method == "to_owned")) && (model == "owned string"))
                && ((layers == 0f64) || (layers == 1f64)))
                && (owner == "library"))
                && (path == "std::string::String")))
}
