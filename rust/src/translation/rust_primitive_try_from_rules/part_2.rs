// Generated from complete js/src/translation/rust-primitive-try-from-rules.js. Do not edit by hand.
use super::accept_rust_primitive_from_method;
use super::accept_rust_primitive_from_source;
use super::accept_rust_scalar_predicate;
use super::ml_array;
use super::read_rust_integer_conversion_error_debug_model;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_try_from_source(
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((kind == "fixed") && self::accept_rust_primitive_from_method(path, "from"))
        && self::accept_rust_primitive_from_source(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_consumer_arity(method: &str) -> f64 {
    if (((method == "is_ok") || (method == "is_err")) || (method == "unwrap_or_default"))
        || (method == "unwrap")
    {
        0f64
    } else if ((method == "unwrap_or") || (method == "expect")) || (method == "is_ok_and") {
        1f64
    } else if method == "map_or" {
        2f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_guards(
    source_bits: f64,
    source_signed: bool,
    target_bits: f64,
    target_signed: bool,
) -> f64 {
    if !(self::ml_array::includes(
        &vec![8f64, 16f64, 32f64, 64f64, 128f64],
        &source_bits,
        0f64,
        false,
        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
    )) || !(self::ml_array::includes(
        &vec![8f64, 16f64, 32f64, 64f64, 128f64],
        &target_bits,
        0f64,
        false,
        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
    )) {
        -1f64
    } else {
        {
            let lower = source_signed && (!target_signed || (source_bits > target_bits));
            {
                let upper = if target_signed {
                    if source_signed {
                        source_bits > target_bits
                    } else {
                        source_bits >= target_bits
                    }
                } else {
                    source_bits > target_bits
                };
                (if lower { 1f64 } else { 0f64 } + if upper { 2f64 } else { 0f64 })
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_primitive_try_from_consumer(method: &str, guards: f64) -> Vec<String> {
    if ((self::read_rust_primitive_try_from_consumer_arity(method)) < 0f64)
        || !(self::ml_array::includes(
            &vec![0f64, 1f64, 2f64, 3f64],
            &guards,
            0f64,
            false,
            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
        ))
    {
        Vec::<String>::new()
    } else {
        {
            let lower = vec![
                String::from("input"),
                String::from("minimum"),
                String::from("ge"),
            ];
            {
                let upper = vec![
                    String::from("input"),
                    String::from("maximum"),
                    String::from("le"),
                ];
                {
                    let condition = if guards == 0f64 {
                        vec![String::from("true")]
                    } else if guards == 1f64 {
                        lower
                    } else if guards == 2f64 {
                        upper
                    } else {
                        {
                            let source_ml_concat_value17 = lower;
                            {
                                let source_ml_concat_value18 = upper;
                                {
                                    let source_ml_concat_value19 = vec![String::from("and")];
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
                    };
                    {
                        let failure = if method == "unwrap" {
                            {
                                let source_ml_concat_value20 = vec![String::from("panic prefix")];
                                {
                                    let source_ml_concat_value21 =
                                        self::plan_rust_primitive_try_from_failure_debug(guards);
                                    {
                                        let source_ml_concat_value22 =
                                            vec![String::from("concat"), String::from("raise")];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<String>::new(),
                                                    source_ml_concat_value20,
                                                ),
                                                source_ml_concat_value21,
                                            ),
                                            source_ml_concat_value22,
                                        )
                                    }
                                }
                            }
                        } else if method == "expect" {
                            {
                                let source_ml_concat_value23 =
                                    vec![String::from("message"), String::from("panic prefix")];
                                {
                                    let source_ml_concat_value24 =
                                        self::plan_rust_primitive_try_from_failure_debug(guards);
                                    {
                                        let source_ml_concat_value25 = vec![
                                            String::from("concat"),
                                            String::from("concat"),
                                            String::from("raise"),
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<String>::new(),
                                                    source_ml_concat_value23,
                                                ),
                                                source_ml_concat_value24,
                                            ),
                                            source_ml_concat_value25,
                                        )
                                    }
                                }
                            }
                        } else {
                            Vec::<String>::new()
                        };
                        if self::test_rust_primitive_try_from_panic_consumer(method) {
                            {
                                let source_ml_concat_value26 = condition;
                                {
                                    let source_ml_concat_value27 = vec![String::from("success")];
                                    {
                                        let source_ml_concat_value28 = failure;
                                        {
                                            let source_ml_concat_value29 = vec![String::from("if")];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<String>::new(),
                                                            source_ml_concat_value26,
                                                        ),
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
                        } else if method == "is_ok" {
                            condition
                        } else if method == "is_err" {
                            {
                                let source_ml_concat_value30 = condition;
                                {
                                    let source_ml_concat_value31 = vec![String::from("not")];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_concat_value30,
                                        ),
                                        source_ml_concat_value31,
                                    )
                                }
                            }
                        } else if method == "is_ok_and" {
                            {
                                let source_ml_concat_value32 = condition;
                                {
                                    let source_ml_concat_value33 = vec![
                                        String::from("predicate"),
                                        String::from("false"),
                                        String::from("if"),
                                    ];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_concat_value32,
                                        ),
                                        source_ml_concat_value33,
                                    )
                                }
                            }
                        } else {
                            {
                                let source_ml_concat_value34 = condition;
                                {
                                    let source_ml_concat_value35 = vec![
                                        String::from("success"),
                                        if method == "unwrap_or_default" {
                                            String::from("default")
                                        } else {
                                            String::from("fallback")
                                        },
                                        String::from("if"),
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
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_result_owner(
    method: &str,
    target: &str,
    mapper: &str,
) -> String {
    if ((method == "is_ok") || (method == "is_err")) || (method == "is_ok_and") {
        String::from("bool")
    } else if ((method == "unwrap_or") || (method == "unwrap_or_default"))
        || self::test_rust_primitive_try_from_panic_consumer(method)
    {
        target.to_owned()
    } else if method == "map_or" {
        mapper.to_owned()
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_primitive_try_from_panic_consumer(consumer: &str) -> bool {
    (consumer == "unwrap") || (consumer == "expect")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_failure_debug(direction: &str) -> String {
    self::read_rust_primitive_try_from_versioned_failure_debug(direction, 99f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_primitive_try_from_failure_debug(guards: f64) -> Vec<String> {
    if (self::ml_array::includes(
        &vec![0f64, 1f64, 2f64, 3f64],
        &guards,
        0f64,
        false,
        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
    )) {
        if (guards == 1f64) || (guards == 3f64) {
            vec![
                String::from("input"),
                String::from("minimum"),
                String::from("ge"),
                String::from("positive error"),
                String::from("negative error"),
                String::from("if"),
            ]
        } else {
            vec![String::from("positive error")]
        }
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_panic_prefix(consumer: &str) -> String {
    if consumer == "unwrap" {
        String::from("called `Result::unwrap()` on an `Err` value: ")
    } else if consumer == "expect" {
        String::from(": ")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_panic_version_range() -> String {
    String::from(">=1.97.0 <1.100.0")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_versioned_failure_debug(direction: &str, minor: f64) -> String {
    if (direction != "negative error") && (direction != "positive error") {
        String::new()
    } else {
        {
            let model = self::read_rust_integer_conversion_error_debug_model(minor);
            if model == "unit payload" {
                String::from("TryFromIntError(())")
            } else if model == "overflow kind" {
                if direction == "negative error" {
                    String::from("TryFromIntError(NegOverflow)")
                } else {
                    String::from("TryFromIntError(PosOverflow)")
                }
            } else {
                String::new()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_predicate_error(
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
            "a Result predicate needs a direct one-argument callable with its actual integer parameter and Boolean result",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_mapper_error(
    kind: &str,
    parameters: f64,
    layers: f64,
    parameter: bool,
    result: bool,
) -> String {
    if ((((kind == "function") && (parameters == 1f64)) && (layers == 0f64)) && parameter) && result
    {
        String::new()
    } else {
        String::from(
            "a Result mapper needs a direct one-argument callable with its actual integer parameter and scalar result",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_primitive_try_from_mapper_owner(
    kind: &str,
    bits: &str,
    signed: bool,
    owner_kind: &str,
    owner: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> String {
    if self::accept_rust_primitive_from_source(
        kind, bits, signed, owner_kind, owner, children, proven, layers,
    ) {
        owner.to_owned()
    } else {
        String::new()
    }
}
