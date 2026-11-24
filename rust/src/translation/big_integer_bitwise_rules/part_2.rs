// Generated from complete js/src/translation/big-integer-bitwise-rules.js. Do not edit by hand.
use super::ml;
use super::ml_number;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_apply_big_integer_bitwise_operation_loop4(mut count: self::ml::Big, mut value: self::ml::Big, mut factor: self::ml::Big) -> self::ml::Big {
    loop {
        if count > (self::ml::Big::from_i128(0)) {
            {
                let value_3 = if (self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1)) {
                    {
                        value.mul(&factor)
                    }
                } else {
                    value.clone()
                };
                {
                    let count_2 = self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if count_2 > (self::ml::Big::from_i128(0)) {
                        {
                            let factor_2 = factor.mul(&factor);
                            {
                                (count, value, factor) = (count_2.clone(), value_3.clone(), factor_2);
                                continue;
                            }
                        }
                    }
{
                        {
                            (count, value) = (count_2.clone(), value_3.clone());
                            continue;
                        }
                    }
                }
            }
        }
return {
            value.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_apply_big_integer_bitwise_operation_loop5(mut count: self::ml::Big, mut value: self::ml::Big) -> self::ml::Big {
    loop {
        if ((count > (self::ml::Big::from_i128(0))) && (value != (self::ml::Big::from_i128(0)))) && (value != (self::ml::Big::from_i128(-1))) {
            {
                let value_2 = if value < (self::ml::Big::from_i128(0)) {
                    self::ml::divide(&value.sub(&self::ml::Big::from_i128(1)), &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                } else {
                    self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                };
                {
                    let count_2 = count.sub(&self::ml::Big::from_i128(1));
                    {
                        (count, value) = (count_2, value_2);
                        continue;
                    }
                }
            }
        }
return {
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
        if (first > (self::ml::Big::from_i128(0))) || (second > (self::ml::Big::from_i128(0))) {
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
                        let selected = if operation == 0f64 {
                            (a == (self::ml::Big::from_i128(1))) && (b == (self::ml::Big::from_i128(1)))
                        } else if operation == 1f64 {
                                (a == (self::ml::Big::from_i128(1))) || (b == (self::ml::Big::from_i128(1)))
                            } else {
                                a != b
                            };
                        {
                            let result_3 = if selected {
                                {
                                    result.add(&weight)
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
                                            (first, second, weight, result) = (first_2, second_2, weight_2, result_3);
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
return {
            self::MlApplyBigIntegerBitwiseOperationLoop6Result::MlApplyBigIntegerBitwiseOperationLoop6Done(weight.clone(), result.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_render_big_integer_bitwise_expression_loop7(text: &str, mut suffix: f64) -> f64 {
    loop {
        if text.contains((format!("ml_integer_left{}", self::ml_number::js_number(suffix))).as_str()) || text.contains((format!("ml_integer_right{}", self::ml_number::js_number(suffix))).as_str()) {
            {
                let suffix_2 = suffix + 1f64;
                {
                    suffix = suffix_2;
                    continue;
                }
            }
        }
return {
            suffix
        };
    }
}
