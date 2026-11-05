// Generated from complete js/src/meta-language-archive-rules.js. Do not edit by hand.
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
pub fn read_meta_language_archive_language() -> String {
    String::from("meta-language complete graph archive")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_meta_language_archive_term(role: &str) -> String {
    if role == "root" {
        String::from("complete meta-language archive")
    } else if role == "group" {
        String::from("complete source network")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_meta_language_archive_record_fields(role: &str) -> String {
    if role == "descriptor" {
        String::from("schemaVersion|entries|groups")
    } else if role == "group" {
        String::from("identities|nextIdentity|terms|syntax")
    } else if role == "entry" {
        String::from("identifier|kind|group|root")
    } else if role == "term" {
        String::from("term|identity")
    } else if role == "syntax" {
        String::from("concept|language|value")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_meta_language_archive_kind(kind: &str) -> bool {
    (((((((((((((kind == "concrete syntax") || (kind == "abstract syntax"))
        || (kind == "grammar"))
        || (kind == "language configuration"))
        || (kind == "global concept space"))
        || (kind == "formal concept definition"))
        || (kind == "ontology"))
        || (kind == "taxonomy"))
        || (kind == "resource description graph"))
        || (kind == "resource description dataset"))
        || (kind == "typed program"))
        || (kind == "discovered concepts"))
        || (kind == "decorators"))
        || (kind == "grammar inference")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_meta_language_archive_version(version: f64) -> bool {
    version == 1f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_meta_language_archive_next_identity(next: f64, maximum: f64) -> bool {
    (self::ml_math::is_safe_integer(next) && (next >= 1f64)) && (next > maximum)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_meta_language_archive_entry_group(group: f64, count: f64) -> bool {
    (self::ml_math::is_safe_integer(group) && (group >= 0f64)) && (group < count)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_meta_language_archive_flags(
    error: bool,
    containing: bool,
    missing: bool,
) -> bool {
    !(error || missing) || containing
}
