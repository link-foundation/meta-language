// Generated from complete js/src/translation/rust-primitive-associated-call-rules.js. Do not edit by hand.
use super::MlTemporaryPlanRustIntegerBitsJoin2;
use super::MlTemporaryPlanRustIntegerBitsLoop1Result;
use super::accept_rust_primitive_selection_receiver;
use super::ml_array;
use super::ml_string;
use super::source_ml_temporary_plan_rust_integer_bits_loop1;
use super::source_ml_temporary_plan_rust_integer_bits_loop3;

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
                        let source_ml_source1_concat_value5 = vec![
                            self::ml_string::Text::from_units(vec![
                                111, 112, 101, 114, 97, 110, 100,
                            ]),
                            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                            self::ml_string::Text::from_units(vec![103, 101]),
                        ];
                        {
                            let source_ml_source1_concat_value6 = forward;
                            {
                                let source_ml_source1_concat_value7 = backward;
                                {
                                    let source_ml_source1_concat_value8 =
                                        vec![self::ml_string::Text::from_units(vec![105, 102])];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::ml_string::Text>::new(),
                                                    source_ml_source1_concat_value5,
                                                ),
                                                source_ml_source1_concat_value6,
                                            ),
                                            source_ml_source1_concat_value7,
                                        ),
                                        source_ml_source1_concat_value8,
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
                    let source_ml_source1_concat_value9 =
                        vec![self::ml_string::Text::from_units(vec![
                            111, 112, 101, 114, 97, 110, 100,
                        ])];
                    {
                        let source_ml_source1_concat_value10 = count;
                        {
                            let source_ml_source1_concat_value11 = vec![if (*method)
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
                                        source_ml_source1_concat_value9,
                                    ),
                                    source_ml_source1_concat_value10,
                                ),
                                source_ml_source1_concat_value11,
                            )
                        }
                    }
                }
            } else {
                {
                    let inverse = {
                        let source_ml_source1_concat_value12 = vec![
                            self::ml_string::Text::from_units(vec![119, 105, 100, 116, 104]),
                        ];
                        {
                            let source_ml_source1_concat_value13 = count.clone();
                            {
                                let source_ml_source1_concat_value14 = vec![
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
                                            source_ml_source1_concat_value12,
                                        ),
                                        source_ml_source1_concat_value13,
                                    ),
                                    source_ml_source1_concat_value14,
                                )
                            }
                        }
                    };
                    {
                        let source_ml_source1_concat_value15 =
                            vec![self::ml_string::Text::from_units(vec![
                                117, 110, 115, 105, 103, 110, 101, 100, 32, 111, 112, 101, 114, 97,
                                110, 100,
                            ])];
                        {
                            let source_ml_source1_concat_value16 = count;
                            {
                                let source_ml_source1_concat_value17 = vec![
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
                                    let source_ml_source1_concat_value18 = inverse;
                                    {
                                        let source_ml_source1_concat_value19 = vec![
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
                                                            source_ml_source1_concat_value15,
                                                        ),
                                                        source_ml_source1_concat_value16,
                                                    ),
                                                    source_ml_source1_concat_value17,
                                                ),
                                                source_ml_source1_concat_value18,
                                            ),
                                            source_ml_source1_concat_value19,
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
                                                        let source_ml_source1_s20 = self::source_ml_temporary_plan_rust_integer_bits_loop1(bits, reverse, trailing, leading, ones, plan, bindings, current, count, active, index, bit);
                                                        match source_ml_source1_s20 {
                                                            self::MlTemporaryPlanRustIntegerBitsLoop1Result::MlTemporaryPlanRustIntegerBitsLoop1Done(plan_2, bindings_2, count_2, index_2) => {
                                                                {
                                                                    let plan_3 = {
                                                                        let source_ml_source1_concat_value21 = plan_2;
                                                                        {
                                                                            let source_ml_source1_concat_value22 = vec![if reverse {
                                                                                8f64
                                                                            } else {
                                                                                9f64
                                                                            }, count_2, 0f64, 0f64, 0f64];
                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value21), source_ml_source1_concat_value22)
                                                                        }
                                                                    };
                                                                    {
                                                                        let body = index_2;
                                                                        {
                                                                            let index_3 = index_2 + 1f64;
                                                                            {
                                                                                let source_ml_source1_s25 = if reverse {
                                                                                    {
                                                                                        let plan_4 = {
                                                                                            let source_ml_source1_concat_value26 = plan_3;
                                                                                            {
                                                                                                let source_ml_source1_concat_value27 = vec![19f64, body, 0f64, 0f64, 0f64];
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value26), source_ml_source1_concat_value27)
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let body_2 = index_3;
                                                                                            {
                                                                                                let index_4 = index_3 + 1f64;
                                                                                                self::MlTemporaryPlanRustIntegerBitsJoin2::MlTemporaryPlanRustIntegerBitsJoin2Next(plan_4, index_4, body_2)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlTemporaryPlanRustIntegerBitsJoin2::MlTemporaryPlanRustIntegerBitsJoin2Next(plan_3, index_3, body)
                                                                                };
                                                                                match source_ml_source1_s25 {
                                                                                    self::MlTemporaryPlanRustIntegerBitsJoin2::MlTemporaryPlanRustIntegerBitsJoin2Next(plan_5, index_5, body_3) => {
                                                                                        {
                                                                                            let at = (bindings_2.len() as f64) - 2f64;
                                                                                            {
                                                                                                self::source_ml_temporary_plan_rust_integer_bits_loop3(plan_5, &(bindings_2), index_5, body_3, at)
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
pub fn read_rust_checked_integer_arity(method: &self::ml_string::Text, ordinary: bool) -> f64 {
    if ordinary {
        if self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 97, 100, 100,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 115, 117, 98,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 109, 117, 108,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 100, 105, 118,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 114, 101, 109,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 108,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 114,
                ]),
            ],
            method,
            0f64,
            false,
            |a, b| a == b,
        ) {
            1f64
        } else if self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 110, 101, 103,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 97, 98, 115,
                ]),
            ],
            method,
            0f64,
            false,
            |a, b| a == b,
        ) {
            0f64
        } else {
            -1f64
        }
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_checked_integer_receiver(
    method: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner: &self::ml_string::Text,
    path: &self::ml_string::Text,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((((self::read_rust_checked_integer_arity(method, true)) >= 0f64)
        && ((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]))))
        && (((*method)
            != (self::ml_string::Text::from_units(vec![
                99, 104, 101, 99, 107, 101, 100, 95, 97, 98, 115,
            ])))
            || signed))
        && self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_consumer_arity(consumer: &self::ml_string::Text) -> f64 {
    if (((*consumer)
        == (self::ml_string::Text::from_units(vec![105, 115, 95, 115, 111, 109, 101])))
        || ((*consumer)
            == (self::ml_string::Text::from_units(vec![105, 115, 95, 110, 111, 110, 101]))))
        || ((*consumer)
            == (self::ml_string::Text::from_units(vec![
                117, 110, 119, 114, 97, 112, 95, 111, 114, 95, 100, 101, 102, 97, 117, 108, 116,
            ])))
    {
        0f64
    } else if (*consumer)
        == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112, 95, 111, 114]))
    {
        1f64
    } else if (*consumer) == (self::ml_string::Text::from_units(vec![109, 97, 112, 95, 111, 114])) {
        2f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_argument_owner(
    method: &self::ml_string::Text,
    receiver: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 108,
        ])))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 114,
            ])))
    {
        self::ml_string::Text::from_units(vec![117, 51, 50])
    } else {
        receiver.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_argument_reason(
    method: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 108,
        ])))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 114,
            ])))
    {
        self::ml_string::Text::from_units(vec![
            116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 105, 102, 116, 32, 99, 111,
            117, 110, 116, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 115, 116,
            97, 110, 100, 97, 114, 100, 32, 117, 51, 50, 32, 111, 119, 110, 101, 114, 115, 104,
            105, 112,
        ])
    } else {
        self::ml_string::Text::from_units(vec![
            116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 97, 114, 103, 117, 109, 101, 110,
            116, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32,
            114, 101, 99, 101, 105, 118, 101, 114, 32, 116, 121, 112, 101, 32, 97, 110, 100, 32,
            115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_shift_operation(
    term: &self::ml_string::Text,
) -> self::ml_string::Text {
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
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}
