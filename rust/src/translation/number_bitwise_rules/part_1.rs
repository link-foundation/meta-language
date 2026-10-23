// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_java_script_unsigned_integer(value: f64) -> f64 {
    if !(value.is_finite()) || (value == 0f64) {
        0f64
    } else {
        {
            let integral = if value < 0f64 {
                -((-value).floor())
            } else {
                value.floor()
            };
            {
                let remainder = integral % 4_294_967_296f64;
                if remainder < 0f64 {
                    remainder + 4_294_967_296f64
                } else if remainder == 0f64 {
                    0f64
                } else {
                    remainder
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_java_script_signed_integer(value: f64) -> f64 {
    {
        let unsigned = self::normalize_java_script_unsigned_integer(value);
        if unsigned >= 2_147_483_648f64 {
            unsigned - 4_294_967_296f64
        } else {
            unsigned
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_bitwise_operation_code(operation: &str) -> f64 {
    if operation == "conjoinJavaScriptNumberBits" {
        0f64
    } else if operation == "disjoinJavaScriptNumberBits" {
        1f64
    } else if operation == "excludeEqualJavaScriptNumberBits" {
        2f64
    } else if operation == "shiftJavaScriptNumberBitsLeft" {
        3f64
    } else if operation == "shiftJavaScriptNumberBitsRight" {
        4f64
    } else if operation == "shiftJavaScriptNumberBitsRightUnsigned" {
        5f64
    } else if operation == "complementJavaScriptNumberBits" {
        6f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_bitwise_operation(symbol: &str) -> String {
    if symbol == "&" {
        String::from("conjoinJavaScriptNumberBits")
    } else if symbol == "|" {
        String::from("disjoinJavaScriptNumberBits")
    } else if symbol == "^" {
        String::from("excludeEqualJavaScriptNumberBits")
    } else if symbol == "<<" {
        String::from("shiftJavaScriptNumberBitsLeft")
    } else if symbol == ">>" {
        String::from("shiftJavaScriptNumberBitsRight")
    } else if symbol == ">>>" {
        String::from("shiftJavaScriptNumberBitsRightUnsigned")
    } else if symbol == "~" {
        String::from("complementJavaScriptNumberBits")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_bitwise_symbol(operation: &str) -> String {
    if operation == "conjoinJavaScriptNumberBits" {
        String::from("&")
    } else if operation == "disjoinJavaScriptNumberBits" {
        String::from("|")
    } else if operation == "excludeEqualJavaScriptNumberBits" {
        String::from("^")
    } else if operation == "shiftJavaScriptNumberBitsLeft" {
        String::from("<<")
    } else if operation == "shiftJavaScriptNumberBitsRight" {
        String::from(">>")
    } else if operation == "shiftJavaScriptNumberBitsRightUnsigned" {
        String::from(">>>")
    } else if operation == "complementJavaScriptNumberBits" {
        String::from("~")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_bitwise_arity(operation: &str) -> f64 {
    {
        let code = self::read_java_script_number_bitwise_operation_code(operation);
        if code == 6f64 {
            1f64
        } else if code >= 0f64 {
            2f64
        } else {
            0f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_number_bitwise_operation(kind: &str, operation: &str) -> bool {
    (kind == "math") && ((self::read_java_script_number_bitwise_arity(operation)) > 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_number_bitwise_operand_count(operation: &str, count: f64) -> bool {
    {
        let arity = self::read_java_script_number_bitwise_arity(operation);
        ((arity > 0f64) && ((count).partial_cmp(&(arity)) == Some(std::cmp::Ordering::Equal)))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_number_bitwise_operation(operation: f64, left: f64, right: f64) -> f64 {
    if operation == 6f64 {
        -1f64 - self::normalize_java_script_signed_integer(left)
    } else if ((operation == 3f64) || (operation == 4f64)) || (operation == 5f64) {
        {
            let count = self::normalize_java_script_unsigned_integer(right) % 32f64;
            {
                let factor = 1f64;
                {
                    let position = 0f64;
                    {
                        let factor_2 = self::ml_apply_java_script_number_bitwise_operation_loop1(
                            count, factor, position,
                        );
                        if operation == 3f64 {
                            self::normalize_java_script_signed_integer(
                                self::normalize_java_script_unsigned_integer(left) * factor_2,
                            )
                        } else if operation == 4f64 {
                            (self::normalize_java_script_signed_integer(left) / factor_2).floor()
                        } else {
                            (self::normalize_java_script_unsigned_integer(left) / factor_2).floor()
                        }
                    }
                }
            }
        }
    } else if (((operation == 0f64) || (operation == 1f64)) || (operation == 2f64)) {
        {
            let first = self::normalize_java_script_unsigned_integer(left);
            {
                let second = self::normalize_java_script_unsigned_integer(right);
                {
                    let result = 0f64;
                    {
                        let weight = 1f64;
                        {
                            let position_2 = 0f64;
                            {
                                let result_2 =
                                    self::ml_apply_java_script_number_bitwise_operation_loop2(
                                        operation, first, second, result, weight, position_2,
                                    );
                                self::normalize_java_script_signed_integer(result_2)
                            }
                        }
                    }
                }
            }
        }
    } else {
        f64::NAN
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
pub fn ml_apply_java_script_number_bitwise_operation_loop1(
    mut count: f64,
    mut factor: f64,
    mut position: f64,
) -> f64 {
    loop {
        if position < count {
            {
                let factor_2 = factor * 2f64;
                {
                    let position_2 = position + 1f64;
                    {
                        (factor, position) = (factor_2, position_2);
                        continue;
                    }
                }
            }
        }
        return { factor };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_apply_java_script_number_bitwise_operation_loop2(
    mut operation: f64,
    mut first: f64,
    mut second: f64,
    mut result: f64,
    mut weight: f64,
    mut position: f64,
) -> f64 {
    loop {
        if position < 32f64 {
            {
                let first_bit = first % 2f64;
                {
                    let second_bit = second % 2f64;
                    {
                        let result_3 = if ((((operation == 0f64) && (first_bit == 1f64))
                            && (second_bit == 1f64))
                            || ((operation == 1f64)
                                && ((first_bit == 1f64) || (second_bit == 1f64))))
                            || ((operation == 2f64) && (first_bit != second_bit))
                        {
                            { result + weight }
                        } else {
                            result
                        };
                        {
                            let first_2 = (first - first_bit) / 2f64;
                            {
                                let second_2 = (second - second_bit) / 2f64;
                                {
                                    let weight_2 = weight * 2f64;
                                    {
                                        let position_2 = position + 1f64;
                                        {
                                            (first, second, result, weight, position) =
                                                (first_2, second_2, result_3, weight_2, position_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}
