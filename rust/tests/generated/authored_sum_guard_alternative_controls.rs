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

pub fn read_rust_standard_sum_guard_mode(owned: bool, guarded: bool) -> String {
    if (owned && guarded) {
        String::from("deferred guard")
    } else {
        String::from("ordinary")
    }
}

pub fn read_rust_standard_sum_deferred_guard_pattern_error(alternatives: f64, aliases: f64) -> String {
    if (alternatives != (1f64)) {
        String::from("guarded owned sum alternatives need their delayed move agreement model")
    } else {
        if (aliases != (0f64)) {
            String::from("guarded owned sum whole-value aliases need their delayed payload move model")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_standard_sum_guard_alternative_pattern_error(alternatives: f64, aliases: f64) -> String {
    if (((!(self::ml_math::is_integer(alternatives)) || (alternatives < (1f64))) || !(self::ml_math::is_integer(aliases))) || (aliases < (0f64))) {
        String::from("guarded owned sum patterns need their actual finite alternative and alias counts")
    } else {
        String::from("")
    }
}

pub fn read_rust_standard_sum_guard_alternative_binding_error(complete: bool, same_type: bool, same_place: bool) -> String {
    if (!complete || !same_type) {
        String::from("guarded owned sum alternatives must bind the same complete actual source types")
    } else {
        if !same_place {
            String::from("guarded owned sum alternatives must retain the same actual root loans and lifetime provenance")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_standard_sum_guard_alias_transfer_error(aliases: f64, owned_bindings: f64) -> String {
    if ((aliases > (0f64)) && (owned_bindings > (1f64))) {
        String::from("guarded owned sum whole-value aliases with owned field bindings need their partial-move agreement model")
    } else {
        String::from("")
    }
}

pub fn read_rust_standard_sum_guard_binding_disposition(deferred: bool, owned: bool) -> String {
    if deferred {
        String::from("readonly")
    } else {
        if owned {
            String::from("transfer")
        } else {
            String::from("plain")
        }
    }
}

fn ml_main() {
    assert!(((self::read_rust_standard_sum_guard_alternative_pattern_error(1f64, 0f64)) == ""), "assertion 1");
    assert!(((self::read_rust_standard_sum_guard_alternative_pattern_error(2f64, 0f64)) == ""), "assertion 2");
    assert!(((self::read_rust_standard_sum_guard_alternative_pattern_error(2f64, 2f64)) == ""), "assertion 3");
    assert!(((self::read_rust_standard_sum_guard_alternative_pattern_error(0f64, 0f64)) == "guarded owned sum patterns need their actual finite alternative and alias counts"), "assertion 4");
    assert!(((self::read_rust_standard_sum_guard_alternative_pattern_error(1f64, (-1f64))) == "guarded owned sum patterns need their actual finite alternative and alias counts"), "assertion 5");
    assert!(((self::read_rust_standard_sum_guard_alternative_binding_error(true, true, true)) == ""), "assertion 6");
    assert!(((self::read_rust_standard_sum_guard_alternative_binding_error(false, true, true)) == "guarded owned sum alternatives must bind the same complete actual source types"), "assertion 7");
    assert!(((self::read_rust_standard_sum_guard_alternative_binding_error(true, false, true)) == "guarded owned sum alternatives must bind the same complete actual source types"), "assertion 8");
    assert!(((self::read_rust_standard_sum_guard_alternative_binding_error(true, true, false)) == "guarded owned sum alternatives must retain the same actual root loans and lifetime provenance"), "assertion 9");
    assert!(((self::read_rust_standard_sum_guard_alias_transfer_error(1f64, 1f64)) == ""), "assertion 10");
    assert!(((self::read_rust_standard_sum_guard_alias_transfer_error(1f64, 0f64)) == ""), "assertion 11");
    assert!(((self::read_rust_standard_sum_guard_alias_transfer_error(2f64, 1f64)) == ""), "assertion 12");
    assert!(((self::read_rust_standard_sum_guard_alias_transfer_error(1f64, 2f64)) == "guarded owned sum whole-value aliases with owned field bindings need their partial-move agreement model"), "assertion 13");
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
fn all_literal_sum_guard_alternative_controls() { main(); }
