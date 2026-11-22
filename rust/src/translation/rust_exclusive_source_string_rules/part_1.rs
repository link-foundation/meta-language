// Generated from complete js/src/translation/rust-exclusive-source-string-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_string_addition_candidate(
    operation: &str,
    ordinary: bool,
    left: &str,
    right: &str,
) -> bool {
    ((operation == "add") && ordinary) && ((left == "string") || (right == "string"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_string_addition_owner(
    side: &str,
    kind: &str,
    path: &str,
    children: f64,
) -> bool {
    (children == 0f64)
        && (((kind == "library") && (path == "std::string::String"))
            || (((side == "right") && (kind == "primitive")) && (path == "str")))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_string_addition_operand(
    side: &str,
    kind: &str,
    model: &str,
    layers: f64,
    proven: bool,
    references: bool,
    known: bool,
) -> bool {
    ((((kind == "string") && proven) && references) && known)
        && ((((side == "left") && (model == "owned string")) && (layers == 0f64))
            || (((side == "right") && ((model == "owned string") || (model == "string slice")))
                && (layers == 1f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_exclusive_source_string_method_arity(method: &str) -> f64 {
    if method == "push_str" {
        1f64
    } else if method == "clear" {
        0f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_exclusive_source_string_receiver(
    kind: &str,
    model: &str,
    layers: f64,
    proven: bool,
    owner: &str,
    path: &str,
    children: f64,
) -> bool {
    ((((kind == "string") && (model == "owned string")) && (layers == 0f64)) && proven)
        && self::accept_rust_string_addition_owner("left", owner, path, children)
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_exclusive_source_string_suffix(
    kind: &str,
    model: &str,
    layers: f64,
    proven: bool,
    references: bool,
    owner: &str,
    path: &str,
    children: f64,
) -> bool {
    self::accept_rust_string_addition_operand(
        "right",
        kind,
        model,
        layers,
        proven,
        references,
        self::accept_rust_string_addition_owner("right", owner, path, children),
    )
}
