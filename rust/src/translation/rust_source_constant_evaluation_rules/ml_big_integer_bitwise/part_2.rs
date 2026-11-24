// Generated from complete js/src/translation/rust-source-constant-evaluation-rules.js. Do not edit by hand.
use super::MlApplyBigIntegerBitwiseOperationLoop6Result;
use super::ml_number;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_apply_big_integer_bitwise_operation_loop6(
    mut operation: f64,
    mut negative_left: bool,
    mut negative_right: bool,
    mut first: self::ml::Big,
    mut second: self::ml::Big,
    mut weight: self::ml::Big,
    mut result: self::ml::Big,
) -> self::MlApplyBigIntegerBitwiseOperationLoop6Result {
    loop {
        if (first > (self::ml::Big::from_i128(0))) || (second > (self::ml::Big::from_i128(0))) {
            {
                let a = if negative_left {
                    (self::ml::Big::from_i128(1)).sub(&self::ml::divide(
                        &first,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        true,
                    ))
                } else {
                    self::ml::divide(
                        &first,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        true,
                    )
                };
                {
                    let b = if negative_right {
                        (self::ml::Big::from_i128(1)).sub(&self::ml::divide(
                            &second,
                            &self::ml::Big::from_i128(2),
                            "trunc",
                            Some("Division by zero"),
                            true,
                        ))
                    } else {
                        self::ml::divide(
                            &second,
                            &self::ml::Big::from_i128(2),
                            "trunc",
                            Some("Division by zero"),
                            true,
                        )
                    };
                    {
                        let selected = if operation == 0f64 {
                            (a == (self::ml::Big::from_i128(1)))
                                && (b == (self::ml::Big::from_i128(1)))
                        } else if operation == 1f64 {
                            (a == (self::ml::Big::from_i128(1)))
                                || (b == (self::ml::Big::from_i128(1)))
                        } else {
                            a != b
                        };
                        {
                            let result_3 = if selected {
                                { result.add(&weight) }
                            } else {
                                result.clone()
                            };
                            {
                                let first_2 = self::ml::divide(
                                    &first,
                                    &self::ml::Big::from_i128(2),
                                    "trunc",
                                    Some("Division by zero"),
                                    false,
                                );
                                {
                                    let second_2 = self::ml::divide(
                                        &second,
                                        &self::ml::Big::from_i128(2),
                                        "trunc",
                                        Some("Division by zero"),
                                        false,
                                    );
                                    {
                                        let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                                        {
                                            (first, second, weight, result) =
                                                (first_2, second_2, weight_2, result_3);
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
        if text
            .contains((format!("ml_integer_left{}", self::ml_number::js_number(suffix))).as_str())
            || text.contains(
                (format!("ml_integer_right{}", self::ml_number::js_number(suffix))).as_str(),
            )
        {
            {
                let suffix_2 = suffix + 1f64;
                {
                    suffix = suffix_2;
                    continue;
                }
            }
        }
        return { suffix };
    }
}
