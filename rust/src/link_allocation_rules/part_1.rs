// Generated from complete js/src/link-allocation-rules.js. Do not edit by hand.
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
pub fn accept_link_allocation_word(value: f64) -> bool {
    (self::ml_math::is_safe_integer(value) && (value >= 0f64)) && (value <= 65_535f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_link_allocation_capacity(
    first: f64,
    second: f64,
    third: f64,
    fourth: f64,
    exhausted: bool,
    count: f64,
) -> bool {
    if (((((!(self::accept_link_allocation_word(first))
        || !(self::accept_link_allocation_word(second)))
        || !(self::accept_link_allocation_word(third)))
        || !(self::accept_link_allocation_word(fourth)))
        || !(self::ml_math::is_safe_integer(count)))
        || (count < 0f64))
        || (count > 4_294_967_295f64)
    {
        false
    } else {
        {
            let zero =
                (((first == 0f64) && (second == 0f64)) && (third == 0f64)) && (fourth == 0f64);
            if exhausted {
                zero && (count == 0f64)
            } else if zero {
                false
            } else if (third < 65_535f64) || (fourth < 65_535f64) {
                true
            } else {
                {
                    let available = (((65_535f64 - second) * 65_536f64) + 65_536f64) - first;
                    (count <= available)
                }
            }
        }
    }
}
