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
