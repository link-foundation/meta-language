// Generated from complete js/src/translation/disjoint-collection-projection-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_string;
use super::ml_string_number;
use super::read_array_callback_result_form;
use super::read_collection_access_result;
use super::read_reference_method_result;
use super::read_required_number_fact;
use super::source_ml_source1_temporary_add_field_loop27;
use super::source_ml_source1_temporary_add_roots_loop18;
use super::source_ml_source1_temporary_add_roots_loop19;
use super::source_ml_source1_temporary_edges_loop20;
use super::source_ml_source1_temporary_read_field_loop23;
use super::source_ml_source1_temporary_read_roots_loop15;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn render_collection_read_expression(
    target: &self::ml_string::Text,
    kind_2: &self::ml_string::Text,
    operation: &self::ml_string::Text,
    object: &self::ml_string::Text,
    search: &self::ml_string::Text,
    float_key: bool,
    comparison: &self::ml_string::Text,
    key_type: &self::ml_string::Text,
    value_type: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*target)
        == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))
    {
        ((((((self::ml_string::Text::from_units(vec![40])).concat(object))
            .concat(&self::ml_string::Text::from_units(vec![41, 46])))
        .concat(operation))
        .concat(&self::ml_string::Text::from_units(vec![40])))
        .concat(search))
        .concat(&self::ml_string::Text::from_units(vec![41]))
    } else if (*target) == (self::ml_string::Text::from_units(vec![82, 117, 115, 116])) {
        {
            let equal = if float_key {
                self::ml_string::Text::from_units(vec![
                    40, 109, 108, 95, 107, 101, 121, 32, 61, 61, 32, 38, 109, 108, 95, 115, 101,
                    97, 114, 99, 104, 32, 124, 124, 32, 40, 109, 108, 95, 107, 101, 121, 46, 105,
                    115, 95, 110, 97, 110, 40, 41, 32, 38, 38, 32, 109, 108, 95, 115, 101, 97, 114,
                    99, 104, 46, 105, 115, 95, 110, 97, 110, 40, 41, 41, 41,
                ])
            } else {
                self::ml_string::Text::from_units(vec![
                    109, 108, 95, 107, 101, 121, 32, 61, 61, 32, 38, 109, 108, 95, 115, 101, 97,
                    114, 99, 104,
                ])
            };
            {
                let bindings =
                    if (*kind_2) == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                        self::ml_string::Text::from_units(vec![
                            40, 109, 108, 95, 107, 101, 121, 44, 32, 109, 108, 95, 118, 97, 108,
                            117, 101, 41,
                        ])
                    } else {
                        self::ml_string::Text::from_units(vec![109, 108, 95, 107, 101, 121])
                    };
                {
                    let action = if (*operation)
                        == (self::ml_string::Text::from_units(vec![103, 101, 116]))
                    {
                        ((((self::ml_string::Text::from_units(vec![
                            102, 105, 110, 100, 95, 109, 97, 112, 40, 124,
                        ]))
                        .concat(&bindings))
                        .concat(&self::ml_string::Text::from_units(vec![
                            124, 32, 105, 102, 32,
                        ])))
                        .concat(&equal))
                        .concat(&self::ml_string::Text::from_units(vec![
                            32, 123, 32, 83, 111, 109, 101, 40, 109, 108, 95, 118, 97, 108, 117,
                            101, 46, 99, 108, 111, 110, 101, 40, 41, 41, 32, 125, 32, 101, 108,
                            115, 101, 32, 123, 32, 78, 111, 110, 101, 32, 125, 41,
                        ]))
                    } else {
                        ((((self::ml_string::Text::from_units(vec![97, 110, 121, 40, 124]))
                            .concat(&bindings))
                        .concat(&self::ml_string::Text::from_units(vec![124, 32])))
                        .concat(&equal))
                        .concat(&self::ml_string::Text::from_units(vec![41]))
                    };
                    ((((((self::ml_string::Text::from_units(vec![
                        123, 32, 108, 101, 116, 32, 109, 108, 95, 99, 111, 108, 108, 101, 99, 116,
                        105, 111, 110, 32, 61, 32,
                    ]))
                    .concat(object))
                    .concat(&self::ml_string::Text::from_units(vec![
                        59, 32, 108, 101, 116, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 32,
                        61, 32,
                    ])))
                    .concat(search))
                    .concat(&self::ml_string::Text::from_units(vec![
                        59, 32, 109, 108, 95, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 46,
                        105, 116, 101, 114, 40, 41, 46,
                    ])))
                    .concat(&action))
                    .concat(&self::ml_string::Text::from_units(vec![32, 125]))
                }
            }
        }
    } else if (*target) == (self::ml_string::Text::from_units(vec![76, 101, 97, 110])) {
        {
            let key = if (*kind_2) == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                self::ml_string::Text::from_units(vec![
                    109, 108, 95, 118, 97, 108, 117, 101, 46, 49,
                ])
            } else {
                self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])
            };
            {
                let equal_2 = if float_key {
                    ((((self::ml_string::Text::from_units(vec![40])).concat(&key)).concat(
                        &self::ml_string::Text::from_units(vec![
                            32, 61, 61, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 32, 124, 124,
                            32, 40,
                        ]),
                    ))
                    .concat(&key))
                    .concat(&self::ml_string::Text::from_units(vec![
                        46, 105, 115, 78, 97, 78, 32, 38, 38, 32, 109, 108, 95, 115, 101, 97, 114,
                        99, 104, 46, 105, 115, 78, 97, 78, 41, 41,
                    ]))
                } else {
                    ((self::ml_string::Text::from_units(vec![40])).concat(&key)).concat(
                        &self::ml_string::Text::from_units(vec![
                            32, 61, 61, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 41,
                        ]),
                    )
                };
                {
                    let result = if (*operation)
                        == (self::ml_string::Text::from_units(vec![103, 101, 116]))
                    {
                        ((self::ml_string::Text::from_units(vec![
                            40, 40, 109, 108, 95, 118, 97, 108, 117, 101, 115, 46, 116, 111, 76,
                            105, 115, 116, 46, 102, 105, 110, 100, 63, 32, 40, 102, 117, 110, 32,
                            109, 108, 95, 118, 97, 108, 117, 101, 32, 61, 62, 32,
                        ]))
                        .concat(&equal_2))
                        .concat(&self::ml_string::Text::from_units(vec![
                            41, 41, 46, 109, 97, 112, 32, 40, 102, 117, 110, 32, 109, 108, 95, 112,
                            97, 105, 114, 32, 61, 62, 32, 109, 108, 95, 112, 97, 105, 114, 46, 50,
                            41, 41,
                        ]))
                    } else {
                        ((self::ml_string::Text::from_units(vec![
                            40, 109, 108, 95, 118, 97, 108, 117, 101, 115, 46, 97, 110, 121, 32,
                            40, 102, 117, 110, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 61,
                            62, 32,
                        ]))
                        .concat(&equal_2))
                        .concat(&self::ml_string::Text::from_units(vec![41, 41]))
                    };
                    ((((((self::ml_string::Text::from_units(vec![
                        40, 108, 101, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 115, 32, 58,
                        61, 32,
                    ]))
                    .concat(object))
                    .concat(&self::ml_string::Text::from_units(vec![
                        59, 32, 108, 101, 116, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 32,
                        58, 61, 32,
                    ])))
                    .concat(search))
                    .concat(&self::ml_string::Text::from_units(vec![59, 32])))
                    .concat(&result))
                    .concat(&self::ml_string::Text::from_units(vec![41]))
                }
            }
        }
    } else if (*target) == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])) {
        {
            let key_2 = if (*kind_2) == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                self::ml_string::Text::from_units(vec![
                    40, 102, 115, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 41,
                ])
            } else {
                self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])
            };
            {
                let equal_3 = if float_key {
                    ((((self::ml_string::Text::from_units(vec![40, 111, 114, 98, 32]))
                        .concat(comparison))
                    .concat(&self::ml_string::Text::from_units(vec![
                        32, 40, 97, 110, 100, 98, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116,
                        46, 105, 115, 95, 110, 97, 110, 32,
                    ])))
                    .concat(&key_2))
                    .concat(&self::ml_string::Text::from_units(vec![
                        41, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 105, 115, 95,
                        110, 97, 110, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 41, 41, 41,
                    ]))
                } else {
                    comparison.clone()
                };
                {
                    let result_2 = if (*operation)
                        == (self::ml_string::Text::from_units(vec![103, 101, 116]))
                    {
                        ((((((self::ml_string::Text::from_units(vec![
                            40, 111, 112, 116, 105, 111, 110, 95, 109, 97, 112, 32, 40, 64, 115,
                            110, 100, 32,
                        ]))
                        .concat(key_type))
                        .concat(&self::ml_string::Text::from_units(vec![32])))
                        .concat(value_type))
                        .concat(&self::ml_string::Text::from_units(vec![
                            41, 32, 40, 76, 105, 115, 116, 46, 102, 105, 110, 100, 32, 40, 102,
                            117, 110, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 61, 62, 32,
                        ])))
                        .concat(&equal_3))
                        .concat(&self::ml_string::Text::from_units(vec![
                            41, 32, 109, 108, 95, 118, 97, 108, 117, 101, 115, 41, 41,
                        ]))
                    } else {
                        ((self::ml_string::Text::from_units(vec![
                            40, 76, 105, 115, 116, 46, 101, 120, 105, 115, 116, 115, 98, 32, 40,
                            102, 117, 110, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 61, 62,
                            32,
                        ]))
                        .concat(&equal_3))
                        .concat(&self::ml_string::Text::from_units(vec![
                            41, 32, 109, 108, 95, 118, 97, 108, 117, 101, 115, 41,
                        ]))
                    };
                    ((((((self::ml_string::Text::from_units(vec![
                        40, 108, 101, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 115, 32, 58,
                        61, 32,
                    ]))
                    .concat(object))
                    .concat(&self::ml_string::Text::from_units(vec![
                        32, 105, 110, 32, 108, 101, 116, 32, 109, 108, 95, 115, 101, 97, 114, 99,
                        104, 32, 58, 61, 32,
                    ])))
                    .concat(search))
                    .concat(&self::ml_string::Text::from_units(vec![32, 105, 110, 32])))
                    .concat(&result_2))
                    .concat(&self::ml_string::Text::from_units(vec![41]))
                }
            }
        }
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_disjoint_collection_iteration_source_operation() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        114, 101, 97, 100, 67, 108, 111, 115, 101, 100, 83, 116, 97, 110, 100, 97, 114, 100, 67,
        111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 105, 111, 110,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_disjoint_collection_spread_source_operation() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        114, 101, 97, 100, 67, 108, 111, 115, 101, 100, 83, 116, 97, 110, 100, 97, 114, 100, 67,
        111, 108, 108, 101, 99, 116, 105, 111, 110, 83, 112, 114, 101, 97, 100,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_disjoint_collection_iteration_marker(
    ordinal: f64,
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    (((self::ml_string::Text::from_units(vec![
        35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32,
        105, 116, 101, 114, 97, 116, 105, 111, 110, 32,
    ]))
    .concat(&{
        let source_ml_source2_string_argument11 = ordinal;
        self::ml_string::Text::from_text(&self::ml_number::js_number(
            source_ml_source2_string_argument11,
        ))
    }))
    .concat(&self::ml_string::Text::from_units(vec![32])))
    .concat(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_disjoint_collection_spread_marker(
    ordinal: f64,
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    (((self::ml_string::Text::from_units(vec![
        35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32,
        115, 112, 114, 101, 97, 100, 32,
    ]))
    .concat(&{
        let source_ml_source2_string_argument12 = ordinal;
        self::ml_string::Text::from_text(&self::ml_number::js_number(
            source_ml_source2_string_argument12,
        ))
    }))
    .concat(&self::ml_string::Text::from_units(vec![32])))
    .concat(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_disjoint_collection_iteration_marker_role(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if operation.starts_with(&self::ml_string::Text::from_units(vec![
        35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32,
        105, 116, 101, 114, 97, 116, 105, 111, 110, 32,
    ])) {
        self::ml_string::Text::from_units(vec![108, 111, 111, 112])
    } else if operation.starts_with(&self::ml_string::Text::from_units(vec![
        35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32,
        115, 112, 114, 101, 97, 100, 32,
    ])) {
        self::ml_string::Text::from_units(vec![101, 97, 103, 101, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_disjoint_collection_spread_receiver(shape: &self::ml_string::Text) -> bool {
    ((*shape) == (self::ml_string::Text::from_units(vec![110, 97, 109, 101])))
        || ((*shape) == (self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_disjoint_collection_iteration_scope(
    known: bool,
    externals: f64,
    literals: f64,
    bindings: f64,
    directory: bool,
    publication: bool,
) -> bool {
    ((((known && (externals == 0f64)) && (literals == 0f64)) && (bindings == 0f64)) && !directory)
        && !publication
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_disjoint_collection_iteration_operation(
    kind_2: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> bool {
    (((*kind_2) == (self::ml_string::Text::from_units(vec![109, 97, 112])))
        || ((*kind_2) == (self::ml_string::Text::from_units(vec![115, 101, 116]))))
        && ((*operation) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_disjoint_collection_iteration_item_form(
    kind_2: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if self::accept_disjoint_collection_iteration_operation(kind_2, operation) {
        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])
    } else {
        {
            let form = self::read_collection_access_result(kind_2, operation);
            if (form == (self::ml_string::Text::from_units(vec![107, 101, 121, 115])))
                || (form
                    == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])))
            {
                form
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_disjoint_collection_iteration_identity_kind(kind_2: &self::ml_string::Text) -> bool {
    self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]),
            self::ml_string::Text::from_units(vec![109, 97, 112]),
            self::ml_string::Text::from_units(vec![115, 101, 116]),
            self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]),
            self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]),
            self::ml_string::Text::from_units(vec![100, 97, 116, 97]),
            self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]),
            self::ml_string::Text::from_units(vec![99, 101, 108, 108]),
            self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]),
        ],
        kind_2,
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_disjoint_collection_iteration_method_effect(
    kind_2: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (((self::read_reference_method_result(kind_2, operation))
        != (self::ml_string::Text::from_units(vec![])))
        && ((*operation) != (self::ml_string::Text::from_units(vec![103, 101, 116]))))
        && ((*operation) != (self::ml_string::Text::from_units(vec![104, 97, 115])))
    {
        self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])
    } else if ((*kind_2) == (self::ml_string::Text::from_units(vec![109, 97, 112])))
        || ((*kind_2) == (self::ml_string::Text::from_units(vec![115, 101, 116])))
    {
        if self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![104, 97, 115]),
                self::ml_string::Text::from_units(vec![107, 101, 121, 115]),
                self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]),
                self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]),
            ],
            operation,
            0f64,
            false,
            |a, b| a == b,
        ) || (((*kind_2) == (self::ml_string::Text::from_units(vec![109, 97, 112])))
            && ((*operation) == (self::ml_string::Text::from_units(vec![103, 101, 116]))))
        {
            self::ml_string::Text::from_units(vec![114, 101, 97, 100])
        } else {
            self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])
        }
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])) {
        if self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![115, 111, 114, 116]),
                self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101]),
                self::ml_string::Text::from_units(vec![102, 105, 108, 108]),
                self::ml_string::Text::from_units(vec![
                    99, 111, 112, 121, 87, 105, 116, 104, 105, 110,
                ]),
            ],
            operation,
            0f64,
            false,
            |a, b| a == b,
        ) {
            self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])
        } else if self::read_disjoint_collection_iteration_callback(operation)
            || self::ml_array::includes(
                &vec![
                    self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101]),
                    self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116]),
                    self::ml_string::Text::from_units(vec![106, 111, 105, 110]),
                    self::ml_string::Text::from_units(vec![105, 110, 99, 108, 117, 100, 101, 115]),
                    self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 79, 102]),
                    self::ml_string::Text::from_units(vec![
                        108, 97, 115, 116, 73, 110, 100, 101, 120, 79, 102,
                    ]),
                    self::ml_string::Text::from_units(vec![97, 116]),
                    self::ml_string::Text::from_units(vec![102, 108, 97, 116]),
                    self::ml_string::Text::from_units(vec![
                        116, 111, 82, 101, 118, 101, 114, 115, 101, 100,
                    ]),
                    self::ml_string::Text::from_units(vec![116, 111, 83, 111, 114, 116, 101, 100]),
                ],
                operation,
                0f64,
                false,
                |a, b| a == b,
            )
        {
            self::ml_string::Text::from_units(vec![114, 101, 97, 100])
        } else {
            self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])
        }
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])) {
        if (self::read_disjoint_collection_iteration_write_field(operation)) >= 0f64 {
            self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])
        } else if (*operation)
            == (self::ml_string::Text::from_units(vec![
                114, 101, 97, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 99, 111,
                114, 100, 80, 114, 111, 112, 101, 114, 116, 121, 79, 114, 100, 101, 114,
            ]))
        {
            self::ml_string::Text::from_units(vec![114, 101, 97, 100])
        } else {
            self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])
        }
    } else {
        self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_disjoint_collection_iteration_read_expression(shape: &self::ml_string::Text) -> bool {
    self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![108, 101, 116]),
            self::ml_string::Text::from_units(vec![105, 102]),
            self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104]),
            self::ml_string::Text::from_units(vec![112, 114, 105, 110, 116]),
            self::ml_string::Text::from_units(vec![99, 111, 97, 108, 101, 115, 99, 101]),
            self::ml_string::Text::from_units(vec![115, 111, 109, 101]),
            self::ml_string::Text::from_units(vec![
                102, 108, 97, 116, 116, 101, 110, 79, 112, 116, 105, 111, 110,
            ]),
            self::ml_string::Text::from_units(vec![
                117, 110, 119, 114, 97, 112, 79, 112, 116, 105, 111, 110,
            ]),
            self::ml_string::Text::from_units(vec![108, 105, 116]),
            self::ml_string::Text::from_units(vec![117, 110, 105, 116]),
            self::ml_string::Text::from_units(vec![110, 111, 110, 101]),
            self::ml_string::Text::from_units(vec![97, 98, 111, 114, 116]),
            self::ml_string::Text::from_units(vec![
                105, 109, 112, 111, 115, 115, 105, 98, 108, 101,
            ]),
            self::ml_string::Text::from_units(vec![108, 97, 109, 98, 100, 97]),
            self::ml_string::Text::from_units(vec![99, 101, 108, 108, 82, 101, 97, 100]),
            self::ml_string::Text::from_units(vec![
                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 83, 105, 122, 101,
            ]),
            self::ml_string::Text::from_units(vec![
                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 82, 101, 97, 100,
            ]),
            self::ml_string::Text::from_units(vec![117, 110, 97, 114, 121]),
            self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]),
            self::ml_string::Text::from_units(vec![
                110, 117, 108, 108, 105, 115, 104, 78, 117, 109, 98, 101, 114,
            ]),
            self::ml_string::Text::from_units(vec![
                111, 112, 116, 105, 111, 110, 97, 108, 73, 110, 100, 101, 120, 78, 117, 109, 98,
                101, 114,
            ]),
            self::ml_string::Text::from_units(vec![116, 114, 117, 116, 104, 121]),
            self::ml_string::Text::from_units(vec![
                115, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108,
            ]),
            self::ml_string::Text::from_units(vec![
                111, 112, 116, 105, 111, 110, 97, 108, 84, 121, 112, 101, 79, 102,
            ]),
            self::ml_string::Text::from_units(vec![
                111, 112, 116, 105, 111, 110, 97, 108, 84, 101, 120, 116,
            ]),
            self::ml_string::Text::from_units(vec![
                111, 112, 116, 105, 111, 110, 97, 108, 80, 114, 101, 100, 105, 99, 97, 116, 101,
            ]),
            self::ml_string::Text::from_units(vec![
                111, 112, 116, 105, 111, 110, 97, 108, 70, 105, 101, 108, 100,
            ]),
            self::ml_string::Text::from_units(vec![
                111, 112, 116, 105, 111, 110, 97, 108, 80, 114, 111, 106, 101, 99, 116,
            ]),
            self::ml_string::Text::from_units(vec![
                111, 112, 116, 105, 111, 110, 97, 108, 73, 110, 100, 101, 120,
            ]),
            self::ml_string::Text::from_units(vec![
                115, 116, 114, 105, 110, 103, 80, 111, 105, 110, 116, 115,
            ]),
            self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103]),
            self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 77, 97, 112]),
            self::ml_string::Text::from_units(vec![
                115, 116, 114, 105, 110, 103, 84, 101, 115, 116,
            ]),
            self::ml_string::Text::from_units(vec![99, 97, 115, 116]),
            self::ml_string::Text::from_units(vec![97, 112, 112, 101, 110, 100]),
            self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120]),
            self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]),
        ],
        shape,
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_disjoint_collection_iteration_write_field(operation: &self::ml_string::Text) -> f64 {
    {
        let prefix = self::ml_string::Text::from_units(vec![
            119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100,
        ]);
        if (operation.starts_with(&prefix)) {
            {
                let text_2 = {
                    let source_ml_source2_string_range_operand13 = operation.clone();
                    {
                        let source_ml_source2_string_range_operand14 = prefix.length();
                        {
                            let source_ml_source2_string_range_position15 =
                                source_ml_source2_string_range_operand14;
                            {
                                let source_ml_source2_string_range_position16 = f64::INFINITY;
                                source_ml_source2_string_range_operand13.slice(
                                    source_ml_source2_string_range_position15,
                                    source_ml_source2_string_range_position16,
                                )
                            }
                        }
                    }
                };
                {
                    let index = self::ml_string_number::convert_java_script_string_number_units(
                        &(text_2.clone())
                            .0
                            .iter()
                            .copied()
                            .map(f64::from)
                            .collect::<Vec<f64>>(),
                    );
                    if (self::ml_math::is_integer(index) && (index >= 0f64))
                        && (({
                            let source_ml_source2_string_argument18 = index;
                            self::ml_string::Text::from_text(&self::ml_number::js_number(
                                source_ml_source2_string_argument18,
                            ))
                        }) == text_2)
                    {
                        index
                    } else {
                        -1f64
                    }
                }
            }
        } else {
            -1f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_disjoint_collection_iteration_callback(operation: &self::ml_string::Text) -> bool {
    (((self::read_array_callback_result_form(operation))
        != (self::ml_string::Text::from_units(vec![])))
        || ((*operation) == (self::ml_string::Text::from_units(vec![115, 111, 114, 116]))))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![116, 111, 83, 111, 114, 116, 101, 100])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_disjoint_collection_iteration_callback_head(role: &self::ml_string::Text) -> bool {
    (((*role) == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))
        || ((*role)
            == (self::ml_string::Text::from_units(vec![
                102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99,
                101,
            ]))))
        || ((*role)
            == (self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 98, 108, 101])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn add_fact(facts: &[f64], fact: f64) -> Vec<f64> {
    if ((self::read_required_number_fact(facts, 0f64)) == -1f64)
        || self::ml_array::includes(facts, &fact, 0f64, false, |a: &f64, b: &f64| {
            a == b || (a.is_nan() && b.is_nan())
        })
    {
        facts.to_vec()
    } else {
        {
            let source_ml_source2_concat_value19 = facts.to_vec();
            {
                let source_ml_source2_concat_value20 = vec![fact];
                self::ml_array::append(
                    &(self::ml_array::append(
                        &(Vec::<f64>::new()),
                        source_ml_source2_concat_value19,
                    )),
                    source_ml_source2_concat_value20,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn add_value(values: &[f64], value: f64) -> Vec<f64> {
    if self::ml_array::includes(values, &value, 0f64, false, |a: &f64, b: &f64| {
        a == b || (a.is_nan() && b.is_nan())
    }) {
        values.to_vec()
    } else {
        {
            let source_ml_source2_concat_value21 = values.to_vec();
            {
                let source_ml_source2_concat_value22 = vec![value];
                self::ml_array::append(
                    &(self::ml_array::append(
                        &(Vec::<f64>::new()),
                        source_ml_source2_concat_value21,
                    )),
                    source_ml_source2_concat_value22,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_roots(facts: &[f64], tag: f64, node: f64, width: f64, span: f64) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let source_ml_source1_temporary_values14 = facts.to_vec();
            {
                let source_ml_source1_temporary_index14 = self::ml::Big::from_i128(0);
                {
                    self::source_ml_source1_temporary_read_roots_loop15(
                        tag,
                        node,
                        width,
                        span,
                        result,
                        &(source_ml_source1_temporary_values14),
                        source_ml_source1_temporary_index14,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn add_roots(
    facts: &[f64],
    tag: f64,
    node: f64,
    roots: &[f64],
    width: f64,
    span: f64,
) -> Vec<f64> {
    {
        let present = self::read_roots(facts, tag, node, width, span);
        {
            let source_ml_source1_temporary_values16 = roots.to_vec();
            {
                let source_ml_source1_temporary_index16 = self::ml::Big::from_i128(0);
                {
                    let present_2 = self::source_ml_source1_temporary_add_roots_loop18(
                        present,
                        &(source_ml_source1_temporary_values16),
                        source_ml_source1_temporary_index16,
                    );
                    if (present_2.len() as f64) > 32f64 {
                        vec![-1f64]
                    } else {
                        {
                            let result = facts.to_vec();
                            {
                                let source_ml_source1_temporary_values17 = roots.to_vec();
                                {
                                    let source_ml_source1_temporary_index17 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        self::source_ml_source1_temporary_add_roots_loop19(
                                            tag,
                                            node,
                                            width,
                                            span,
                                            result,
                                            &(source_ml_source1_temporary_values17),
                                            source_ml_source1_temporary_index17,
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn edges(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    label: &self::ml_string::Text,
) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = self::read_required_number_fact(starts, node);
            {
                self::source_ml_source1_temporary_edges_loop20(
                    starts, counts, labels, targets, node, label, result, at,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_field(
    facts: &[f64],
    roots: &[f64],
    field_2: f64,
    field_width: f64,
    width: f64,
    span: f64,
) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let source_ml_source1_temporary_values21 = roots.to_vec();
            {
                let source_ml_source1_temporary_index21 = self::ml::Big::from_i128(0);
                {
                    self::source_ml_source1_temporary_read_field_loop23(
                        facts,
                        field_2,
                        field_width,
                        width,
                        span,
                        result,
                        &(source_ml_source1_temporary_values21),
                        source_ml_source1_temporary_index21,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn add_field(
    facts: &[f64],
    owners: &[f64],
    field_2: f64,
    roots: &[f64],
    field_width: f64,
    width: f64,
    span: f64,
) -> Vec<f64> {
    {
        let result = facts.to_vec();
        {
            let source_ml_source1_temporary_values25 = owners.to_vec();
            {
                let source_ml_source1_temporary_index25 = self::ml::Big::from_i128(0);
                {
                    self::source_ml_source1_temporary_add_field_loop27(
                        field_2,
                        roots,
                        field_width,
                        width,
                        span,
                        result,
                        &(source_ml_source1_temporary_values25),
                        source_ml_source1_temporary_index25,
                    )
                }
            }
        }
    }
}
