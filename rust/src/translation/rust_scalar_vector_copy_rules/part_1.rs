// Generated from complete js/src/translation/rust-scalar-vector-copy-rules.js. Do not edit by hand.
/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    #[must_use]
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if (-0.5..0.0).contains(&x) {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 { floor + 1.0 } else { floor }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where `f64::signum` does not.
    #[must_use]
    pub fn sign(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            x
        }
    }

    /// Math.max of two Numbers: NaN when either is, and 0 above -0.
    #[must_use]
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else if b > a {
            b
        } else if a == b {
            if a.is_sign_negative() { b } else { a }
        } else {
            f64::NAN
        }
    }

    /// Math.min of two Numbers: NaN when either is, and -0 below 0.
    #[must_use]
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else if b < a {
            b
        } else if a == b {
            if a.is_sign_negative() { a } else { b }
        } else {
            f64::NAN
        }
    }

    #[must_use]
    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    #[must_use]
    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    #[must_use]
    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    #[must_use]
    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9_007_199_254_740_991.0
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_vector_copy_query(
    ordinary: bool,
    operation: &str,
    model: &str,
    layers: f64,
) -> bool {
    if ((!ordinary || !(self::ml_math::is_integer(layers))) || (layers < 0f64)) || (layers > 256f64)
    {
        false
    } else if operation == "to_vec" {
        (model == "owned vector") || ((model == "immutable slice") && (layers > 0f64))
    } else {
        (((operation == "clone") || (operation == "to_owned")) && (model == "owned vector"))
            && (layers == 0f64)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_vector_copy_receiver(
    kind: &str,
    element: &str,
    proven: bool,
    references: bool,
    copy: bool,
    arguments_count: f64,
) -> bool {
    (((((kind == "array") && proven) && references) && copy) && (arguments_count == 0f64))
        && ((((element == "fixed") || (element == "float")) || (element == "bool"))
            || (element == "unit"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_vector_copy_owner(
    kind: &str,
    path: &str,
    paths: f64,
    children: f64,
) -> bool {
    (children == 1f64)
        && (((kind == "slice") && (paths == 0f64))
            || (((kind == "library") && (path == "std::vec::Vec")) && (paths == 3f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_vector_copy_operation() -> String {
    String::from("slice")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_vector_copy_occurrence(
    kind: &str,
    model: &str,
    wrappers: f64,
    recorded: bool,
) -> bool {
    ((recorded && (kind == "method")) && (model == "owned vector")) && (wrappers == 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_scalar_vector_copy_value(
    language: &str,
    kind: &str,
    operation: &str,
    receiver: &str,
    element: &str,
    reference: bool,
    count: f64,
    start_kind: &str,
    start: &str,
    end_kind: &str,
    end: &str,
) -> bool {
    ((((((((((language == "Rust") && (kind == "method")) && (operation == "slice"))
        && (receiver == "array"))
        && !reference)
        && (count == 2f64))
        && (start_kind == "float"))
        && ((start == "0") || (start == "0.0")))
        && (end_kind == "float"))
        && (end == "Infinity"))
        && ((((element == "fixed") || (element == "float")) || (element == "bool"))
            || (element == "unit"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_scalar_vector_copy_value(target: &str, object: &str) -> String {
    if target == "Rust" {
        format!("::std::clone::Clone::clone({})", object.to_owned())
    } else if target == "JavaScript" {
        format!("[...({})]", object.to_owned())
    } else if (target == "Lean") || (target == "Rocq") {
        format!("({})", object.to_owned())
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_vector_copy_encoding() -> String {
    String::from("standard-scalar-vector-copy")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_scalar_vector_copy_statement() -> String {
    String::from(
        "standard dense scalar vector copies evaluate one receiver and copy every element; Rust uses the actual Clone operation without floating-point length conversion, JavaScript creates a fresh array and proof targets retain immutable list values",
    )
}
