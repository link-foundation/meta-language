// Generated from complete js/src/translation/rust-fresh-vector-argument-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_fresh_vector_argument_form(
    kind: &str,
    diverging: bool,
    model: &str,
    wrappers: f64,
    constructor: bool,
) -> String {
    if diverging {
        String::from("diverging")
    } else if (((kind == "array") && (model == "owned vector")) && (wrappers == 0f64))
        && constructor
    {
        String::from("constructor")
    } else if (kind == "let") || (kind == "print") {
        String::from("body")
    } else if kind == "if" {
        String::from("alternatives")
    } else if kind == "match" {
        String::from("rows")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_fresh_owned_vector_argument(
    ordinary: bool,
    model: &str,
    wrappers: f64,
    element: &str,
    fresh: bool,
) -> bool {
    (((ordinary && (model == "owned vector")) && (wrappers == 0f64)) && fresh)
        && ((((element == "bool") || (element == "float")) || (element == "fixed"))
            || (element == "unit"))
}
