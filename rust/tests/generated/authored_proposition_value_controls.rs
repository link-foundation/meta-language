// Translated from JavaScript by meta-language: portable core, Rust target.
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

/// JavaScript's Number operations that Rust's f64 does not share.
pub mod ml_number {
    /// ECMAScript Number::toString: the shortest decimal that reads back as
    /// the value, the even one on a tie, laid out as JavaScript prints it.
    pub fn js_number(x: f64) -> String {
        if x.is_nan() {
            return String::from("NaN");
        }
        if x.is_infinite() {
            return String::from(if x > 0.0 { "Infinity" } else { "-Infinity" });
        }
        if x == 0.0 {
            return String::from("0");
        }
        let (mut digits, n) = decimal(&format!("{:e}", x.abs()));
        // Rust's shortest digits round a tie away from zero; ECMAScript takes the even neighbour.
        let (exact, _) = decimal(&format!("{:.800e}", x.abs()));
        let rest = &exact[digits.len().min(exact.len())..];
        if digits.len() > 1 && rest.starts_with('5') && rest[1..].bytes().all(|b| b == b'0') {
            let last = digits.as_bytes()[digits.len() - 1] - b'0';
            let down = exact[..digits.len()].to_string();
            let other = if digits == down { increment(&down) } else { Some(down) };
            if let Some(other) = other {
                let back = format!("{}.{}e{}", &other[..1], &other[1..], n - 1);
                if last % 2 == 1 && back.parse::<f64>() == Ok(x.abs()) {
                    digits = other.trim_end_matches('0').to_string();
                }
            }
        }
        let k = digits.len() as i32;
        let body = if k <= n && n <= 21 {
            format!("{digits}{}", "0".repeat((n - k) as usize))
        } else if 0 < n && n <= 21 {
            format!("{}.{}", &digits[..n as usize], &digits[n as usize..])
        } else if -6 < n && n <= 0 {
            format!("0.{}{digits}", "0".repeat((-n) as usize))
        } else {
            let sign = if n - 1 < 0 { '-' } else { '+' };
            let power = (n - 1).abs();
            if k == 1 {
                format!("{digits}e{sign}{power}")
            } else {
                format!("{}.{}e{sign}{power}", &digits[..1], &digits[1..])
            }
        };
        if x < 0.0 {
            format!("-{body}")
        } else {
            body
        }
    }

    /// What console.log prints: -0 as "-0", where String(-0) is "0".
    pub fn js_console(x: f64) -> String {
        if x == 0.0 && x.is_sign_negative() {
            String::from("-0")
        } else {
            js_number(x)
        }
    }

    /// SameValue, as Object.is and assert.strictEqual compare: NaN equals
    /// NaN, and 0 differs from -0.
    pub fn same_value(a: f64, b: f64) -> bool {
        if a.is_nan() || b.is_nan() {
            a.is_nan() && b.is_nan()
        } else {
            a.to_bits() == b.to_bits()
        }
    }

    /// A decimal digit string plus one, or None when it gains a digit.
    fn increment(digits: &str) -> Option<String> {
        let mut bytes = digits.as_bytes().to_vec();
        for index in (0..bytes.len()).rev() {
            if bytes[index] == b'9' {
                bytes[index] = b'0';
            } else {
                bytes[index] += 1;
                return String::from_utf8(bytes).ok();
            }
        }
        None
    }

    /// The significant digits without trailing zeros, and the exponent n with
    /// value 0.digits × 10^n, of Rust's {:e} text.
    fn decimal(text: &str) -> (String, i32) {
        let (mantissa, exponent) = text.split_once('e').expect("exponent");
        let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
        let digits = digits.trim_end_matches('0').to_string();
        let digits = if digits.is_empty() { String::from("0") } else { digits };
        (digits, exponent.parse::<i32>().expect("exponent") + 1)
    }
}

pub fn read_proposition_value_form(kind: String) -> String {
    if (kind == "bool") {
        String::from("value")
    } else {
        if (kind == "not") {
            String::from("negation")
        } else {
            if (((kind == "and") || (kind == "or")) || (kind == "implies")) {
                kind.clone()
            } else {
                if (kind == "forall") {
                    String::from("finite quantification")
                } else {
                    if self::accept_proposition_value_comparison(kind.clone()) {
                        String::from("comparison")
                    } else {
                        String::from("")
                    }
                }
            }
        }
    }
}

