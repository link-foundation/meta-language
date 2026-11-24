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

pub fn read_formal_datatype_alias_budget_error(rows: f64, units: f64) -> String {
    if (((((!(self::ml_math::is_safe_integer(rows)) || !(self::ml_math::is_safe_integer(units))) || (rows < (0f64))) || (units < (0f64))) || (rows > (8192f64))) || (units > (16777216f64))) {
        String::from("formal datatype public aliases exceed their finite target budget")
    } else {
        String::from("")
    }
}

pub fn read_formal_datatype_alias_traversal_error(visits: f64, depth: f64) -> String {
    if (((((!(self::ml_math::is_safe_integer(visits)) || !(self::ml_math::is_safe_integer(depth))) || (visits < (0f64))) || (depth < (0f64))) || (visits > (1048576f64))) || (depth > (256f64))) {
        String::from("formal datatype alias scope traversal exceeds its finite target budget")
    } else {
        String::from("")
    }
}

pub fn render_formal_datatype_type_alias(target: String, name: String, value: String) -> String {
    if (target == "Lean") {
        format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("abbrev ")), name)), (String::from(" := _root_.")))), value)
    } else {
        if (target == "Rocq") {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("Definition ")), name)), (String::from(" : Type := ")))), value)), (String::from(".")))
        } else {
            String::from("")
        }
    }
}

pub fn render_formal_datatype_constructor_alias(target: String, data: String, name: String, value: String) -> String {
    if (target == "Lean") {
        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("namespace ")), data)), (String::from("\n@[match_pattern] abbrev ")))), name)), (String::from(" := _root_.")))), value)), (String::from("\nend ")))), data)
    } else {
        if (target == "Rocq") {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("Notation ")), name)), (String::from(" := ")))), value)), (String::from(".")))
        } else {
            String::from("")
        }
    }
}

pub fn render_formal_datatype_alias_module(target: String, name: String, open: bool) -> String {
    if (target == "Lean") {
        format!("{}{}", (if open {
            String::from("namespace ")
        } else {
            String::from("end ")
        }), name)
    } else {
        if (target == "Rocq") {
            format!("{}{}", (format!("{}{}", (if open {
                String::from("Module ")
            } else {
                String::from("End ")
            }), name)), (String::from(".")))
        } else {
            String::from("")
        }
    }
}

fn ml_main() {
    assert!(((self::render_formal_datatype_type_alias(String::from("Lean"), String::from("Public"), String::from("private.Scope"))) == "abbrev Public := _root_.private.Scope"), "assertion 1");
    assert!(((self::render_formal_datatype_type_alias(String::from("Rocq"), String::from("Public"), String::from("private.Scope"))) == "Definition Public : Type := private.Scope."), "assertion 2");
    assert!(((self::render_formal_datatype_constructor_alias(String::from("Lean"), String::from("Public"), String::from("More"), String::from("private.Scope.More"))) == "namespace Public\n@[match_pattern] abbrev More := _root_.private.Scope.More\nend Public"), "assertion 3");
    assert!(((self::render_formal_datatype_constructor_alias(String::from("Rocq"), String::from("Public"), String::from("More"), String::from("private.More"))) == "Notation More := private.More."), "assertion 4");
    assert!(((self::render_formal_datatype_alias_module(String::from("Lean"), String::from("inner"), true)) == "namespace inner"), "assertion 5");
    assert!(((self::render_formal_datatype_alias_module(String::from("Lean"), String::from("inner"), false)) == "end inner"), "assertion 6");
    assert!(((self::render_formal_datatype_alias_module(String::from("Rocq"), String::from("inner"), true)) == "Module inner."), "assertion 7");
    assert!(((self::render_formal_datatype_alias_module(String::from("Rocq"), String::from("inner"), false)) == "End inner."), "assertion 8");
    assert!(((self::render_formal_datatype_type_alias(String::from("unknown"), String::from("Public"), String::from("Value"))) == ""), "assertion 9");
    assert!(((self::read_formal_datatype_alias_budget_error(8192f64, 16777216f64)) == ""), "assertion 10");
    assert!(((self::read_formal_datatype_alias_budget_error(8193f64, 0f64)) == "formal datatype public aliases exceed their finite target budget"), "assertion 11");
    assert!(((self::read_formal_datatype_alias_budget_error(0f64, 16777217f64)) == "formal datatype public aliases exceed their finite target budget"), "assertion 12");
    assert!(((self::read_formal_datatype_alias_budget_error(f64::NAN, 0f64)) == "formal datatype public aliases exceed their finite target budget"), "assertion 13");
    assert!(((self::read_formal_datatype_alias_budget_error(0f64, f64::INFINITY)) == "formal datatype public aliases exceed their finite target budget"), "assertion 14");
    assert!(((self::read_formal_datatype_alias_budget_error((-1f64), 0f64)) == "formal datatype public aliases exceed their finite target budget"), "assertion 15");
    assert!(((self::read_formal_datatype_alias_budget_error(0f64, 0.5f64)) == "formal datatype public aliases exceed their finite target budget"), "assertion 16");
    assert!(((self::read_formal_datatype_alias_traversal_error(1048576f64, 256f64)) == ""), "assertion 17");
    assert!(((self::read_formal_datatype_alias_traversal_error(1048577f64, 0f64)) == "formal datatype alias scope traversal exceeds its finite target budget"), "assertion 18");
    assert!(((self::read_formal_datatype_alias_traversal_error(0f64, 257f64)) == "formal datatype alias scope traversal exceeds its finite target budget"), "assertion 19");
    assert!(((self::read_formal_datatype_alias_traversal_error(f64::NAN, 0f64)) == "formal datatype alias scope traversal exceeds its finite target budget"), "assertion 20");
    assert!(((self::read_formal_datatype_alias_traversal_error(0f64, (-1f64))) == "formal datatype alias scope traversal exceeds its finite target budget"), "assertion 21");
    assert!(((self::read_formal_datatype_alias_traversal_error(0f64, 1.5f64)) == "formal datatype alias scope traversal exceeds its finite target budget"), "assertion 22");
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
fn all_literal_formal_datatype_alias_controls() { main(); }
