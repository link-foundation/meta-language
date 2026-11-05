// Generated from complete js/src/translation/rust-standard-string-view-rules.js. Do not edit by hand.
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
pub fn accept_rust_standard_string_view_path(path: &str, ordinary: bool, shadowed: bool) -> bool {
    (ordinary && !shadowed) && (path == "std::string::String::as_str")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_string_view_layers(layers: f64) -> bool {
    (self::ml_math::is_integer(layers) && (layers >= 1f64)) && (layers <= 256f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_string_view_consumer(
    parameter: &str,
    layers: f64,
    known_slice: bool,
    static_lifetime: bool,
    result: &str,
    result_layers: f64,
    known_result: bool,
) -> bool {
    ((((((parameter == "string") && self::accept_rust_standard_string_view_layers(layers))
        && known_slice)
        && !static_lifetime)
        && (result_layers == 0f64))
        && known_result)
        && (((((result == "string") || (result == "fixed")) || (result == "float"))
            || (result == "bool"))
            || (result == "unit"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_string_view_receiver(
    kind: &str,
    model: &str,
    layers: f64,
    proven: bool,
    references: bool,
    known: bool,
    arguments_count: f64,
) -> bool {
    ((((((kind == "string") && (model == "owned string")) && (layers == 1f64)) && proven)
        && references)
        && known)
        && (arguments_count == 1f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_string_view_argument(
    sites: f64,
    root: bool,
    kind: &str,
    layers: f64,
    expected: f64,
    references: bool,
) -> bool {
    (((((sites == 1f64) && root) && (kind == "string"))
        && ((layers).partial_cmp(&(expected)) == Some(std::cmp::Ordering::Equal)))
        && self::accept_rust_standard_string_view_layers(layers))
        && references
}
