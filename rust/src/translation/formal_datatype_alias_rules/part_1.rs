// Generated from complete js/src/translation/formal-datatype-alias-rules.js. Do not edit by hand.
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
pub fn read_formal_datatype_alias_budget_error(rows: f64, units: f64) -> String {
    if ((((!(self::ml_math::is_safe_integer(rows)) || !(self::ml_math::is_safe_integer(units)))
        || (rows < 0f64))
        || (units < 0f64))
        || (rows > 8192f64))
        || (units > 16_777_216f64)
    {
        String::from("formal datatype public aliases exceed their finite target budget")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_formal_datatype_alias_traversal_error(visits: f64, depth: f64) -> String {
    if ((((!(self::ml_math::is_safe_integer(visits)) || !(self::ml_math::is_safe_integer(depth)))
        || (visits < 0f64))
        || (depth < 0f64))
        || (visits > 1_048_576f64))
        || (depth > 256f64)
    {
        String::from("formal datatype alias scope traversal exceeds its finite target budget")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_formal_datatype_type_alias(target: &str, name: &str, value: &str) -> String {
    if target == "Lean" {
        format!("abbrev {} := _root_.{}", name.to_owned(), value.to_owned())
    } else if target == "Rocq" {
        format!(
            "Definition {} : Type := {}.",
            name.to_owned(),
            value.to_owned()
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_formal_datatype_constructor_alias(
    target: &str,
    data: &str,
    name: &str,
    value: &str,
) -> String {
    if target == "Lean" {
        format!(
            "namespace {}\n@[match_pattern] abbrev {} := _root_.{}\nend {}",
            data.to_owned(),
            name.to_owned(),
            value.to_owned(),
            data.to_owned()
        )
    } else if target == "Rocq" {
        format!("Notation {} := {}.", name.to_owned(), value.to_owned())
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_formal_datatype_alias_module(target: &str, name: &str, open: bool) -> String {
    if target == "Lean" {
        format!(
            "{}{}",
            if open {
                String::from("namespace ")
            } else {
                String::from("end ")
            },
            name.to_owned()
        )
    } else if target == "Rocq" {
        format!(
            "{}{}.",
            if open {
                String::from("Module ")
            } else {
                String::from("End ")
            },
            name.to_owned()
        )
    } else {
        String::new()
    }
}
