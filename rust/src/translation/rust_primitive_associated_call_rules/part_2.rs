// Generated from complete js/src/translation/rust-primitive-associated-call-rules.js. Do not edit by hand.
use super::MlSource1TemporaryPlanRustIntegerBitsJoin2;
use super::MlSource1TemporaryPlanRustIntegerBitsLoop1Result;
use super::accept_rust_primitive_selection_receiver;
use super::ml_array;
use super::ml_string;
use super::source_ml_source1_temporary_plan_rust_integer_bits_loop1;
use super::source_ml_source1_temporary_plan_rust_integer_bits_loop3;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_integer_magnitude_result_owner(
    receiver: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*receiver) == (self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101])) {
        self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101])
    } else if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![105, 56]),
            self::ml_string::Text::from_units(vec![105, 49, 54]),
            self::ml_string::Text::from_units(vec![105, 51, 50]),
            self::ml_string::Text::from_units(vec![105, 54, 52]),
            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
        ],
        receiver,
        0f64,
        false,
        |a, b| a == b,
    ) {
        (self::ml_string::Text::from_units(vec![117])).concat(&{
            let source_ml_source2_string_range_operand7 = receiver.clone();
            {
                let source_ml_source2_string_range_operand8 = 1f64;
                {
                    let source_ml_source2_string_range_position9 =
                        source_ml_source2_string_range_operand8;
                    {
                        let source_ml_source2_string_range_position10 = f64::INFINITY;
                        source_ml_source2_string_range_operand7.slice(
                            source_ml_source2_string_range_position9,
                            source_ml_source2_string_range_position10,
                        )
                    }
                }
            }
        })
    } else if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![117, 56]),
            self::ml_string::Text::from_units(vec![117, 49, 54]),
            self::ml_string::Text::from_units(vec![117, 51, 50]),
            self::ml_string::Text::from_units(vec![117, 54, 52]),
            self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
            self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]),
        ],
        receiver,
        0f64,
        false,
        |a, b| a == b,
    ) {
        receiver.clone()
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_integer_magnitude(method: &self::ml_string::Text) -> Vec<self::ml_string::Text> {
    if (*method)
        == (self::ml_string::Text::from_units(vec![
            117, 110, 115, 105, 103, 110, 101, 100, 95, 97, 98, 115,
        ]))
    {
        vec![
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
            self::ml_string::Text::from_units(vec![108, 116]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![
                112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 105, 116,
            ]),
            self::ml_string::Text::from_units(vec![97, 100, 100]),
            self::ml_string::Text::from_units(vec![110, 101, 103]),
            self::ml_string::Text::from_units(vec![
                117, 110, 115, 105, 103, 110, 101, 100, 32, 99, 97, 115, 116,
            ]),
            self::ml_string::Text::from_units(vec![
                117, 110, 115, 105, 103, 110, 101, 100, 32, 117, 110, 105, 116,
            ]),
            self::ml_string::Text::from_units(vec![97, 100, 100]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![
                117, 110, 115, 105, 103, 110, 101, 100, 32, 99, 97, 115, 116,
            ]),
            self::ml_string::Text::from_units(vec![105, 102]),
        ]
    } else {
        let ml_library_inequality_1 = (*method)
            != (self::ml_string::Text::from_units(vec![97, 98, 115, 95, 100, 105, 102, 102]));
        if ml_library_inequality_1 {
            Vec::<self::ml_string::Text>::new()
        } else {
            {
                let forward = vec![
                    self::ml_string::Text::from_units(vec![
                        117, 110, 115, 105, 103, 110, 101, 100, 32, 111, 112, 101, 114, 97, 110,
                        100,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        117, 110, 115, 105, 103, 110, 101, 100, 32, 114, 105, 103, 104, 116,
                    ]),
                    self::ml_string::Text::from_units(vec![108, 116]),
                    self::ml_string::Text::from_units(vec![
                        117, 110, 115, 105, 103, 110, 101, 100, 32, 109, 97, 120, 105, 109, 117,
                        109,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        117, 110, 115, 105, 103, 110, 101, 100, 32, 114, 105, 103, 104, 116,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        117, 110, 115, 105, 103, 110, 101, 100, 32, 111, 112, 101, 114, 97, 110,
                        100,
                    ]),
                    self::ml_string::Text::from_units(vec![115, 117, 98]),
                    self::ml_string::Text::from_units(vec![115, 117, 98]),
                    self::ml_string::Text::from_units(vec![
                        117, 110, 115, 105, 103, 110, 101, 100, 32, 117, 110, 105, 116,
                    ]),
                    self::ml_string::Text::from_units(vec![97, 100, 100]),
                    self::ml_string::Text::from_units(vec![
                        117, 110, 115, 105, 103, 110, 101, 100, 32, 111, 112, 101, 114, 97, 110,
                        100,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        117, 110, 115, 105, 103, 110, 101, 100, 32, 114, 105, 103, 104, 116,
                    ]),
                    self::ml_string::Text::from_units(vec![115, 117, 98]),
                    self::ml_string::Text::from_units(vec![105, 102]),
                ];
                {
                    let backward = vec![
                        self::ml_string::Text::from_units(vec![
                            117, 110, 115, 105, 103, 110, 101, 100, 32, 114, 105, 103, 104, 116,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            117, 110, 115, 105, 103, 110, 101, 100, 32, 111, 112, 101, 114, 97,
                            110, 100,
                        ]),
                        self::ml_string::Text::from_units(vec![108, 116]),
                        self::ml_string::Text::from_units(vec![
                            117, 110, 115, 105, 103, 110, 101, 100, 32, 109, 97, 120, 105, 109,
                            117, 109,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            117, 110, 115, 105, 103, 110, 101, 100, 32, 111, 112, 101, 114, 97,
                            110, 100,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            117, 110, 115, 105, 103, 110, 101, 100, 32, 114, 105, 103, 104, 116,
                        ]),
                        self::ml_string::Text::from_units(vec![115, 117, 98]),
                        self::ml_string::Text::from_units(vec![115, 117, 98]),
                        self::ml_string::Text::from_units(vec![
                            117, 110, 115, 105, 103, 110, 101, 100, 32, 117, 110, 105, 116,
                        ]),
                        self::ml_string::Text::from_units(vec![97, 100, 100]),
                        self::ml_string::Text::from_units(vec![
                            117, 110, 115, 105, 103, 110, 101, 100, 32, 114, 105, 103, 104, 116,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            117, 110, 115, 105, 103, 110, 101, 100, 32, 111, 112, 101, 114, 97,
                            110, 100,
                        ]),
                        self::ml_string::Text::from_units(vec![115, 117, 98]),
                        self::ml_string::Text::from_units(vec![105, 102]),
                    ];
                    {
                        let source_ml_source2_concat_value11 = vec![
                            self::ml_string::Text::from_units(vec![
                                111, 112, 101, 114, 97, 110, 100,
                            ]),
                            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                            self::ml_string::Text::from_units(vec![103, 101]),
                        ];
                        {
                            let source_ml_source2_concat_value12 = forward;
                            {
                                let source_ml_source2_concat_value13 = backward;
                                {
                                    let source_ml_source2_concat_value14 =
                                        vec![self::ml_string::Text::from_units(vec![105, 102])];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::ml_string::Text>::new(),
                                                    source_ml_source2_concat_value11,
                                                ),
                                                source_ml_source2_concat_value12,
                                            ),
                                            source_ml_source2_concat_value13,
                                        ),
                                        source_ml_source2_concat_value14,
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
pub fn accept_rust_wrapping_shift_method(method: &self::ml_string::Text) -> bool {
    self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![
                119, 114, 97, 112, 112, 105, 110, 103, 95, 115, 104, 108,
            ]),
            self::ml_string::Text::from_units(vec![
                119, 114, 97, 112, 112, 105, 110, 103, 95, 115, 104, 114,
            ]),
            self::ml_string::Text::from_units(vec![
                114, 111, 116, 97, 116, 101, 95, 108, 101, 102, 116,
            ]),
            self::ml_string::Text::from_units(vec![
                114, 111, 116, 97, 116, 101, 95, 114, 105, 103, 104, 116,
            ]),
        ],
        method,
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_wrapping_shift_receiver(
    kind: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner: &self::ml_string::Text,
    path: &self::ml_string::Text,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
        && self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_wrapping_shift_operation(term: &self::ml_string::Text) -> self::ml_string::Text {
    if (*term)
        == (self::ml_string::Text::from_units(vec![
            115, 104, 105, 102, 116, 32, 108, 101, 102, 116,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            115, 104, 105, 102, 116, 82, 117, 115, 116, 80, 114, 105, 109, 105, 116, 105, 118, 101,
            66, 105, 116, 115, 76, 101, 102, 116,
        ])
    } else if (*term)
        == (self::ml_string::Text::from_units(vec![
            115, 104, 105, 102, 116, 32, 114, 105, 103, 104, 116,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            115, 104, 105, 102, 116, 82, 117, 115, 116, 80, 114, 105, 109, 105, 116, 105, 118, 101,
            66, 105, 116, 115, 82, 105, 103, 104, 116,
        ])
    } else if (*term) == (self::ml_string::Text::from_units(vec![98, 105, 116, 115, 32, 111, 114]))
    {
        self::ml_string::Text::from_units(vec![
            100, 105, 115, 106, 111, 105, 110, 82, 117, 115, 116, 80, 114, 105, 109, 105, 116, 105,
            118, 101, 66, 105, 116, 115,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_wrapping_shift(method: &self::ml_string::Text) -> Vec<self::ml_string::Text> {
    if (self::accept_rust_wrapping_shift_method(method)) {
        {
            let count = vec![
                self::ml_string::Text::from_units(vec![99, 111, 117, 110, 116]),
                self::ml_string::Text::from_units(vec![119, 105, 100, 116, 104]),
                self::ml_string::Text::from_units(vec![114, 101, 109]),
            ];
            if ((*method)
                == (self::ml_string::Text::from_units(vec![
                    119, 114, 97, 112, 112, 105, 110, 103, 95, 115, 104, 108,
                ])))
                || ((*method)
                    == (self::ml_string::Text::from_units(vec![
                        119, 114, 97, 112, 112, 105, 110, 103, 95, 115, 104, 114,
                    ])))
            {
                {
                    let source_ml_source2_concat_value15 =
                        vec![self::ml_string::Text::from_units(vec![
                            111, 112, 101, 114, 97, 110, 100,
                        ])];
                    {
                        let source_ml_source2_concat_value16 = count;
                        {
                            let source_ml_source2_concat_value17 = vec![if (*method)
                                == (self::ml_string::Text::from_units(vec![
                                    119, 114, 97, 112, 112, 105, 110, 103, 95, 115, 104, 108,
                                ])) {
                                self::ml_string::Text::from_units(vec![
                                    115, 104, 105, 102, 116, 32, 108, 101, 102, 116,
                                ])
                            } else {
                                self::ml_string::Text::from_units(vec![
                                    115, 104, 105, 102, 116, 32, 114, 105, 103, 104, 116,
                                ])
                            }];
                            self::ml_array::append(
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source2_concat_value15,
                                    ),
                                    source_ml_source2_concat_value16,
                                ),
                                source_ml_source2_concat_value17,
                            )
                        }
                    }
                }
            } else {
                {
                    let inverse = {
                        let source_ml_source2_concat_value18 = vec![
                            self::ml_string::Text::from_units(vec![119, 105, 100, 116, 104]),
                        ];
                        {
                            let source_ml_source2_concat_value19 = count.clone();
                            {
                                let source_ml_source2_concat_value20 = vec![
                                    self::ml_string::Text::from_units(vec![115, 117, 98]),
                                    self::ml_string::Text::from_units(vec![
                                        119, 105, 100, 116, 104,
                                    ]),
                                    self::ml_string::Text::from_units(vec![114, 101, 109]),
                                ];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::ml_string::Text>::new(),
                                            source_ml_source2_concat_value18,
                                        ),
                                        source_ml_source2_concat_value19,
                                    ),
                                    source_ml_source2_concat_value20,
                                )
                            }
                        }
                    };
                    {
                        let source_ml_source2_concat_value21 =
                            vec![self::ml_string::Text::from_units(vec![
                                117, 110, 115, 105, 103, 110, 101, 100, 32, 111, 112, 101, 114, 97,
                                110, 100,
                            ])];
                        {
                            let source_ml_source2_concat_value22 = count;
                            {
                                let source_ml_source2_concat_value23 = vec![
                                    if (*method)
                                        == (self::ml_string::Text::from_units(vec![
                                            114, 111, 116, 97, 116, 101, 95, 108, 101, 102, 116,
                                        ]))
                                    {
                                        self::ml_string::Text::from_units(vec![
                                            115, 104, 105, 102, 116, 32, 108, 101, 102, 116,
                                        ])
                                    } else {
                                        self::ml_string::Text::from_units(vec![
                                            115, 104, 105, 102, 116, 32, 114, 105, 103, 104, 116,
                                        ])
                                    },
                                    self::ml_string::Text::from_units(vec![
                                        117, 110, 115, 105, 103, 110, 101, 100, 32, 111, 112, 101,
                                        114, 97, 110, 100,
                                    ]),
                                ];
                                {
                                    let source_ml_source2_concat_value24 = inverse;
                                    {
                                        let source_ml_source2_concat_value25 = vec![
                                            if (*method)
                                                == (self::ml_string::Text::from_units(vec![
                                                    114, 111, 116, 97, 116, 101, 95, 108, 101, 102,
                                                    116,
                                                ]))
                                            {
                                                self::ml_string::Text::from_units(vec![
                                                    115, 104, 105, 102, 116, 32, 114, 105, 103,
                                                    104, 116,
                                                ])
                                            } else {
                                                self::ml_string::Text::from_units(vec![
                                                    115, 104, 105, 102, 116, 32, 108, 101, 102, 116,
                                                ])
                                            },
                                            self::ml_string::Text::from_units(vec![
                                                98, 105, 116, 115, 32, 111, 114,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                115, 111, 117, 114, 99, 101, 32, 99, 97, 115, 116,
                                            ]),
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<self::ml_string::Text>::new(),
                                                            source_ml_source2_concat_value21,
                                                        ),
                                                        source_ml_source2_concat_value22,
                                                    ),
                                                    source_ml_source2_concat_value23,
                                                ),
                                                source_ml_source2_concat_value24,
                                            ),
                                            source_ml_source2_concat_value25,
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        Vec::<self::ml_string::Text>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_integer_bit_method(method: &self::ml_string::Text) -> bool {
    self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![99, 111, 117, 110, 116, 95, 111, 110, 101, 115]),
            self::ml_string::Text::from_units(vec![
                99, 111, 117, 110, 116, 95, 122, 101, 114, 111, 115,
            ]),
            self::ml_string::Text::from_units(vec![
                108, 101, 97, 100, 105, 110, 103, 95, 111, 110, 101, 115,
            ]),
            self::ml_string::Text::from_units(vec![
                108, 101, 97, 100, 105, 110, 103, 95, 122, 101, 114, 111, 115,
            ]),
            self::ml_string::Text::from_units(vec![
                116, 114, 97, 105, 108, 105, 110, 103, 95, 111, 110, 101, 115,
            ]),
            self::ml_string::Text::from_units(vec![
                116, 114, 97, 105, 108, 105, 110, 103, 95, 122, 101, 114, 111, 115,
            ]),
            self::ml_string::Text::from_units(vec![
                114, 101, 118, 101, 114, 115, 101, 95, 98, 105, 116, 115,
            ]),
        ],
        method,
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_integer_bit_receiver(
    kind: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner: &self::ml_string::Text,
    path: &self::ml_string::Text,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
        && self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_integer_bit_result_owner(
    method: &self::ml_string::Text,
    receiver: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (self::accept_rust_integer_bit_method(method)) {
        if (*method)
            == (self::ml_string::Text::from_units(vec![
                114, 101, 118, 101, 114, 115, 101, 95, 98, 105, 116, 115,
            ]))
        {
            receiver.clone()
        } else {
            self::ml_string::Text::from_units(vec![117, 51, 50])
        }
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_integer_bit_plan_operation(code: f64) -> self::ml_string::Text {
    if code == 0f64 {
        self::ml_string::Text::from_units(vec![
            117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 112, 117, 116,
        ])
    } else if code == 1f64 {
        self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100, 32, 122, 101, 114, 111])
    } else if code == 2f64 {
        self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100, 32, 111, 110, 101])
    } else if code == 3f64 {
        self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100, 32, 116, 119, 111])
    } else if code == 4f64 {
        self::ml_string::Text::from_units(vec![99, 111, 117, 110, 116, 32, 122, 101, 114, 111])
    } else if code == 5f64 {
        self::ml_string::Text::from_units(vec![99, 111, 117, 110, 116, 32, 111, 110, 101])
    } else if code == 6f64 {
        self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
    } else if code == 7f64 {
        self::ml_string::Text::from_units(vec![116, 114, 117, 101])
    } else if code == 8f64 {
        self::ml_string::Text::from_units(vec![
            102, 105, 120, 101, 100, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ])
    } else if code == 9f64 {
        self::ml_string::Text::from_units(vec![
            99, 111, 117, 110, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ])
    } else if code == 10f64 {
        self::ml_string::Text::from_units(vec![
            98, 111, 111, 108, 101, 97, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ])
    } else if code == 11f64 {
        self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])
    } else if code == 12f64 {
        self::ml_string::Text::from_units(vec![114, 101, 109, 97, 105, 110, 100, 101, 114])
    } else if code == 13f64 {
        self::ml_string::Text::from_units(vec![100, 105, 118, 105, 115, 105, 111, 110])
    } else if code == 14f64 {
        self::ml_string::Text::from_units(vec![
            99, 111, 117, 110, 116, 32, 97, 100, 100, 105, 116, 105, 111, 110,
        ])
    } else if code == 15f64 {
        self::ml_string::Text::from_units(vec![
            102, 105, 120, 101, 100, 32, 97, 100, 100, 105, 116, 105, 111, 110,
        ])
    } else if code == 16f64 {
        self::ml_string::Text::from_units(vec![
            102, 105, 120, 101, 100, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105,
            111, 110,
        ])
    } else if code == 17f64 {
        self::ml_string::Text::from_units(vec![99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])
    } else if code == 18f64 {
        self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103])
    } else if code == 19f64 {
        self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 115, 116])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_integer_bits(method: &self::ml_string::Text, bits: f64) -> Vec<f64> {
    if !(self::accept_rust_integer_bit_method(method))
        || !(self::ml_array::includes(
            &vec![8f64, 16f64, 32f64, 64f64, 128f64],
            &bits,
            0f64,
            false,
            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
        ))
    {
        Vec::<f64>::new()
    } else {
        {
            let reverse = (*method)
                == (self::ml_string::Text::from_units(vec![
                    114, 101, 118, 101, 114, 115, 101, 95, 98, 105, 116, 115,
                ]));
            {
                let trailing = ((*method)
                    == (self::ml_string::Text::from_units(vec![
                        116, 114, 97, 105, 108, 105, 110, 103, 95, 122, 101, 114, 111, 115,
                    ])))
                    || ((*method)
                        == (self::ml_string::Text::from_units(vec![
                            116, 114, 97, 105, 108, 105, 110, 103, 95, 111, 110, 101, 115,
                        ])));
                {
                    let leading = ((*method)
                        == (self::ml_string::Text::from_units(vec![
                            108, 101, 97, 100, 105, 110, 103, 95, 122, 101, 114, 111, 115,
                        ])))
                        || ((*method)
                            == (self::ml_string::Text::from_units(vec![
                                108, 101, 97, 100, 105, 110, 103, 95, 111, 110, 101, 115,
                            ])));
                    {
                        let ones = method.ends_with(&self::ml_string::Text::from_units(vec![
                            111, 110, 101, 115,
                        ]));
                        {
                            let plan = vec![
                                0f64, 0f64, 0f64, 0f64, 0f64, 1f64, 0f64, 0f64, 0f64, 0f64, 2f64,
                                0f64, 0f64, 0f64, 0f64, 3f64, 0f64, 0f64, 0f64, 0f64, 4f64, 0f64,
                                0f64, 0f64, 0f64, 5f64, 0f64, 0f64, 0f64, 0f64, 6f64, 0f64, 0f64,
                                0f64, 0f64, 7f64, 0f64, 0f64, 0f64, 0f64,
                            ];
                            {
                                let bindings = vec![
                                    0f64,
                                    0f64,
                                    1f64,
                                    if reverse { 1f64 } else { 4f64 },
                                    2f64,
                                    7f64,
                                ];
                                {
                                    let current = 0f64;
                                    {
                                        let count = 1f64;
                                        {
                                            let active = 2f64;
                                            {
                                                let index = 8f64;
                                                {
                                                    let bit = 0f64;
                                                    {
                                                        let source_ml_source2_s26 = self::source_ml_source1_temporary_plan_rust_integer_bits_loop1(bits, reverse, trailing, leading, ones, plan, bindings, current, count, active, index, bit);
                                                        match source_ml_source2_s26 {
                                                            self::MlSource1TemporaryPlanRustIntegerBitsLoop1Result::MlSource1TemporaryPlanRustIntegerBitsLoop1Done(plan_2, bindings_2, count_2, index_2) => {
                                                                {
                                                                    let plan_3 = {
                                                                        let source_ml_source2_concat_value27 = plan_2;
                                                                        {
                                                                            let source_ml_source2_concat_value28 = vec![if reverse {
                                                                                8f64
                                                                            } else {
                                                                                9f64
                                                                            }, count_2, 0f64, 0f64, 0f64];
                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value27), source_ml_source2_concat_value28)
                                                                        }
                                                                    };
                                                                    {
                                                                        let body = index_2;
                                                                        {
                                                                            let index_3 = index_2 + 1f64;
                                                                            {
                                                                                let source_ml_source2_s31 = if reverse {
                                                                                    {
                                                                                        let plan_4 = {
                                                                                            let source_ml_source2_concat_value32 = plan_3;
                                                                                            {
                                                                                                let source_ml_source2_concat_value33 = vec![19f64, body, 0f64, 0f64, 0f64];
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value32), source_ml_source2_concat_value33)
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let body_2 = index_3;
                                                                                            {
                                                                                                let index_4 = index_3 + 1f64;
                                                                                                self::MlSource1TemporaryPlanRustIntegerBitsJoin2::MlSource1TemporaryPlanRustIntegerBitsJoin2Next(plan_4, index_4, body_2)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryPlanRustIntegerBitsJoin2::MlSource1TemporaryPlanRustIntegerBitsJoin2Next(plan_3, index_3, body)
                                                                                };
                                                                                match source_ml_source2_s31 {
                                                                                    self::MlSource1TemporaryPlanRustIntegerBitsJoin2::MlSource1TemporaryPlanRustIntegerBitsJoin2Next(plan_5, index_5, body_3) => {
                                                                                        {
                                                                                            let at = (bindings_2.len() as f64) - 2f64;
                                                                                            {
                                                                                                self::source_ml_source1_temporary_plan_rust_integer_bits_loop3(plan_5, &(bindings_2), index_5, body_3, at)
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
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_panic_value_operation() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        114, 97, 105, 115, 101, 82, 117, 115, 116, 80, 97, 110, 105, 99, 86, 97, 108, 117, 101,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_rust_panic_value(
    kind: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> bool {
    ((*kind) == (self::ml_string::Text::from_units(vec![109, 97, 116, 104])))
        && ((*operation) == (self::read_rust_panic_value_operation()))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_panic_value_argument_error(
    language: &self::ml_string::Text,
    count: f64,
    spread: bool,
    kind: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((((*language) == (self::ml_string::Text::from_units(vec![82, 117, 115, 116])))
        && (count == 1f64))
        && !spread)
        && ((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 32, 82, 117, 115, 116, 32, 112, 97, 110, 105, 99, 32, 118, 97, 108, 117, 101, 32,
            110, 101, 101, 100, 115, 32, 111, 110, 101, 32, 99, 104, 101, 99, 107, 101, 100, 32,
            115, 111, 117, 114, 99, 101, 32, 83, 116, 114, 105, 110, 103, 32, 109, 101, 115, 115,
            97, 103, 101,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_panic_value_expression(
    target: &self::ml_string::Text,
    message: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*target)
        == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))
    {
        ((self::ml_string::Text::from_units(vec![
            40, 40, 109, 108, 95, 109, 101, 115, 115, 97, 103, 101, 41, 61, 62, 123, 116, 104, 114,
            111, 119, 32, 110, 101, 119, 32, 69, 114, 114, 111, 114, 40, 109, 108, 95, 109, 101,
            115, 115, 97, 103, 101, 41, 59, 125, 41, 40,
        ]))
        .concat(message))
        .concat(&self::ml_string::Text::from_units(vec![41]))
    } else if (*target) == (self::ml_string::Text::from_units(vec![82, 117, 115, 116])) {
        ((self::ml_string::Text::from_units(vec![
            112, 97, 110, 105, 99, 33, 40, 34, 123, 125, 34, 44, 32,
        ]))
        .concat(message))
        .concat(&self::ml_string::Text::from_units(vec![41]))
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_panic_formatting_macro(name: &self::ml_string::Text, ordinary: bool) -> bool {
    ordinary
        && (((((*name) == (self::ml_string::Text::from_units(vec![112, 97, 110, 105, 99])))
            || ((*name)
                == (self::ml_string::Text::from_units(vec![
                    117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101,
                ]))))
            || ((*name)
                == (self::ml_string::Text::from_units(vec![
                    117, 110, 105, 109, 112, 108, 101, 109, 101, 110, 116, 101, 100,
                ]))))
            || ((*name) == (self::ml_string::Text::from_units(vec![116, 111, 100, 111]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_panic_formatting_value(
    name: &self::ml_string::Text,
    ordinary: bool,
    kind: &self::ml_string::Text,
    count: f64,
    braces: bool,
) -> bool {
    (self::accept_rust_panic_formatting_macro(name, ordinary) && (count > 0f64))
        && ((count > 1f64)
            || (((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114]))) && braces))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_panic_formatting_prefix(name: &self::ml_string::Text) -> self::ml_string::Text {
    if (*name)
        == (self::ml_string::Text::from_units(vec![
            117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            105, 110, 116, 101, 114, 110, 97, 108, 32, 101, 114, 114, 111, 114, 58, 32, 101, 110,
            116, 101, 114, 101, 100, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 32, 99,
            111, 100, 101, 58, 32,
        ])
    } else if (*name)
        == (self::ml_string::Text::from_units(vec![
            117, 110, 105, 109, 112, 108, 101, 109, 101, 110, 116, 101, 100,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            110, 111, 116, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 101, 100, 58, 32,
        ])
    } else if (*name) == (self::ml_string::Text::from_units(vec![116, 111, 100, 111])) {
        self::ml_string::Text::from_units(vec![
            110, 111, 116, 32, 121, 101, 116, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 101,
            100, 58, 32,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_panic_completion_kind(
    kind: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if self::accept_linked_rust_panic_value(kind, operation) {
        self::ml_string::Text::from_units(vec![97, 98, 111, 114, 116])
    } else {
        kind.clone()
    }
}
