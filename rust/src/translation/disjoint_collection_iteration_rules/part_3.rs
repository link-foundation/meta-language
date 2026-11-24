// Generated from complete js/src/translation/disjoint-collection-iteration-rules.js. Do not edit by hand.
use super::ml_string;
use super::read_reference_method_result;
use super::source_ml_source1_temporary_render_array_callback_call_loop8;
use super::source_ml_source1_temporary_render_reference_method_expression_loop6;
use super::source_ml_source1_temporary_render_reference_method_expression_loop7;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_reference_method_expression(
    target: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    operation: &self::ml_string::Text,
    receiver: &self::ml_string::Text,
    arguments_: &[self::ml_string::Text],
    float_key: bool,
    suffix: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (self::read_reference_method_result(kind, operation))
        == (self::ml_string::Text::from_units(vec![]))
    {
        self::ml_string::Text::from_units(vec![])
    } else if (*target)
        == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))
    {
        ((((((self::ml_string::Text::from_units(vec![40])).concat(receiver))
            .concat(&self::ml_string::Text::from_units(vec![41, 46])))
        .concat(operation))
        .concat(&self::ml_string::Text::from_units(vec![40])))
        .concat(&self::ml_string::join(
            arguments_,
            &self::ml_string::Text::from_units(vec![44, 32]),
        )))
        .concat(&self::ml_string::Text::from_units(vec![41]))
    } else {
        let ml_library_inequality_1 =
            (*target) != (self::ml_string::Text::from_units(vec![82, 117, 115, 116]));
        if ml_library_inequality_1 {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let owner = (self::ml_string::Text::from_units(vec![
                    109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 111, 119, 110, 101, 114,
                ]))
                .concat(suffix);
                {
                    let values = (self::ml_string::Text::from_units(vec![
                        109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 118, 97, 108, 117, 101, 115,
                    ]))
                    .concat(suffix);
                    {
                        let prefix = ((((self::ml_string::Text::from_units(vec![
                            123, 32, 108, 101, 116, 32,
                        ]))
                        .concat(&owner))
                        .concat(&self::ml_string::Text::from_units(vec![32, 61, 32])))
                        .concat(receiver))
                        .concat(&self::ml_string::Text::from_units(vec![59, 32]));
                        {
                            let index = 0f64;
                            {
                                let prefix_2 = self::source_ml_source1_temporary_render_reference_method_expression_loop6(arguments_, suffix, prefix, index);
                                {
                                    let first = ((self::ml_string::Text::from_units(vec![
                                        109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 97, 114,
                                        103, 117, 109, 101, 110, 116,
                                    ]))
                                    .concat(suffix))
                                    .concat(&self::ml_string::Text::from_units(vec![95, 48]));
                                    {
                                        let second = ((self::ml_string::Text::from_units(vec![
                                            109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 97,
                                            114, 103, 117, 109, 101, 110, 116,
                                        ]))
                                        .concat(suffix))
                                        .concat(&self::ml_string::Text::from_units(vec![95, 49]));
                                        if (*kind)
                                            == (self::ml_string::Text::from_units(vec![
                                                97, 114, 114, 97, 121,
                                            ]))
                                        {
                                            if ((*operation)
                                                == (self::ml_string::Text::from_units(vec![
                                                    112, 117, 115, 104,
                                                ])))
                                                || ((*operation)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        117, 110, 115, 104, 105, 102, 116,
                                                    ])))
                                            {
                                                {
                                                    let names = Vec::<self::ml_string::Text>::new();
                                                    {
                                                        let index_2 = 0f64;
                                                        {
                                                            let names_2 = self::source_ml_source1_temporary_render_reference_method_expression_loop7(arguments_, suffix, names, index_2);
                                                            {
                                                                let changes = if (*operation) == (self::ml_string::Text::from_units(vec![112, 117, 115, 104])) {
                                                                        values.concat(&self::ml_string::Text::from_units(vec![46, 101, 120, 116, 101, 110, 100, 40, 109, 108, 95, 105, 110, 115, 101, 114, 116, 101, 100, 41, 59]))
                                                                    } else {
                                                                        values.concat(&self::ml_string::Text::from_units(vec![46, 115, 112, 108, 105, 99, 101, 40, 48, 46, 46, 48, 44, 32, 109, 108, 95, 105, 110, 115, 101, 114, 116, 101, 100, 41, 59]))
                                                                    };
                                                                (((((((((((prefix_2.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 105, 110, 115, 101, 114, 116, 101, 100, 32, 61, 32, 118, 101, 99, 33, 91]))).concat(&self::ml_string::join(&names_2, &self::ml_string::Text::from_units(vec![44, 32])))).concat(&self::ml_string::Text::from_units(vec![93, 59, 32, 108, 101, 116, 32, 109, 117, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 108, 101, 110, 103, 116, 104, 32, 61, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![46, 108, 101, 110, 40, 41, 46, 99, 104, 101, 99, 107, 101, 100, 95, 97, 100, 100, 40, 109, 108, 95, 105, 110, 115, 101, 114, 116, 101, 100, 46, 108, 101, 110, 40, 41, 41, 46, 102, 105, 108, 116, 101, 114, 40, 124, 108, 101, 110, 103, 116, 104, 124, 32, 42, 108, 101, 110, 103, 116, 104, 32, 60, 61, 32, 52, 50, 57, 52, 57, 54, 55, 50, 57, 53, 117, 115, 105, 122, 101, 41, 46, 101, 120, 112, 101, 99, 116, 40, 34, 73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 34, 41, 59, 32]))).concat(&changes)).concat(&self::ml_string::Text::from_units(vec![32, 109, 108, 95, 108, 101, 110, 103, 116, 104, 32, 97, 115, 32, 102, 54, 52, 32, 125]))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else if (*operation)
                                                == (self::ml_string::Text::from_units(vec![
                                                    112, 111, 112,
                                                ]))
                                            {
                                                ((prefix_2.concat(
                                                    &self::ml_string::Text::from_units(vec![
                                                        108, 101, 116, 32, 109, 108, 95, 114, 101,
                                                        115, 117, 108, 116, 32, 61, 32,
                                                    ]),
                                                ))
                                                .concat(&owner))
                                                .concat(&self::ml_string::Text::from_units(vec![
                                                    46, 98, 111, 114, 114, 111, 119, 95, 109, 117,
                                                    116, 40, 41, 46, 112, 111, 112, 40, 41, 59, 32,
                                                    109, 108, 95, 114, 101, 115, 117, 108, 116, 32,
                                                    125,
                                                ]))
                                            } else if (*operation)
                                                == (self::ml_string::Text::from_units(vec![
                                                    115, 104, 105, 102, 116,
                                                ]))
                                            {
                                                ((((((((prefix_2.concat(
                                                    &self::ml_string::Text::from_units(vec![
                                                        108, 101, 116, 32, 109, 117, 116, 32,
                                                    ]),
                                                ))
                                                .concat(&values))
                                                .concat(&self::ml_string::Text::from_units(
                                                    vec![32, 61, 32],
                                                )))
                                                .concat(&owner))
                                                .concat(&self::ml_string::Text::from_units(
                                                    vec![
                                                        46, 98, 111, 114, 114, 111, 119, 95, 109,
                                                        117, 116, 40, 41, 59, 32, 105, 102, 32,
                                                    ],
                                                )))
                                                .concat(&values))
                                                .concat(&self::ml_string::Text::from_units(vec![
                                                    46, 105, 115, 95, 101, 109, 112, 116, 121, 40,
                                                    41, 32, 123, 32, 78, 111, 110, 101, 32, 125,
                                                    32, 101, 108, 115, 101, 32, 123, 32, 83, 111,
                                                    109, 101, 40,
                                                ])))
                                                .concat(&values))
                                                .concat(&self::ml_string::Text::from_units(vec![
                                                    46, 114, 101, 109, 111, 118, 101, 40, 48, 41,
                                                    41, 32, 125, 32, 125,
                                                ]))
                                            } else {
                                                (((prefix_2.concat(&owner)).concat(
                                                    &self::ml_string::Text::from_units(vec![
                                                        46, 98, 111, 114, 114, 111, 119, 95, 109,
                                                        117, 116, 40, 41, 46, 114, 101, 118, 101,
                                                        114, 115, 101, 40, 41, 59, 32,
                                                    ]),
                                                ))
                                                .concat(&owner))
                                                .concat(&self::ml_string::Text::from_units(vec![
                                                    32, 125,
                                                ]))
                                            }
                                        } else if (*operation)
                                            == (self::ml_string::Text::from_units(vec![
                                                99, 108, 101, 97, 114,
                                            ]))
                                        {
                                            (prefix_2.concat(&owner)).concat(
                                                &self::ml_string::Text::from_units(vec![
                                                    46, 98, 111, 114, 114, 111, 119, 95, 109, 117,
                                                    116, 40, 41, 46, 99, 108, 101, 97, 114, 40, 41,
                                                    59, 32, 40, 41, 32, 125,
                                                ]),
                                            )
                                        } else {
                                            {
                                                let key =
                                                    (self::ml_string::Text::from_units(vec![
                                                        109, 108, 95, 109, 101, 116, 104, 111, 100,
                                                        95, 107, 101, 121,
                                                    ]))
                                                    .concat(suffix);
                                                {
                                                    let prefix_3 = prefix_2.concat(&((((self::ml_string::Text::from_units(vec![108, 101, 116, 32])).concat(&key)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&if float_key {
                                                                ((((self::ml_string::Text::from_units(vec![105, 102, 32])).concat(&first)).concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32, 48, 46, 48, 32, 123, 32, 48, 46, 48, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32]))).concat(&first)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                                                            } else {
                                                                first.clone()
                                                            })).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                                                    {
                                                        let pattern = if (*kind)
                                                            == (self::ml_string::Text::from_units(
                                                                vec![109, 97, 112],
                                                            )) {
                                                            self::ml_string::Text::from_units(vec![
                                                                40, 109, 108, 95, 107, 101, 121,
                                                                44, 32, 95, 41,
                                                            ])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![
                                                                109, 108, 95, 107, 101, 121,
                                                            ])
                                                        };
                                                        {
                                                            let equality = if float_key {
                                                                ((((self::ml_string::Text::from_units(vec![40, 109, 108, 95, 107, 101, 121, 32, 61, 61, 32, 38])).concat(&key)).concat(&self::ml_string::Text::from_units(vec![32, 124, 124, 32, 40, 109, 108, 95, 107, 101, 121, 46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 38, 38, 32]))).concat(&key)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 110, 97, 110, 40, 41, 41, 41]))
                                                            } else {
                                                                (self::ml_string::Text::from_units(
                                                                    vec![
                                                                        109, 108, 95, 107, 101,
                                                                        121, 32, 61, 61, 32, 38,
                                                                    ],
                                                                ))
                                                                .concat(&key)
                                                            };
                                                            {
                                                                let position = ((((values.concat(&self::ml_string::Text::from_units(vec![46, 105, 116, 101, 114, 40, 41, 46, 112, 111, 115, 105, 116, 105, 111, 110, 40, 124]))).concat(&pattern)).concat(&self::ml_string::Text::from_units(vec![124, 32]))).concat(&equality)).concat(&self::ml_string::Text::from_units(vec![41]));
                                                                if (*operation) == (self::ml_string::Text::from_units(vec![104, 97, 115])) {
                                                                            ((((((prefix_3.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 40, 41, 59, 32]))).concat(&position)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 115, 111, 109, 101, 40, 41, 32, 125]))
                                                                        } else if (*operation) == (self::ml_string::Text::from_units(vec![103, 101, 116])) {
                                                                                ((((((((prefix_3.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 40, 41, 59, 32]))).concat(&position)).concat(&self::ml_string::Text::from_units(vec![46, 109, 97, 112, 40, 124, 105, 110, 100, 101, 120, 124, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![91, 105, 110, 100, 101, 120, 93, 46, 49, 46, 99, 108, 111, 110, 101, 40, 41, 41, 32, 125]))
                                                                            } else if (*operation) == (self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101])) {
                                                                                    ((((((((prefix_3.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 117, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32, 109, 97, 116, 99, 104, 32]))).concat(&position)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 83, 111, 109, 101, 40, 105, 110, 100, 101, 120, 41, 32, 61, 62, 32, 123, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![46, 114, 101, 109, 111, 118, 101, 40, 105, 110, 100, 101, 120, 41, 59, 32, 116, 114, 117, 101, 32, 125, 44, 32, 78, 111, 110, 101, 32, 61, 62, 32, 102, 97, 108, 115, 101, 32, 125, 32, 125]))
                                                                                } else {
                                                                                    {
                                                                                        let insertion = if (*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                                                                                            ((((values.concat(&self::ml_string::Text::from_units(vec![46, 112, 117, 115, 104, 40, 40]))).concat(&key)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&second)).concat(&self::ml_string::Text::from_units(vec![41, 41, 59]))
                                                                                        } else {
                                                                                            ((values.concat(&self::ml_string::Text::from_units(vec![46, 112, 117, 115, 104, 40]))).concat(&key)).concat(&self::ml_string::Text::from_units(vec![41, 59]))
                                                                                        };
                                                                                        {
                                                                                            let replacement = if (*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                                                                                                ((values.concat(&self::ml_string::Text::from_units(vec![91, 105, 110, 100, 101, 120, 93, 46, 49, 32, 61, 32]))).concat(&second)).concat(&self::ml_string::Text::from_units(vec![59]))
                                                                                            } else {
                                                                                                self::ml_string::Text::from_units(vec![108, 101, 116, 32, 95, 32, 61, 32, 105, 110, 100, 101, 120, 59])
                                                                                            };
                                                                                            ((((((((((((prefix_3.concat(&self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 117, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32, 109, 97, 116, 99, 104, 32]))).concat(&position)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 83, 111, 109, 101, 40, 105, 110, 100, 101, 120, 41, 32, 61, 62, 32, 123, 32]))).concat(&replacement)).concat(&self::ml_string::Text::from_units(vec![32, 125, 44, 32, 78, 111, 110, 101, 32, 61, 62, 32, 123, 32]))).concat(&insertion)).concat(&self::ml_string::Text::from_units(vec![32, 125, 32, 125, 32, 125, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
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
pub fn read_array_callback_result_form(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if (*operation) == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
        self::ml_string::Text::from_units(vec![
            109, 97, 112, 112, 101, 100, 32, 97, 114, 114, 97, 121,
        ])
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![102, 105, 108, 116, 101, 114]))
    {
        self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])
    } else if (*operation) == (self::ml_string::Text::from_units(vec![102, 105, 110, 100])) {
        self::ml_string::Text::from_units(vec![
            111, 112, 116, 105, 111, 110, 97, 108, 32, 101, 108, 101, 109, 101, 110, 116,
        ])
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 73, 110, 100, 101, 120]))
    {
        self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])
    } else if ((*operation) == (self::ml_string::Text::from_units(vec![115, 111, 109, 101])))
        || ((*operation) == (self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121])))
    {
        self::ml_string::Text::from_units(vec![98, 111, 111, 108])
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112]))
    {
        self::ml_string::Text::from_units(vec![
            99, 97, 108, 108, 98, 97, 99, 107, 32, 97, 114, 114, 97, 121,
        ])
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![102, 111, 114, 69, 97, 99, 104]))
    {
        self::ml_string::Text::from_units(vec![117, 110, 105, 116])
    } else if (*operation) == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))
    {
        self::ml_string::Text::from_units(vec![97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_expected_result_form(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (((((*operation)
        == (self::ml_string::Text::from_units(vec![102, 105, 108, 116, 101, 114])))
        || ((*operation) == (self::ml_string::Text::from_units(vec![102, 105, 110, 100]))))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                102, 105, 110, 100, 73, 110, 100, 101, 120,
            ]))))
        || ((*operation) == (self::ml_string::Text::from_units(vec![115, 111, 109, 101]))))
        || ((*operation) == (self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121])))
    {
        self::ml_string::Text::from_units(vec![98, 111, 111, 108])
    } else if (*operation) == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))
    {
        self::ml_string::Text::from_units(vec![97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114])
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112]))
    {
        self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_maximum_arguments(operation: &self::ml_string::Text) -> f64 {
    if (*operation) == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101])) {
        2f64
    } else if (self::read_array_callback_result_form(operation))
        == (self::ml_string::Text::from_units(vec![]))
    {
        -1f64
    } else {
        1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_signature_error(
    operation: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    declared_count: f64,
    provided_count: f64,
    parameters_match: bool,
    result_matches: bool,
    expected_result: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let ml_library_inequality_1 = (*kind)
            != (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]));
        if ml_library_inequality_1 {
            ((self::ml_string::Text::from_units(vec![46])).concat(operation)).concat(
                &self::ml_string::Text::from_units(vec![
                    40, 41, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 102, 117, 110,
                    99, 116, 105, 111, 110,
                ]),
            )
        } else if (declared_count > provided_count) || !parameters_match {
            ((self::ml_string::Text::from_units(vec![46])).concat(operation)).concat(
                &self::ml_string::Text::from_units(vec![
                    40, 41, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 112, 97, 114, 97, 109, 101,
                    116, 101, 114, 115, 32, 100, 105, 102, 102, 101, 114,
                ]),
            )
        } else if ((*expected_result) != (self::ml_string::Text::from_units(vec![])))
            && !result_matches
        {
            (((self::ml_string::Text::from_units(vec![46])).concat(operation)).concat(
                &self::ml_string::Text::from_units(vec![
                    40, 41, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 114, 101, 115, 117, 108,
                    116, 32, 109, 117, 115, 116, 32, 98, 101, 32,
                ]),
            ))
            .concat(expected_result)
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_parameter_role(
    operation: &self::ml_string::Text,
    position: f64,
) -> self::ml_string::Text {
    if (*operation) == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101])) {
        if position == 0f64 {
            self::ml_string::Text::from_units(vec![
                97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114,
            ])
        } else if position == 1f64 {
            self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
        } else if position == 2f64 {
            self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])
        } else if position == 3f64 {
            self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else if position == 0f64 {
        self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
    } else if position == 1f64 {
        self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])
    } else if position == 2f64 {
        self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_callback_argument(
    operation: &self::ml_string::Text,
    position: f64,
    suffix: &self::ml_string::Text,
    clone_element: bool,
) -> self::ml_string::Text {
    {
        let role = self::read_array_callback_parameter_role(operation, position);
        if role
            == (self::ml_string::Text::from_units(vec![
                97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114,
            ]))
        {
            (self::ml_string::Text::from_units(vec![
                109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114,
            ]))
            .concat(suffix)
        } else if role
            == (self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116]))
        {
            ((self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101]))
                .concat(suffix))
            .concat(&if clone_element {
                self::ml_string::Text::from_units(vec![46, 99, 108, 111, 110, 101, 40, 41])
            } else {
                self::ml_string::Text::from_units(vec![])
            })
        } else if role == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])) {
            ((self::ml_string::Text::from_units(vec![40, 109, 108, 95, 105, 110, 100, 101, 120]))
                .concat(suffix))
            .concat(&self::ml_string::Text::from_units(vec![
                32, 97, 115, 32, 102, 54, 52, 41,
            ]))
        } else if role
            == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114]))
        {
            ((self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101, 115]))
                .concat(suffix))
            .concat(&self::ml_string::Text::from_units(vec![
                46, 99, 108, 111, 110, 101, 40, 41,
            ]))
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_callback_call(
    operation: &self::ml_string::Text,
    parameter_count: f64,
    suffix: &self::ml_string::Text,
    clone_element: bool,
) -> self::ml_string::Text {
    {
        let arguments_text = self::ml_string::Text::from_units(vec![]);
        {
            let position = 0f64;
            {
                let maximum = if (*operation)
                    == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))
                {
                    4f64
                } else {
                    3f64
                };
                {
                    let arguments_text_2 =
                        self::source_ml_source1_temporary_render_array_callback_call_loop8(
                            operation,
                            parameter_count,
                            suffix,
                            clone_element,
                            arguments_text,
                            position,
                            maximum,
                        );
                    ((((self::ml_string::Text::from_units(vec![
                        109, 108, 95, 99, 97, 108, 108, 98, 97, 99, 107,
                    ]))
                    .concat(suffix))
                    .concat(&self::ml_string::Text::from_units(vec![40])))
                    .concat(&arguments_text_2))
                    .concat(&self::ml_string::Text::from_units(vec![41]))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_callback_expression(
    operation: &self::ml_string::Text,
    object: &self::ml_string::Text,
    callback: &self::ml_string::Text,
    parameter_count: f64,
    suffix: &self::ml_string::Text,
    initial: &self::ml_string::Text,
    has_initial: bool,
) -> self::ml_string::Text {
    {
        let values =
            (self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101, 115]))
                .concat(suffix);
        {
            let index =
                (self::ml_string::Text::from_units(vec![109, 108, 95, 105, 110, 100, 101, 120]))
                    .concat(suffix);
            {
                let value =
                    (self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101]))
                        .concat(suffix);
                {
                    let call =
                        self::render_array_callback_call(operation, parameter_count, suffix, false);
                    {
                        let head = ((((((((self::ml_string::Text::from_units(vec![
                            108, 101, 116, 32,
                        ]))
                        .concat(&values))
                        .concat(&self::ml_string::Text::from_units(vec![32, 61, 32])))
                        .concat(object))
                        .concat(&self::ml_string::Text::from_units(vec![
                            59, 32, 108, 101, 116, 32, 109, 108, 95, 99, 97, 108, 108, 98, 97, 99,
                            107,
                        ])))
                        .concat(suffix))
                        .concat(&self::ml_string::Text::from_units(vec![32, 61, 32])))
                        .concat(callback))
                        .concat(&self::ml_string::Text::from_units(vec![59]));
                        {
                            let iterator = values.concat(&self::ml_string::Text::from_units(vec![
                                46, 105, 116, 101, 114, 40, 41, 46, 99, 108, 111, 110, 101, 100,
                                40, 41, 46, 101, 110, 117, 109, 101, 114, 97, 116, 101, 40, 41,
                            ]));
                            if ((*operation)
                                == (self::ml_string::Text::from_units(vec![115, 111, 109, 101])))
                                || ((*operation)
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 118, 101, 114, 121,
                                    ])))
                            {
                                ((((((((((((self::ml_string::Text::from_units(vec![
                                    123, 32,
                                ]))
                                .concat(&head))
                                .concat(&self::ml_string::Text::from_units(vec![32])))
                                .concat(&iterator))
                                .concat(&self::ml_string::Text::from_units(vec![46])))
                                .concat(&if (*operation)
                                    == (self::ml_string::Text::from_units(vec![115, 111, 109, 101]))
                                {
                                    self::ml_string::Text::from_units(vec![97, 110, 121])
                                } else {
                                    self::ml_string::Text::from_units(vec![97, 108, 108])
                                }))
                                .concat(&self::ml_string::Text::from_units(vec![40, 124, 40])))
                                .concat(&index))
                                .concat(&self::ml_string::Text::from_units(vec![44, 32])))
                                .concat(&value))
                                .concat(&self::ml_string::Text::from_units(vec![41, 124, 32])))
                                .concat(&call))
                                .concat(&self::ml_string::Text::from_units(vec![41, 32, 125]))
                            } else if (*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    102, 105, 110, 100, 73, 110, 100, 101, 120,
                                ]))
                            {
                                ((((((((((self::ml_string::Text::from_units(vec![123, 32]))
                                    .concat(&head))
                                .concat(&self::ml_string::Text::from_units(vec![32])))
                                .concat(&iterator))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    46, 112, 111, 115, 105, 116, 105, 111, 110, 40, 124, 40,
                                ])))
                                .concat(&index))
                                .concat(&self::ml_string::Text::from_units(vec![44, 32])))
                                .concat(&value))
                                .concat(&self::ml_string::Text::from_units(vec![41, 124, 32])))
                                .concat(&call))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    41, 46, 109, 97, 112, 95, 111, 114, 40, 45, 49, 102, 54, 52,
                                    44, 32, 124, 109, 108, 95, 112, 111, 115, 105, 116, 105, 111,
                                    110, 124, 32, 109, 108, 95, 112, 111, 115, 105, 116, 105, 111,
                                    110, 32, 97, 115, 32, 102, 54, 52, 41, 32, 125,
                                ]))
                            } else if (*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    102, 108, 97, 116, 77, 97, 112,
                                ]))
                            {
                                ((((((((((self::ml_string::Text::from_units(vec![123, 32]))
                                    .concat(&head))
                                .concat(&self::ml_string::Text::from_units(vec![32])))
                                .concat(&iterator))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    46, 102, 108, 97, 116, 95, 109, 97, 112, 40, 124, 40,
                                ])))
                                .concat(&index))
                                .concat(&self::ml_string::Text::from_units(vec![44, 32])))
                                .concat(&value))
                                .concat(&self::ml_string::Text::from_units(vec![41, 124, 32])))
                                .concat(&call))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101,
                                    99, 60, 95, 62, 62, 40, 41, 32, 125,
                                ]))
                            } else if ((*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    102, 105, 108, 116, 101, 114,
                                ])))
                                || ((*operation)
                                    == (self::ml_string::Text::from_units(vec![
                                        102, 105, 110, 100,
                                    ])))
                            {
                                {
                                    let predicate = self::render_array_callback_call(
                                        operation,
                                        parameter_count,
                                        suffix,
                                        true,
                                    );
                                    if (*operation)
                                        == (self::ml_string::Text::from_units(vec![
                                            102, 105, 108, 116, 101, 114,
                                        ]))
                                    {
                                        ((((((((((((self::ml_string::Text::from_units(
                                            vec![123, 32],
                                        ))
                                        .concat(&head))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32,
                                        ])))
                                        .concat(&iterator))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            46, 102, 105, 108, 116, 101, 114, 95, 109, 97, 112, 40,
                                            124, 40,
                                        ])))
                                        .concat(&index))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            44, 32,
                                        ])))
                                        .concat(&value))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            41, 124, 32, 105, 102, 32,
                                        ])))
                                        .concat(&predicate))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32, 123, 32, 83, 111, 109, 101, 40,
                                        ])))
                                        .concat(&value))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            41, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 78,
                                            111, 110, 101, 32, 125, 41, 46, 99, 111, 108, 108, 101,
                                            99, 116, 58, 58, 60, 86, 101, 99, 60, 95, 62, 62, 40,
                                            41, 32, 125,
                                        ]))
                                    } else {
                                        ((((((((((((self::ml_string::Text::from_units(
                                            vec![123, 32],
                                        ))
                                        .concat(&head))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32,
                                        ])))
                                        .concat(&iterator))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            46, 102, 105, 110, 100, 95, 109, 97, 112, 40, 124, 40,
                                        ])))
                                        .concat(&index))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            44, 32,
                                        ])))
                                        .concat(&value))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            41, 124, 32, 105, 102, 32,
                                        ])))
                                        .concat(&predicate))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32, 123, 32, 83, 111, 109, 101, 40,
                                        ])))
                                        .concat(&value))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            41, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 78,
                                            111, 110, 101, 32, 125, 41, 32, 125,
                                        ]))
                                    }
                                }
                            } else if (*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    102, 111, 114, 69, 97, 99, 104,
                                ]))
                            {
                                ((((((((((self::ml_string::Text::from_units(vec![123, 32]))
                                    .concat(&head))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    32, 102, 111, 114, 32, 40,
                                ])))
                                .concat(&index))
                                .concat(&self::ml_string::Text::from_units(vec![44, 32])))
                                .concat(&value))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    41, 32, 105, 110, 32,
                                ])))
                                .concat(&iterator))
                                .concat(&self::ml_string::Text::from_units(vec![32, 123, 32])))
                                .concat(&call))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    59, 32, 125, 32, 40, 41, 32, 125,
                                ]))
                            } else if (*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    114, 101, 100, 117, 99, 101,
                                ]))
                            {
                                if has_initial {
                                    ((((((((((((((self::ml_string::Text::from_units(
                                        vec![123, 32],
                                    ))
                                    .concat(&head))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        32,
                                    ])))
                                    .concat(&iterator))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        46, 102, 111, 108, 100, 40,
                                    ])))
                                    .concat(initial))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        44, 32, 124, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108,
                                        97, 116, 111, 114,
                                    ])))
                                    .concat(suffix))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        44, 32, 40,
                                    ])))
                                    .concat(&index))
                                    .concat(&self::ml_string::Text::from_units(vec![44, 32])))
                                    .concat(&value))
                                    .concat(&self::ml_string::Text::from_units(vec![41, 124, 32])))
                                    .concat(&call))
                                    .concat(&self::ml_string::Text::from_units(vec![41, 32, 125]))
                                } else {
                                    ((((((((((((((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32, 108, 101, 116, 32, 109, 117, 116, 32, 109, 108, 95, 105, 116, 101, 114, 97, 116, 111, 114]))).concat(suffix)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 105, 110, 105, 116, 105, 97, 108]))).concat(suffix)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 109, 108, 95, 105, 116, 101, 114, 97, 116, 111, 114]))).concat(suffix)).concat(&self::ml_string::Text::from_units(vec![46, 110, 101, 120, 116, 40, 41, 46, 109, 97, 112, 40, 124, 40, 95, 44, 32, 109, 108, 95, 102, 105, 114, 115, 116, 41, 124, 32, 109, 108, 95, 102, 105, 114, 115, 116, 41, 46, 101, 120, 112, 101, 99, 116, 40, 34, 82, 101, 100, 117, 99, 101, 32, 111, 102, 32, 101, 109, 112, 116, 121, 32, 97, 114, 114, 97, 121, 32, 119, 105, 116, 104, 32, 110, 111, 32, 105, 110, 105, 116, 105, 97, 108, 32, 118, 97, 108, 117, 101, 34, 41, 59, 32, 109, 108, 95, 105, 116, 101, 114, 97, 116, 111, 114]))).concat(suffix)).concat(&self::ml_string::Text::from_units(vec![46, 102, 111, 108, 100, 40, 109, 108, 95, 105, 110, 105, 116, 105, 97, 108]))).concat(suffix)).concat(&self::ml_string::Text::from_units(vec![44, 32, 124, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114]))).concat(suffix)).concat(&self::ml_string::Text::from_units(vec![44, 32, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32]))).concat(&call)).concat(&self::ml_string::Text::from_units(vec![41, 32, 125]))
                                }
                            } else {
                                self::ml_string::Text::from_units(vec![])
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
pub fn read_array_callback_argument_count_error(
    operation: &self::ml_string::Text,
    count: f64,
) -> self::ml_string::Text {
    {
        let maximum = self::read_array_callback_maximum_arguments(operation);
        if (maximum < 0f64) || ((count >= 1f64) && (count <= maximum)) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let expected = if maximum == 1f64 {
                    self::ml_string::Text::from_units(vec![49])
                } else {
                    self::ml_string::Text::from_units(vec![49, 32, 116, 111, 32, 50])
                };
                ((((self::ml_string::Text::from_units(vec![46])).concat(operation)).concat(
                    &self::ml_string::Text::from_units(vec![
                        40, 41, 32, 101, 120, 112, 101, 99, 116, 115, 32,
                    ]),
                ))
                .concat(&expected))
                .concat(&self::ml_string::Text::from_units(vec![
                    32, 97, 114, 103, 117, 109, 101, 110, 116, 115,
                ]))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_checked_result_context(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let expected = self::read_array_callback_expected_result_form(operation);
        if expected == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])) {
            self::ml_string::Text::from_units(vec![])
        } else {
            expected
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_result_error(
    operation: &self::ml_string::Text,
    result_kind: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*operation) == (self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112])))
        && ((*result_kind) != (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))
    {
        self::ml_string::Text::from_units(vec![
            102, 108, 97, 116, 77, 97, 112, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 109, 117,
            115, 116, 32, 114, 101, 116, 117, 114, 110, 32, 97, 110, 32, 97, 114, 114, 97, 121,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_checked_argument_position(
    operation: &self::ml_string::Text,
    has_initial: bool,
    position: f64,
) -> f64 {
    if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101])))
        && has_initial
    {
        if position == 0f64 {
            1f64
        } else if position == 1f64 {
            0f64
        } else {
            -1f64
        }
    } else if position == 0f64 {
        0f64
    } else {
        -1f64
    }
}
