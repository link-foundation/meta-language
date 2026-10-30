// Generated from complete js/src/translation/rust-primitive-associated-call-rules.js. Do not edit by hand.
use super::accept_rust_primitive_selection_receiver;
use super::accept_rust_scalar_predicate;
use super::ml_array;
use super::ml_string;
use super::test_rust_option_panic_consumer;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_checked_integer_condition(
    method: &self::ml_string::Text,
    signed: bool,
) -> Vec<self::ml_string::Text> {
    if ((*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 108,
        ])))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 114,
            ])))
    {
        vec![
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![119, 105, 100, 116, 104]),
            self::ml_string::Text::from_units(vec![108, 116]),
        ]
    } else if (*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 110, 101, 103,
        ]))
    {
        if signed {
            vec![
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]),
                self::ml_string::Text::from_units(vec![110, 101]),
            ]
        } else {
            vec![
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                self::ml_string::Text::from_units(vec![101, 113]),
            ]
        }
    } else if (*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 97, 98, 115,
        ]))
    {
        if signed {
            vec![
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]),
                self::ml_string::Text::from_units(vec![110, 101]),
            ]
        } else {
            Vec::<self::ml_string::Text>::new()
        }
    } else if (*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 97, 100, 100,
        ]))
    {
        if signed {
            vec![
                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                self::ml_string::Text::from_units(vec![103, 116]),
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109]),
                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                self::ml_string::Text::from_units(vec![115, 117, 98]),
                self::ml_string::Text::from_units(vec![108, 101]),
                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                self::ml_string::Text::from_units(vec![108, 116]),
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]),
                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                self::ml_string::Text::from_units(vec![115, 117, 98]),
                self::ml_string::Text::from_units(vec![103, 101]),
                self::ml_string::Text::from_units(vec![116, 114, 117, 101]),
                self::ml_string::Text::from_units(vec![105, 102]),
                self::ml_string::Text::from_units(vec![105, 102]),
            ]
        } else {
            vec![
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109]),
                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                self::ml_string::Text::from_units(vec![115, 117, 98]),
                self::ml_string::Text::from_units(vec![108, 101]),
            ]
        }
    } else if (*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 115, 117, 98,
        ]))
    {
        if signed {
            vec![
                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                self::ml_string::Text::from_units(vec![108, 116]),
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109]),
                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                self::ml_string::Text::from_units(vec![97, 100, 100]),
                self::ml_string::Text::from_units(vec![108, 101]),
                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                self::ml_string::Text::from_units(vec![103, 116]),
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]),
                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                self::ml_string::Text::from_units(vec![97, 100, 100]),
                self::ml_string::Text::from_units(vec![103, 101]),
                self::ml_string::Text::from_units(vec![116, 114, 117, 101]),
                self::ml_string::Text::from_units(vec![105, 102]),
                self::ml_string::Text::from_units(vec![105, 102]),
            ]
        } else {
            vec![
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                self::ml_string::Text::from_units(vec![103, 101]),
            ]
        }
    } else if (*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 109, 117, 108,
        ]))
    {
        {
            let positive_positive = vec![
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109]),
                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                self::ml_string::Text::from_units(vec![100, 105, 118]),
                self::ml_string::Text::from_units(vec![108, 101]),
            ];
            if signed {
                {
                    let positive_negative = vec![
                        self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                        self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]),
                        self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                        self::ml_string::Text::from_units(vec![100, 105, 118]),
                        self::ml_string::Text::from_units(vec![103, 101]),
                    ];
                    {
                        let negative_positive = vec![
                            self::ml_string::Text::from_units(vec![
                                111, 112, 101, 114, 97, 110, 100,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                109, 105, 110, 105, 109, 117, 109,
                            ]),
                            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                            self::ml_string::Text::from_units(vec![100, 105, 118]),
                            self::ml_string::Text::from_units(vec![103, 101]),
                        ];
                        {
                            let negative_negative = vec![
                                self::ml_string::Text::from_units(vec![
                                    111, 112, 101, 114, 97, 110, 100,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    109, 97, 120, 105, 109, 117, 109,
                                ]),
                                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                                self::ml_string::Text::from_units(vec![100, 105, 118]),
                                self::ml_string::Text::from_units(vec![103, 101]),
                            ];
                            {
                                let signs = {
                                    let source_ml_source2_concat_value47 = vec![
                                        self::ml_string::Text::from_units(vec![
                                            111, 112, 101, 114, 97, 110, 100,
                                        ]),
                                        self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                                        self::ml_string::Text::from_units(vec![103, 116]),
                                        self::ml_string::Text::from_units(vec![
                                            114, 105, 103, 104, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                                        self::ml_string::Text::from_units(vec![103, 116]),
                                    ];
                                    {
                                        let source_ml_source2_concat_value48 = positive_positive;
                                        {
                                            let source_ml_source2_concat_value49 =
                                                positive_negative;
                                            {
                                                let source_ml_source2_concat_value50 = vec![
                                                    self::ml_string::Text::from_units(vec![
                                                        105, 102,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        114, 105, 103, 104, 116,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        122, 101, 114, 111,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        103, 116,
                                                    ]),
                                                ];
                                                {
                                                    let source_ml_source2_concat_value51 =
                                                        negative_positive;
                                                    {
                                                        let source_ml_source2_concat_value52 =
                                                            negative_negative;
                                                        {
                                                            let source_ml_source2_concat_value53 = vec![
                                                                self::ml_string::Text::from_units(
                                                                    vec![105, 102],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![105, 102],
                                                                ),
                                                            ];
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value47), source_ml_source2_concat_value48), source_ml_source2_concat_value49), source_ml_source2_concat_value50), source_ml_source2_concat_value51), source_ml_source2_concat_value52), source_ml_source2_concat_value53)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                                {
                                    let source_ml_source2_concat_value54 = vec![
                                        self::ml_string::Text::from_units(vec![
                                            111, 112, 101, 114, 97, 110, 100,
                                        ]),
                                        self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                                        self::ml_string::Text::from_units(vec![101, 113]),
                                        self::ml_string::Text::from_units(vec![116, 114, 117, 101]),
                                        self::ml_string::Text::from_units(vec![
                                            114, 105, 103, 104, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                                        self::ml_string::Text::from_units(vec![101, 113]),
                                        self::ml_string::Text::from_units(vec![116, 114, 117, 101]),
                                    ];
                                    {
                                        let source_ml_source2_concat_value55 = signs;
                                        {
                                            let source_ml_source2_concat_value56 = vec![
                                                self::ml_string::Text::from_units(vec![105, 102]),
                                                self::ml_string::Text::from_units(vec![105, 102]),
                                            ];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<self::ml_string::Text>::new(),
                                                        source_ml_source2_concat_value54,
                                                    ),
                                                    source_ml_source2_concat_value55,
                                                ),
                                                source_ml_source2_concat_value56,
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
                    let source_ml_source2_concat_value44 = vec![
                        self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                        self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                        self::ml_string::Text::from_units(vec![101, 113]),
                        self::ml_string::Text::from_units(vec![116, 114, 117, 101]),
                        self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                        self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                        self::ml_string::Text::from_units(vec![101, 113]),
                        self::ml_string::Text::from_units(vec![116, 114, 117, 101]),
                    ];
                    {
                        let source_ml_source2_concat_value45 = positive_positive;
                        {
                            let source_ml_source2_concat_value46 = vec![
                                self::ml_string::Text::from_units(vec![105, 102]),
                                self::ml_string::Text::from_units(vec![105, 102]),
                            ];
                            self::ml_array::append(
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source2_concat_value44,
                                    ),
                                    source_ml_source2_concat_value45,
                                ),
                                source_ml_source2_concat_value46,
                            )
                        }
                    }
                }
            }
        }
    } else if ((((*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 100, 105, 118,
        ])))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                99, 104, 101, 99, 107, 101, 100, 95, 114, 101, 109,
            ]))))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                99, 104, 101, 99, 107, 101, 100, 95, 100, 105, 118, 95, 101, 117, 99, 108, 105, 100,
            ]))))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                99, 104, 101, 99, 107, 101, 100, 95, 114, 101, 109, 95, 101, 117, 99, 108, 105, 100,
            ])))
    {
        if signed {
            vec![
                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                self::ml_string::Text::from_units(vec![101, 113]),
                self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101]),
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]),
                self::ml_string::Text::from_units(vec![101, 113]),
                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                self::ml_string::Text::from_units(vec![
                    110, 101, 103, 97, 116, 105, 118, 101, 32, 117, 110, 105, 116,
                ]),
                self::ml_string::Text::from_units(vec![110, 101]),
                self::ml_string::Text::from_units(vec![116, 114, 117, 101]),
                self::ml_string::Text::from_units(vec![105, 102]),
                self::ml_string::Text::from_units(vec![105, 102]),
            ]
        } else {
            vec![
                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                self::ml_string::Text::from_units(vec![110, 101]),
            ]
        }
    } else {
        Vec::<self::ml_string::Text>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_checked_integer_success(
    method: &self::ml_string::Text,
    signed: bool,
) -> Vec<self::ml_string::Text> {
    if ((*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 100, 105, 118, 95, 101, 117, 99, 108, 105, 100,
        ])))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                99, 104, 101, 99, 107, 101, 100, 95, 114, 101, 109, 95, 101, 117, 99, 108, 105, 100,
            ])))
    {
        {
            let quotient = vec![
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                self::ml_string::Text::from_units(vec![100, 105, 118]),
            ];
            {
                let remainder = vec![
                    self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                    self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                    self::ml_string::Text::from_units(vec![114, 101, 109]),
                ];
                if signed {
                    {
                        let positive = if (*method)
                            == (self::ml_string::Text::from_units(vec![
                                99, 104, 101, 99, 107, 101, 100, 95, 100, 105, 118, 95, 101, 117,
                                99, 108, 105, 100,
                            ])) {
                            {
                                let source_ml_source2_concat_value57 = quotient.clone();
                                {
                                    let source_ml_source2_concat_value58 = vec![
                                        self::ml_string::Text::from_units(vec![
                                            110, 101, 103, 97, 116, 105, 118, 101, 32, 117, 110,
                                            105, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![97, 100, 100]),
                                    ];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::ml_string::Text>::new(),
                                            source_ml_source2_concat_value57,
                                        ),
                                        source_ml_source2_concat_value58,
                                    )
                                }
                            }
                        } else {
                            {
                                let source_ml_source2_concat_value59 = remainder.clone();
                                {
                                    let source_ml_source2_concat_value60 = vec![
                                        self::ml_string::Text::from_units(vec![
                                            114, 105, 103, 104, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![97, 100, 100]),
                                    ];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::ml_string::Text>::new(),
                                            source_ml_source2_concat_value59,
                                        ),
                                        source_ml_source2_concat_value60,
                                    )
                                }
                            }
                        };
                        {
                            let negative = if (*method)
                                == (self::ml_string::Text::from_units(vec![
                                    99, 104, 101, 99, 107, 101, 100, 95, 100, 105, 118, 95, 101,
                                    117, 99, 108, 105, 100,
                                ])) {
                                {
                                    let source_ml_source2_concat_value61 = quotient.clone();
                                    {
                                        let source_ml_source2_concat_value62 = vec![
                                            self::ml_string::Text::from_units(vec![
                                                110, 101, 103, 97, 116, 105, 118, 101, 32, 117,
                                                110, 105, 116,
                                            ]),
                                            self::ml_string::Text::from_units(vec![115, 117, 98]),
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<self::ml_string::Text>::new(),
                                                source_ml_source2_concat_value61,
                                            ),
                                            source_ml_source2_concat_value62,
                                        )
                                    }
                                }
                            } else {
                                {
                                    let source_ml_source2_concat_value63 = remainder.clone();
                                    {
                                        let source_ml_source2_concat_value64 = vec![
                                            self::ml_string::Text::from_units(vec![
                                                114, 105, 103, 104, 116,
                                            ]),
                                            self::ml_string::Text::from_units(vec![115, 117, 98]),
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<self::ml_string::Text>::new(),
                                                source_ml_source2_concat_value63,
                                            ),
                                            source_ml_source2_concat_value64,
                                        )
                                    }
                                }
                            };
                            {
                                let source_ml_source2_concat_value65 = remainder.clone();
                                {
                                    let source_ml_source2_concat_value66 = vec![
                                        self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                                        self::ml_string::Text::from_units(vec![108, 116]),
                                        self::ml_string::Text::from_units(vec![
                                            114, 105, 103, 104, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                                        self::ml_string::Text::from_units(vec![103, 116]),
                                    ];
                                    {
                                        let source_ml_source2_concat_value67 = positive;
                                        {
                                            let source_ml_source2_concat_value68 = negative;
                                            {
                                                let source_ml_source2_concat_value69 =
                                                    vec![self::ml_string::Text::from_units(vec![
                                                        105, 102,
                                                    ])];
                                                {
                                                    let source_ml_source2_concat_value70 =
                                                        if (*method)
                                                            == (self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 104, 101, 99, 107, 101,
                                                                    100, 95, 100, 105, 118, 95,
                                                                    101, 117, 99, 108, 105, 100,
                                                                ],
                                                            ))
                                                        {
                                                            quotient
                                                        } else {
                                                            remainder
                                                        };
                                                    {
                                                        let source_ml_source2_concat_value71 = vec![
                                                            self::ml_string::Text::from_units(
                                                                vec![105, 102],
                                                            ),
                                                        ];
                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value65), source_ml_source2_concat_value66), source_ml_source2_concat_value67), source_ml_source2_concat_value68), source_ml_source2_concat_value69), source_ml_source2_concat_value70), source_ml_source2_concat_value71)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if (*method)
                    == (self::ml_string::Text::from_units(vec![
                        99, 104, 101, 99, 107, 101, 100, 95, 100, 105, 118, 95, 101, 117, 99, 108,
                        105, 100,
                    ]))
                {
                    quotient
                } else {
                    remainder
                }
            }
        }
    } else if ((*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 108,
        ])))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 114,
            ])))
    {
        vec![
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            if (*method)
                == (self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 108,
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
        ]
    } else if (*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 110, 101, 103,
        ]))
    {
        if signed {
            vec![
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![110, 101, 103]),
            ]
        } else {
            vec![self::ml_string::Text::from_units(vec![122, 101, 114, 111])]
        }
    } else if (*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 97, 98, 115,
        ]))
    {
        if signed {
            vec![
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
                self::ml_string::Text::from_units(vec![108, 116]),
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![110, 101, 103]),
                self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                self::ml_string::Text::from_units(vec![105, 102]),
            ]
        } else {
            Vec::<self::ml_string::Text>::new()
        }
    } else {
        {
            let operation = if (*method)
                == (self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 97, 100, 100,
                ])) {
                self::ml_string::Text::from_units(vec![97, 100, 100])
            } else if (*method)
                == (self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 115, 117, 98,
                ]))
            {
                self::ml_string::Text::from_units(vec![115, 117, 98])
            } else if (*method)
                == (self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 109, 117, 108,
                ]))
            {
                self::ml_string::Text::from_units(vec![109, 117, 108])
            } else if (*method)
                == (self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 100, 105, 118,
                ]))
            {
                self::ml_string::Text::from_units(vec![100, 105, 118])
            } else if (*method)
                == (self::ml_string::Text::from_units(vec![
                    99, 104, 101, 99, 107, 101, 100, 95, 114, 101, 109,
                ]))
            {
                self::ml_string::Text::from_units(vec![114, 101, 109])
            } else {
                self::ml_string::Text::from_units(vec![])
            };
            if operation.0.is_empty() {
                Vec::<self::ml_string::Text>::new()
            } else {
                vec![
                    self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
                    self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
                    operation.clone(),
                ]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_checked_integer_consumer(
    consumer: &self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    if self::test_rust_checked_integer_predicate_consumer(consumer) {
        vec![
            self::ml_string::Text::from_units(vec![99, 111, 110, 100, 105, 116, 105, 111, 110]),
            self::ml_string::Text::from_units(vec![102, 97, 108, 108, 98, 97, 99, 107]),
            self::ml_string::Text::from_units(vec![115, 117, 99, 99, 101, 115, 115]),
            self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 111, 110, 101]),
            if (*consumer)
                == (self::ml_string::Text::from_units(vec![
                    105, 115, 95, 110, 111, 110, 101, 95, 111, 114,
                ]))
            {
                self::ml_string::Text::from_units(vec![116, 114, 117, 101])
            } else {
                self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
            },
            self::ml_string::Text::from_units(vec![105, 102]),
        ]
    } else if (*consumer)
        == (self::ml_string::Text::from_units(vec![105, 115, 95, 115, 111, 109, 101]))
    {
        vec![self::ml_string::Text::from_units(vec![
            99, 111, 110, 100, 105, 116, 105, 111, 110,
        ])]
    } else if (*consumer)
        == (self::ml_string::Text::from_units(vec![105, 115, 95, 110, 111, 110, 101]))
    {
        vec![
            self::ml_string::Text::from_units(vec![99, 111, 110, 100, 105, 116, 105, 111, 110]),
            self::ml_string::Text::from_units(vec![110, 111, 116]),
        ]
    } else if (*consumer) == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112]))
    {
        vec![
            self::ml_string::Text::from_units(vec![99, 111, 110, 100, 105, 116, 105, 111, 110]),
            self::ml_string::Text::from_units(vec![115, 117, 99, 99, 101, 115, 115]),
            self::ml_string::Text::from_units(vec![
                117, 110, 119, 114, 97, 112, 32, 109, 101, 115, 115, 97, 103, 101,
            ]),
            self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101]),
            self::ml_string::Text::from_units(vec![105, 102]),
        ]
    } else if (*consumer) == (self::ml_string::Text::from_units(vec![101, 120, 112, 101, 99, 116]))
    {
        vec![
            self::ml_string::Text::from_units(vec![99, 111, 110, 100, 105, 116, 105, 111, 110]),
            self::ml_string::Text::from_units(vec![115, 117, 99, 99, 101, 115, 115]),
            self::ml_string::Text::from_units(vec![102, 97, 108, 108, 98, 97, 99, 107]),
            self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101]),
            self::ml_string::Text::from_units(vec![105, 102]),
        ]
    } else if (*consumer)
        == (self::ml_string::Text::from_units(vec![
            117, 110, 119, 114, 97, 112, 95, 111, 114, 95, 100, 101, 102, 97, 117, 108, 116,
        ]))
    {
        vec![
            self::ml_string::Text::from_units(vec![99, 111, 110, 100, 105, 116, 105, 111, 110]),
            self::ml_string::Text::from_units(vec![115, 117, 99, 99, 101, 115, 115]),
            self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]),
            self::ml_string::Text::from_units(vec![105, 102]),
        ]
    } else if self::test_rust_checked_integer_lazy_fallback(consumer)
        || self::test_rust_checked_integer_lazy_mapper(consumer)
    {
        vec![
            self::ml_string::Text::from_units(vec![99, 111, 110, 100, 105, 116, 105, 111, 110]),
            self::ml_string::Text::from_units(vec![115, 117, 99, 99, 101, 115, 115]),
            self::ml_string::Text::from_units(vec![102, 97, 108, 108, 98, 97, 99, 107]),
            self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 122, 101, 114, 111]),
            self::ml_string::Text::from_units(vec![105, 102]),
        ]
    } else if ((*consumer)
        == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112, 95, 111, 114])))
        || ((*consumer) == (self::ml_string::Text::from_units(vec![109, 97, 112, 95, 111, 114])))
    {
        vec![
            self::ml_string::Text::from_units(vec![99, 111, 110, 100, 105, 116, 105, 111, 110]),
            self::ml_string::Text::from_units(vec![115, 117, 99, 99, 101, 115, 115]),
            self::ml_string::Text::from_units(vec![102, 97, 108, 108, 98, 97, 99, 107]),
            self::ml_string::Text::from_units(vec![105, 102]),
        ]
    } else {
        Vec::<self::ml_string::Text>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_result_owner(
    consumer: &self::ml_string::Text,
    receiver: &self::ml_string::Text,
    mapper: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (((*consumer)
        == (self::ml_string::Text::from_units(vec![105, 115, 95, 115, 111, 109, 101])))
        || ((*consumer)
            == (self::ml_string::Text::from_units(vec![105, 115, 95, 110, 111, 110, 101]))))
        || self::test_rust_checked_integer_predicate_consumer(consumer)
    {
        self::ml_string::Text::from_units(vec![98, 111, 111, 108])
    } else if ((((*consumer)
        == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112, 95, 111, 114])))
        || ((*consumer)
            == (self::ml_string::Text::from_units(vec![
                117, 110, 119, 114, 97, 112, 95, 111, 114, 95, 100, 101, 102, 97, 117, 108, 116,
            ]))))
        || self::test_rust_option_panic_consumer(consumer))
        || self::test_rust_checked_integer_lazy_fallback(consumer)
    {
        receiver.clone()
    } else if self::test_rust_checked_integer_mapping_consumer(consumer) {
        mapper.clone()
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_checked_integer_lazy_fallback(consumer: &self::ml_string::Text) -> bool {
    (*consumer)
        == (self::ml_string::Text::from_units(vec![
            117, 110, 119, 114, 97, 112, 95, 111, 114, 95, 101, 108, 115, 101,
        ]))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_checked_integer_fallback_error(
    kind: &self::ml_string::Text,
    parameters: f64,
    layers: f64,
    result: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner_kind: &self::ml_string::Text,
    owner: &self::ml_string::Text,
    children: f64,
    proven: bool,
    result_layers: f64,
    expected_bits: &self::ml_string::Text,
    expected_signed: bool,
    expected_owner: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((((((((*kind)
        == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))
        && (parameters == 0f64))
        && (layers == 0f64))
        && ((*result) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]))))
        && ((*bits) == (*expected_bits)))
        && (signed == expected_signed))
        && ((*owner) == (*expected_owner)))
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
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 32, 99, 104, 101, 99, 107, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32,
            102, 97, 108, 108, 98, 97, 99, 107, 32, 110, 101, 101, 100, 115, 32, 97, 32, 100, 105,
            114, 101, 99, 116, 32, 122, 101, 114, 111, 45, 97, 114, 103, 117, 109, 101, 110, 116,
            32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 119, 105, 116, 104, 32, 116, 104, 101, 32,
            97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 114, 101,
            115, 117, 108, 116, 32, 111, 119, 110, 101, 114,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_checked_integer_predicate_consumer(consumer: &self::ml_string::Text) -> bool {
    ((*consumer)
        == (self::ml_string::Text::from_units(vec![
            105, 115, 95, 115, 111, 109, 101, 95, 97, 110, 100,
        ])))
        || ((*consumer)
            == (self::ml_string::Text::from_units(vec![
                105, 115, 95, 110, 111, 110, 101, 95, 111, 114,
            ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_predicate_error(
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
            97, 32, 99, 104, 101, 99, 107, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32,
            112, 114, 101, 100, 105, 99, 97, 116, 101, 32, 110, 101, 101, 100, 115, 32, 97, 32,
            100, 105, 114, 101, 99, 116, 32, 111, 110, 101, 45, 97, 114, 103, 117, 109, 101, 110,
            116, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115,
            32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 97, 114,
            97, 109, 101, 116, 101, 114, 32, 97, 110, 100, 32, 66, 111, 111, 108, 101, 97, 110, 32,
            114, 101, 115, 117, 108, 116,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_checked_integer_lazy_mapper(consumer: &self::ml_string::Text) -> bool {
    (*consumer)
        == (self::ml_string::Text::from_units(vec![
            109, 97, 112, 95, 111, 114, 95, 101, 108, 115, 101,
        ]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_checked_integer_mapping_consumer(consumer: &self::ml_string::Text) -> bool {
    ((*consumer) == (self::ml_string::Text::from_units(vec![109, 97, 112, 95, 111, 114])))
        || self::test_rust_checked_integer_lazy_mapper(consumer)
}
