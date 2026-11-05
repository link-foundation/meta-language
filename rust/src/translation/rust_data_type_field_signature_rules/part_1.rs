// Generated from complete js/src/translation/rust-data-type-field-signature-rules.js. Do not edit by hand.
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
pub fn accept_rust_data_type_field_signature(
    model: &str,
    language: &str,
    declared: &str,
    actual: &str,
    constructors: f64,
    expected: f64,
) -> bool {
    (((((model == "Rust data type field signatures 1") && (language == "Rust"))
        && (declared == actual))
        && ((constructors).partial_cmp(&(expected)) == Some(std::cmp::Ordering::Equal)))
        && self::accept_rust_data_type_field_signature_budget(constructors, 0f64))
        && self::accept_rust_data_type_field_signature_budget(expected, 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_constructor_field_signature(
    name: &str,
    actual: &str,
    fields: f64,
    expected: f64,
) -> bool {
    (((name == actual) && ((fields).partial_cmp(&(expected)) == Some(std::cmp::Ordering::Equal)))
        && self::accept_rust_data_type_field_signature_budget(fields, 0f64))
        && self::accept_rust_data_type_field_signature_budget(expected, 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_data_type_field_signature_budget(visits: f64, depth: f64) -> bool {
    ((((self::ml_math::is_integer(visits) && (visits >= 0f64)) && (visits <= 100_000f64))
        && self::ml_math::is_integer(depth))
        && (depth >= 0f64))
        && (depth <= 256f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_data_type_field_signature_container(
    kind: &str,
    dense: bool,
    unrepresented: bool,
) -> bool {
    !unrepresented && ((kind == "record") || ((kind == "array") && dense))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_data_type_field_signature_program_container(
    kind: &str,
    dense: bool,
    unrepresented: bool,
) -> bool {
    if kind == "map" {
        !unrepresented
    } else {
        self::accept_rust_data_type_field_signature_container(kind, dense, unrepresented)
    }
}
