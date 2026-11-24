// Generated from complete js/src/translation/disjoint-collection-projection-rules.js. Do not edit by hand.
use super::ml_string;
use super::read_collection_construction_binding;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_collection_construction(
    target: &self::ml_string::Text,
    kind_2: &self::ml_string::Text,
    key_kind: &self::ml_string::Text,
    values: &self::ml_string::Text,
    collection_type: &self::ml_string::Text,
    element_type: &self::ml_string::Text,
    comparison: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let set = (*kind_2) == (self::ml_string::Text::from_units(vec![115, 101, 116]));
        if (*target)
            == (self::ml_string::Text::from_units(vec![
                74, 97, 118, 97, 83, 99, 114, 105, 112, 116,
            ]))
        {
            ((((self::ml_string::Text::from_units(vec![110, 101, 119, 32])).concat(&if set {
                self::ml_string::Text::from_units(vec![83, 101, 116])
            } else {
                self::ml_string::Text::from_units(vec![77, 97, 112])
            }))
            .concat(&self::ml_string::Text::from_units(vec![40])))
            .concat(values))
            .concat(&self::ml_string::Text::from_units(vec![41]))
        } else if (*target) == (self::ml_string::Text::from_units(vec![82, 117, 115, 116])) {
            {
                let comparator = if (*key_kind)
                    == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))
                {
                    self::ml_string::Text::from_units(vec![
                        124, 97, 58, 32, 38, 102, 54, 52, 44, 32, 98, 58, 32, 38, 102, 54, 52, 124,
                        32, 97, 32, 61, 61, 32, 98, 32, 124, 124, 32, 40, 97, 46, 105, 115, 95,
                        110, 97, 110, 40, 41, 32, 38, 38, 32, 98, 46, 105, 115, 95, 110, 97, 110,
                        40, 41, 41,
                    ])
                } else {
                    self::ml_string::Text::from_units(vec![
                        124, 97, 44, 32, 98, 124, 32, 97, 32, 61, 61, 32, 98,
                    ])
                };
                {
                    let values_3 = if (*key_kind)
                        == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))
                    {
                        {
                            ((((self::ml_string::Text::from_units(vec![40])).concat(values))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    41, 46, 105, 110, 116, 111, 95, 105, 116, 101, 114, 40, 41, 46,
                                    109, 97, 112, 40, 124, 109, 108, 95, 118, 97, 108, 117, 101,
                                    124, 32,
                                ])))
                            .concat(&if set {
                                self::ml_string::Text::from_units(vec![
                                    105, 102, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 61, 61,
                                    32, 48, 102, 54, 52, 32, 123, 32, 48, 102, 54, 52, 32, 125, 32,
                                    101, 108, 115, 101, 32, 123, 32, 109, 108, 95, 118, 97, 108,
                                    117, 101, 32, 125,
                                ])
                            } else {
                                self::ml_string::Text::from_units(vec![
                                    40, 105, 102, 32, 109, 108, 95, 118, 97, 108, 117, 101, 46, 48,
                                    32, 61, 61, 32, 48, 102, 54, 52, 32, 123, 32, 48, 102, 54, 52,
                                    32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 109, 108, 95,
                                    118, 97, 108, 117, 101, 46, 48, 32, 125, 44, 32, 109, 108, 95,
                                    118, 97, 108, 117, 101, 46, 49, 41,
                                ])
                            }))
                            .concat(
                                &self::ml_string::Text::from_units(vec![
                                    41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101,
                                    99, 60, 95, 62, 62, 40, 41,
                                ]),
                            )
                        }
                    } else {
                        values.clone()
                    };
                    ((((((self::ml_string::Text::from_units(vec![
                        99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 97, 114, 114, 97, 121, 58, 58,
                    ]))
                    .concat(kind_2))
                    .concat(&self::ml_string::Text::from_units(vec![40])))
                    .concat(&values_3))
                    .concat(&self::ml_string::Text::from_units(vec![44, 32])))
                    .concat(&comparator))
                    .concat(&self::ml_string::Text::from_units(vec![41]))
                }
            }
        } else {
            {
                let value = self::read_collection_construction_binding(
                    target,
                    kind_2,
                    &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])),
                );
                {
                    let previous = self::read_collection_construction_binding(
                        target,
                        kind_2,
                        &(self::ml_string::Text::from_units(vec![
                            112, 114, 101, 118, 105, 111, 117, 115,
                        ])),
                    );
                    if (*target) == (self::ml_string::Text::from_units(vec![76, 101, 97, 110])) {
                        {
                            let equal = if (*key_kind)
                                == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))
                            {
                                ((((((((self::ml_string::Text::from_units(vec![40]))
                                    .concat(&previous))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    32, 61, 61, 32,
                                ])))
                                .concat(&value))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    32, 124, 124, 32, 40,
                                ])))
                                .concat(&previous))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    46, 105, 115, 78, 97, 78, 32, 38, 38, 32,
                                ])))
                                .concat(&value))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    46, 105, 115, 78, 97, 78, 41, 41,
                                ]))
                            } else {
                                ((((self::ml_string::Text::from_units(vec![40]))
                                    .concat(&previous))
                                .concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32])))
                                .concat(&value))
                                .concat(&self::ml_string::Text::from_units(vec![41]))
                            };
                            {
                                let normalized = if (*key_kind)
                                    == (self::ml_string::Text::from_units(vec![
                                        102, 108, 111, 97, 116,
                                    ])) {
                                    if set {
                                        self::ml_string::Text::from_units(vec![
                                            40, 105, 102, 32, 109, 108, 95, 111, 114, 105, 103,
                                            105, 110, 97, 108, 32, 61, 61, 32, 40, 48, 32, 58, 32,
                                            70, 108, 111, 97, 116, 41, 32, 116, 104, 101, 110, 32,
                                            40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 101,
                                            108, 115, 101, 32, 109, 108, 95, 111, 114, 105, 103,
                                            105, 110, 97, 108, 41,
                                        ])
                                    } else {
                                        self::ml_string::Text::from_units(vec![
                                            40, 40, 105, 102, 32, 109, 108, 95, 111, 114, 105, 103,
                                            105, 110, 97, 108, 46, 49, 32, 61, 61, 32, 40, 48, 32,
                                            58, 32, 70, 108, 111, 97, 116, 41, 32, 116, 104, 101,
                                            110, 32, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41,
                                            32, 101, 108, 115, 101, 32, 109, 108, 95, 111, 114,
                                            105, 103, 105, 110, 97, 108, 46, 49, 41, 44, 32, 109,
                                            108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 46, 50,
                                            41,
                                        ])
                                    }
                                } else {
                                    self::ml_string::Text::from_units(vec![
                                        109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108,
                                    ])
                                };
                                {
                                    let existing = if set {
                                        self::ml_string::Text::from_units(vec![
                                            109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116,
                                            111, 114,
                                        ])
                                    } else {
                                        ((self::ml_string::Text::from_units(vec![
                                            40, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97,
                                            116, 111, 114, 46, 109, 97, 112, 32, 40, 102, 117, 110,
                                            32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117,
                                            115, 32, 61, 62, 32, 105, 102, 32,
                                        ]))
                                        .concat(&equal))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32, 116, 104, 101, 110, 32, 40, 109, 108, 95, 112, 114,
                                            101, 118, 105, 111, 117, 115, 46, 49, 44, 32, 109, 108,
                                            95, 118, 97, 108, 117, 101, 46, 50, 41, 32, 101, 108,
                                            115, 101, 32, 109, 108, 95, 112, 114, 101, 118, 105,
                                            111, 117, 115, 41, 41,
                                        ]))
                                    };
                                    ((((((((((self::ml_string::Text::from_units(vec![40]))
                                        .concat(values))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        46, 102, 111, 108, 100, 108, 32, 40, 102, 117, 110, 32,
                                        109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111,
                                        114, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97,
                                        108, 32, 61, 62, 32, 108, 101, 116, 32, 109, 108, 95, 118,
                                        97, 108, 117, 101, 32, 58, 61, 32,
                                    ])))
                                    .concat(&normalized))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        59, 32, 105, 102, 32, 109, 108, 95, 97, 99, 99, 117, 109,
                                        117, 108, 97, 116, 111, 114, 46, 97, 110, 121, 32, 40, 102,
                                        117, 110, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111,
                                        117, 115, 32, 61, 62, 32,
                                    ])))
                                    .concat(&equal))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        41, 32, 116, 104, 101, 110, 32,
                                    ])))
                                    .concat(&existing))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        32, 101, 108, 115, 101, 32, 109, 108, 95, 97, 99, 99, 117,
                                        109, 117, 108, 97, 116, 111, 114, 46, 112, 117, 115, 104,
                                        32, 109, 108, 95, 118, 97, 108, 117, 101, 41, 32, 40, 35,
                                        91, 93, 32, 58, 32,
                                    ])))
                                    .concat(collection_type))
                                    .concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                }
                            }
                        }
                    } else {
                        {
                            let equal_2 = comparison.clone();
                            {
                                let equal_4 = if (*key_kind)
                                    == (self::ml_string::Text::from_units(vec![
                                        102, 108, 111, 97, 116,
                                    ])) {
                                    {
                                        ((((((self::ml_string::Text::from_units(vec![
                                            40, 111, 114, 98, 32,
                                        ]))
                                        .concat(&equal_2))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32, 40, 97, 110, 100, 98, 32, 40, 80, 114, 105, 109,
                                            70, 108, 111, 97, 116, 46, 105, 115, 95, 110, 97, 110,
                                            32,
                                        ])))
                                        .concat(&previous))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            41, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116,
                                            46, 105, 115, 95, 110, 97, 110, 32,
                                        ])))
                                        .concat(&value))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            41, 41, 41,
                                        ]))
                                    }
                                } else {
                                    equal_2.clone()
                                };
                                {
                                    let normalized_2 = if (*key_kind)
                                        == (self::ml_string::Text::from_units(vec![
                                            102, 108, 111, 97, 116,
                                        ])) {
                                        if set {
                                            self::ml_string::Text::from_units(vec![
                                                40, 105, 102, 32, 80, 114, 105, 109, 70, 108, 111,
                                                97, 116, 46, 101, 113, 98, 32, 109, 108, 95, 111,
                                                114, 105, 103, 105, 110, 97, 108, 32, 48, 37, 102,
                                                108, 111, 97, 116, 32, 116, 104, 101, 110, 32, 48,
                                                37, 102, 108, 111, 97, 116, 32, 101, 108, 115, 101,
                                                32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97,
                                                108, 41,
                                            ])
                                        } else {
                                            self::ml_string::Text::from_units(vec![
                                                40, 40, 105, 102, 32, 80, 114, 105, 109, 70, 108,
                                                111, 97, 116, 46, 101, 113, 98, 32, 40, 102, 115,
                                                116, 32, 109, 108, 95, 111, 114, 105, 103, 105,
                                                110, 97, 108, 41, 32, 48, 37, 102, 108, 111, 97,
                                                116, 32, 116, 104, 101, 110, 32, 48, 37, 102, 108,
                                                111, 97, 116, 32, 101, 108, 115, 101, 32, 102, 115,
                                                116, 32, 109, 108, 95, 111, 114, 105, 103, 105,
                                                110, 97, 108, 41, 44, 32, 115, 110, 100, 32, 109,
                                                108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 41,
                                            ])
                                        }
                                    } else {
                                        self::ml_string::Text::from_units(vec![
                                            109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108,
                                        ])
                                    };
                                    {
                                        let existing_2 = if set {
                                            self::ml_string::Text::from_units(vec![
                                                109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97,
                                                116, 111, 114,
                                            ])
                                        } else {
                                            ((self::ml_string::Text::from_units(vec![
                                                40, 76, 105, 115, 116, 46, 109, 97, 112, 32, 40,
                                                102, 117, 110, 32, 109, 108, 95, 112, 114, 101,
                                                118, 105, 111, 117, 115, 32, 61, 62, 32, 105, 102,
                                                32,
                                            ]))
                                            .concat(&equal_4))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                32, 116, 104, 101, 110, 32, 40, 102, 115, 116, 32,
                                                109, 108, 95, 112, 114, 101, 118, 105, 111, 117,
                                                115, 44, 32, 115, 110, 100, 32, 109, 108, 95, 118,
                                                97, 108, 117, 101, 41, 32, 101, 108, 115, 101, 32,
                                                109, 108, 95, 112, 114, 101, 118, 105, 111, 117,
                                                115, 41, 32, 109, 108, 95, 97, 99, 99, 117, 109,
                                                117, 108, 97, 116, 111, 114, 41,
                                            ]))
                                        };
                                        ((((((((((self::ml_string::Text::from_units(vec![
                                            40, 76, 105, 115, 116, 46, 102, 111, 108, 100, 95, 108,
                                            101, 102, 116, 32, 40, 102, 117, 110, 32, 109, 108, 95,
                                            97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 32,
                                            109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108,
                                            32, 61, 62, 32, 108, 101, 116, 32, 109, 108, 95, 118,
                                            97, 108, 117, 101, 32, 58, 61, 32,
                                        ]))
                                        .concat(&normalized_2))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32, 105, 110, 32, 105, 102, 32, 76, 105, 115, 116, 46,
                                            101, 120, 105, 115, 116, 115, 98, 32, 40, 102, 117,
                                            110, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111,
                                            117, 115, 32, 61, 62, 32,
                                        ])))
                                        .concat(&equal_4))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            41, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108,
                                            97, 116, 111, 114, 32, 116, 104, 101, 110, 32,
                                        ])))
                                        .concat(&existing_2))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32, 101, 108, 115, 101, 32, 76, 105, 115, 116, 46, 97,
                                            112, 112, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117,
                                            108, 97, 116, 111, 114, 32, 40, 109, 108, 95, 118, 97,
                                            108, 117, 101, 32, 58, 58, 32, 110, 105, 108, 41, 41,
                                            32,
                                        ])))
                                        .concat(values))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32, 40, 64, 110, 105, 108, 32,
                                        ])))
                                        .concat(element_type))
                                        .concat(&self::ml_string::Text::from_units(vec![41, 41]))
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
pub fn read_program_container_kind(term: &self::ml_string::Text) -> self::ml_string::Text {
    if (*term)
        == (self::ml_string::Text::from_units(vec![
            112, 114, 111, 103, 114, 97, 109, 32, 114, 101, 99, 111, 114, 100,
        ]))
    {
        self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])
    } else if (*term)
        == (self::ml_string::Text::from_units(vec![
            111, 114, 100, 101, 114, 101, 100, 32, 112, 114, 111, 103, 114, 97, 109, 32, 108, 105,
            115, 116,
        ]))
    {
        self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])
    } else if (*term)
        == (self::ml_string::Text::from_units(vec![
            112, 114, 111, 103, 114, 97, 109, 32, 109, 97, 112, 112, 105, 110, 103,
        ]))
    {
        self::ml_string::Text::from_units(vec![109, 97, 112])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_program_container_fields(kind_2: &self::ml_string::Text, has_fields: bool) -> bool {
    if (*kind_2) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])) {
        true
    } else if ((*kind_2) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))
        || ((*kind_2) == (self::ml_string::Text::from_units(vec![109, 97, 112])))
    {
        !has_fields
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_program_scalar_kind(term: &self::ml_string::Text) -> self::ml_string::Text {
    if (*term)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 114, 105, 110, 103, 32, 118, 97, 108, 117, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
    } else if (*term)
        == (self::ml_string::Text::from_units(vec![
            110, 117, 109, 98, 101, 114, 32, 118, 97, 108, 117, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
    } else if (*term)
        == (self::ml_string::Text::from_units(vec![
            98, 111, 111, 108, 101, 97, 110, 32, 118, 97, 108, 117, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
    } else if (*term)
        == (self::ml_string::Text::from_units(vec![
            105, 110, 116, 101, 103, 101, 114, 32, 118, 97, 108, 117, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
    } else if (*term)
        == (self::ml_string::Text::from_units(vec![110, 117, 108, 108, 32, 118, 97, 108, 117, 101]))
    {
        self::ml_string::Text::from_units(vec![110, 117, 108, 108])
    } else if (*term)
        == (self::ml_string::Text::from_units(vec![
            117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 118, 97, 108, 117, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_program_reference_role(term: &self::ml_string::Text) -> self::ml_string::Text {
    if (*term)
        == (self::ml_string::Text::from_units(vec![
            98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103])
    } else if (*term)
        == (self::ml_string::Text::from_units(vec![
            100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101,
            110, 99, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110])
    } else if (*term)
        == (self::ml_string::Text::from_units(vec![
            100, 97, 116, 97, 32, 116, 121, 112, 101, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105,
            111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![100, 97, 116, 97, 116, 121, 112, 101])
    } else if (*term)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 100, 101, 99, 108, 97, 114,
            97, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])
    } else if (*term)
        == (self::ml_string::Text::from_units(vec![
            100, 97, 116, 97, 116, 121, 112, 101, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105,
            111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![100, 97, 116, 97, 116, 121, 112, 101])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_program_pattern_binding_form(kind_2: &self::ml_string::Text) -> self::ml_string::Text {
    if (*kind_2) == (self::ml_string::Text::from_units(vec![119, 105, 108, 100])) {
        self::ml_string::Text::from_units(vec![110, 111, 110, 101])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![98, 105, 110, 100])) {
        self::ml_string::Text::from_units(vec![115, 105, 110, 103, 108, 101])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![110, 97, 116, 90, 101, 114, 111]))
    {
        self::ml_string::Text::from_units(vec![110, 111, 110, 101])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![110, 97, 116, 83, 117, 99, 99]))
    {
        self::ml_string::Text::from_units(vec![115, 105, 110, 103, 108, 101])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![99, 116, 111, 114])) {
        self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_program_expression_operand_fields(
    kind_2: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((((*kind_2) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108])))
        || ((*kind_2) == (self::ml_string::Text::from_units(vec![99, 116, 111, 114]))))
        || ((*kind_2) == (self::ml_string::Text::from_units(vec![109, 97, 116, 104]))))
        || ((*kind_2) == (self::ml_string::Text::from_units(vec![114, 101, 99, 117, 114])))
    {
        self::ml_string::Text::from_units(vec![97, 114, 103, 115])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])) {
        self::ml_string::Text::from_units(vec![102, 110, 32, 97, 114, 103, 115])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])) {
        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 97, 114, 103, 115])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![
            99, 101, 108, 108, 65, 108, 108, 111, 99, 97, 116, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![99, 101, 108, 108, 82, 101, 97, 100]))
    {
        self::ml_string::Text::from_units(vec![99, 101, 108, 108])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![99, 101, 108, 108, 87, 114, 105, 116, 101]))
    {
        self::ml_string::Text::from_units(vec![
            99, 101, 108, 108, 32, 118, 97, 108, 117, 101, 32, 98, 111, 100, 121,
        ])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![
            114, 101, 102, 101, 114, 101, 110, 99, 101, 65, 108, 108, 111, 99, 97, 116, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![
            114, 101, 102, 101, 114, 101, 110, 99, 101, 82, 101, 97, 100,
        ]))
    {
        self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101])
    } else if ((*kind_2)
        == (self::ml_string::Text::from_units(vec![
            114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100,
        ])))
        || ((*kind_2)
            == (self::ml_string::Text::from_units(vec![
                116, 117, 112, 108, 101, 73, 110, 100, 101, 120,
            ])))
    {
        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
    } else if ((*kind_2) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))
        || ((*kind_2) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))
    {
        self::ml_string::Text::from_units(vec![105, 116, 101, 109, 115])
    } else if ((*kind_2) == (self::ml_string::Text::from_units(vec![115, 111, 109, 101])))
        || ((*kind_2)
            == (self::ml_string::Text::from_units(vec![
                102, 108, 97, 116, 116, 101, 110, 79, 112, 116, 105, 111, 110,
            ])))
    {
        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])
    } else if (((*kind_2)
        == (self::ml_string::Text::from_units(vec![
            110, 117, 108, 108, 105, 115, 104, 78, 117, 109, 98, 101, 114,
        ])))
        || ((*kind_2)
            == (self::ml_string::Text::from_units(vec![
                111, 112, 116, 105, 111, 110, 97, 108, 73, 110, 100, 101, 120, 78, 117, 109, 98,
                101, 114,
            ]))))
        || ((*kind_2) == (self::ml_string::Text::from_units(vec![116, 114, 117, 116, 104, 121])))
    {
        self::ml_string::Text::from_units(vec![97, 114, 103])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108,
        ]))
    {
        self::ml_string::Text::from_units(vec![108, 101, 102, 116, 32, 114, 105, 103, 104, 116])
    } else if ((((*kind_2)
        == (self::ml_string::Text::from_units(vec![
            111, 112, 116, 105, 111, 110, 97, 108, 84, 121, 112, 101, 79, 102,
        ])))
        || ((*kind_2)
            == (self::ml_string::Text::from_units(vec![
                111, 112, 116, 105, 111, 110, 97, 108, 84, 101, 120, 116,
            ]))))
        || ((*kind_2)
            == (self::ml_string::Text::from_units(vec![
                117, 110, 119, 114, 97, 112, 79, 112, 116, 105, 111, 110,
            ]))))
        || ((*kind_2)
            == (self::ml_string::Text::from_units(vec![
                111, 112, 116, 105, 111, 110, 97, 108, 80, 114, 101, 100, 105, 99, 97, 116, 101,
            ])))
    {
        self::ml_string::Text::from_units(vec![97, 114, 103])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![99, 111, 97, 108, 101, 115, 99, 101]))
    {
        self::ml_string::Text::from_units(vec![108, 101, 102, 116, 32, 114, 105, 103, 104, 116])
    } else if ((*kind_2)
        == (self::ml_string::Text::from_units(vec![
            111, 112, 116, 105, 111, 110, 97, 108, 70, 105, 101, 108, 100,
        ])))
        || ((*kind_2)
            == (self::ml_string::Text::from_units(vec![
                111, 112, 116, 105, 111, 110, 97, 108, 80, 114, 111, 106, 101, 99, 116,
            ])))
    {
        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![
            111, 112, 116, 105, 111, 110, 97, 108, 73, 110, 100, 101, 120,
        ]))
    {
        self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 105, 110, 100, 101, 120])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 114, 105, 110, 103, 80, 111, 105, 110, 116, 115,
        ]))
    {
        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
        self::ml_string::Text::from_units(vec![105, 116, 101, 109, 115])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
        self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 83, 105, 122, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 82, 101, 97, 100,
        ]))
    {
        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 107, 101, 121])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 109, 115,
        ]))
    {
        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![117, 110, 97, 114, 121])) {
        self::ml_string::Text::from_units(vec![97, 114, 103])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121])) {
        self::ml_string::Text::from_units(vec![108, 101, 102, 116, 32, 114, 105, 103, 104, 116])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![105, 102])) {
        self::ml_string::Text::from_units(vec![
            99, 111, 110, 100, 32, 116, 104, 101, 110, 32, 101, 108, 115, 101,
        ])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![112, 114, 105, 110, 116])) {
        self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 98, 111, 100, 121])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103]))
    {
        self::ml_string::Text::from_units(vec![97, 114, 103])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 77, 97, 112]))
    {
        self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
    } else if (*kind_2)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 114, 105, 110, 103, 84, 101, 115, 116,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            115, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104,
        ])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![99, 97, 115, 116])) {
        self::ml_string::Text::from_units(vec![97, 114, 103])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![97, 112, 112, 101, 110, 100])) {
        self::ml_string::Text::from_units(vec![108, 101, 102, 116, 32, 114, 105, 103, 104, 116])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])) {
        self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 105, 110, 100, 101, 120])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])) {
        self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])) {
        self::ml_string::Text::from_units(vec![
            101, 110, 116, 114, 105, 101, 115, 32, 111, 114, 100, 101, 114,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_program_reference_source_field(
    label: &self::ml_string::Text,
    construct_kind: &self::ml_string::Text,
    type_kind: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (((*label)
        == (self::ml_string::Text::from_units(vec![
            98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ])))
        && ((*construct_kind) == (self::ml_string::Text::from_units(vec![118, 97, 114]))))
        && ((*type_kind) == (self::ml_string::Text::from_units(vec![])))
    {
        self::ml_string::Text::from_units(vec![110, 97, 109, 101])
    } else if ((((*label)
        == (self::ml_string::Text::from_units(vec![
            100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101,
            110, 99, 101,
        ])))
        && ((*construct_kind) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))))
        && ((*type_kind) == (self::ml_string::Text::from_units(vec![]))))
        || ((((*label)
            == (self::ml_string::Text::from_units(vec![
                100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114,
                101, 110, 99, 101,
            ])))
            && ((*construct_kind)
                == (self::ml_string::Text::from_units(vec![
                    102, 117, 110, 99, 116, 105, 111, 110, 82, 101, 102,
                ]))))
            && ((*type_kind) == (self::ml_string::Text::from_units(vec![]))))
    {
        self::ml_string::Text::from_units(vec![102, 110])
    } else if (((*label)
        == (self::ml_string::Text::from_units(vec![
            100, 97, 116, 97, 32, 116, 121, 112, 101, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105,
            111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ])))
        && ((*construct_kind) == (self::ml_string::Text::from_units(vec![]))))
        && ((*type_kind) == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])))
    {
        self::ml_string::Text::from_units(vec![110, 97, 109, 101])
    } else if (((*label)
        == (self::ml_string::Text::from_units(vec![
            100, 97, 116, 97, 32, 116, 121, 112, 101, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105,
            111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ])))
        && ((*construct_kind) == (self::ml_string::Text::from_units(vec![99, 116, 111, 114]))))
        && ((*type_kind) == (self::ml_string::Text::from_units(vec![])))
    {
        self::ml_string::Text::from_units(vec![100, 97, 116, 97])
    } else if (((*label)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 100, 101, 99, 108, 97, 114,
            97, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ])))
        && ((*construct_kind) == (self::ml_string::Text::from_units(vec![99, 116, 111, 114]))))
        && ((*type_kind) == (self::ml_string::Text::from_units(vec![])))
    {
        self::ml_string::Text::from_units(vec![99, 116, 111, 114])
    } else if (((*label)
        == (self::ml_string::Text::from_units(vec![
            100, 97, 116, 97, 116, 121, 112, 101, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105,
            111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ])))
        && ((*construct_kind) == (self::ml_string::Text::from_units(vec![]))))
        && ((*type_kind) == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])))
    {
        self::ml_string::Text::from_units(vec![110, 97, 109, 101])
    } else if (((*label)
        == (self::ml_string::Text::from_units(vec![
            100, 97, 116, 97, 116, 121, 112, 101, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105,
            111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ])))
        && ((*construct_kind) == (self::ml_string::Text::from_units(vec![99, 116, 111, 114]))))
        && ((*type_kind) == (self::ml_string::Text::from_units(vec![])))
    {
        self::ml_string::Text::from_units(vec![100, 97, 116, 97])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_program_reference_target_field(label: &self::ml_string::Text) -> self::ml_string::Text {
    if (*label)
        == (self::ml_string::Text::from_units(vec![
            98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![110, 97, 109, 101])
    } else if ((*label)
        == (self::ml_string::Text::from_units(vec![
            100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101,
            110, 99, 101,
        ])))
        || ((*label)
            == (self::ml_string::Text::from_units(vec![
                100, 97, 116, 97, 32, 116, 121, 112, 101, 32, 100, 101, 99, 108, 97, 114, 97, 116,
                105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
            ])))
    {
        self::ml_string::Text::from_units(vec![102, 117, 108, 108, 78, 97, 109, 101])
    } else if (*label)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 100, 101, 99, 108, 97, 114,
            97, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![110, 97, 109, 101])
    } else if (*label)
        == (self::ml_string::Text::from_units(vec![
            100, 97, 116, 97, 116, 121, 112, 101, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105,
            111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![102, 117, 108, 108, 78, 97, 109, 101])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}
