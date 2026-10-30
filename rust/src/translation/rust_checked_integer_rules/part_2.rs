// Generated from complete js/src/translation/rust-checked-integer-rules.js. Do not edit by hand.
use super::ml_array;
use super::test_rust_option_panic_consumer;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_checked_integer_success(method: &str, signed: bool) -> Vec<String> {
    if (method == "checked_div_euclid") || (method == "checked_rem_euclid") {
        {
            let quotient = vec![
                String::from("operand"),
                String::from("right"),
                String::from("div"),
            ];
            {
                let remainder = vec![
                    String::from("operand"),
                    String::from("right"),
                    String::from("rem"),
                ];
                if signed {
                    {
                        let positive = if method == "checked_div_euclid" {
                            {
                                let source_ml_concat_value14 = quotient.clone();
                                {
                                    let source_ml_concat_value15 =
                                        vec![String::from("negative unit"), String::from("add")];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_concat_value14,
                                        ),
                                        source_ml_concat_value15,
                                    )
                                }
                            }
                        } else {
                            {
                                let source_ml_concat_value16 = remainder.clone();
                                {
                                    let source_ml_concat_value17 =
                                        vec![String::from("right"), String::from("add")];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_concat_value16,
                                        ),
                                        source_ml_concat_value17,
                                    )
                                }
                            }
                        };
                        {
                            let negative = if method == "checked_div_euclid" {
                                {
                                    let source_ml_concat_value18 = quotient.clone();
                                    {
                                        let source_ml_concat_value19 = vec![
                                            String::from("negative unit"),
                                            String::from("sub"),
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<String>::new(),
                                                source_ml_concat_value18,
                                            ),
                                            source_ml_concat_value19,
                                        )
                                    }
                                }
                            } else {
                                {
                                    let source_ml_concat_value20 = remainder.clone();
                                    {
                                        let source_ml_concat_value21 =
                                            vec![String::from("right"), String::from("sub")];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<String>::new(),
                                                source_ml_concat_value20,
                                            ),
                                            source_ml_concat_value21,
                                        )
                                    }
                                }
                            };
                            {
                                let source_ml_concat_value22 = remainder.clone();
                                {
                                    let source_ml_concat_value23 = vec![
                                        String::from("zero"),
                                        String::from("lt"),
                                        String::from("right"),
                                        String::from("zero"),
                                        String::from("gt"),
                                    ];
                                    {
                                        let source_ml_concat_value24 = positive;
                                        {
                                            let source_ml_concat_value25 = negative;
                                            {
                                                let source_ml_concat_value26 =
                                                    vec![String::from("if")];
                                                {
                                                    let source_ml_concat_value27 =
                                                        if method == "checked_div_euclid" {
                                                            quotient
                                                        } else {
                                                            remainder
                                                        };
                                                    {
                                                        let source_ml_concat_value28 =
                                                            vec![String::from("if")];
                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<String>::new(), source_ml_concat_value22), source_ml_concat_value23), source_ml_concat_value24), source_ml_concat_value25), source_ml_concat_value26), source_ml_concat_value27), source_ml_concat_value28)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if method == "checked_div_euclid" {
                    quotient
                } else {
                    remainder
                }
            }
        }
    } else if (method == "checked_shl") || (method == "checked_shr") {
        vec![
            String::from("operand"),
            String::from("right"),
            if method == "checked_shl" {
                String::from("shift left")
            } else {
                String::from("shift right")
            },
        ]
    } else if method == "checked_neg" {
        if signed {
            vec![String::from("operand"), String::from("neg")]
        } else {
            vec![String::from("zero")]
        }
    } else if method == "checked_abs" {
        if signed {
            vec![
                String::from("operand"),
                String::from("zero"),
                String::from("lt"),
                String::from("operand"),
                String::from("neg"),
                String::from("operand"),
                String::from("if"),
            ]
        } else {
            Vec::<String>::new()
        }
    } else {
        {
            let operation = if method == "checked_add" {
                String::from("add")
            } else if method == "checked_sub" {
                String::from("sub")
            } else if method == "checked_mul" {
                String::from("mul")
            } else if method == "checked_div" {
                String::from("div")
            } else if method == "checked_rem" {
                String::from("rem")
            } else {
                String::new()
            };
            if operation.is_empty() {
                Vec::<String>::new()
            } else {
                vec![String::from("operand"), String::from("right"), operation]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_checked_integer_consumer(consumer: &str) -> Vec<String> {
    if consumer == "is_some" {
        vec![String::from("condition")]
    } else if consumer == "is_none" {
        vec![String::from("condition"), String::from("not")]
    } else if consumer == "unwrap" {
        vec![
            String::from("condition"),
            String::from("success"),
            String::from("unwrap message"),
            String::from("raise"),
            String::from("if"),
        ]
    } else if consumer == "expect" {
        vec![
            String::from("condition"),
            String::from("success"),
            String::from("fallback"),
            String::from("raise"),
            String::from("if"),
        ]
    } else if consumer == "unwrap_or_default" {
        vec![
            String::from("condition"),
            String::from("success"),
            String::from("default"),
            String::from("if"),
        ]
    } else if (consumer == "unwrap_or") || (consumer == "map_or") {
        vec![
            String::from("condition"),
            String::from("success"),
            String::from("fallback"),
            String::from("if"),
        ]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_result_owner(
    consumer: &str,
    receiver: &str,
    mapper: &str,
) -> String {
    if (consumer == "is_some") || (consumer == "is_none") {
        String::from("bool")
    } else if ((consumer == "unwrap_or") || (consumer == "unwrap_or_default"))
        || self::test_rust_option_panic_consumer(consumer)
    {
        receiver.to_owned()
    } else if consumer == "map_or" {
        mapper.to_owned()
    } else {
        String::new()
    }
}
