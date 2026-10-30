// Generated from complete js/src/translation/rust-shift-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_big_integer_bitwise;
use super::ml_number;
use super::read_big_integer_bitwise_arity;
use super::read_big_integer_bitwise_operation_code;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_big_integer_bitwise_expression(target: &str, operation: &str, arguments_: &[String]) -> String {
    {
        let code = self::read_big_integer_bitwise_operation_code(operation);
        if ((code < (0f64)) || (((arguments_.len() as f64)) != (self::read_big_integer_bitwise_arity(operation)))) {
            String::from("")
        } else {
            {
                let left = (self::ml_array::optional_at(arguments_, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| String::from(""));
                {
                    let right = (self::ml_array::optional_at(arguments_, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| String::from(""));
                    if (target == "Rocq") {
                        {
                            let operation_name = if (code == (0f64)) {
                                String::from("Z.land")
                            } else {
                                if (code == (1f64)) {
                                    String::from("Z.lor")
                                } else {
                                    if (code == (2f64)) {
                                        String::from("Z.lxor")
                                    } else {
                                        if (code == (3f64)) {
                                            String::from("Z.shiftl")
                                        } else {
                                            if (code == (4f64)) {
                                                String::from("Z.shiftr")
                                            } else {
                                                String::from("Z.lnot")
                                            }
                                        }
                                    }
                                }
                            };
                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), operation_name)), (String::from(" ")))), left)), (if (code == (6f64)) {
                                String::from("")
                            } else {
                                format!("{}{}", (String::from(" ")), right)
                            }))), (String::from(")")))
                        }
                    } else {
                        if (target == "Lean") {
                            if (code == (6f64)) {
                                format!("{}{}", (format!("{}{}", (String::from("(-1 - (")), left)), (String::from("))")))
                            } else {
                                {
                                    let text = format!("{}{}", (format!("{}{}", left, (String::from(" ")))), right);
                                    {
                                        let suffix = 0f64;
                                        {
                                            let suffix_2 = self::ml_render_big_integer_bitwise_expression_loop7(&(text), suffix);
                                            {
                                                let a = format!("{}{}", (String::from("ml_integer_left")), (self::ml_number::js_number(suffix_2)));
                                                {
                                                    let b = format!("{}{}", (String::from("ml_integer_right")), (self::ml_number::js_number(suffix_2)));
                                                    {
                                                        let am = format!("{}{}", (format!("{}{}", (String::from("(Int.toNat (-1 - ")), a)), (String::from("))")));
                                                        {
                                                            let bm = format!("{}{}", (format!("{}{}", (String::from("(Int.toNat (-1 - ")), b)), (String::from("))")));
                                                            {
                                                                let ap = format!("{}{}", (format!("{}{}", (String::from("(Int.toNat ")), a)), (String::from(")")));
                                                                {
                                                                    let bp = format!("{}{}", (format!("{}{}", (String::from("(Int.toNat ")), b)), (String::from(")")));
                                                                    {
                                                                        let prefix = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(let ")), a)), (String::from(" : Int := ")))), left)), (String::from("; let ")))), b)), (String::from(" : Int := ")))), right)), (String::from("; ")));
                                                                        if ((code == (3f64)) || (code == (4f64))) {
                                                                            {
                                                                                let count = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(Int.toNat (if ")), b)), (String::from(" < 0 then -")))), b)), (String::from(" else ")))), b)), (String::from("))")));
                                                                                {
                                                                                    let shift_left = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), a)), (String::from(" * Int.ofNat (Nat.shiftLeft 1 ")))), count)), (String::from("))")));
                                                                                    {
                                                                                        let shift_right = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(if ")), a)), (String::from(" < 0 then -1 - Int.ofNat (Nat.shiftRight ")))), am)), (String::from(" ")))), count)), (String::from(") else Int.ofNat (Nat.shiftRight ")))), ap)), (String::from(" ")))), count)), (String::from("))")));
                                                                                        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", prefix, (String::from("if ")))), a)), (String::from(" = 0 then 0 else if ")))), b)), (String::from(" < 0 then ")))), (if (code == (3f64)) {
                                                                                            shift_right.clone()
                                                                                        } else {
                                                                                            shift_left.clone()
                                                                                        }))), (String::from(" else ")))), (if (code == (3f64)) {
                                                                                            shift_left.clone()
                                                                                        } else {
                                                                                            shift_right.clone()
                                                                                        }))), (String::from(")")))
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            {
                                                                                let difference = String::from("(Nat.bitwise (fun ml_bit_left ml_bit_right => ml_bit_left && !ml_bit_right) ");
                                                                                {
                                                                                    let both = if (code == (0f64)) {
                                                                                        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(-1 - Int.ofNat (Nat.lor ")), am)), (String::from(" ")))), bm)), (String::from("))")))
                                                                                    } else {
                                                                                        if (code == (1f64)) {
                                                                                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(-1 - Int.ofNat (Nat.land ")), am)), (String::from(" ")))), bm)), (String::from("))")))
                                                                                        } else {
                                                                                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(Int.ofNat (Nat.xor ")), am)), (String::from(" ")))), bm)), (String::from("))")))
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let left_negative = if (code == (0f64)) {
                                                                                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(Int.ofNat ")), difference)), bp)), (String::from(" ")))), am)), (String::from("))")))
                                                                                        } else {
                                                                                            if (code == (1f64)) {
                                                                                                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(-1 - Int.ofNat ")), difference)), am)), (String::from(" ")))), bp)), (String::from("))")))
                                                                                            } else {
                                                                                                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(-1 - Int.ofNat (Nat.xor ")), am)), (String::from(" ")))), bp)), (String::from("))")))
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let right_negative = if (code == (0f64)) {
                                                                                                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(Int.ofNat ")), difference)), ap)), (String::from(" ")))), bm)), (String::from("))")))
                                                                                            } else {
                                                                                                if (code == (1f64)) {
                                                                                                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(-1 - Int.ofNat ")), difference)), bm)), (String::from(" ")))), ap)), (String::from("))")))
                                                                                                } else {
                                                                                                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(-1 - Int.ofNat (Nat.xor ")), ap)), (String::from(" ")))), bm)), (String::from("))")))
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let positive = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(Int.ofNat (")), (if (code == (0f64)) {
                                                                                                    String::from("Nat.land")
                                                                                                } else {
                                                                                                    if (code == (1f64)) {
                                                                                                        String::from("Nat.lor")
                                                                                                    } else {
                                                                                                        String::from("Nat.xor")
                                                                                                    }
                                                                                                }))), (String::from(" ")))), ap)), (String::from(" ")))), bp)), (String::from("))")));
                                                                                                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", prefix, (String::from("if ")))), a)), (String::from(" < 0 then (if ")))), b)), (String::from(" < 0 then ")))), both)), (String::from(" else ")))), left_negative)), (String::from(") else (if ")))), b)), (String::from(" < 0 then ")))), right_negative)), (String::from(" else ")))), positive)), (String::from("))")))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if (target == "Rust") {
                                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_big_integer_bitwise::apply_big_integer_bitwise_operation(")), (self::ml_number::js_number(code)))), (String::from("f64, ")))), left)), (String::from(", ")))), (if (code == (6f64)) {
                                    String::from("ml::Big::zero()")
                                } else {
                                    right.clone()
                                }))), (String::from(")")))
                            } else {
                                if (target != "JavaScript") {
                                    String::from("")
                                } else {
                                    if (code == (6f64)) {
                                        format!("{}{}", (format!("{}{}", (String::from("(~(")), left)), (String::from("))")))
                                    } else {
                                        {
                                            let symbol = if (code == (0f64)) {
                                                String::from("&")
                                            } else {
                                                if (code == (1f64)) {
                                                    String::from("|")
                                                } else {
                                                    if (code == (2f64)) {
                                                        String::from("^")
                                                    } else {
                                                        if (code == (3f64)) {
                                                            String::from("<<")
                                                        } else {
                                                            String::from(">>")
                                                        }
                                                    }
                                                }
                                            };
                                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("((")), left)), (String::from(") ")))), symbol)), (String::from(" (")))), right)), (String::from("))")))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_big_integer_bitwise_statement() -> String {
    String::from("BigInt bitwise operations use exact unbounded integers with infinite sign extension, negative shift reversal and arithmetic right shifts; operands are evaluated once in source order without Number coercion or modulo-32 shift counts. Native source and the emitted integer runtime are generated from the same checked definition.")
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadPrimitiveBitwiseDomainLoop2Result {
    MlReadPrimitiveBitwiseDomainLoop2Done(bool, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_primitive_bitwise_domain_loop2(mut number: bool, mut integer: bool, ml_values1: &[String], mut ml_index1: self::ml::Big) -> self::MlReadPrimitiveBitwiseDomainLoop2Result {
    loop {
        return if (ml_index1 < (self::ml::Big::from_u128(ml_values1.len() as u128))) {
            {
                let kind = self::ml_array::at(ml_values1, ml_index1.to_index());
                {
                    let number_2 = (number && (kind == "float"));
                    {
                        let integer_2 = (integer && ((kind == "int") || (kind == "nat")));
                        {
                            let ml_index1_2 = ml_index1.add(&self::ml::Big::from_i128(1));
                            {
                                (number, integer, ml_index1) = (number_2, integer_2, ml_index1_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadPrimitiveBitwiseDomainLoop2Result::MlReadPrimitiveBitwiseDomainLoop2Done(number, integer)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_apply_big_integer_bitwise_operation_loop4(mut count: self::ml::Big, mut value: self::ml::Big, mut factor: self::ml::Big) -> self::ml::Big {
    loop {
        return if (count > (self::ml::Big::from_i128(0))) {
            {
                let value_3 = if ((self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let value_2 = value.mul(&factor);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let count_2 = self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (count_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let factor_2 = factor.mul(&factor);
                            {
                                (count, value, factor) = (count_2.clone(), value_3.clone(), factor_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (count, value) = (count_2.clone(), value_3.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_apply_big_integer_bitwise_operation_loop5(mut count: self::ml::Big, mut value: self::ml::Big) -> self::ml::Big {
    loop {
        return if (((count > (self::ml::Big::from_i128(0))) && (value != (self::ml::Big::from_i128(0)))) && (value != (self::ml::Big::from_i128(-1)))) {
            {
                let value_2 = if (value < (self::ml::Big::from_i128(0))) {
                    self::ml::divide(&value.sub(&self::ml::Big::from_i128(1)), &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                } else {
                    self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                };
                {
                    let count_2 = count.sub(&self::ml::Big::from_i128(1));
                    {
                        (count, value) = (count_2.clone(), value_2.clone());
                        continue;
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlApplyBigIntegerBitwiseOperationJoin3 {
    MlApplyBigIntegerBitwiseOperationJoin3Next(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlApplyBigIntegerBitwiseOperationLoop6Result {
    MlApplyBigIntegerBitwiseOperationLoop6Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_apply_big_integer_bitwise_operation_loop6(mut operation: f64, mut negative_left: bool, mut negative_right: bool, mut first: self::ml::Big, mut second: self::ml::Big, mut weight: self::ml::Big, mut result: self::ml::Big) -> self::MlApplyBigIntegerBitwiseOperationLoop6Result {
    loop {
        return if ((first > (self::ml::Big::from_i128(0))) || (second > (self::ml::Big::from_i128(0)))) {
            {
                let a = if negative_left {
                    (self::ml::Big::from_i128(1)).sub(&self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                } else {
                    self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                };
                {
                    let b = if negative_right {
                        (self::ml::Big::from_i128(1)).sub(&self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                    } else {
                        self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                    };
                    {
                        let selected = if (operation == (0f64)) {
                            ((a == (self::ml::Big::from_i128(1))) && (b == (self::ml::Big::from_i128(1))))
                        } else {
                            if (operation == (1f64)) {
                                ((a == (self::ml::Big::from_i128(1))) || (b == (self::ml::Big::from_i128(1))))
                            } else {
                                (a != b)
                            }
                        };
                        {
                            let result_3 = if selected {
                                {
                                    let result_2 = result.add(&weight);
                                    result_2.clone()
                                }
                            } else {
                                result.clone()
                            };
                            {
                                let first_2 = self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                {
                                    let second_2 = self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                    {
                                        let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                                        {
                                            (first, second, weight, result) = (first_2.clone(), second_2.clone(), weight_2.clone(), result_3.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlApplyBigIntegerBitwiseOperationLoop6Result::MlApplyBigIntegerBitwiseOperationLoop6Done(weight.clone(), result.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_render_big_integer_bitwise_expression_loop7(text: &str, mut suffix: f64) -> f64 {
    loop {
        return if (text.contains((format!("{}{}", (String::from("ml_integer_left")), (self::ml_number::js_number(suffix)))).as_str()) || text.contains((format!("{}{}", (String::from("ml_integer_right")), (self::ml_number::js_number(suffix)))).as_str())) {
            {
                let suffix_2 = (suffix + 1f64);
                {
                    suffix = suffix_2;
                    continue;
                }
            }
        } else {
            suffix
        };
    }
}
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_shift_source_operation(symbol: &str) -> String {
    if (symbol == "<<") {
        String::from("shiftRustPrimitiveBitsLeft")
    } else {
        if (symbol == ">>") {
            String::from("shiftRustPrimitiveBitsRight")
        } else {
            String::from("")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_shift_checked_operation(operation: &str) -> String {
    if ((operation == "shiftRustPrimitiveBitsLeft") || (operation == "readCheckedRustLeftShift")) {
        String::from("readCheckedRustLeftShift")
    } else {
        if ((operation == "shiftRustPrimitiveBitsRight") || (operation == "readCheckedRustRightShift")) {
            String::from("readCheckedRustRightShift")
        } else {
            String::from("")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_shift_source_operation(operation: &str) -> bool {
    ((operation == "shiftRustPrimitiveBitsLeft") || (operation == "shiftRustPrimitiveBitsRight"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_rust_checked_shift(expression: &str, operation: &str) -> bool {
    ((expression == "math") && ((operation == "readCheckedRustLeftShift") || (operation == "readCheckedRustRightShift")))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_shift_overflow_message(operation: &str) -> String {
    if ((self::read_rust_shift_checked_operation(operation)) == "readCheckedRustLeftShift") {
        String::from("attempt to shift left with overflow")
    } else {
        String::from("attempt to shift right with overflow")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_shift_width_bound(bits: f64) -> self::ml::Big {
    if (bits == (8f64)) {
        self::ml::Big::from_i128(8)
    } else {
        if (bits == (16f64)) {
            self::ml::Big::from_i128(16)
        } else {
            if (bits == (32f64)) {
                self::ml::Big::from_i128(32)
            } else {
                if (bits == (64f64)) {
                    self::ml::Big::from_i128(64)
                } else {
                    if (bits == (128f64)) {
                        self::ml::Big::from_i128(128)
                    } else {
                        self::ml::Big::from_i128(0)
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_shift_modulus(bits: f64) -> self::ml::Big {
    if (bits == (8f64)) {
        self::ml::Big::from_i128(256)
    } else {
        if (bits == (16f64)) {
            self::ml::Big::from_i128(65536)
        } else {
            if (bits == (32f64)) {
                self::ml::Big::from_i128(4294967296)
            } else {
                if (bits == (64f64)) {
                    self::ml::Big::from_i128(18446744073709551616)
                } else {
                    if (bits == (128f64)) {
                        self::ml::Big::parse("340282366920938463463374607431768211456")
                    } else {
                        self::ml::Big::from_i128(0)
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_checked_rust_shift(left: self::ml::Big, count: self::ml::Big, bits: f64, signed: bool, forward: bool) -> Option<self::ml::Big> {
    {
        let bound = self::read_rust_shift_width_bound(bits);
        if ((count < (self::ml::Big::from_i128(0))) || (count >= bound)) {
            None::<self::ml::Big>
        } else {
            Some({
                let value = if forward {
                    self::ml_big_integer_bitwise::apply_big_integer_bitwise_operation(3f64, left.clone(), count.clone())
                } else {
                    self::ml_big_integer_bitwise::apply_big_integer_bitwise_operation(4f64, left.clone(), count.clone())
                };
                {
                    let modulus = self::read_rust_shift_modulus(bits);
                    {
                        let remainder = self::ml::divide(&value, &modulus, "trunc", Some("Division by zero"), true);
                        {
                            let unsigned = if (remainder < (self::ml::Big::from_i128(0))) {
                                remainder.add(&modulus)
                            } else {
                                remainder.clone()
                            };
                            if (signed && (unsigned >= (self::ml::divide(&modulus, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)))) {
                                unsigned.sub(&modulus)
                            } else {
                                unsigned.clone()
                            }
                        }
                    }
                }
            })
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_shift_temporary_suffix(arguments_: &[String]) -> String {
    {
        let text = arguments_.join((String::from(" ")).as_str());
        {
            let suffix = 0f64;
            {
                let suffix_2 = self::ml_read_rust_shift_temporary_suffix_loop1(&(text), suffix);
                format!("{}{}", (String::from("")), (self::ml_number::js_number(suffix_2)))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_shift_integer_argument(target: &str, argument: &str, signed: bool) -> String {
    if (!signed && (target == "Lean")) {
        format!("{}{}", (format!("{}{}", (String::from("(Int.ofNat (")), argument.to_owned())), (String::from("))")))
    } else {
        if (!signed && (target == "Rocq")) {
            format!("{}{}", (format!("{}{}", (String::from("(Z.of_N (")), argument.to_owned())), (String::from("))")))
        } else {
            argument.to_owned()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_checked_shift(target: &str, operation: &str, arguments_: &[String], bits: f64, signed: bool, count_signed: bool, modulus: &str) -> String {
    if (!(self::accept_linked_rust_checked_shift("math", operation)) || (((arguments_.len() as f64)) != (2f64))) {
        String::from("")
    } else {
        {
            let suffix = self::read_rust_shift_temporary_suffix(arguments_);
            {
                let left = format!("{}{}", (String::from("ml_shift_left")), suffix);
                {
                    let count = format!("{}{}", (String::from("ml_shift_count")), suffix);
                    {
                        let first = (self::ml_array::optional_at(arguments_, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| String::from(""));
                        {
                            let second = (self::ml_array::optional_at(arguments_, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| String::from(""));
                            {
                                let forward = (operation == "readCheckedRustLeftShift");
                                if (target == "Rust") {
                                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("{ let ")), left)), (String::from(" = ")))), first)), (String::from("; let ")))), count)), (String::from(" = ")))), second)), (String::from("; ")))), (String::from("if ")))), (if count_signed {
                                        format!("{}{}", count, (String::from(" < 0 || ")))
                                    } else {
                                        String::from("")
                                    }))), (String::from("(")))), count)), (String::from(" as u128) >= ")))), (self::ml_number::js_number(bits)))), (String::from("u128 { None } else { Some(")))), left)), (String::from(".")))), (if forward {
                                        String::from("wrapping_shl")
                                    } else {
                                        String::from("wrapping_shr")
                                    }))), (String::from("(")))), count)), (String::from(" as u32)) } }")))
                                } else {
                                    if (target == "JavaScript") {
                                        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("((")), left)), (String::from(", ")))), count)), (String::from(") => ")))), count)), (String::from(" < 0n || ")))), count)), (String::from(" >= ")))), (self::ml_number::js_number(bits)))), (String::from("n ? undefined : BigInt.")))), (if signed {
                                            String::from("asIntN")
                                        } else {
                                            String::from("asUintN")
                                        }))), (String::from("(")))), (self::ml_number::js_number(bits)))), (String::from(", ")))), left)), (if forward {
                                            String::from(" << ")
                                        } else {
                                            String::from(" >> ")
                                        }))), count)), (String::from("))(")))), first)), (String::from(", ")))), second)), (String::from(")")))
                                    } else {
                                        if (target == "Lean") {
                                            {
                                                let divisor = format!("{}{}", (format!("{}{}", (String::from("(Int.ofNat (Nat.shiftLeft 1 (Int.toNat ")), count)), (String::from(")))")));
                                                {
                                                    let value = if forward {
                                                        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), left)), (String::from(" * ")))), divisor)), (String::from(")")))
                                                    } else {
                                                        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(Int.ediv ")), left)), (String::from(" ")))), divisor)), (String::from(")")))
                                                    };
                                                    {
                                                        let projected = if signed {
                                                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(if ")), value)), (String::from(" % (")))), modulus.to_owned())), (String::from(" : Int) < (")))), modulus.to_owned())), (String::from(" : Int) / 2 then ")))), value)), (String::from(" % (")))), modulus.to_owned())), (String::from(" : Int) else ")))), value)), (String::from(" % (")))), modulus.to_owned())), (String::from(" : Int) - (")))), modulus.to_owned())), (String::from(" : Int))")))
                                                        } else {
                                                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(Int.toNat (Int.emod ")), value)), (String::from(" (")))), modulus.to_owned())), (String::from(" : Int)))")))
                                                        };
                                                        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(let ")), left)), (String::from(" : Int := ")))), first)), (String::from("; let ")))), count)), (String::from(" : Int := ")))), second)), (String::from("; if ")))), count)), (String::from(" < 0 then none else if ")))), count)), (String::from(" >= (")))), (self::ml_number::js_number(bits)))), (String::from(" : Int) then none else some ")))), projected)), (String::from(")")))
                                                    }
                                                }
                                            }
                                        } else {
                                            if (target == "Rocq") {
                                                {
                                                    let value_2 = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), (if forward {
                                                        String::from("Z.shiftl")
                                                    } else {
                                                        String::from("Z.shiftr")
                                                    }))), (String::from(" ")))), left)), (String::from(" ")))), count)), (String::from(")")));
                                                    {
                                                        let residue = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(Z.modulo ")), value_2)), (String::from(" (")))), modulus.to_owned())), (String::from("%Z))")));
                                                        {
                                                            let projected_2 = if signed {
                                                                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(if Z.ltb ")), residue)), (String::from(" (Z.div (")))), modulus.to_owned())), (String::from("%Z) 2) then ")))), residue)), (String::from(" else Z.sub ")))), residue)), (String::from(" (")))), modulus.to_owned())), (String::from("%Z))")))
                                                            } else {
                                                                format!("{}{}", (format!("{}{}", (String::from("(Z.to_N ")), residue)), (String::from(")")))
                                                            };
                                                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(let ")), left)), (String::from(" : Z := ")))), first)), (String::from(" in let ")))), count)), (String::from(" : Z := ")))), second)), (String::from(" in if orb (Z.ltb ")))), count)), (String::from(" 0) (Z.leb ")))), (self::ml_number::js_number(bits)))), (String::from(" ")))), count)), (String::from(") then None else Some ")))), projected_2)), (String::from(")")))
                                                        }
                                                    }
                                                }
                                            } else {
                                                String::from("")
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_rust_shift_temporary_suffix_loop1(text: &str, mut suffix: f64) -> f64 {
    loop {
        return if (text.contains((format!("{}{}", (String::from("ml_shift_left")), (self::ml_number::js_number(suffix)))).as_str()) || text.contains((format!("{}{}", (String::from("ml_shift_count")), (self::ml_number::js_number(suffix)))).as_str())) {
            {
                let suffix_2 = (suffix + 1f64);
                {
                    suffix = suffix_2;
                    continue;
                }
            }
        } else {
            suffix
        };
    }
}
