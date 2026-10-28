// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::MlSource1TemporaryAcceptStringNumberInfinityLoop29Result;
use super::MlSource1TemporaryConvertStringNumberRationalJoin21;
use super::MlSource1TemporaryConvertStringNumberRationalJoin22;
use super::MlSource1TemporaryConvertStringNumberRationalLoop23Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop24Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop27Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop28Result;
use super::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result;
use super::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Result;
use super::MlSource1TemporaryReadStringNumberBinaryPowerLoop19Result;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::read_big_integer_bitwise_arity;
use super::read_big_integer_bitwise_operation_code;
use super::source_ml_source1_temporary_accept_string_number_infinity_loop29;
use super::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop11;
use super::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop12;
use super::source_ml_source1_temporary_convert_string_number_rational_loop23;
use super::source_ml_source1_temporary_convert_string_number_rational_loop24;
use super::source_ml_source1_temporary_convert_string_number_rational_loop25;
use super::source_ml_source1_temporary_convert_string_number_rational_loop26;
use super::source_ml_source1_temporary_convert_string_number_rational_loop27;
use super::source_ml_source1_temporary_convert_string_number_rational_loop28;
use super::source_ml_source1_temporary_read_java_script_utf8_string_units_loop13;
use super::source_ml_source1_temporary_read_java_script_utf8_string_units_loop14;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop19;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop20;
use super::source_ml_source1_temporary_read_string_number_decimal_power_loop17;
use super::source_ml_source1_temporary_read_string_number_exact_mantissa_loop18;
use super::source_ml_source1_temporary_render_big_integer_bitwise_expression_loop10;

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
                                            let suffix_2 = self::source_ml_source1_temporary_render_big_integer_bitwise_expression_loop10(&(text), suffix);
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function1(value: f64) -> f64 {
    if (!(value.is_finite()) || (value == (0f64))) {
        0f64
    } else {
        {
            let integral = if (value < (0f64)) {
                (-(((-value)).floor()))
            } else {
                value.floor()
            };
            {
                let remainder = (integral % 4294967296f64);
                if (remainder < (0f64)) {
                    (remainder + 4294967296f64)
                } else {
                    if (remainder == (0f64)) {
                        0f64
                    } else {
                        remainder
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_java_script_signed_integer(value: f64) -> f64 {
    {
        let unsigned = self::dependency_function1(value);
        if (unsigned >= (2147483648f64)) {
            (unsigned - 4294967296f64)
        } else {
            unsigned
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_bitwise_operation_code(operation: &str) -> f64 {
    if (operation == "conjoinJavaScriptNumberBits") {
        0f64
    } else {
        if (operation == "disjoinJavaScriptNumberBits") {
            1f64
        } else {
            if (operation == "excludeEqualJavaScriptNumberBits") {
                2f64
            } else {
                if (operation == "shiftJavaScriptNumberBitsLeft") {
                    3f64
                } else {
                    if (operation == "shiftJavaScriptNumberBitsRight") {
                        4f64
                    } else {
                        if (operation == "shiftJavaScriptNumberBitsRightUnsigned") {
                            5f64
                        } else {
                            if (operation == "complementJavaScriptNumberBits") {
                                6f64
                            } else {
                                (-1f64)
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
pub fn read_java_script_number_bitwise_operation(symbol: &str) -> String {
    if (symbol == "&") {
        String::from("conjoinJavaScriptNumberBits")
    } else {
        if (symbol == "|") {
            String::from("disjoinJavaScriptNumberBits")
        } else {
            if (symbol == "^") {
                String::from("excludeEqualJavaScriptNumberBits")
            } else {
                if (symbol == "<<") {
                    String::from("shiftJavaScriptNumberBitsLeft")
                } else {
                    if (symbol == ">>") {
                        String::from("shiftJavaScriptNumberBitsRight")
                    } else {
                        if (symbol == ">>>") {
                            String::from("shiftJavaScriptNumberBitsRightUnsigned")
                        } else {
                            if (symbol == "~") {
                                String::from("complementJavaScriptNumberBits")
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_bitwise_assignment(symbol: &str) -> String {
    if (symbol == "&=") {
        self::read_java_script_number_bitwise_operation("&")
    } else {
        if (symbol == "|=") {
            self::read_java_script_number_bitwise_operation("|")
        } else {
            if (symbol == "^=") {
                self::read_java_script_number_bitwise_operation("^")
            } else {
                if (symbol == "<<=") {
                    self::read_java_script_number_bitwise_operation("<<")
                } else {
                    if (symbol == ">>=") {
                        self::read_java_script_number_bitwise_operation(">>")
                    } else {
                        if (symbol == ">>>=") {
                            self::read_java_script_number_bitwise_operation(">>>")
                        } else {
                            String::from("")
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_bitwise_symbol(operation: &str) -> String {
    if (operation == "conjoinJavaScriptNumberBits") {
        String::from("&")
    } else {
        if (operation == "disjoinJavaScriptNumberBits") {
            String::from("|")
        } else {
            if (operation == "excludeEqualJavaScriptNumberBits") {
                String::from("^")
            } else {
                if (operation == "shiftJavaScriptNumberBitsLeft") {
                    String::from("<<")
                } else {
                    if (operation == "shiftJavaScriptNumberBitsRight") {
                        String::from(">>")
                    } else {
                        if (operation == "shiftJavaScriptNumberBitsRightUnsigned") {
                            String::from(">>>")
                        } else {
                            if (operation == "complementJavaScriptNumberBits") {
                                String::from("~")
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_bitwise_arity(operation: &str) -> f64 {
    {
        let code = self::read_java_script_number_bitwise_operation_code(operation);
        if (code == (6f64)) {
            1f64
        } else {
            if (code >= (0f64)) {
                2f64
            } else {
                0f64
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_number_bitwise_operation(kind: &str, operation: &str) -> bool {
    ((kind == "math") && ((self::read_java_script_number_bitwise_arity(operation)) > (0f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_number_bitwise_operand_count(operation: &str, count: f64) -> bool {
    {
        let arity = self::read_java_script_number_bitwise_arity(operation);
        ((arity > (0f64)) && (count == arity))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_number_bitwise_precedence(symbol: &str, level: f64) -> bool {
    ((((level == (0f64)) && (symbol == "|")) || ((level == (1f64)) && (symbol == "^"))) || ((level == (2f64)) && (symbol == "&")))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_number_bitwise_operation(operation: f64, left: f64, right: f64) -> f64 {
    if (operation == (6f64)) {
        ((-1f64) - self::normalize_java_script_signed_integer(left))
    } else {
        if (((operation == (3f64)) || (operation == (4f64))) || (operation == (5f64))) {
            {
                let count = (self::dependency_function1(right) % 32f64);
                {
                    let factor = 1f64;
                    {
                        let position = 0f64;
                        {
                            let factor_2 = self::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop11(count, factor, position);
                            if (operation == (3f64)) {
                                self::normalize_java_script_signed_integer((self::dependency_function1(left) * factor_2))
                            } else {
                                if (operation == (4f64)) {
                                    ((self::normalize_java_script_signed_integer(left) / factor_2)).floor()
                                } else {
                                    ((self::dependency_function1(left) / factor_2)).floor()
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if !((((operation == (0f64)) || (operation == (1f64))) || (operation == (2f64)))) {
                f64::NAN
            } else {
                {
                    let first = self::dependency_function1(left);
                    {
                        let second = self::dependency_function1(right);
                        {
                            let result = 0f64;
                            {
                                let weight = 1f64;
                                {
                                    let position_2 = 0f64;
                                    {
                                        let result_2 = self::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop12(operation, first, second, result, weight, position_2);
                                        self::normalize_java_script_signed_integer(result_2)
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
pub fn conjoin_java_script_number_bits(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(0f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn disjoin_java_script_number_bits(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(1f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn exclude_equal_java_script_number_bits(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(2f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn shift_java_script_number_bits_left(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(3f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn shift_java_script_number_bits_right(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(4f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn shift_java_script_number_bits_right_unsigned(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(5f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn complement_java_script_number_bits(left: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(6f64, left, 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_whitespace_unit(unit: f64) -> bool {
    ((((((((((((unit >= (9f64)) && (unit <= (13f64))) || (unit == (32f64))) || (unit == (160f64))) || (unit == (5760f64))) || ((unit >= (8192f64)) && (unit <= (8202f64)))) || (unit == (8232f64))) || (unit == (8233f64))) || (unit == (8239f64))) || (unit == (8287f64))) || (unit == (12288f64))) || (unit == (65279f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_numeric_digit_unit(unit: f64) -> f64 {
    if ((unit >= (48f64)) && (unit <= (57f64))) {
        (unit - 48f64)
    } else {
        if ((unit >= (65f64)) && (unit <= (70f64))) {
            (unit - 55f64)
        } else {
            if ((unit >= (97f64)) && (unit <= (102f64))) {
                (unit - 87f64)
            } else {
                (-1f64)
            }
        }
    }
}

pub fn read_java_script_utf8_string_units(bytes: Vec<f64>) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s138 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop13(bytes.clone(), index);
            match source_ml_source2_s138.clone() {
                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Done => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source2_s139 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop14(bytes, units, index_2);
                                match source_ml_source2_s139.clone() {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Done(units_2) => {
                                        units_2
                                    }
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Return(source_ml_source1_temporary_result14) => {
                                        source_ml_source1_temporary_result14
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Return(source_ml_source1_temporary_result16) => {
                    source_ml_source1_temporary_result16
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_digit(unit: f64) -> f64 {
    self::read_java_script_numeric_digit_unit(unit)
}

pub fn read_string_number_decimal_power(exponent: f64) -> self::ml::Big {
    {
        let value = self::ml::Big::from_i128(1);
        {
            let base = self::ml::Big::from_i128(10);
            {
                let value_2 = self::source_ml_source1_temporary_read_string_number_decimal_power_loop17(exponent, value.clone(), base.clone());
                value_2.clone()
            }
        }
    }
}

pub fn read_string_number_exact_mantissa(value: self::ml::Big) -> f64 {
    {
        let result = 0f64;
        {
            let scale = 1f64;
            {
                let result_2 = self::source_ml_source1_temporary_read_string_number_exact_mantissa_loop18(value.clone(), result, scale);
                result_2
            }
        }
    }
}

pub fn read_string_number_binary_power(exponent: f64) -> f64 {
    {
        let result = 1f64;
        {
            let source_ml_source2_s140 = self::source_ml_source1_temporary_read_string_number_binary_power_loop19(exponent, result);
            match source_ml_source2_s140.clone() {
                self::MlSource1TemporaryReadStringNumberBinaryPowerLoop19Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop19Done(exponent_2, result_2) => {
                    {
                        let result_3 = self::source_ml_source1_temporary_read_string_number_binary_power_loop20(exponent_2, result_2);
                        result_3
                    }
                }
            }
        }
    }
}

pub fn convert_string_number_rational(numerator: self::ml::Big, denominator: self::ml::Big) -> f64 {
    if ((numerator < (self::ml::Big::from_i128(0))) || (denominator <= (self::ml::Big::from_i128(0)))) {
        f64::NAN
    } else {
        if (numerator == (self::ml::Big::from_i128(0))) {
            0f64
        } else {
            {
                let exponent = 0f64;
                {
                    let source_ml_source2_s143 = if (numerator >= denominator) {
                        {
                            let scale = denominator.clone();
                            {
                                let source_ml_source2_s144 = self::source_ml_source1_temporary_convert_string_number_rational_loop27(numerator.clone(), exponent, scale.clone());
                                match source_ml_source2_s144.clone() {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop27Result::MlSource1TemporaryConvertStringNumberRationalLoop27Done(exponent_2) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin21::MlSource1TemporaryConvertStringNumberRationalJoin21Next(exponent_2)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop27Result::MlSource1TemporaryConvertStringNumberRationalLoop27Return(source_ml_source1_temporary_result27) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin21::MlSource1TemporaryConvertStringNumberRationalJoin21Return(source_ml_source1_temporary_result27)
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let scale_2 = numerator.clone();
                            {
                                let source_ml_source2_s145 = self::source_ml_source1_temporary_convert_string_number_rational_loop28(denominator.clone(), exponent, scale_2.clone());
                                match source_ml_source2_s145.clone() {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop28Result::MlSource1TemporaryConvertStringNumberRationalLoop28Done(exponent_3) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin21::MlSource1TemporaryConvertStringNumberRationalJoin21Next(exponent_3)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop28Result::MlSource1TemporaryConvertStringNumberRationalLoop28Return(source_ml_source1_temporary_result28) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin21::MlSource1TemporaryConvertStringNumberRationalJoin21Return(source_ml_source1_temporary_result28)
                                    }
                                }
                            }
                        }
                    };
                    match source_ml_source2_s143.clone() {
                        self::MlSource1TemporaryConvertStringNumberRationalJoin21::MlSource1TemporaryConvertStringNumberRationalJoin21Next(exponent_4) => {
                            {
                                let shift = if (exponent_4 >= ((-1022f64))) {
                                    (52f64 - exponent_4)
                                } else {
                                    1074f64
                                };
                                {
                                    let source_ml_source2_s146 = if (shift >= (0f64)) {
                                        {
                                            let index = 0f64;
                                            {
                                                let numerator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop25(numerator.clone(), shift, index);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin22::MlSource1TemporaryConvertStringNumberRationalJoin22Next(numerator_2.clone(), denominator.clone())
                                            }
                                        }
                                    } else {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let denominator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop26(denominator.clone(), shift, index_2);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin22::MlSource1TemporaryConvertStringNumberRationalJoin22Next(numerator.clone(), denominator_2.clone())
                                            }
                                        }
                                    };
                                    match source_ml_source2_s146.clone() {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin22::MlSource1TemporaryConvertStringNumberRationalJoin22Next(numerator_3, denominator_3) => {
                                            {
                                                let quotient = self::ml::Big::from_i128(0);
                                                {
                                                    let remainder = numerator_3.clone();
                                                    {
                                                        let scale_3 = denominator_3.clone();
                                                        {
                                                            let weight = self::ml::Big::from_i128(1);
                                                            {
                                                                let source_ml_source2_s147 = self::source_ml_source1_temporary_convert_string_number_rational_loop23(remainder.clone(), scale_3.clone(), weight.clone());
                                                                match source_ml_source2_s147.clone() {
                                                                    self::MlSource1TemporaryConvertStringNumberRationalLoop23Result::MlSource1TemporaryConvertStringNumberRationalLoop23Done(scale_4, weight_2) => {
                                                                        {
                                                                            let source_ml_source2_s148 = self::source_ml_source1_temporary_convert_string_number_rational_loop24(quotient.clone(), remainder.clone(), scale_4.clone(), weight_2.clone());
                                                                            match source_ml_source2_s148.clone() {
                                                                                self::MlSource1TemporaryConvertStringNumberRationalLoop24Result::MlSource1TemporaryConvertStringNumberRationalLoop24Done(quotient_2, remainder_2) => {
                                                                                    {
                                                                                        let twice = remainder_2.mul(&self::ml::Big::from_i128(2));
                                                                                        {
                                                                                            let quotient_4 = if ((twice > denominator_3) || ((twice == denominator_3) && ((self::ml::divide(&quotient_2, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))))) {
                                                                                                {
                                                                                                    let quotient_3 = quotient_2.add(&self::ml::Big::from_i128(1));
                                                                                                    quotient_3.clone()
                                                                                                }
                                                                                            } else {
                                                                                                quotient_2.clone()
                                                                                            };
                                                                                            (self::read_string_number_exact_mantissa(quotient_4.clone()) * self::read_string_number_binary_power((-shift)))
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
                        self::MlSource1TemporaryConvertStringNumberRationalJoin21::MlSource1TemporaryConvertStringNumberRationalJoin21Return(source_ml_source1_temporary_result26) => {
                            source_ml_source1_temporary_result26
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_string_number_infinity(units: Vec<f64>, start: f64, end: f64) -> bool {
    if (((end - start)) != (8f64)) {
        false
    } else {
        {
            let expected = vec![73f64, 110f64, 102f64, 105f64, 110f64, 105f64, 116f64, 121f64];
            {
                let index = 0f64;
                {
                    let source_ml_source2_s149 = self::source_ml_source1_temporary_accept_string_number_infinity_loop29(units, start, expected, index);
                    match source_ml_source2_s149.clone() {
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop29Result::MlSource1TemporaryAcceptStringNumberInfinityLoop29Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop29Result::MlSource1TemporaryAcceptStringNumberInfinityLoop29Return(source_ml_source1_temporary_result29) => {
                            source_ml_source1_temporary_result29
                        }
                    }
                }
            }
        }
    }
}
