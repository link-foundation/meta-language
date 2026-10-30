// Generated from complete js/src/translation/rust-source-owner-retention-rules.js. Do not edit by hand.
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
pub fn accept_rust_retained_source_signature_model(model: &str) -> bool {
    model == "Rust source function signature 2"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn select_rust_retained_source_signature_model(model: &str, retain: bool) -> String {
    if !(model).is_empty() {
        model.to_owned()
    } else if retain {
        String::from("Rust source function signature 2")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_retained_function_owner(
    model: &str,
    owner: &str,
    type_: &str,
    paths: f64,
    children: f64,
    parameters: f64,
    reference: bool,
) -> bool {
    (((((((self::accept_rust_retained_source_signature_model(model) && (owner == "function"))
        && (type_ == "function"))
        && (paths == 0f64))
        && self::ml_math::is_integer(parameters))
        && (parameters >= 0f64))
        && (parameters <= 100_000f64))
        && (children == (parameters + 1f64)))
        && !reference
}
