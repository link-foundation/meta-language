// Generated from complete js/src/translation/rust-source-constant-evaluation-rules.js. Do not edit by hand.
use super::MlTemporaryConstantFieldLoop23Result;
use super::MlTemporaryEvaluateRustConstantIntegerOperationJoin21;
use super::accept_rust_source_signature_primitive;
use super::ml;
use super::ml_array;
use super::ml_big_integer_bitwise;
use super::ml_big_integer_conversion;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::read_rust_constant_integer_decimal;
use super::source_ml_temporary_constant_domain_loop24;
use super::source_ml_temporary_constant_field_loop23;
use super::source_ml_temporary_evaluate_rust_constant_integer_operation_loop20;
use super::source_ml_temporary_evaluate_rust_constant_integer_operation_loop22;
use super::test_rust_integer_range_canonical_decimal;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn evaluate_rust_constant_integer_operation(
    operation: &self::ml_string::Text,
    left: &self::ml_string::Text,
    right: &self::ml_string::Text,
    minimum: &self::ml_string::Text,
    maximum: &self::ml_string::Text,
    bits: f64,
) -> Vec<self::ml_string::Text> {
    if (((!(self::test_rust_integer_range_canonical_decimal(left))
        || !(self::test_rust_integer_range_canonical_decimal(right)))
        || !(self::test_rust_integer_range_canonical_decimal(minimum)))
        || !(self::test_rust_integer_range_canonical_decimal(maximum)))
        || !(self::ml_array::includes(
            &vec![8f64, 16f64, 32f64, 64f64, 128f64],
            &bits,
            0f64,
            false,
            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
        ))
    {
        vec![
            self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32,
                101, 118, 97, 108, 117, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32,
                101, 120, 97, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 110, 100,
                32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 103, 101, 114,
                32, 100, 111, 109, 97, 105, 110,
            ]),
        ]
    } else {
        {
            let a = self::read_rust_constant_integer_decimal(left);
            {
                let b = self::read_rust_constant_integer_decimal(right);
                {
                    let low = self::read_rust_constant_integer_decimal(minimum);
                    {
                        let high = self::read_rust_constant_integer_decimal(maximum);
                        {
                            let modulus = self::ml::Big::from_i128(1);
                            {
                                let at = 0f64;
                                {
                                    let modulus_2 = self::source_ml_temporary_evaluate_rust_constant_integer_operation_loop20(bits, modulus, at);
                                    {
                                        let expected_low = if low < (self::ml::Big::from_i128(0)) {
                                            (self::ml::divide(
                                                &modulus_2,
                                                &self::ml::Big::from_i128(2),
                                                "trunc",
                                                Some("Division by zero"),
                                                false,
                                            ))
                                            .neg()
                                        } else {
                                            self::ml::Big::from_i128(0)
                                        };
                                        {
                                            let expected_high =
                                                if low < (self::ml::Big::from_i128(0)) {
                                                    (self::ml::divide(
                                                        &modulus_2,
                                                        &self::ml::Big::from_i128(2),
                                                        "trunc",
                                                        Some("Division by zero"),
                                                        false,
                                                    ))
                                                    .sub(&self::ml::Big::from_i128(1))
                                                } else {
                                                    modulus_2.sub(&self::ml::Big::from_i128(1))
                                                };
                                            if (low != expected_low) || (high != expected_high) {
                                                vec![
                                                    self::ml_string::Text::from_units(vec![
                                                        101, 114, 114, 111, 114,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        99, 111, 110, 115, 116, 97, 110, 116, 32,
                                                        105, 110, 116, 101, 103, 101, 114, 32, 101,
                                                        118, 97, 108, 117, 97, 116, 105, 111, 110,
                                                        32, 110, 101, 101, 100, 115, 32, 97, 110,
                                                        32, 97, 99, 116, 117, 97, 108, 32, 105,
                                                        110, 116, 101, 103, 101, 114, 32, 100, 111,
                                                        109, 97, 105, 110,
                                                    ]),
                                                ]
                                            } else if ((*operation)
                                                != (self::ml_string::Text::from_units(vec![
                                                    99, 97, 115, 116,
                                                ])))
                                                && ((a < low) || (a > high))
                                            {
                                                vec![
                                                    self::ml_string::Text::from_units(vec![
                                                        101, 114, 114, 111, 114,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        97, 32, 99, 111, 110, 115, 116, 97, 110,
                                                        116, 32, 105, 110, 116, 101, 103, 101, 114,
                                                        32, 111, 112, 101, 114, 97, 110, 100, 32,
                                                        105, 115, 32, 111, 117, 116, 115, 105, 100,
                                                        101, 32, 105, 116, 115, 32, 97, 99, 116,
                                                        117, 97, 108, 32, 115, 111, 117, 114, 99,
                                                        101, 32, 100, 111, 109, 97, 105, 110,
                                                    ]),
                                                ]
                                            } else {
                                                {
                                                    let shift = ((*operation)
                                                        == (self::ml_string::Text::from_units(
                                                            vec![115, 104, 108],
                                                        )))
                                                        || ((*operation)
                                                            == (self::ml_string::Text::from_units(
                                                                vec![115, 104, 114],
                                                            )));
                                                    {
                                                        let unary = (((*operation) == (self::ml_string::Text::from_units(vec![110, 101, 103]))) || ((*operation) == (self::ml_string::Text::from_units(vec![98, 105, 116, 110, 111, 116])))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 97, 115, 116])));
                                                        if (!shift && !unary)
                                                            && ((b < low) || (b > high))
                                                        {
                                                            vec![
                                                                self::ml_string::Text::from_units(
                                                                    vec![101, 114, 114, 111, 114],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        97, 32, 99, 111, 110, 115,
                                                                        116, 97, 110, 116, 32, 105,
                                                                        110, 116, 101, 103, 101,
                                                                        114, 32, 111, 112, 101,
                                                                        114, 97, 110, 100, 32, 105,
                                                                        115, 32, 111, 117, 116,
                                                                        115, 105, 100, 101, 32,
                                                                        105, 116, 115, 32, 97, 99,
                                                                        116, 117, 97, 108, 32, 115,
                                                                        111, 117, 114, 99, 101, 32,
                                                                        100, 111, 109, 97, 105,
                                                                        110,
                                                                    ],
                                                                ),
                                                            ]
                                                        } else if (*operation)
                                                            == (self::ml_string::Text::from_units(
                                                                vec![101, 113],
                                                            ))
                                                        {
                                                            vec![
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        98, 111, 111, 108, 101, 97,
                                                                        110,
                                                                    ],
                                                                ),
                                                                if a == b {
                                                                    self::ml_string::Text::from_units(vec![116, 114, 117, 101])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
                                                                },
                                                            ]
                                                        } else if (*operation)
                                                            == (self::ml_string::Text::from_units(
                                                                vec![110, 101],
                                                            ))
                                                        {
                                                            vec![
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        98, 111, 111, 108, 101, 97,
                                                                        110,
                                                                    ],
                                                                ),
                                                                if a != b {
                                                                    self::ml_string::Text::from_units(vec![116, 114, 117, 101])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
                                                                },
                                                            ]
                                                        } else if (*operation)
                                                            == (self::ml_string::Text::from_units(
                                                                vec![108, 116],
                                                            ))
                                                        {
                                                            vec![
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        98, 111, 111, 108, 101, 97,
                                                                        110,
                                                                    ],
                                                                ),
                                                                if a < b {
                                                                    self::ml_string::Text::from_units(vec![116, 114, 117, 101])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
                                                                },
                                                            ]
                                                        } else if (*operation)
                                                            == (self::ml_string::Text::from_units(
                                                                vec![108, 101],
                                                            ))
                                                        {
                                                            vec![
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        98, 111, 111, 108, 101, 97,
                                                                        110,
                                                                    ],
                                                                ),
                                                                if a <= b {
                                                                    self::ml_string::Text::from_units(vec![116, 114, 117, 101])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
                                                                },
                                                            ]
                                                        } else if (*operation)
                                                            == (self::ml_string::Text::from_units(
                                                                vec![103, 116],
                                                            ))
                                                        {
                                                            vec![
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        98, 111, 111, 108, 101, 97,
                                                                        110,
                                                                    ],
                                                                ),
                                                                if a > b {
                                                                    self::ml_string::Text::from_units(vec![116, 114, 117, 101])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
                                                                },
                                                            ]
                                                        } else if (*operation)
                                                            == (self::ml_string::Text::from_units(
                                                                vec![103, 101],
                                                            ))
                                                        {
                                                            vec![
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        98, 111, 111, 108, 101, 97,
                                                                        110,
                                                                    ],
                                                                ),
                                                                if a >= b {
                                                                    self::ml_string::Text::from_units(vec![116, 114, 117, 101])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
                                                                },
                                                            ]
                                                        } else {
                                                            {
                                                                let value =
                                                                    self::ml::Big::from_i128(0);
                                                                {
                                                                    let source_ml_source1_s36 = if (*operation) == (self::ml_string::Text::from_units(vec![97, 100, 100])) {
                                                                                                    {
                                                                                                        let value_2 = a.add(&b);
                                                                                                        self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Next(value_2)
                                                                                                    }
                                                                                                } else if (*operation) == (self::ml_string::Text::from_units(vec![115, 117, 98])) {
                                                                                                        {
                                                                                                            let value_3 = a.sub(&b);
                                                                                                            self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Next(value_3)
                                                                                                        }
                                                                                                    } else if (*operation) == (self::ml_string::Text::from_units(vec![109, 117, 108])) {
                                                                                                            {
                                                                                                                let value_4 = a.mul(&b);
                                                                                                                self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Next(value_4)
                                                                                                            }
                                                                                                        } else if ((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 118]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 109]))) {
                                                                                                                if b == (self::ml::Big::from_i128(0)) {
                                                                                                                    self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111, 32, 105, 110, 32, 97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110])])
                                                                                                                } else if ((low < (self::ml::Big::from_i128(0))) && (a == low)) && (b == (self::ml::Big::from_i128(-1))) {
                                                                                                                        self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 118, 101, 114, 102, 108, 111, 119, 32, 105, 110, 32, 97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110])])
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let value_5 = if (*operation) == (self::ml_string::Text::from_units(vec![100, 105, 118])) {
                                                                                                                                {
                                                                                                                                    let source_ml_source1_integer_division37 = self::ml_big_integer_conversion::divide_java_script_big_integer(a.clone(),b.clone());
                                                                                                                                    if (source_ml_source1_integer_division37.len() as f64) == 2f64 {
                                                                                                                                        (self::ml_array::optional_at(&source_ml_source1_integer_division37, self::ml_array::number_index(0f64))).unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                                                                                    } else {
                                                                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111])).0, message_present: true }).raise()
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                {
                                                                                                                                    let source_ml_source1_integer_division38 = self::ml_big_integer_conversion::divide_java_script_big_integer(a.clone(),b.clone());
                                                                                                                                    if (source_ml_source1_integer_division38.len() as f64) == 2f64 {
                                                                                                                                        (self::ml_array::optional_at(&source_ml_source1_integer_division38, self::ml_array::number_index(1f64))).unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                                                                                    } else {
                                                                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111])).0, message_present: true }).raise()
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            };
                                                                                                                            self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Next(value_5)
                                                                                                                        }
                                                                                                                    }
                                                                                                            } else if (*operation) == (self::ml_string::Text::from_units(vec![110, 101, 103])) {
                                                                                                                    if low == (self::ml::Big::from_i128(0)) {
                                                                                                                        self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 99, 97, 110, 110, 111, 116, 32, 117, 115, 101, 32, 117, 110, 97, 114, 121, 32, 110, 101, 103, 97, 116, 105, 111, 110])])
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let value_6 = a.neg();
                                                                                                                            self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Next(value_6)
                                                                                                                        }
                                                                                                                    }
                                                                                                                } else if (*operation) == (self::ml_string::Text::from_units(vec![98, 105, 116, 110, 111, 116])) {
                                                                                                                        {
                                                                                                                            let value_7 = if low == (self::ml::Big::from_i128(0)) {
                                                                                                                                self::ml_big_integer_bitwise::apply_big_integer_bitwise_operation(0f64, self::ml_big_integer_bitwise::apply_big_integer_bitwise_operation(6f64, a.clone(), ml::Big::zero()), high.clone())
                                                                                                                            } else {
                                                                                                                                self::ml_big_integer_bitwise::apply_big_integer_bitwise_operation(6f64, a.clone(), ml::Big::zero())
                                                                                                                            };
                                                                                                                            self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Next(value_7)
                                                                                                                        }
                                                                                                                    } else if (*operation) == (self::ml_string::Text::from_units(vec![98, 105, 116, 97, 110, 100])) {
                                                                                                                            {
                                                                                                                                let value_8 = self::ml_big_integer_bitwise::apply_big_integer_bitwise_operation(0f64, a.clone(), b.clone());
                                                                                                                                self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Next(value_8)
                                                                                                                            }
                                                                                                                        } else if (*operation) == (self::ml_string::Text::from_units(vec![98, 105, 116, 111, 114])) {
                                                                                                                                {
                                                                                                                                    let value_9 = self::ml_big_integer_bitwise::apply_big_integer_bitwise_operation(1f64, a.clone(), b.clone());
                                                                                                                                    self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Next(value_9)
                                                                                                                                }
                                                                                                                            } else if (*operation) == (self::ml_string::Text::from_units(vec![98, 105, 116, 120, 111, 114])) {
                                                                                                                                    {
                                                                                                                                        let value_10 = self::ml_big_integer_bitwise::apply_big_integer_bitwise_operation(2f64, a.clone(), b.clone());
                                                                                                                                        self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Next(value_10)
                                                                                                                                    }
                                                                                                                                } else if shift {
                                                                                                                                        {
                                                                                                                                            let width = self::ml::Big::from_i128(0);
                                                                                                                                            {
                                                                                                                                                let at_2 = 0f64;
                                                                                                                                                {
                                                                                                                                                    let width_2 = self::source_ml_temporary_evaluate_rust_constant_integer_operation_loop22(bits, width, at_2);
                                                                                                                                                    if (b < (self::ml::Big::from_i128(0))) || (b >= width_2) {
                                                                                                                                                        self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 118, 101, 114, 102, 108, 111, 119, 32, 105, 110, 32, 97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 115, 104, 105, 102, 116])])
                                                                                                                                                    } else {
                                                                                                                                                        {
                                                                                                                                                            let value_11 = if (*operation) == (self::ml_string::Text::from_units(vec![115, 104, 108])) {
                                                                                                                                                                self::ml_big_integer_bitwise::apply_big_integer_bitwise_operation(3f64, a.clone(), b.clone())
                                                                                                                                                            } else {
                                                                                                                                                                self::ml_big_integer_bitwise::apply_big_integer_bitwise_operation(4f64, a.clone(), b.clone())
                                                                                                                                                            };
                                                                                                                                                            self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Next(value_11)
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    } else if (*operation) == (self::ml_string::Text::from_units(vec![99, 97, 115, 116])) {
                                                                                                                                            {
                                                                                                                                                let value_12 = a.clone();
                                                                                                                                                self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Next(value_12)
                                                                                                                                            }
                                                                                                                                        } else {
                                                                                                                                            self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 116, 101, 114, 112, 114, 101, 116, 97, 116, 105, 111, 110])])
                                                                                                                                        };
                                                                    match source_ml_source1_s36 {
                                                                                                    self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Next(value_13) => {
                                                                                                        {
                                                                                                            let value_16 = if ((*operation) == (self::ml_string::Text::from_units(vec![99, 97, 115, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 108]))) {
                                                                                                                {
                                                                                                                    let source_ml_shadow_modulus_1 = (high.sub(&low)).add(&self::ml::Big::from_i128(1));
                                                                                                                    {
                                                                                                                        let value_14 = {
                                                                                                                            let source_ml_source1_integer_division40 = self::ml_big_integer_conversion::divide_java_script_big_integer(({
                                                                                                                                let source_ml_source1_integer_division39 = self::ml_big_integer_conversion::divide_java_script_big_integer(value_13.clone(),source_ml_shadow_modulus_1.clone());
                                                                                                                                if (source_ml_source1_integer_division39.len() as f64) == 2f64 {
                                                                                                                                    (self::ml_array::optional_at(&source_ml_source1_integer_division39, self::ml_array::number_index(1f64))).unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                                                                                } else {
                                                                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111])).0, message_present: true }).raise()
                                                                                                                                }
                                                                                                                            }).add(&source_ml_shadow_modulus_1),source_ml_shadow_modulus_1.clone());
                                                                                                                            if (source_ml_source1_integer_division40.len() as f64) == 2f64 {
                                                                                                                                (self::ml_array::optional_at(&source_ml_source1_integer_division40, self::ml_array::number_index(1f64))).unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                                                                            } else {
                                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111])).0, message_present: true }).raise()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        if value_14 > high {
                                                                                                                            {
                                                                                                                                value_14.sub(&source_ml_shadow_modulus_1)
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            value_14.clone()
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                value_13.clone()
                                                                                                            };
                                                                                                            if (value_16 < low) || (value_16 > high) {
                                                                                                                vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 118, 101, 114, 102, 108, 111, 119, 32, 105, 110, 32, 97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110])]
                                                                                                            } else {
                                                                                                                vec![self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114]), {
                                                                                                                    let source_ml_source1_string_argument41 = value_16.clone();
                                                                                                                    self::ml_string::Text::from_text(&source_ml_source1_string_argument41.to_string())
                                                                                                                }]
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    self::MlTemporaryEvaluateRustConstantIntegerOperationJoin21::MlTemporaryEvaluateRustConstantIntegerOperationJoin21Return(source_ml_temporary_result21) => {
                                                                                                        source_ml_temporary_result21
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn evaluate_rust_constant_boolean_operation(
    operation: &self::ml_string::Text,
    left: bool,
    right: bool,
) -> Vec<self::ml_string::Text> {
    if ((*operation) == (self::ml_string::Text::from_units(vec![110, 111, 116])))
        || ((*operation) == (self::ml_string::Text::from_units(vec![98, 105, 116, 110, 111, 116])))
    {
        vec![
            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]),
            if left {
                self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
            } else {
                self::ml_string::Text::from_units(vec![116, 114, 117, 101])
            },
        ]
    } else if ((*operation) == (self::ml_string::Text::from_units(vec![97, 110, 100])))
        || ((*operation) == (self::ml_string::Text::from_units(vec![98, 105, 116, 97, 110, 100])))
    {
        vec![
            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]),
            if left && right {
                self::ml_string::Text::from_units(vec![116, 114, 117, 101])
            } else {
                self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
            },
        ]
    } else if ((*operation) == (self::ml_string::Text::from_units(vec![111, 114])))
        || ((*operation) == (self::ml_string::Text::from_units(vec![98, 105, 116, 111, 114])))
    {
        vec![
            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]),
            if left || right {
                self::ml_string::Text::from_units(vec![116, 114, 117, 101])
            } else {
                self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
            },
        ]
    } else if (*operation) == (self::ml_string::Text::from_units(vec![101, 113])) {
        vec![
            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]),
            if left == right {
                self::ml_string::Text::from_units(vec![116, 114, 117, 101])
            } else {
                self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
            },
        ]
    } else if ((*operation) == (self::ml_string::Text::from_units(vec![110, 101])))
        || ((*operation) == (self::ml_string::Text::from_units(vec![98, 105, 116, 120, 111, 114])))
    {
        vec![
            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]),
            if left != right {
                self::ml_string::Text::from_units(vec![116, 114, 117, 101])
            } else {
                self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
            },
        ]
    } else {
        vec![
            self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                116, 104, 101, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 66, 111, 111, 108,
                101, 97, 110, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101,
                100, 115, 32, 105, 116, 115, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 116,
                101, 114, 112, 114, 101, 116, 97, 116, 105, 111, 110,
            ]),
        ]
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_rust_constant_boolean_right(operation: &self::ml_string::Text, left: bool) -> bool {
    if (*operation) == (self::ml_string::Text::from_units(vec![97, 110, 100])) {
        left
    } else if (*operation) == (self::ml_string::Text::from_units(vec![111, 114])) {
        !left
    } else {
        true
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_constant_pattern_evaluation_error(
    depth: f64,
    steps: f64,
    cycle: bool,
) -> self::ml_string::Text {
    if cycle {
        self::ml_string::Text::from_units(vec![
            97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 112, 97, 116, 116, 101, 114, 110, 32,
            100, 101, 112, 101, 110, 100, 101, 110, 99, 121, 32, 99, 97, 110, 110, 111, 116, 32,
            101, 118, 97, 108, 117, 97, 116, 101, 32, 105, 116, 115, 101, 108, 102, 32, 114, 101,
            99, 117, 114, 115, 105, 118, 101, 108, 121,
        ])
    } else if ((((self::ml_math::is_integer(depth) && (depth >= 0f64)) && (depth <= 256f64))
        && self::ml_math::is_integer(steps))
        && (steps >= 0f64))
        && (steps <= 65_536f64)
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            99, 111, 110, 115, 116, 97, 110, 116, 32, 112, 97, 116, 116, 101, 114, 110, 32, 101,
            118, 97, 108, 117, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32,
            105, 116, 115, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 115, 111, 117, 114, 99,
            101, 32, 98, 117, 100, 103, 101, 116,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_constant_primitive_alias_owner(
    kind: &self::ml_string::Text,
    owner_kind: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    arguments_count: f64,
    wrappers: f64,
    bits: &self::ml_string::Text,
    signed: bool,
) -> self::ml_string::Text {
    if (((((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
        && ((*owner_kind)
            == (self::ml_string::Text::from_units(vec![
                112, 114, 105, 109, 105, 116, 105, 118, 101,
            ]))))
        && (arguments_count == 0f64))
        && (wrappers == 0f64))
        && self::accept_rust_source_signature_primitive(kind, owner, bits, signed)
    {
        owner.clone()
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn constant_field(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    name: &self::ml_string::Text,
) -> f64 {
    if node < 0f64 {
        -1f64
    } else {
        {
            let start = self::read_required_number_fact(starts, node);
            {
                let count = self::read_required_number_fact(counts, node);
                {
                    let at = start;
                    {
                        let source_ml_source1_s42 = self::source_ml_temporary_constant_field_loop23(
                            labels, targets, name, start, count, at,
                        );
                        match source_ml_source1_s42 {
                            self::MlTemporaryConstantFieldLoop23Result::MlTemporaryConstantFieldLoop23Done => {
                                -1f64
                            }
                            self::MlTemporaryConstantFieldLoop23Result::MlTemporaryConstantFieldLoop23Return(source_ml_temporary_result23) => {
                                source_ml_temporary_result23
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
pub fn constant_text(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    name: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let child = self::constant_field(starts, counts, labels, targets, node, name);
        if child < 0f64 {
            self::ml_string::Text::from_units(vec![])
        } else {
            self::read_required_text_fact(texts, child)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn constant_element(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    node: f64,
    index: f64,
) -> f64 {
    if ((node < 0f64) || (index < 0f64))
        || (index >= (self::read_required_number_fact(counts, node)))
    {
        -1f64
    } else {
        self::read_required_number_fact(
            targets,
            (self::read_required_number_fact(starts, node) + index),
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn constant_bits(owner: &self::ml_string::Text) -> f64 {
    if ((*owner) == (self::ml_string::Text::from_units(vec![117, 56])))
        || ((*owner) == (self::ml_string::Text::from_units(vec![105, 56])))
    {
        8f64
    } else if ((*owner) == (self::ml_string::Text::from_units(vec![117, 49, 54])))
        || ((*owner) == (self::ml_string::Text::from_units(vec![105, 49, 54])))
    {
        16f64
    } else if ((*owner) == (self::ml_string::Text::from_units(vec![117, 51, 50])))
        || ((*owner) == (self::ml_string::Text::from_units(vec![105, 51, 50])))
    {
        32f64
    } else if ((((*owner) == (self::ml_string::Text::from_units(vec![117, 54, 52])))
        || ((*owner) == (self::ml_string::Text::from_units(vec![105, 54, 52]))))
        || ((*owner) == (self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]))))
        || ((*owner) == (self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101])))
    {
        64f64
    } else if ((*owner) == (self::ml_string::Text::from_units(vec![117, 49, 50, 56])))
        || ((*owner) == (self::ml_string::Text::from_units(vec![105, 49, 50, 56])))
    {
        128f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn constant_domain(owner: &self::ml_string::Text) -> Vec<self::ml_string::Text> {
    {
        let bits = self::constant_bits(owner);
        if bits == 0f64 {
            Vec::<self::ml_string::Text>::new()
        } else {
            {
                let modulus = self::ml::Big::from_i128(1);
                {
                    let at = 0f64;
                    {
                        let modulus_2 =
                            self::source_ml_temporary_constant_domain_loop24(bits, modulus, at);
                        {
                            let signed =
                                owner.starts_with(&self::ml_string::Text::from_units(vec![105]));
                            vec![
                                {
                                    let source_ml_source1_string_argument43 = if signed {
                                        (self::ml::divide(
                                            &modulus_2,
                                            &self::ml::Big::from_i128(2),
                                            "trunc",
                                            Some("Division by zero"),
                                            false,
                                        ))
                                        .neg()
                                    } else {
                                        self::ml::Big::from_i128(0)
                                    };
                                    self::ml_string::Text::from_text(
                                        &source_ml_source1_string_argument43.to_string(),
                                    )
                                },
                                {
                                    let source_ml_source1_string_argument44 = if signed {
                                        (self::ml::divide(
                                            &modulus_2,
                                            &self::ml::Big::from_i128(2),
                                            "trunc",
                                            Some("Division by zero"),
                                            false,
                                        ))
                                        .sub(&self::ml::Big::from_i128(1))
                                    } else {
                                        modulus_2.sub(&self::ml::Big::from_i128(1))
                                    };
                                    self::ml_string::Text::from_text(
                                        &source_ml_source1_string_argument44.to_string(),
                                    )
                                },
                            ]
                        }
                    }
                }
            }
        }
    }
}
