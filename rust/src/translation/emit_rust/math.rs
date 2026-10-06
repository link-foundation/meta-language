//! The `Math` and `Number` functions: `f64` methods where they agree with
//! JavaScript, and the generated `ml_math` module where they do not.

use super::{Expr, Node, Prelude, Result, RustEmitter};

pub(super) const MATH_PRELUDE: &str = r"/// The Math functions that Rust's f64 methods do not share with JavaScript.
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
}";

impl RustEmitter<'_> {
    pub(super) fn math(&mut self, e: &Expr) -> Result<String> {
        let Node::Math { op, args } = &e.node else {
            unreachable!("not a Math expression")
        };
        self.floats();
        match op.as_str() {
            "abs" | "floor" | "ceil" | "trunc" | "sqrt" => {
                Ok(format!("{}.{op}()", self.receiver(&args[0])?))
            }
            "isFinite" => Ok(format!("{}.is_finite()", self.receiver(&args[0])?)),
            "isNaN" => Ok(format!("{}.is_nan()", self.receiver(&args[0])?)),
            "max" | "min" => {
                if args.is_empty() {
                    return Ok(if op == "max" {
                        "f64::NEG_INFINITY"
                    } else {
                        "f64::INFINITY"
                    }
                    .to_owned());
                }
                self.preludes.insert(Prelude::Math);
                let mut texts = Vec::with_capacity(args.len());
                for arg in args {
                    texts.push(self.expr(arg)?);
                }
                Ok(texts
                    .into_iter()
                    .reduce(|left, right| format!("crate::ml_math::{op}({left}, {right})"))
                    .unwrap_or_default())
            }
            _ => {
                self.preludes.insert(Prelude::Math);
                let name = match op.as_str() {
                    "maxOf" => "max_of",
                    "minOf" => "min_of",
                    "isInteger" => "is_integer",
                    "isSafeInteger" => "is_safe_integer",
                    other => other,
                };
                let arg = if op.ends_with("Of") {
                    self.borrow(&args[0])?
                } else {
                    self.expr(&args[0])?
                };
                Ok(format!("crate::ml_math::{name}({arg})"))
            }
        }
    }
}
