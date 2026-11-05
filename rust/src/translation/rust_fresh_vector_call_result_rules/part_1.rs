// Generated from complete js/src/translation/rust-fresh-vector-call-result-rules.js. Do not edit by hand.
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_ordinary_fresh_vector_result(
    ordinary: bool,
    model: &str,
    wrappers: f64,
    element: &str,
    fresh: bool,
) -> bool {
    self::accept_rust_fresh_owned_vector_argument(ordinary, model, wrappers, element, fresh)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_known_fresh_vector_call(
    kind: &str,
    model: &str,
    wrappers: f64,
    checked: bool,
) -> bool {
    ((checked && ((kind == "app") || (kind == "name"))) && (model == "owned vector"))
        && (wrappers == 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_fresh_vector_result_declaration(
    ordinary: bool,
    model: &str,
    wrappers: f64,
    element: &str,
    element_copy: bool,
) -> bool {
    element_copy
        && self::accept_rust_ordinary_fresh_vector_result(ordinary, model, wrappers, element, true)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_fresh_vector_result_call(
    kind: &str,
    model: &str,
    wrappers: f64,
    constructed: bool,
    declared: bool,
    called: bool,
) -> bool {
    called
        && self::accept_rust_known_fresh_vector_call(
            kind,
            model,
            wrappers,
            (constructed || declared),
        )
}