pub fn read_proposition_value_conditional_branch(operation: String, truth: bool) -> String {
    if (operation == "and") {
        if truth {
            String::from("right")
        } else {
            String::from("false")
        }
    } else {
        if (operation == "or") {
            if truth {
                String::from("true")
            } else {
                String::from("right")
            }
        } else {
            if (operation == "implies") {
                if truth {
                    String::from("right")
                } else {
                    String::from("true")
                }
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_proposition_value_quantifier_join() -> String {
    String::from("and")
}

pub fn accept_proposition_value_comparison(operation: String) -> bool {
    ((((((operation == "eq") || (operation == "ne")) || (operation == "lt")) || (operation == "le")) || (operation == "gt")) || (operation == "ge"))
}

pub fn plan_proposition_value_quantifier(kind: String) -> Vec<f64> {
    if (kind == "bool") {
        vec![0f64, 1f64]
    } else {
        if (kind == "unit") {
            vec![0f64]
        } else {
            Vec::<f64>::new()
        }
    }
}

pub fn read_proposition_value_expansion_limit() -> f64 {
    256f64
}

pub fn read_proposition_value_domain_error(kind: String) -> String {
    if { let ml_value = ((self::plan_proposition_value_quantifier(kind.clone())).len() as f64); ml_value != 0f64 && !ml_value.is_nan() } {
        String::from("")
    } else {
        String::from("a proposition value requires an exhaustive Bool or Unit quantifier domain")
    }
}

pub fn render_rocq_proposition_equality_value(equality: String, negate: bool) -> String {
    {
        let decision = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(if (ltac:(decide equality) : {")), equality)), (String::from("} + {~ (")))), equality)), (String::from(")}) then true else false)")));
        if negate {
            format!("{}{}", (format!("{}{}", (String::from("(negb ")), decision)), (String::from(")")))
        } else {
            decision.clone()
        }
    }
}

fn ml_main() {
    assert!(((self::read_proposition_value_form(String::from("bool"))) == "value"), "assertion 1");
    assert!(((self::read_proposition_value_form(String::from("eq"))) == "comparison"), "assertion 2");
    assert!(((self::read_proposition_value_form(String::from("forall"))) == "finite quantification"), "assertion 3");
    assert!(((self::read_proposition_value_form(String::from("unknown"))) == ""), "assertion 4");
    assert!(((self::read_proposition_value_conditional_branch(String::from("and"), true)) == "right"), "assertion 5");
    assert!(((self::read_proposition_value_conditional_branch(String::from("and"), false)) == "false"), "assertion 6");
    assert!(((self::read_proposition_value_conditional_branch(String::from("or"), true)) == "true"), "assertion 7");
    assert!(((self::read_proposition_value_conditional_branch(String::from("or"), false)) == "right"), "assertion 8");
    assert!(((self::read_proposition_value_conditional_branch(String::from("implies"), true)) == "right"), "assertion 9");
    assert!(((self::read_proposition_value_conditional_branch(String::from("implies"), false)) == "true"), "assertion 10");
    assert!(((self::plan_proposition_value_quantifier(String::from("bool"))) == (vec![0f64, 1f64])), "assertion 11");
    assert!(((self::plan_proposition_value_quantifier(String::from("unit"))) == (vec![0f64])), "assertion 12");
    assert!(((self::plan_proposition_value_quantifier(String::from("nat"))) == (Vec::<f64>::new())), "assertion 13");
    assert!(((self::read_proposition_value_domain_error(String::from("string"))) == "a proposition value requires an exhaustive Bool or Unit quantifier domain"), "assertion 14");
    assert!(self::ml_number::same_value(self::read_proposition_value_expansion_limit(), 256f64), "assertion 15");
    assert!(((self::read_proposition_value_quantifier_join()) == "and"), "assertion 16");
    assert!(((self::accept_proposition_value_comparison(String::from("eq"))) == (true)), "assertion 17");
    assert!(((self::accept_proposition_value_comparison(String::from("add"))) == (false)), "assertion 18");
    assert!(((self::render_rocq_proposition_equality_value(String::from("(left = right)"), false)) == "(if (ltac:(decide equality) : {(left = right)} + {~ ((left = right))}) then true else false)"), "assertion 19");
    assert!(((self::render_rocq_proposition_equality_value(String::from("(left = right)"), true)) == "(negb (if (ltac:(decide equality) : {(left = right)} + {~ ((left = right))}) then true else false))"), "assertion 20");
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
fn all_literal_proposition_value_controls() { main(); }
