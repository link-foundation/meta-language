// Generated from complete js/src/translation/rust-checked-integer-rules.js. Do not edit by hand.
use super::accept_rust_primitive_from_method;
use super::accept_rust_primitive_selection_receiver;
use super::accept_rust_scalar_fallback;
use super::accept_rust_scalar_predicate;
use super::ml_array;
use super::test_rust_option_panic_consumer;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_consumer_arity(consumer: &str) -> f64 {
    if (((consumer == "is_some") || (consumer == "is_none")) || (consumer == "unwrap_or_default"))
        || (consumer == "unwrap")
    {
        0f64
    } else if (((consumer == "unwrap_or") || (consumer == "expect"))
        || self::test_rust_checked_integer_lazy_fallback(consumer))
        || self::test_rust_checked_integer_predicate_consumer(consumer)
    {
        1f64
    } else if self::test_rust_checked_integer_mapping_consumer(consumer) {
        2f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_argument_owner(method: &str, receiver: &str) -> String {
    if (method == "checked_shl") || (method == "checked_shr") {
        String::from("u32")
    } else {
        receiver.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_argument_reason(method: &str) -> String {
    if (method == "checked_shl") || (method == "checked_shr") {
        String::from("the actual shift count must retain standard u32 ownership")
    } else {
        String::from("the actual argument must retain the receiver type and source owner")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_shift_operation(term: &str) -> String {
    if term == "shift left" {
        String::from("shiftRustPrimitiveBitsLeft")
    } else if term == "shift right" {
        String::from("shiftRustPrimitiveBitsRight")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_checked_integer_condition(method: &str, signed: bool) -> Vec<String> {
    if (method == "checked_shl") || (method == "checked_shr") {
        vec![
            String::from("right"),
            String::from("width"),
            String::from("lt"),
        ]
    } else if method == "checked_neg" {
        if signed {
            vec![
                String::from("operand"),
                String::from("minimum"),
                String::from("ne"),
            ]
        } else {
            vec![
                String::from("operand"),
                String::from("zero"),
                String::from("eq"),
            ]
        }
    } else if method == "checked_abs" {
        if signed {
            vec![
                String::from("operand"),
                String::from("minimum"),
                String::from("ne"),
            ]
        } else {
            Vec::<String>::new()
        }
    } else if method == "checked_add" {
        if signed {
            vec![
                String::from("right"),
                String::from("zero"),
                String::from("gt"),
                String::from("operand"),
                String::from("maximum"),
                String::from("right"),
                String::from("sub"),
                String::from("le"),
                String::from("right"),
                String::from("zero"),
                String::from("lt"),
                String::from("operand"),
                String::from("minimum"),
                String::from("right"),
                String::from("sub"),
                String::from("ge"),
                String::from("true"),
                String::from("if"),
                String::from("if"),
            ]
        } else {
            vec![
                String::from("operand"),
                String::from("maximum"),
                String::from("right"),
                String::from("sub"),
                String::from("le"),
            ]
        }
    } else if method == "checked_sub" {
        if signed {
            vec![
                String::from("right"),
                String::from("zero"),
                String::from("lt"),
                String::from("operand"),
                String::from("maximum"),
                String::from("right"),
                String::from("add"),
                String::from("le"),
                String::from("right"),
                String::from("zero"),
                String::from("gt"),
                String::from("operand"),
                String::from("minimum"),
                String::from("right"),
                String::from("add"),
                String::from("ge"),
                String::from("true"),
                String::from("if"),
                String::from("if"),
            ]
        } else {
            vec![
                String::from("operand"),
                String::from("right"),
                String::from("ge"),
            ]
        }
    } else if method == "checked_mul" {
        {
            let positive_positive = vec![
                String::from("operand"),
                String::from("maximum"),
                String::from("right"),
                String::from("div"),
                String::from("le"),
            ];
            if signed {
                {
                    let positive_negative = vec![
                        String::from("right"),
                        String::from("minimum"),
                        String::from("operand"),
                        String::from("div"),
                        String::from("ge"),
                    ];
                    {
                        let negative_positive = vec![
                            String::from("operand"),
                            String::from("minimum"),
                            String::from("right"),
                            String::from("div"),
                            String::from("ge"),
                        ];
                        {
                            let negative_negative = vec![
                                String::from("operand"),
                                String::from("maximum"),
                                String::from("right"),
                                String::from("div"),
                                String::from("ge"),
                            ];
                            {
                                let signs = {
                                    let source_ml_concat_value20 = vec![
                                        String::from("operand"),
                                        String::from("zero"),
                                        String::from("gt"),
                                        String::from("right"),
                                        String::from("zero"),
                                        String::from("gt"),
                                    ];
                                    {
                                        let source_ml_concat_value21 = positive_positive;
                                        {
                                            let source_ml_concat_value22 = positive_negative;
                                            {
                                                let source_ml_concat_value23 = vec![
                                                    String::from("if"),
                                                    String::from("right"),
                                                    String::from("zero"),
                                                    String::from("gt"),
                                                ];
                                                {
                                                    let source_ml_concat_value24 =
                                                        negative_positive;
                                                    {
                                                        let source_ml_concat_value25 =
                                                            negative_negative;
                                                        {
                                                            let source_ml_concat_value26 = vec![
                                                                String::from("if"),
                                                                String::from("if"),
                                                            ];
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<String>::new(), source_ml_concat_value20), source_ml_concat_value21), source_ml_concat_value22), source_ml_concat_value23), source_ml_concat_value24), source_ml_concat_value25), source_ml_concat_value26)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                                {
                                    let source_ml_concat_value27 = vec![
                                        String::from("operand"),
                                        String::from("zero"),
                                        String::from("eq"),
                                        String::from("true"),
                                        String::from("right"),
                                        String::from("zero"),
                                        String::from("eq"),
                                        String::from("true"),
                                    ];
                                    {
                                        let source_ml_concat_value28 = signs;
                                        {
                                            let source_ml_concat_value29 =
                                                vec![String::from("if"), String::from("if")];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<String>::new(),
                                                        source_ml_concat_value27,
                                                    ),
                                                    source_ml_concat_value28,
                                                ),
                                                source_ml_concat_value29,
                                            )
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                {
                    let source_ml_concat_value17 = vec![
                        String::from("operand"),
                        String::from("zero"),
                        String::from("eq"),
                        String::from("true"),
                        String::from("right"),
                        String::from("zero"),
                        String::from("eq"),
                        String::from("true"),
                    ];
                    {
                        let source_ml_concat_value18 = positive_positive;
                        {
                            let source_ml_concat_value19 =
                                vec![String::from("if"), String::from("if")];
                            self::ml_array::append(
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_concat_value17,
                                    ),
                                    source_ml_concat_value18,
                                ),
                                source_ml_concat_value19,
                            )
                        }
                    }
                }
            }
        }
    } else if (((method == "checked_div") || (method == "checked_rem"))
        || (method == "checked_div_euclid"))
        || (method == "checked_rem_euclid")
    {
        if signed {
            vec![
                String::from("right"),
                String::from("zero"),
                String::from("eq"),
                String::from("false"),
                String::from("operand"),
                String::from("minimum"),
                String::from("eq"),
                String::from("right"),
                String::from("negative unit"),
                String::from("ne"),
                String::from("true"),
                String::from("if"),
                String::from("if"),
            ]
        } else {
            vec![
                String::from("right"),
                String::from("zero"),
                String::from("ne"),
            ]
        }
    } else {
        Vec::<String>::new()
    }
}

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
                                let source_ml_concat_value30 = quotient.clone();
                                {
                                    let source_ml_concat_value31 =
                                        vec![String::from("negative unit"), String::from("add")];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_concat_value30,
                                        ),
                                        source_ml_concat_value31,
                                    )
                                }
                            }
                        } else {
                            {
                                let source_ml_concat_value32 = remainder.clone();
                                {
                                    let source_ml_concat_value33 =
                                        vec![String::from("right"), String::from("add")];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_concat_value32,
                                        ),
                                        source_ml_concat_value33,
                                    )
                                }
                            }
                        };
                        {
                            let negative = if method == "checked_div_euclid" {
                                {
                                    let source_ml_concat_value34 = quotient.clone();
                                    {
                                        let source_ml_concat_value35 = vec![
                                            String::from("negative unit"),
                                            String::from("sub"),
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<String>::new(),
                                                source_ml_concat_value34,
                                            ),
                                            source_ml_concat_value35,
                                        )
                                    }
                                }
                            } else {
                                {
                                    let source_ml_concat_value36 = remainder.clone();
                                    {
                                        let source_ml_concat_value37 =
                                            vec![String::from("right"), String::from("sub")];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<String>::new(),
                                                source_ml_concat_value36,
                                            ),
                                            source_ml_concat_value37,
                                        )
                                    }
                                }
                            };
                            {
                                let source_ml_concat_value38 = remainder.clone();
                                {
                                    let source_ml_concat_value39 = vec![
                                        String::from("zero"),
                                        String::from("lt"),
                                        String::from("right"),
                                        String::from("zero"),
                                        String::from("gt"),
                                    ];
                                    {
                                        let source_ml_concat_value40 = positive;
                                        {
                                            let source_ml_concat_value41 = negative;
                                            {
                                                let source_ml_concat_value42 =
                                                    vec![String::from("if")];
                                                {
                                                    let source_ml_concat_value43 =
                                                        if method == "checked_div_euclid" {
                                                            quotient
                                                        } else {
                                                            remainder
                                                        };
                                                    {
                                                        let source_ml_concat_value44 =
                                                            vec![String::from("if")];
                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<String>::new(), source_ml_concat_value38), source_ml_concat_value39), source_ml_concat_value40), source_ml_concat_value41), source_ml_concat_value42), source_ml_concat_value43), source_ml_concat_value44)
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
    if self::test_rust_checked_integer_predicate_consumer(consumer) {
        vec![
            String::from("condition"),
            String::from("fallback"),
            String::from("success"),
            String::from("call one"),
            if consumer == "is_none_or" {
                String::from("true")
            } else {
                String::from("false")
            },
            String::from("if"),
        ]
    } else if consumer == "is_some" {
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
    } else if self::test_rust_checked_integer_lazy_fallback(consumer)
        || self::test_rust_checked_integer_lazy_mapper(consumer)
    {
        vec![
            String::from("condition"),
            String::from("success"),
            String::from("fallback"),
            String::from("call zero"),
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
    if ((consumer == "is_some") || (consumer == "is_none"))
        || self::test_rust_checked_integer_predicate_consumer(consumer)
    {
        String::from("bool")
    } else if (((consumer == "unwrap_or") || (consumer == "unwrap_or_default"))
        || self::test_rust_option_panic_consumer(consumer))
        || self::test_rust_checked_integer_lazy_fallback(consumer)
    {
        receiver.to_owned()
    } else if self::test_rust_checked_integer_mapping_consumer(consumer) {
        mapper.to_owned()
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_checked_integer_lazy_fallback(consumer: &str) -> bool {
    consumer == "unwrap_or_else"
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_checked_integer_fallback_error(
    kind: &str,
    parameters: f64,
    layers: f64,
    result: &str,
    bits: &str,
    signed: bool,
    owner_kind: &str,
    owner: &str,
    children: f64,
    proven: bool,
    result_layers: f64,
    expected_bits: &str,
    expected_signed: bool,
    expected_owner: &str,
) -> String {
    if (((((((kind == "function") && (parameters == 0f64)) && (layers == 0f64))
        && (result == "fixed"))
        && (bits == expected_bits))
        && (signed == expected_signed))
        && (owner == expected_owner))
        && self::accept_rust_primitive_selection_receiver(
            result,
            bits,
            signed,
            owner_kind,
            owner,
            children,
            proven,
            result_layers,
        )
    {
        String::new()
    } else {
        String::from(
            "a checked integer fallback needs a direct zero-argument callable with the actual receiver result owner",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_checked_integer_predicate_consumer(consumer: &str) -> bool {
    (consumer == "is_some_and") || (consumer == "is_none_or")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_predicate_error(
    kind: &str,
    parameters: f64,
    layers: f64,
    parameter: bool,
    result: bool,
) -> String {
    if self::accept_rust_scalar_predicate(kind, parameters, layers, parameter, result) {
        String::new()
    } else {
        String::from(
            "a checked integer predicate needs a direct one-argument callable with its actual integer parameter and Boolean result",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_checked_integer_lazy_mapper(consumer: &str) -> bool {
    consumer == "map_or_else"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_checked_integer_mapping_consumer(consumer: &str) -> bool {
    (consumer == "map_or") || self::test_rust_checked_integer_lazy_mapper(consumer)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_mapped_fallback_error(
    kind: &str,
    parameters: f64,
    layers: f64,
    owner: &str,
    expected: &str,
) -> String {
    if self::accept_rust_scalar_fallback(
        kind,
        parameters,
        layers,
        (self::accept_rust_primitive_from_method(owner, "from") && (owner == expected)),
    ) {
        String::new()
    } else {
        String::from(
            "a checked integer mapped fallback needs a direct zero-argument callable with its actual scalar result owner",
        )
    }
}
