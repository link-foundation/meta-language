// Generated from complete js/src/translation/rust-type-alias-rules.js. Do not edit by hand.
use super::MlTemporaryGroupAfterLoop1Result;
use super::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result;
use super::MlTemporaryReadRustTypeAliasLookupKeysJoin4;
use super::accept_rust_primitive_from_method;
use super::accept_rust_primitive_selection_receiver;
use super::accept_rust_scalar_predicate;
use super::ml_array;
use super::ml_source_exception;
use super::read_rust_integer_conversion_error_debug_model;
use super::read_rust_primitive_integer_constant_owner;
use super::source_ml_temporary_group_after_loop1;
use super::source_ml_temporary_read_rust_type_alias_declaration_plan_loop2;
use super::source_ml_temporary_read_rust_type_alias_lookup_keys_loop5;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_from_source(
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((((((((kind == "float") && ((bits).is_empty())) && !signed) && (owner == "primitive"))
        && (path == "f64"))
        && (children == 0f64))
        && proven)
        && (layers == 0f64))
        || self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_primitive_from(target: &str, source: &str) -> String {
    if !(self::accept_rust_primitive_from_method(target, "from"))
        || !(self::accept_rust_primitive_from_method(source, "from"))
    {
        String::new()
    } else if target == source {
        String::from("identity")
    } else if ((source == "bool") && (target != "bool")) && (target != "f64") {
        String::from("boolean integer")
    } else if (target == "f64")
        && self::ml_array::includes(
            &vec![
                String::from("i8"),
                String::from("i16"),
                String::from("i32"),
                String::from("u8"),
                String::from("u16"),
                String::from("u32"),
            ],
            &source.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        )
    {
        String::from("floating exact")
    } else {
        {
            let admitted = ((((((((source == "u8")
                && self::ml_array::includes(
                    &vec![
                        String::from("u16"),
                        String::from("u32"),
                        String::from("u64"),
                        String::from("u128"),
                        String::from("usize"),
                        String::from("i16"),
                        String::from("i32"),
                        String::from("i64"),
                        String::from("i128"),
                        String::from("isize"),
                    ],
                    &target.to_owned(),
                    0f64,
                    false,
                    |a, b| a == b,
                ))
                || ((source == "u16")
                    && self::ml_array::includes(
                        &vec![
                            String::from("u32"),
                            String::from("u64"),
                            String::from("u128"),
                            String::from("usize"),
                            String::from("i32"),
                            String::from("i64"),
                            String::from("i128"),
                        ],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "u32")
                    && self::ml_array::includes(
                        &vec![
                            String::from("u64"),
                            String::from("u128"),
                            String::from("i64"),
                            String::from("i128"),
                        ],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "u64")
                    && self::ml_array::includes(
                        &vec![String::from("u128"), String::from("i128")],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "i8")
                    && self::ml_array::includes(
                        &vec![
                            String::from("i16"),
                            String::from("i32"),
                            String::from("i64"),
                            String::from("i128"),
                            String::from("isize"),
                        ],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "i16")
                    && self::ml_array::includes(
                        &vec![
                            String::from("i32"),
                            String::from("i64"),
                            String::from("i128"),
                            String::from("isize"),
                        ],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "i32")
                    && self::ml_array::includes(
                        &vec![String::from("i64"), String::from("i128")],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "i64") && (target == "i128"));
            if admitted {
                String::from("integer exact")
            } else {
                String::new()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_owner(
    segments: &[String],
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> String {
    self::read_rust_primitive_integer_constant_owner(segments, ordinary, shadowed, absolute)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_primitive_try_from_import(path: &str, ordinary: bool, shadowed: bool) -> bool {
    (ordinary && !shadowed)
        && ((path == "std::convert::TryFrom") || (path == "core::convert::TryFrom"))
}

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
                            let source_ml_source1_concat_value17 = lower;
                            {
                                let source_ml_source1_concat_value18 = upper;
                                {
                                    let source_ml_source1_concat_value19 =
                                        vec![String::from("and")];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<String>::new(),
                                                source_ml_source1_concat_value17,
                                            ),
                                            source_ml_source1_concat_value18,
                                        ),
                                        source_ml_source1_concat_value19,
                                    )
                                }
                            }
                        }
                    };
                    {
                        let failure = if method == "unwrap" {
                            {
                                let source_ml_source1_concat_value20 =
                                    vec![String::from("panic prefix")];
                                {
                                    let source_ml_source1_concat_value21 =
                                        self::plan_rust_primitive_try_from_failure_debug(guards);
                                    {
                                        let source_ml_source1_concat_value22 =
                                            vec![String::from("concat"), String::from("raise")];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<String>::new(),
                                                    source_ml_source1_concat_value20,
                                                ),
                                                source_ml_source1_concat_value21,
                                            ),
                                            source_ml_source1_concat_value22,
                                        )
                                    }
                                }
                            }
                        } else if method == "expect" {
                            {
                                let source_ml_source1_concat_value23 =
                                    vec![String::from("message"), String::from("panic prefix")];
                                {
                                    let source_ml_source1_concat_value24 =
                                        self::plan_rust_primitive_try_from_failure_debug(guards);
                                    {
                                        let source_ml_source1_concat_value25 = vec![
                                            String::from("concat"),
                                            String::from("concat"),
                                            String::from("raise"),
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<String>::new(),
                                                    source_ml_source1_concat_value23,
                                                ),
                                                source_ml_source1_concat_value24,
                                            ),
                                            source_ml_source1_concat_value25,
                                        )
                                    }
                                }
                            }
                        } else {
                            Vec::<String>::new()
                        };
                        if self::test_rust_primitive_try_from_panic_consumer(method) {
                            {
                                let source_ml_source1_concat_value26 = condition;
                                {
                                    let source_ml_source1_concat_value27 =
                                        vec![String::from("success")];
                                    {
                                        let source_ml_source1_concat_value28 = failure;
                                        {
                                            let source_ml_source1_concat_value29 =
                                                vec![String::from("if")];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<String>::new(),
                                                            source_ml_source1_concat_value26,
                                                        ),
                                                        source_ml_source1_concat_value27,
                                                    ),
                                                    source_ml_source1_concat_value28,
                                                ),
                                                source_ml_source1_concat_value29,
                                            )
                                        }
                                    }
                                }
                            }
                        } else if method == "is_ok" {
                            condition
                        } else if method == "is_err" {
                            {
                                let source_ml_source1_concat_value30 = condition;
                                {
                                    let source_ml_source1_concat_value31 =
                                        vec![String::from("not")];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_source1_concat_value30,
                                        ),
                                        source_ml_source1_concat_value31,
                                    )
                                }
                            }
                        } else if method == "is_ok_and" {
                            {
                                let source_ml_source1_concat_value32 = condition;
                                {
                                    let source_ml_source1_concat_value33 = vec![
                                        String::from("predicate"),
                                        String::from("false"),
                                        String::from("if"),
                                    ];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_source1_concat_value32,
                                        ),
                                        source_ml_source1_concat_value33,
                                    )
                                }
                            }
                        } else {
                            {
                                let source_ml_source1_concat_value34 = condition;
                                {
                                    let source_ml_source1_concat_value35 = vec![
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
                                            source_ml_source1_concat_value34,
                                        ),
                                        source_ml_source1_concat_value35,
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Number fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required text fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Boolean fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required Number fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_number_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required text fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required Boolean fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn group_after(kinds: &[String], values: &[String], position: f64) -> f64 {
    {
        let open = self::read_required_text_fact(values, position);
        {
            let stack = vec![if open == "{" {
                String::from("}")
            } else if open == "[" {
                String::from("]")
            } else {
                String::from(")")
            }];
            {
                let at = position + 1f64;
                {
                    let source_ml_source1_s36 =
                        self::source_ml_temporary_group_after_loop1(kinds, values, stack, at);
                    match source_ml_source1_s36 {
                        self::MlTemporaryGroupAfterLoop1Result::MlTemporaryGroupAfterLoop1Done => {
                            -1f64
                        }
                        self::MlTemporaryGroupAfterLoop1Result::MlTemporaryGroupAfterLoop1Return(source_ml_temporary_result1) => {
                            source_ml_temporary_result1
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_declaration_plan(kinds: &[String], values: &[String]) -> Vec<String> {
    {
        let ml_library_inequality_1 = (kinds.len() as f64) != (values.len() as f64);
        if ml_library_inequality_1 {
            vec![
                String::from("error"),
                String::from("unaligned source type alias columns"),
            ]
        } else {
            {
                let result = vec![String::from("aliases")];
                {
                    let names = Vec::<String>::new();
                    {
                        let owners = Vec::<String>::new();
                        {
                            let closes = Vec::<f64>::new();
                            {
                                let position = 0f64;
                                {
                                    let source_ml_source1_s37 = self::source_ml_temporary_read_rust_type_alias_declaration_plan_loop2(kinds, values, result, names, owners, closes, position);
                                    match source_ml_source1_s37 {
                                    self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Done(result_2) => {
                                        result_2
                                    }
                                    self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(source_ml_temporary_result2) => {
                                        source_ml_temporary_result2
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
pub fn read_rust_type_alias_lookup_keys(segments: &[String], context: &[String]) -> Vec<String> {
    {
        let names = segments.to_vec();
        {
            let start = context.to_vec();
            {
                let source_ml_source1_s38 = if ((self::ml_array::optional_at(
                    &names,
                    self::ml_array::number_index(0f64),
                ))
                .unwrap_or_else(|| String::new()))
                    == "crate"
                {
                    {
                        let names_2 = self::ml_array::slice(&names, 1f64, f64::INFINITY);
                        {
                            let start_2 = Vec::<String>::new();
                            self::MlTemporaryReadRustTypeAliasLookupKeysJoin4::MlTemporaryReadRustTypeAliasLookupKeysJoin4Next(names_2, start_2)
                        }
                    }
                } else if ((self::ml_array::optional_at(
                    &names,
                    self::ml_array::number_index(0f64),
                ))
                .unwrap_or_else(|| String::new()))
                    == "self"
                {
                    {
                        let names_3 = self::ml_array::slice(&names, 1f64, f64::INFINITY);
                        self::MlTemporaryReadRustTypeAliasLookupKeysJoin4::MlTemporaryReadRustTypeAliasLookupKeysJoin4Next(names_3, start)
                    }
                } else if ((self::ml_array::optional_at(
                    &names,
                    self::ml_array::number_index(0f64),
                ))
                .unwrap_or_else(|| String::new()))
                    == "super"
                {
                    {
                        let count = 0f64;
                        {
                            let count_2 =
                                self::source_ml_temporary_read_rust_type_alias_lookup_keys_loop5(
                                    &(names),
                                    count,
                                );
                            if count_2 > (start.len() as f64) {
                                self::MlTemporaryReadRustTypeAliasLookupKeysJoin4::MlTemporaryReadRustTypeAliasLookupKeysJoin4Return(Vec::<String>::new())
                            } else {
                                {
                                    let names_4 =
                                        self::ml_array::slice(&names, count_2, f64::INFINITY);
                                    {
                                        let start_3 = self::ml_array::slice(
                                            &start,
                                            0f64,
                                            ((start.len() as f64) - count_2),
                                        );
                                        self::MlTemporaryReadRustTypeAliasLookupKeysJoin4::MlTemporaryReadRustTypeAliasLookupKeysJoin4Next(names_4, start_3)
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlTemporaryReadRustTypeAliasLookupKeysJoin4::MlTemporaryReadRustTypeAliasLookupKeysJoin4Next(names, start)
                };
                match source_ml_source1_s38 {
                    self::MlTemporaryReadRustTypeAliasLookupKeysJoin4::MlTemporaryReadRustTypeAliasLookupKeysJoin4Next(names_5, start_4) => {
                        if (names_5.len() as f64) == 0f64 {
                            Vec::<String>::new()
                        } else {
                            vec![({ let (ml_append_right, ml_append_left) = (names_5, start_4); self::ml_array::append(ml_append_left, ml_append_right) }).join((String::from(".")).as_str())]
                        }
                    }
                    self::MlTemporaryReadRustTypeAliasLookupKeysJoin4::MlTemporaryReadRustTypeAliasLookupKeysJoin4Return(source_ml_temporary_result4) => {
                        source_ml_temporary_result4
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_expansion_error(
    depth: f64,
    expansions: f64,
    recursive: bool,
) -> String {
    if recursive {
        String::from("recursive source type alias")
    } else if depth >= 128f64 {
        String::from("source type alias depth limit exceeded")
    } else if expansions >= 4096f64 {
        String::from("source type alias expansion limit exceeded")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_clone_error(current: f64, needed: f64) -> String {
    if needed > (262_144f64 - current) {
        String::from("source type alias materialization limit exceeded")
    } else {
        String::new()
    }
}
