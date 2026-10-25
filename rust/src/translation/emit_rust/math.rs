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
        if crate::translation::rust_string_byte_length_rules::accept_rust_string_byte_length_operation("math",op) {
            crate::translation::rust_string_byte_lengths::validate(e,self.program.source_language,self.utf16_strings)?;
            return Ok(crate::translation::rust_string_byte_length_rules::render_rust_string_byte_length("Rust",&self.receiver(&args[0])?));
        }
        if crate::translation::string_construction_rules::accept_linked_java_script_string_construction("math",op){
            self.preludes.insert(Prelude::Utf16Text);
            let values=self.expr(&args[0])?;
            return Ok(crate::translation::string_construction_rules::render_java_script_string_construction("Rust",op,&values));
        }
        if crate::translation::javascript_string_unit_rules::accept_linked_java_script_string_unit_construction("math",op){
            let units=self.expr(&args[0])?;
            return Ok(crate::translation::javascript_string_unit_rules::render_java_script_string_unit_construction("Rust",&units));
        }
        if crate::translation::big_integer_conversion_rules::accept_linked_java_script_big_integer_result("math",op) {
            self.preludes.insert(Prelude::Big);self.preludes.insert(Prelude::BigIntegerConversion);
            let form=crate::translation::big_integer_conversion_rules::read_java_script_big_integer_result_operation(op);
            let value=self.expr(&args[0])?;
            if form=="number-compare"||form=="string-compare" {
                let right=self.expr(&args[1])?;
                return Ok(crate::translation::big_integer_conversion_rules::render_java_script_big_integer_comparison("Rust",&form,&value,&right,self.utf16_strings));
            }
            if form=="number-power" {
                let right=self.expr(&args[1])?;
                return Ok(crate::translation::big_integer_conversion_rules::render_java_script_number_power("Rust",&value,&right));
            }
            if form=="power" {
                let right=self.expr(&args[1])?;
                return Ok(crate::translation::big_integer_conversion_rules::render_java_script_big_integer_power("Rust",&value,&right));
            }
            if form=="division" {
                let right=self.expr(&args[1])?;
                return Ok(crate::translation::big_integer_conversion_rules::render_java_script_big_integer_division("Rust",&value,&right));
            }
            return Ok(crate::translation::big_integer_conversion_rules::render_java_script_big_integer_result_expression("Rust",&form,&value,self.utf16_strings));
        }
        if crate::translation::source_exception_rules::accept_linked_java_script_source_exception(
            "math", op,
        ) {
            self.preludes.insert(Prelude::SourceException);
            let message = self.expr(&args[0])?;
            let present = self.expr(&args[1])?;
            return Ok(crate::translation::source_exception_rules::render_java_script_source_exception_expression("Rust",op,&message,&present,self.utf16_strings));
        }
        if crate::translation::string_number_rules::accept_linked_java_script_number_conversion(
            "math", op,
        ) {
            return self.javascript_number_conversion(e);
        }
        if crate::translation::rust_shift_rules::accept_linked_rust_checked_shift("math", op) {
            let values = args
                .iter()
                .map(|argument| self.expr(argument))
                .collect::<Result<Vec<_>>>()?;
            return Ok(crate::translation::rust_shifts::emit_rust_checked_shift(
                "Rust", e, &values,
            ));
        }
        if crate::translation::rust_primitive_bitwise_rules::read_rust_primitive_bitwise_arity(op)
            > 0.0
        {
            let values = args
                .iter()
                .map(|argument| self.expr(argument))
                .collect::<Result<Vec<_>>>()?;
            return Ok(
                crate::translation::rust_primitive_bitwise::emit_rust_primitive_bitwise(
                    "Rust", e, &values,
                ),
            );
        }
        if crate::translation::big_integer_bitwise_rules::read_big_integer_bitwise_arity(op) > 0.0 {
            self.preludes.insert(Prelude::Big);
            self.preludes.insert(Prelude::BigIntegerBitwise);
            let values = args
                .iter()
                .map(|argument| self.expr(argument))
                .collect::<Result<Vec<_>>>()?;
            let text = crate::translation::big_integer_bitwise_rules::render_big_integer_bitwise_expression("Rust", op, &values);
            if text.is_empty() {
                return Err(super::unsupported(
                    "BigInt bitwise arity",
                    "the operation requires its exact checked operand inventory",
                    e.span,
                ));
            }
            self.state.encode(
                "big-integer-bitwise",
                &crate::translation::big_integer_bitwise_rules::read_big_integer_bitwise_statement(
                ),
            );
            return Ok(text);
        }
        self.floats();
        let code = crate::translation::number_bitwise_rules::read_java_script_number_bitwise_operation_code(op);
        if code >= 0.0 {
            let count = match args.len() {
                1 => 1.0,
                2 => 2.0,
                _ => 0.0,
            };
            if !crate::translation::number_bitwise_rules::accept_java_script_number_bitwise_operand_count(op, count) {
                return Err(super::unsupported("Number bitwise arity", "the operation requires its exact checked operand inventory", e.span));
            }
            self.preludes.insert(Prelude::NumberBitwise);
            let arguments = args
                .iter()
                .map(|arg| self.expr(arg))
                .collect::<Result<Vec<_>>>()?;
            return Ok(format!(
                "crate::ml_number_bitwise::{}({})",
                super::snake(op),
                arguments.join(", ")
            ));
        }
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
