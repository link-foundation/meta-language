// Experimental automatic translation; not a native runtime integration.
/// Experimental generated whole owned source decision.
#[must_use]
pub fn accept_generated_owned_rust_model(model: &str) -> bool {
    model == "closed generated owned Rust module 1"
}

/// Experimental generated whole owned source decision.
#[must_use]
pub fn accept_generated_owned_rust_clone(
    standard: bool,
    string: bool,
    receiver: bool,
    arity: f64,
) -> bool {
    ((standard && string) && receiver) && (arity == (0f64))
}

/// Experimental generated whole owned source decision.
#[must_use]
pub fn accept_generated_owned_rust_code_unit(value: f64, negative_zero: bool) -> bool {
    ((!negative_zero && (value >= (0f64))) && (value <= (65535f64))) && ((value % 1f64) == (0f64))
}

/// Experimental generated whole owned source decision.
#[must_use]
pub fn accept_generated_owned_rust_literal_code_point(value: f64) -> bool {
    (((value >= (32f64)) || (value == (9f64))) || (value == (10f64))) || (value == (13f64))
}

/// Experimental generated whole owned source decision.
#[must_use]
pub fn accept_generated_owned_rust_equality_grouping(
    comparison: bool,
    parenthesized: bool,
) -> bool {
    !comparison || parenthesized
}

/// Experimental generated whole owned source decision.
#[must_use]
pub fn accept_generated_owned_rust_ordinary_comment(line: bool, third: f64, fourth: f64) -> bool {
    (third != (33f64))
        && if line {
            ((third != (47f64)) || (fourth == (47f64)))
        } else {
            (((third != (42f64)) || (fourth == (42f64))) || (fourth == (47f64)))
        }
}
