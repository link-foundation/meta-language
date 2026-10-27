// Generated from complete js/src/translation/rust-primitive-from-rules.js. Do not edit by hand.
use super::accept_rust_integer_bit_method;
use super::accept_rust_primitive_selection_receiver;
use super::accept_rust_wrapping_shift_method;
use super::ml_array;
use super::ml_string;
use super::read_rust_checked_integer_arity;
use super::read_rust_integer_magnitude_arity;

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
                                    let source_ml_source1_concat_value31 = vec![
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
                                        let source_ml_source1_concat_value32 = positive_positive;
                                        {
                                            let source_ml_source1_concat_value33 =
                                                positive_negative;
                                            {
                                                let source_ml_source1_concat_value34 = vec![
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
                                                    let source_ml_source1_concat_value35 =
                                                        negative_positive;
                                                    {
                                                        let source_ml_source1_concat_value36 =
                                                            negative_negative;
                                                        {
                                                            let source_ml_source1_concat_value37 = vec![
                                                                self::ml_string::Text::from_units(
                                                                    vec![105, 102],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![105, 102],
                                                                ),
                                                            ];
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value31), source_ml_source1_concat_value32), source_ml_source1_concat_value33), source_ml_source1_concat_value34), source_ml_source1_concat_value35), source_ml_source1_concat_value36), source_ml_source1_concat_value37)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                                {
                                    let source_ml_source1_concat_value38 = vec![
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
                                        let source_ml_source1_concat_value39 = signs;
                                        {
                                            let source_ml_source1_concat_value40 = vec![
                                                self::ml_string::Text::from_units(vec![105, 102]),
                                                self::ml_string::Text::from_units(vec![105, 102]),
                                            ];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<self::ml_string::Text>::new(),
                                                        source_ml_source1_concat_value38,
                                                    ),
                                                    source_ml_source1_concat_value39,
                                                ),
                                                source_ml_source1_concat_value40,
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
                    let source_ml_source1_concat_value28 = vec![
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
                        let source_ml_source1_concat_value29 = positive_positive;
                        {
                            let source_ml_source1_concat_value30 = vec![
                                self::ml_string::Text::from_units(vec![105, 102]),
                                self::ml_string::Text::from_units(vec![105, 102]),
                            ];
                            self::ml_array::append(
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source1_concat_value28,
                                    ),
                                    source_ml_source1_concat_value29,
                                ),
                                source_ml_source1_concat_value30,
                            )
                        }
                    }
                }
            }
        }
    } else if ((*method)
        == (self::ml_string::Text::from_units(vec![
            99, 104, 101, 99, 107, 101, 100, 95, 100, 105, 118,
        ])))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                99, 104, 101, 99, 107, 101, 100, 95, 114, 101, 109,
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
    if (*consumer) == (self::ml_string::Text::from_units(vec![105, 115, 95, 115, 111, 109, 101])) {
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
    if ((*consumer) == (self::ml_string::Text::from_units(vec![105, 115, 95, 115, 111, 109, 101])))
        || ((*consumer)
            == (self::ml_string::Text::from_units(vec![105, 115, 95, 110, 111, 110, 101])))
    {
        self::ml_string::Text::from_units(vec![98, 111, 111, 108])
    } else if ((*consumer)
        == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112, 95, 111, 114])))
        || ((*consumer)
            == (self::ml_string::Text::from_units(vec![
                117, 110, 119, 114, 97, 112, 95, 111, 114, 95, 100, 101, 102, 97, 117, 108, 116,
            ])))
    {
        receiver.clone()
    } else if (*consumer) == (self::ml_string::Text::from_units(vec![109, 97, 112, 95, 111, 114])) {
        mapper.clone()
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_associated_owner(
    segments: &[self::ml_string::Text],
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> self::ml_string::Text {
    if !ordinary || shadowed {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let owner = self::ml_string::Text::from_units(vec![]);
            {
                let owner_4 = if ((segments.len() as f64) == 2f64) && !absolute {
                    {
                        (self::ml_array::optional_at(segments, self::ml_array::number_index(0f64)))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))
                    }
                } else {
                    if (((segments.len() as f64) == 4f64)
                        && ({
                            let source_ml_source1_comparison_operand41 =
                                self::ml_array::optional_at(
                                    segments,
                                    self::ml_array::number_index(0f64),
                                );
                            {
                                let source_ml_source1_comparison_operand42 =
                                    self::ml_string::Text::from_units(vec![115, 116, 100]);
                                match (
                                    source_ml_source1_comparison_operand41,
                                    source_ml_source1_comparison_operand42,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        } || {
                            let source_ml_source1_comparison_operand43 =
                                self::ml_array::optional_at(
                                    segments,
                                    self::ml_array::number_index(0f64),
                                );
                            {
                                let source_ml_source1_comparison_operand44 =
                                    self::ml_string::Text::from_units(vec![99, 111, 114, 101]);
                                match (
                                    source_ml_source1_comparison_operand43,
                                    source_ml_source1_comparison_operand44,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        }))
                        && {
                            let source_ml_source1_comparison_operand45 =
                                self::ml_array::optional_at(
                                    segments,
                                    self::ml_array::number_index(1f64),
                                );
                            {
                                let source_ml_source1_comparison_operand46 =
                                    self::ml_string::Text::from_units(vec![
                                        112, 114, 105, 109, 105, 116, 105, 118, 101,
                                    ]);
                                match (
                                    source_ml_source1_comparison_operand45,
                                    source_ml_source1_comparison_operand46,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        }
                    {
                        {
                            (self::ml_array::optional_at(
                                segments,
                                self::ml_array::number_index(2f64),
                            ))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))
                        }
                    } else {
                        owner
                    }
                };
                if self::ml_array::includes(
                    &vec![
                        self::ml_string::Text::from_units(vec![98, 111, 111, 108]),
                        self::ml_string::Text::from_units(vec![105, 56]),
                        self::ml_string::Text::from_units(vec![117, 56]),
                        self::ml_string::Text::from_units(vec![105, 49, 54]),
                        self::ml_string::Text::from_units(vec![117, 49, 54]),
                        self::ml_string::Text::from_units(vec![105, 51, 50]),
                        self::ml_string::Text::from_units(vec![117, 51, 50]),
                        self::ml_string::Text::from_units(vec![105, 54, 52]),
                        self::ml_string::Text::from_units(vec![117, 54, 52]),
                        self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                        self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
                        self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101]),
                        self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]),
                    ],
                    &owner_4.clone(),
                    0f64,
                    false,
                    |a, b| a == b,
                ) {
                    owner_4.clone()
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_associated_arity(
    owner: &self::ml_string::Text,
    method: &self::ml_string::Text,
) -> f64 {
    if (self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![98, 111, 111, 108]),
            self::ml_string::Text::from_units(vec![105, 56]),
            self::ml_string::Text::from_units(vec![117, 56]),
            self::ml_string::Text::from_units(vec![105, 49, 54]),
            self::ml_string::Text::from_units(vec![117, 49, 54]),
            self::ml_string::Text::from_units(vec![105, 51, 50]),
            self::ml_string::Text::from_units(vec![117, 51, 50]),
            self::ml_string::Text::from_units(vec![105, 54, 52]),
            self::ml_string::Text::from_units(vec![117, 54, 52]),
            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
            self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
            self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101]),
            self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]),
        ],
        owner,
        0f64,
        false,
        |a, b| a == b,
    )) {
        if ((*method) == (self::ml_string::Text::from_units(vec![109, 105, 110])))
            || ((*method) == (self::ml_string::Text::from_units(vec![109, 97, 120])))
        {
            2f64
        } else if (*owner) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
            -1f64
        } else {
            {
                let magnitude = self::read_rust_integer_magnitude_arity(method, true);
                if (magnitude >= 0f64)
                    && (((*method)
                        != (self::ml_string::Text::from_units(vec![
                            117, 110, 115, 105, 103, 110, 101, 100, 95, 97, 98, 115,
                        ])))
                        || owner.starts_with(&self::ml_string::Text::from_units(vec![105])))
                {
                    magnitude + 1f64
                } else if self::accept_rust_wrapping_shift_method(method) {
                    2f64
                } else if self::accept_rust_integer_bit_method(method) {
                    1f64
                } else {
                    {
                        let checked = self::read_rust_checked_integer_arity(method, true);
                        if (checked >= 0f64)
                            && (((*method)
                                != (self::ml_string::Text::from_units(vec![
                                    99, 104, 101, 99, 107, 101, 100, 95, 97, 98, 115,
                                ])))
                                || owner.starts_with(&self::ml_string::Text::from_units(vec![105])))
                        {
                            checked + 1f64
                        } else if self::ml_array::includes(
                            &vec![
                                self::ml_string::Text::from_units(vec![
                                    115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 97, 100,
                                    100,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 115, 117,
                                    98,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 109, 117,
                                    108,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    119, 114, 97, 112, 112, 105, 110, 103, 95, 97, 100, 100,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    119, 114, 97, 112, 112, 105, 110, 103, 95, 115, 117, 98,
                                ]),
                            ],
                            method,
                            0f64,
                            false,
                            |a, b| a == b,
                        ) {
                            2f64
                        } else if (*method)
                            == (self::ml_string::Text::from_units(vec![
                                119, 114, 97, 112, 112, 105, 110, 103, 95, 110, 101, 103,
                            ]))
                        {
                            1f64
                        } else if (owner.starts_with(&self::ml_string::Text::from_units(vec![105])))
                        {
                            if self::ml_array::includes(
                                &vec![
                                    self::ml_string::Text::from_units(vec![
                                        105, 115, 95, 110, 101, 103, 97, 116, 105, 118, 101,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        105, 115, 95, 112, 111, 115, 105, 116, 105, 118, 101,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        115, 105, 103, 110, 117, 109,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 110,
                                        101, 103,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 97, 98,
                                        115,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        119, 114, 97, 112, 112, 105, 110, 103, 95, 97, 98, 115,
                                    ]),
                                ],
                                method,
                                0f64,
                                false,
                                |a, b| a == b,
                            ) {
                                1f64
                            } else {
                                -1f64
                            }
                        } else {
                            -1f64
                        }
                    }
                }
            }
        }
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_primitive_associated_literal(
    expected: &self::ml_string::Text,
    form: &self::ml_string::Text,
    typed: bool,
) -> bool {
    ((((*expected) != (self::ml_string::Text::from_units(vec![])))
        && ((*expected) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))))
        && !typed)
        && (((*form)
            == (self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 97, 108])))
            || ((*form)
                == (self::ml_string::Text::from_units(vec![
                    110, 101, 103, 97, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 97, 108,
                ]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_associated_receiver(
    expected: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner: &self::ml_string::Text,
    path: &self::ml_string::Text,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((*expected) == (*path))
        && self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_primitive_from_method(
    owner: &self::ml_string::Text,
    method: &self::ml_string::Text,
) -> bool {
    ((*method) == (self::ml_string::Text::from_units(vec![102, 114, 111, 109])))
        && self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![98, 111, 111, 108]),
                self::ml_string::Text::from_units(vec![105, 56]),
                self::ml_string::Text::from_units(vec![117, 56]),
                self::ml_string::Text::from_units(vec![105, 49, 54]),
                self::ml_string::Text::from_units(vec![117, 49, 54]),
                self::ml_string::Text::from_units(vec![105, 51, 50]),
                self::ml_string::Text::from_units(vec![117, 51, 50]),
                self::ml_string::Text::from_units(vec![105, 54, 52]),
                self::ml_string::Text::from_units(vec![117, 54, 52]),
                self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
                self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101]),
                self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]),
                self::ml_string::Text::from_units(vec![102, 54, 52]),
            ],
            owner,
            0f64,
            false,
            |a, b| a == b,
        )
}
