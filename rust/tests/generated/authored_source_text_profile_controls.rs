// Translated from JavaScript by meta-language: portable core, Rust target.
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if x < 0.0 && x >= -0.5 {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 {
            floor + 1.0
        } else {
            floor
        }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where f64::signum does not.
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
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else if b > a {
            b
        } else if a == b {
            if a.is_sign_negative() {
                b
            } else {
                a
            }
        } else {
            f64::NAN
        }
    }

    /// Math.min of two Numbers: NaN when either is, and -0 below 0.
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else if b < a {
            b
        } else if a == b {
            if a.is_sign_negative() {
                a
            } else {
                b
            }
        } else {
            f64::NAN
        }
    }

    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9007199254740991.0
    }
}

pub fn read_java_script_source_text_profile_error(profile: String, length: f64) -> String {
    if (profile != "inspectable UTF16 source text 1") {
        String::from("source text requires its explicit inspectable UTF16 resource profile")
    } else {
        if (((!(self::ml_math::is_safe_integer(length)) || (length < (0f64))) || (length > (16777216f64))) || ((length == (0f64)) && (((1f64 / length)) < (0f64)))) {
            String::from("source text exceeds the explicit 16777216 UTF16 unit budget")
        } else {
            String::from("")
        }
    }
}

fn ml_main() {
    assert!(((self::read_java_script_source_text_profile_error(String::from("inspectable UTF16 source text 1"), 0f64)) == ""), "assertion 1");
    assert!(((self::read_java_script_source_text_profile_error(String::from("inspectable UTF16 source text 1"), 8388608f64)) == ""), "assertion 2");
    assert!(((self::read_java_script_source_text_profile_error(String::from("inspectable UTF16 source text 1"), 16777216f64)) == ""), "assertion 3");
    assert!(((self::read_java_script_source_text_profile_error(String::from("inspectable UTF16 source text 1"), 16777217f64)) == "source text exceeds the explicit 16777216 UTF16 unit budget"), "assertion 4");
    assert!(((self::read_java_script_source_text_profile_error(String::from("opaque host string"), 8f64)) == "source text requires its explicit inspectable UTF16 resource profile"), "assertion 5");
    assert!(((self::read_java_script_source_text_profile_error(String::from("inspectable UTF16 source text 1"), (-0f64))) == "source text exceeds the explicit 16777216 UTF16 unit budget"), "assertion 6");
    assert!(((self::read_java_script_source_text_profile_error(String::from("inspectable UTF16 source text 1"), f64::NAN)) == "source text exceeds the explicit 16777216 UTF16 unit budget"), "assertion 7");
}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || { ml_main() })
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}

#[test]
fn all_literal_source_text_profile_controls() { main(); }
