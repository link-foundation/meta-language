// Generated from complete js/src/translation/rust-standard-sum-guard-rules.js. Do not edit by hand.
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
pub fn read_rust_standard_sum_guard_mode(owned: bool, guarded: bool) -> String {
    if owned && guarded {
        String::from("deferred guard")
    } else {
        String::from("ordinary")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_deferred_guard_pattern_error(
    alternatives: f64,
    aliases: f64,
) -> String {
    if alternatives == 1f64 {
        if aliases == 0f64 {
            String::new()
        } else {
            String::from(
                "guarded owned sum whole-value aliases need their delayed payload move model",
            )
        }
    } else {
        String::from("guarded owned sum alternatives need their delayed move agreement model")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_guard_alternative_pattern_error(
    alternatives: f64,
    aliases: f64,
) -> String {
    if ((!(self::ml_math::is_integer(alternatives)) || (alternatives < 1f64))
        || !(self::ml_math::is_integer(aliases)))
        || (aliases < 0f64)
    {
        String::from(
            "guarded owned sum patterns need their actual finite alternative and alias counts",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_guard_alternative_binding_error(
    complete: bool,
    same_type: bool,
    same_place: bool,
) -> String {
    if !complete || !same_type {
        String::from(
            "guarded owned sum alternatives must bind the same complete actual source types",
        )
    } else if same_place {
        String::new()
    } else {
        String::from(
            "guarded owned sum alternatives must retain the same actual root loans and lifetime provenance",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_guard_alias_transfer_error(
    aliases: f64,
    owned_bindings: f64,
) -> String {
    if (aliases > 0f64) && (owned_bindings > 1f64) {
        String::from(
            "guarded owned sum whole-value aliases with owned field bindings need their partial-move agreement model",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_guard_binding_disposition(deferred: bool, owned: bool) -> String {
    if deferred {
        String::from("readonly")
    } else if owned {
        String::from("transfer")
    } else {
        String::from("plain")
    }
}
