// Generated from complete js/src/translation/rust-generic-type-alias-rules.js. Do not edit by hand.
use super::MlSource1TemporaryDependencyFunction1Loop1Result;
use super::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Result;
use super::MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4;
use super::accept_rust_primitive_from_source;
use super::accept_rust_scalar_predicate;
use super::accept_rust_source_array_element;
use super::ml_array;
use super::ml_string;
use super::read_required_text_fact;
use super::read_rust_integer_conversion_error_debug_model;
use super::read_rust_primitive_try_from_consumer_arity;
use super::source_ml_source1_temporary_dependency_function1_loop1;
use super::source_ml_source1_temporary_read_rust_type_alias_access_prefix_lengths_loop6;
use super::source_ml_source1_temporary_read_rust_type_alias_declaration_plan_loop2;
use super::source_ml_source1_temporary_read_rust_type_alias_lookup_keys_loop5;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_primitive_try_from_consumer(
    method: &self::ml_string::Text,
    guards: f64,
) -> Vec<self::ml_string::Text> {
    if ((self::read_rust_primitive_try_from_consumer_arity(method)) < 0f64)
        || !(self::ml_array::includes(
            &vec![0f64, 1f64, 2f64, 3f64],
            &guards,
            0f64,
            false,
            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
        ))
    {
        Vec::<self::ml_string::Text>::new()
    } else {
        {
            let lower = vec![
                self::ml_string::Text::from_units(vec![105, 110, 112, 117, 116]),
                self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]),
                self::ml_string::Text::from_units(vec![103, 101]),
            ];
            {
                let upper = vec![
                    self::ml_string::Text::from_units(vec![105, 110, 112, 117, 116]),
                    self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109]),
                    self::ml_string::Text::from_units(vec![108, 101]),
                ];
                {
                    let condition = if guards == 0f64 {
                        vec![self::ml_string::Text::from_units(vec![116, 114, 117, 101])]
                    } else if guards == 1f64 {
                        lower
                    } else if guards == 2f64 {
                        upper
                    } else {
                        {
                            let source_ml_source2_concat_value17 = lower;
                            {
                                let source_ml_source2_concat_value18 = upper;
                                {
                                    let source_ml_source2_concat_value19 =
                                        vec![self::ml_string::Text::from_units(vec![97, 110, 100])];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<self::ml_string::Text>::new(),
                                                source_ml_source2_concat_value17,
                                            ),
                                            source_ml_source2_concat_value18,
                                        ),
                                        source_ml_source2_concat_value19,
                                    )
                                }
                            }
                        }
                    };
                    {
                        let failure = if (*method)
                            == (self::ml_string::Text::from_units(vec![
                                117, 110, 119, 114, 97, 112,
                            ])) {
                            {
                                let source_ml_source2_concat_value20 =
                                    vec![self::ml_string::Text::from_units(vec![
                                        112, 97, 110, 105, 99, 32, 112, 114, 101, 102, 105, 120,
                                    ])];
                                {
                                    let source_ml_source2_concat_value21 =
                                        self::plan_rust_primitive_try_from_failure_debug(guards);
                                    {
                                        let source_ml_source2_concat_value22 = vec![
                                            self::ml_string::Text::from_units(vec![
                                                99, 111, 110, 99, 97, 116,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                114, 97, 105, 115, 101,
                                            ]),
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::ml_string::Text>::new(),
                                                    source_ml_source2_concat_value20,
                                                ),
                                                source_ml_source2_concat_value21,
                                            ),
                                            source_ml_source2_concat_value22,
                                        )
                                    }
                                }
                            }
                        } else if (*method)
                            == (self::ml_string::Text::from_units(vec![
                                101, 120, 112, 101, 99, 116,
                            ]))
                        {
                            {
                                let source_ml_source2_concat_value23 = vec![
                                    self::ml_string::Text::from_units(vec![
                                        109, 101, 115, 115, 97, 103, 101,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        112, 97, 110, 105, 99, 32, 112, 114, 101, 102, 105, 120,
                                    ]),
                                ];
                                {
                                    let source_ml_source2_concat_value24 =
                                        self::plan_rust_primitive_try_from_failure_debug(guards);
                                    {
                                        let source_ml_source2_concat_value25 = vec![
                                            self::ml_string::Text::from_units(vec![
                                                99, 111, 110, 99, 97, 116,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                99, 111, 110, 99, 97, 116,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                114, 97, 105, 115, 101,
                                            ]),
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::ml_string::Text>::new(),
                                                    source_ml_source2_concat_value23,
                                                ),
                                                source_ml_source2_concat_value24,
                                            ),
                                            source_ml_source2_concat_value25,
                                        )
                                    }
                                }
                            }
                        } else {
                            Vec::<self::ml_string::Text>::new()
                        };
                        if self::test_rust_primitive_try_from_panic_consumer(method) {
                            {
                                let source_ml_source2_concat_value26 = condition;
                                {
                                    let source_ml_source2_concat_value27 =
                                        vec![self::ml_string::Text::from_units(vec![
                                            115, 117, 99, 99, 101, 115, 115,
                                        ])];
                                    {
                                        let source_ml_source2_concat_value28 = failure;
                                        {
                                            let source_ml_source2_concat_value29 = vec![
                                                self::ml_string::Text::from_units(vec![105, 102]),
                                            ];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<self::ml_string::Text>::new(),
                                                            source_ml_source2_concat_value26,
                                                        ),
                                                        source_ml_source2_concat_value27,
                                                    ),
                                                    source_ml_source2_concat_value28,
                                                ),
                                                source_ml_source2_concat_value29,
                                            )
                                        }
                                    }
                                }
                            }
                        } else if (*method)
                            == (self::ml_string::Text::from_units(vec![105, 115, 95, 111, 107]))
                        {
                            condition
                        } else if (*method)
                            == (self::ml_string::Text::from_units(vec![
                                105, 115, 95, 101, 114, 114,
                            ]))
                        {
                            {
                                let source_ml_source2_concat_value30 = condition;
                                {
                                    let source_ml_source2_concat_value31 = vec![
                                        self::ml_string::Text::from_units(vec![110, 111, 116]),
                                    ];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::ml_string::Text>::new(),
                                            source_ml_source2_concat_value30,
                                        ),
                                        source_ml_source2_concat_value31,
                                    )
                                }
                            }
                        } else if (*method)
                            == (self::ml_string::Text::from_units(vec![
                                105, 115, 95, 111, 107, 95, 97, 110, 100,
                            ]))
                        {
                            {
                                let source_ml_source2_concat_value32 = condition;
                                {
                                    let source_ml_source2_concat_value33 = vec![
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 101, 100, 105, 99, 97, 116, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            102, 97, 108, 115, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![105, 102]),
                                    ];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::ml_string::Text>::new(),
                                            source_ml_source2_concat_value32,
                                        ),
                                        source_ml_source2_concat_value33,
                                    )
                                }
                            }
                        } else {
                            {
                                let source_ml_source2_concat_value34 = condition;
                                {
                                    let source_ml_source2_concat_value35 = vec![
                                        self::ml_string::Text::from_units(vec![
                                            115, 117, 99, 99, 101, 115, 115,
                                        ]),
                                        if (*method)
                                            == (self::ml_string::Text::from_units(vec![
                                                117, 110, 119, 114, 97, 112, 95, 111, 114, 95, 100,
                                                101, 102, 97, 117, 108, 116,
                                            ]))
                                        {
                                            self::ml_string::Text::from_units(vec![
                                                100, 101, 102, 97, 117, 108, 116,
                                            ])
                                        } else {
                                            self::ml_string::Text::from_units(vec![
                                                102, 97, 108, 108, 98, 97, 99, 107,
                                            ])
                                        },
                                        self::ml_string::Text::from_units(vec![105, 102]),
                                    ];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::ml_string::Text>::new(),
                                            source_ml_source2_concat_value34,
                                        ),
                                        source_ml_source2_concat_value35,
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
    method: &self::ml_string::Text,
    target: &self::ml_string::Text,
    mapper: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (((*method) == (self::ml_string::Text::from_units(vec![105, 115, 95, 111, 107])))
        || ((*method) == (self::ml_string::Text::from_units(vec![105, 115, 95, 101, 114, 114]))))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![105, 115, 95, 111, 107, 95, 97, 110, 100])))
    {
        self::ml_string::Text::from_units(vec![98, 111, 111, 108])
    } else if (((*method)
        == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112, 95, 111, 114])))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                117, 110, 119, 114, 97, 112, 95, 111, 114, 95, 100, 101, 102, 97, 117, 108, 116,
            ]))))
        || self::test_rust_primitive_try_from_panic_consumer(method)
    {
        target.clone()
    } else if (*method) == (self::ml_string::Text::from_units(vec![109, 97, 112, 95, 111, 114])) {
        mapper.clone()
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_primitive_try_from_panic_consumer(consumer: &self::ml_string::Text) -> bool {
    ((*consumer) == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112])))
        || ((*consumer) == (self::ml_string::Text::from_units(vec![101, 120, 112, 101, 99, 116])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_failure_debug(
    direction: &self::ml_string::Text,
) -> self::ml_string::Text {
    self::read_rust_primitive_try_from_versioned_failure_debug(direction, 99f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_primitive_try_from_failure_debug(guards: f64) -> Vec<self::ml_string::Text> {
    if (self::ml_array::includes(
        &vec![0f64, 1f64, 2f64, 3f64],
        &guards,
        0f64,
        false,
        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
    )) {
        if (guards == 1f64) || (guards == 3f64) {
            vec![
                self::ml_string::Text::from_units(vec![105, 110, 112, 117, 116]),
                self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]),
                self::ml_string::Text::from_units(vec![103, 101]),
                self::ml_string::Text::from_units(vec![
                    112, 111, 115, 105, 116, 105, 118, 101, 32, 101, 114, 114, 111, 114,
                ]),
                self::ml_string::Text::from_units(vec![
                    110, 101, 103, 97, 116, 105, 118, 101, 32, 101, 114, 114, 111, 114,
                ]),
                self::ml_string::Text::from_units(vec![105, 102]),
            ]
        } else {
            vec![self::ml_string::Text::from_units(vec![
                112, 111, 115, 105, 116, 105, 118, 101, 32, 101, 114, 114, 111, 114,
            ])]
        }
    } else {
        Vec::<self::ml_string::Text>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_panic_prefix(
    consumer: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*consumer) == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112])) {
        self::ml_string::Text::from_units(vec![
            99, 97, 108, 108, 101, 100, 32, 96, 82, 101, 115, 117, 108, 116, 58, 58, 117, 110, 119,
            114, 97, 112, 40, 41, 96, 32, 111, 110, 32, 97, 110, 32, 96, 69, 114, 114, 96, 32, 118,
            97, 108, 117, 101, 58, 32,
        ])
    } else if (*consumer) == (self::ml_string::Text::from_units(vec![101, 120, 112, 101, 99, 116]))
    {
        self::ml_string::Text::from_units(vec![58, 32])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_panic_version_range() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        62, 61, 49, 46, 57, 55, 46, 48, 32, 60, 49, 46, 49, 48, 48, 46, 48,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_versioned_failure_debug(
    direction: &self::ml_string::Text,
    minor: f64,
) -> self::ml_string::Text {
    if ((*direction)
        != (self::ml_string::Text::from_units(vec![
            110, 101, 103, 97, 116, 105, 118, 101, 32, 101, 114, 114, 111, 114,
        ])))
        && ((*direction)
            != (self::ml_string::Text::from_units(vec![
                112, 111, 115, 105, 116, 105, 118, 101, 32, 101, 114, 114, 111, 114,
            ])))
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let model = self::read_rust_integer_conversion_error_debug_model(minor);
            if model
                == (self::ml_string::Text::from_units(vec![
                    117, 110, 105, 116, 32, 112, 97, 121, 108, 111, 97, 100,
                ]))
            {
                self::ml_string::Text::from_units(vec![
                    84, 114, 121, 70, 114, 111, 109, 73, 110, 116, 69, 114, 114, 111, 114, 40, 40,
                    41, 41,
                ])
            } else if model
                == (self::ml_string::Text::from_units(vec![
                    111, 118, 101, 114, 102, 108, 111, 119, 32, 107, 105, 110, 100,
                ]))
            {
                if (*direction)
                    == (self::ml_string::Text::from_units(vec![
                        110, 101, 103, 97, 116, 105, 118, 101, 32, 101, 114, 114, 111, 114,
                    ]))
                {
                    self::ml_string::Text::from_units(vec![
                        84, 114, 121, 70, 114, 111, 109, 73, 110, 116, 69, 114, 114, 111, 114, 40,
                        78, 101, 103, 79, 118, 101, 114, 102, 108, 111, 119, 41,
                    ])
                } else {
                    self::ml_string::Text::from_units(vec![
                        84, 114, 121, 70, 114, 111, 109, 73, 110, 116, 69, 114, 114, 111, 114, 40,
                        80, 111, 115, 79, 118, 101, 114, 102, 108, 111, 119, 41,
                    ])
                }
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_predicate_error(
    kind: &self::ml_string::Text,
    parameters: f64,
    layers: f64,
    parameter: bool,
    result: bool,
) -> self::ml_string::Text {
    if self::accept_rust_scalar_predicate(kind, parameters, layers, parameter, result) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 32, 82, 101, 115, 117, 108, 116, 32, 112, 114, 101, 100, 105, 99, 97, 116, 101, 32,
            110, 101, 101, 100, 115, 32, 97, 32, 100, 105, 114, 101, 99, 116, 32, 111, 110, 101,
            45, 97, 114, 103, 117, 109, 101, 110, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32,
            119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110,
            116, 101, 103, 101, 114, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 97, 110,
            100, 32, 66, 111, 111, 108, 101, 97, 110, 32, 114, 101, 115, 117, 108, 116,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_mapper_error(
    kind: &self::ml_string::Text,
    parameters: f64,
    layers: f64,
    parameter: bool,
    result: bool,
) -> self::ml_string::Text {
    if (((((*kind)
        == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))
        && (parameters == 1f64))
        && (layers == 0f64))
        && parameter)
        && result
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 32, 82, 101, 115, 117, 108, 116, 32, 109, 97, 112, 112, 101, 114, 32, 110, 101,
            101, 100, 115, 32, 97, 32, 100, 105, 114, 101, 99, 116, 32, 111, 110, 101, 45, 97, 114,
            103, 117, 109, 101, 110, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 119, 105,
            116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101,
            103, 101, 114, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 97, 110, 100, 32,
            115, 99, 97, 108, 97, 114, 32, 114, 101, 115, 117, 108, 116,
        ])
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
    kind: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner_kind: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    children: f64,
    proven: bool,
    layers: f64,
) -> self::ml_string::Text {
    if self::accept_rust_primitive_from_source(
        kind, bits, signed, owner_kind, owner, children, proven, layers,
    ) {
        owner.clone()
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function1(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    position: f64,
) -> f64 {
    {
        let open = self::read_required_text_fact(values, position);
        {
            let stack = vec![if open == (self::ml_string::Text::from_units(vec![123])) {
                self::ml_string::Text::from_units(vec![125])
            } else if open == (self::ml_string::Text::from_units(vec![91])) {
                self::ml_string::Text::from_units(vec![93])
            } else {
                self::ml_string::Text::from_units(vec![41])
            }];
            {
                let at = position + 1f64;
                {
                    let source_ml_source2_s36 =
                        self::source_ml_source1_temporary_dependency_function1_loop1(
                            kinds, values, stack, at,
                        );
                    match source_ml_source2_s36 {
                        self::MlSource1TemporaryDependencyFunction1Loop1Result::MlSource1TemporaryDependencyFunction1Loop1Done => {
                            -1f64
                        }
                        self::MlSource1TemporaryDependencyFunction1Loop1Result::MlSource1TemporaryDependencyFunction1Loop1Return(source_ml_source1_temporary_result1) => {
                            source_ml_source1_temporary_result1
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_declaration_plan(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
) -> Vec<self::ml_string::Text> {
    {
        let ml_library_inequality_1 = (kinds.len() as f64) != (values.len() as f64);
        if ml_library_inequality_1 {
            vec![
                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                self::ml_string::Text::from_units(vec![
                    117, 110, 97, 108, 105, 103, 110, 101, 100, 32, 115, 111, 117, 114, 99, 101,
                    32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 32, 99, 111, 108, 117, 109,
                    110, 115,
                ]),
            ]
        } else {
            {
                let result = vec![self::ml_string::Text::from_units(vec![
                    97, 108, 105, 97, 115, 101, 115,
                ])];
                {
                    let names = Vec::<self::ml_string::Text>::new();
                    {
                        let owners = Vec::<self::ml_string::Text>::new();
                        {
                            let closes = Vec::<f64>::new();
                            {
                                let position = 0f64;
                                {
                                    let source_ml_source2_s37 = self::source_ml_source1_temporary_read_rust_type_alias_declaration_plan_loop2(kinds, values, result, names, owners, closes, position);
                                    match source_ml_source2_s37 {
                                    self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Done(result_2) => {
                                        result_2
                                    }
                                    self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Return(source_ml_source1_temporary_result2) => {
                                        source_ml_source1_temporary_result2
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
pub fn read_rust_type_alias_lookup_keys(
    segments: &[self::ml_string::Text],
    context: &[self::ml_string::Text],
) -> Vec<self::ml_string::Text> {
    {
        let names = segments.to_vec();
        {
            let start = context.to_vec();
            {
                let source_ml_source2_s38 = if ((self::ml_array::optional_at(
                    &names,
                    self::ml_array::number_index(0f64),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                    == (self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101]))
                {
                    {
                        let names_2 = self::ml_array::slice(&names, 1f64, f64::INFINITY);
                        {
                            let start_2 = Vec::<self::ml_string::Text>::new();
                            self::MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4::MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4Next(names_2, start_2)
                        }
                    }
                } else if ((self::ml_array::optional_at(
                    &names,
                    self::ml_array::number_index(0f64),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                    == (self::ml_string::Text::from_units(vec![115, 101, 108, 102]))
                {
                    {
                        let names_3 = self::ml_array::slice(&names, 1f64, f64::INFINITY);
                        self::MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4::MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4Next(names_3, start)
                    }
                } else if ((self::ml_array::optional_at(
                    &names,
                    self::ml_array::number_index(0f64),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                    == (self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]))
                {
                    {
                        let count = 0f64;
                        {
                            let count_2 = self::source_ml_source1_temporary_read_rust_type_alias_lookup_keys_loop5(&(names), count);
                            if count_2 > (start.len() as f64) {
                                self::MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4::MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4Return(Vec::<self::ml_string::Text>::new())
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
                                        self::MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4::MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4Next(names_4, start_3)
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4::MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4Next(names, start)
                };
                match source_ml_source2_s38 {
                    self::MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4::MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4Next(names_5, start_4) => {
                        if (names_5.len() as f64) == 0f64 {
                            Vec::<self::ml_string::Text>::new()
                        } else {
                            vec![self::ml_string::join(&{ let (ml_append_right, ml_append_left) = (names_5, start_4); self::ml_array::append(ml_append_left, ml_append_right) }, &self::ml_string::Text::from_units(vec![46]))]
                        }
                    }
                    self::MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4::MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4Return(source_ml_source1_temporary_result4) => {
                        source_ml_source1_temporary_result4
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
) -> self::ml_string::Text {
    if recursive {
        self::ml_string::Text::from_units(vec![
            114, 101, 99, 117, 114, 115, 105, 118, 101, 32, 115, 111, 117, 114, 99, 101, 32, 116,
            121, 112, 101, 32, 97, 108, 105, 97, 115,
        ])
    } else if depth >= 128f64 {
        self::ml_string::Text::from_units(vec![
            115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 32,
            100, 101, 112, 116, 104, 32, 108, 105, 109, 105, 116, 32, 101, 120, 99, 101, 101, 100,
            101, 100,
        ])
    } else if expansions >= 4096f64 {
        self::ml_string::Text::from_units(vec![
            115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 32,
            101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 108, 105, 109, 105, 116, 32, 101, 120,
            99, 101, 101, 100, 101, 100,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_clone_error(current: f64, needed: f64) -> self::ml_string::Text {
    if needed > (262_144f64 - current) {
        self::ml_string::Text::from_units(vec![
            115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 32,
            109, 97, 116, 101, 114, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 108, 105,
            109, 105, 116, 32, 101, 120, 99, 101, 101, 100, 101, 100,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_access_prefix_lengths(count: f64, bare: bool) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let length = if bare { count } else { count - 1f64 };
            {
                self::source_ml_source1_temporary_read_rust_type_alias_access_prefix_lengths_loop6(
                    result, length,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_type_alias_nominal_access(
    kind: &self::ml_string::Text,
    wrappers: f64,
    suffix: f64,
    role: &self::ml_string::Text,
    syntax: &self::ml_string::Text,
) -> bool {
    (((*kind) == (self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100])))
        && (wrappers == 0f64))
        && (((suffix > 0f64)
            || ((*role)
                == (self::ml_string::Text::from_units(vec![
                    105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 111,
                    119, 110, 101, 114,
                ]))))
            || (((*role)
                == (self::ml_string::Text::from_units(vec![
                    98, 114, 97, 99, 101, 100, 32, 114, 101, 99, 111, 114, 100,
                ])))
                && ((*syntax)
                    == (self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100])))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_namespace_disposition(
    alias: bool,
    owner: &self::ml_string::Text,
) -> self::ml_string::Text {
    if alias {
        self::ml_string::Text::from_units(vec![97, 108, 105, 97, 115])
    } else if (((*owner) == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])))
        || ((*owner) == (self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101]))))
        || ((*owner)
            == (self::ml_string::Text::from_units(vec![
                101, 120, 116, 101, 114, 110, 97, 108, 67, 114, 97, 116, 101,
            ])))
    {
        self::ml_string::Text::from_units(vec![98, 108, 111, 99, 107])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_alias_value_constructor_error(
    local: bool,
    value: bool,
    alias: bool,
) -> self::ml_string::Text {
    if (!local && !value) && alias {
        self::ml_string::Text::from_units(vec![
            97, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115,
            32, 104, 97, 115, 32, 110, 111, 32, 105, 110, 100, 101, 112, 101, 110, 100, 101, 110,
            116, 32, 118, 97, 108, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111,
            114,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_type_alias_primitive_member_owner(
    kind: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner_kind: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    children: f64,
    proven: bool,
    layers: f64,
    suffix: f64,
) -> self::ml_string::Text {
    if suffix == 1f64 {
        self::read_rust_primitive_try_from_mapper_owner(
            kind, bits, signed, owner_kind, owner, children, proven, layers,
        )
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_type_alias_owned_string_member(
    kind: &self::ml_string::Text,
    owner_kind: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    children: f64,
    proven: bool,
    layers: f64,
    member: &self::ml_string::Text,
) -> bool {
    (((((((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
        && ((*owner_kind)
            == (self::ml_string::Text::from_units(vec![108, 105, 98, 114, 97, 114, 121]))))
        && ((*owner)
            == (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 115, 116, 114, 105, 110, 103, 58, 58, 83, 116, 114, 105,
                110, 103,
            ]))))
        && (children == 0f64))
        && proven)
        && (layers == 0f64))
        && (((*member) == (self::ml_string::Text::from_units(vec![110, 101, 119])))
            || ((*member) == (self::ml_string::Text::from_units(vec![102, 114, 111, 109]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_type_alias_empty_vector_member(
    kind: &self::ml_string::Text,
    owner_kind: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    children: f64,
    element: &self::ml_string::Text,
    model: &self::ml_string::Text,
    proven: bool,
    layers: f64,
    member: &self::ml_string::Text,
) -> bool {
    (((((((((*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))
        && ((*owner_kind)
            == (self::ml_string::Text::from_units(vec![108, 105, 98, 114, 97, 114, 121]))))
        && ((*owner)
            == (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 118, 101, 99, 58, 58, 86, 101, 99,
            ]))))
        && (children == 1f64))
        && self::accept_rust_source_array_element(element, false))
        && ((*model)
            == (self::ml_string::Text::from_units(vec![
                111, 119, 110, 101, 100, 32, 118, 101, 99, 116, 111, 114,
            ]))))
        && proven)
        && (layers == 0f64))
        && ((*member) == (self::ml_string::Text::from_units(vec![110, 101, 119])))
}
