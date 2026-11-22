// Generated from complete js/src/translation/disjoint-collection-iteration-rules.js. Do not edit by hand.
use super::ml_array;
use super::ml_string;
use super::ml_utf16_text;
use super::render_array_callback_call;
use super::source_ml_source1_temporary_read_array_state_method_suffix_loop9;
use super::source_ml_source1_temporary_render_array_state_method_expression_loop10;
use super::source_ml_source1_temporary_render_array_state_method_expression_loop11;
use super::source_ml_source1_temporary_render_array_state_method_expression_loop12;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_callback_reference_expression(
    operation: &self::ml_string::Text,
    object: &self::ml_string::Text,
    callback: &self::ml_string::Text,
    parameter_count: f64,
    suffix: &self::ml_string::Text,
    reference_result: bool,
) -> self::ml_string::Text {
    if ((*operation) != (self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112])))
        || !reference_result
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let values = (self::ml_string::Text::from_units(vec![
                109, 108, 95, 118, 97, 108, 117, 101, 115,
            ]))
            .concat(suffix);
            {
                let index = (self::ml_string::Text::from_units(vec![
                    109, 108, 95, 105, 110, 100, 101, 120,
                ]))
                .concat(suffix);
                {
                    let value = (self::ml_string::Text::from_units(vec![
                        109, 108, 95, 118, 97, 108, 117, 101,
                    ]))
                    .concat(suffix);
                    {
                        let result = (self::ml_string::Text::from_units(vec![
                            109, 108, 95, 99, 97, 108, 108, 98, 97, 99, 107, 95, 118, 97, 108, 117,
                            101, 115,
                        ]))
                        .concat(suffix);
                        {
                            let call = self::render_array_callback_call(
                                operation,
                                parameter_count,
                                suffix,
                                false,
                            );
                            {
                                let head = ((((((((self::ml_string::Text::from_units(vec![
                                    108, 101, 116, 32,
                                ]))
                                .concat(&values))
                                .concat(&self::ml_string::Text::from_units(vec![32, 61, 32])))
                                .concat(object))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    59, 32, 108, 101, 116, 32, 109, 108, 95, 99, 97, 108, 108, 98,
                                    97, 99, 107,
                                ])))
                                .concat(suffix))
                                .concat(&self::ml_string::Text::from_units(vec![32, 61, 32])))
                                .concat(callback))
                                .concat(&self::ml_string::Text::from_units(vec![59]));
                                {
                                    let iterator =
                                        values.concat(&self::ml_string::Text::from_units(vec![
                                            46, 105, 116, 101, 114, 40, 41, 46, 99, 108, 111, 110,
                                            101, 100, 40, 41, 46, 101, 110, 117, 109, 101, 114, 97,
                                            116, 101, 40, 41,
                                        ]));
                                    ((((((((((((((self::ml_string::Text::from_units(
                                        vec![123, 32],
                                    ))
                                    .concat(&head))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        32, 115, 116, 100, 58, 58, 114, 99, 58, 58, 82, 99, 58, 58,
                                        110, 101, 119, 40, 115, 116, 100, 58, 58, 99, 101, 108,
                                        108, 58, 58, 82, 101, 102, 67, 101, 108, 108, 58, 58, 110,
                                        101, 119, 40,
                                    ])))
                                    .concat(&iterator))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        46, 102, 108, 97, 116, 95, 109, 97, 112, 40, 124, 40,
                                    ])))
                                    .concat(&index))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        44, 32,
                                    ])))
                                    .concat(&value))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        41, 124, 32, 123, 32, 108, 101, 116, 32,
                                    ])))
                                    .concat(&result))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        32, 61, 32,
                                    ])))
                                    .concat(&call))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        46, 98, 111, 114, 114, 111, 119, 40, 41, 46, 99, 108, 111,
                                        110, 101, 40, 41, 59, 32,
                                    ])))
                                    .concat(&result))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        32, 125, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58,
                                        60, 86, 101, 99, 60, 95, 62, 62, 40, 41, 41, 41, 32, 125,
                                    ]))
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
pub fn read_array_state_method_result(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if (*operation) == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101])) {
        self::ml_string::Text::from_units(vec![
            114, 101, 109, 111, 118, 101, 100, 32, 97, 114, 114, 97, 121,
        ])
    } else if ((*operation) == (self::ml_string::Text::from_units(vec![102, 105, 108, 108])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 112, 121, 87, 105, 116, 104, 105, 110,
            ])))
    {
        self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_array_state_method(
    expression: &self::ml_string::Text,
    operation: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    reference: bool,
) -> bool {
    ((((*expression) == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])))
        && ((*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))))
        && reference)
        && ((self::read_array_state_method_result(operation))
            != (self::ml_string::Text::from_units(vec![])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_state_method_required_arguments(operation: &self::ml_string::Text) -> f64 {
    if (*operation) == (self::ml_string::Text::from_units(vec![102, 105, 108, 108])) {
        1f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_state_method_argument_role(
    operation: &self::ml_string::Text,
    position: f64,
) -> self::ml_string::Text {
    if (*operation) == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101])) {
        if position < 2f64 {
            self::ml_string::Text::from_units(vec![
                110, 117, 109, 98, 101, 114, 32, 105, 110, 100, 101, 120,
            ])
        } else {
            self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
        }
    } else if (*operation) == (self::ml_string::Text::from_units(vec![102, 105, 108, 108])) {
        if position == 0f64 {
            self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
        } else if position == 1f64 {
            self::ml_string::Text::from_units(vec![
                110, 117, 109, 98, 101, 114, 32, 105, 110, 100, 101, 120,
            ])
        } else if position == 2f64 {
            self::ml_string::Text::from_units(vec![101, 110, 100, 32, 105, 110, 100, 101, 120])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![99, 111, 112, 121, 87, 105, 116, 104, 105, 110]))
    {
        if position < 2f64 {
            self::ml_string::Text::from_units(vec![
                110, 117, 109, 98, 101, 114, 32, 105, 110, 100, 101, 120,
            ])
        } else if position == 2f64 {
            self::ml_string::Text::from_units(vec![101, 110, 100, 32, 105, 110, 100, 101, 120])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_state_method_argument_profile(
    kind: &self::ml_string::Text,
    element: &self::ml_string::Text,
    absence: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (((*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))
    {
        kind.clone()
    } else if (((*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
        && ((((*element) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))
            || ((*element) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))))
            || ((*element) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))))
        && ((((*absence)
            == (self::ml_string::Text::from_units(vec![
                117, 110, 100, 101, 102, 105, 110, 101, 100,
            ])))
            || ((*absence) == (self::ml_string::Text::from_units(vec![110, 117, 108, 108]))))
            || ((*absence) == (self::ml_string::Text::from_units(vec![98, 111, 116, 104]))))
    {
        (((self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 32]))
            .concat(absence))
        .concat(&self::ml_string::Text::from_units(vec![32])))
        .concat(element)
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_state_method_suffix(
    initial: f64,
    receiver: &self::ml_string::Text,
    arguments_: &[self::ml_string::Text],
) -> f64 {
    {
        let text = (receiver.concat(&self::ml_string::Text::from_units(vec![32]))).concat(
            &self::ml_string::join(arguments_, &self::ml_string::Text::from_units(vec![32])),
        );
        {
            let suffix = initial;
            {
                self::source_ml_source1_temporary_read_array_state_method_suffix_loop9(
                    &(text),
                    suffix,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_state_numeric_value(
    profile: &self::ml_string::Text,
    value: &self::ml_string::Text,
    undefined_value: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*profile) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])) {
        value.clone()
    } else if (*profile) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
        ((self::ml_string::Text::from_units(vec![105, 102, 32])).concat(value)).concat(
            &self::ml_string::Text::from_units(vec![
                32, 123, 32, 49, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 48,
                102, 54, 52, 32, 125,
            ]),
        )
    } else if (*profile) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])) {
        undefined_value.clone()
    } else {
        {
            let parts = {
                let source_ml_source2_string_unit_operand6 = profile.clone();
                {
                    let source_ml_source2_string_unit_operand7 =
                        self::ml_string::Text::from_units(vec![32]);
                    {
                        let source_ml_source2_string_split_limit8 = 4_294_967_295f64;
                        {
                            let source_ml_source2_string_split_separator9 =
                                source_ml_source2_string_unit_operand7;
                            (|ml_text: &self::ml_string::Text,
                              ml_separator: Option<&self::ml_string::Text>,
                              ml_limit: f64| {
                                self::ml_utf16_text::split_utf16_text(
                                    &(ml_text
                                        .0
                                        .iter()
                                        .map(|ml_unit| f64::from(*ml_unit))
                                        .collect::<Vec<f64>>()),
                                    &(ml_separator
                                        .map(|ml_value| {
                                            ml_value
                                                .0
                                                .iter()
                                                .map(|ml_unit| f64::from(*ml_unit))
                                                .collect::<Vec<f64>>()
                                        })
                                        .unwrap_or_default()),
                                    ml_limit,
                                    ml_separator.is_none(),
                                )
                                .into_iter()
                                .map(|ml_units| {
                                    self::ml_string::Text::from_units(
                                        ml_units
                                            .into_iter()
                                            .map(|ml_unit| ml_unit as u16)
                                            .collect(),
                                    )
                                })
                                .collect::<Vec<_>>()
                            })(
                                &(source_ml_source2_string_unit_operand6),
                                Some(&source_ml_source2_string_split_separator9),
                                source_ml_source2_string_split_limit8,
                            )
                        }
                    }
                }
            };
            {
                let absence =
                    (self::ml_array::optional_at(&parts, self::ml_array::number_index(1f64)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let element =
                        (self::ml_array::optional_at(&parts, self::ml_array::number_index(2f64)))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    {
                        let present = if element
                            == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))
                        {
                            self::ml_string::Text::from_units(vec![
                                109, 108, 95, 105, 110, 100, 101, 120, 95, 112, 114, 101, 115, 101,
                                110, 116,
                            ])
                        } else if element
                            == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))
                        {
                            self::ml_string::Text::from_units(vec![
                                105, 102, 32, 109, 108, 95, 105, 110, 100, 101, 120, 95, 112, 114,
                                101, 115, 101, 110, 116, 32, 123, 32, 49, 102, 54, 52, 32, 125, 32,
                                101, 108, 115, 101, 32, 123, 32, 48, 102, 54, 52, 32, 125,
                            ])
                        } else {
                            self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78])
                        };
                        if absence == (self::ml_string::Text::from_units(vec![98, 111, 116, 104])) {
                            ((((((self::ml_string::Text::from_units(vec![
                                109, 97, 116, 99, 104, 32,
                            ]))
                            .concat(value))
                            .concat(&self::ml_string::Text::from_units(vec![
                                32, 123, 32, 99, 114, 97, 116, 101, 58, 58, 77, 108, 79, 112, 116,
                                105, 111, 110, 97, 108, 58, 58, 86, 97, 108, 117, 101, 40, 109,
                                108, 95, 105, 110, 100, 101, 120, 95, 112, 114, 101, 115, 101, 110,
                                116, 41, 32, 61, 62, 32,
                            ])))
                            .concat(&present))
                            .concat(&self::ml_string::Text::from_units(vec![
                                44, 32, 99, 114, 97, 116, 101, 58, 58, 77, 108, 79, 112, 116, 105,
                                111, 110, 97, 108, 58, 58, 78, 117, 108, 108, 32, 61, 62, 32, 48,
                                102, 54, 52, 44, 32, 99, 114, 97, 116, 101, 58, 58, 77, 108, 79,
                                112, 116, 105, 111, 110, 97, 108, 58, 58, 85, 110, 100, 101, 102,
                                105, 110, 101, 100, 32, 61, 62, 32,
                            ])))
                            .concat(undefined_value))
                            .concat(&self::ml_string::Text::from_units(vec![32, 125]))
                        } else {
                            ((((((self::ml_string::Text::from_units(vec![
                                109, 97, 116, 99, 104, 32,
                            ]))
                            .concat(value))
                            .concat(&self::ml_string::Text::from_units(vec![
                                32, 123, 32, 83, 111, 109, 101, 40, 109, 108, 95, 105, 110, 100,
                                101, 120, 95, 112, 114, 101, 115, 101, 110, 116, 41, 32, 61, 62,
                                32,
                            ])))
                            .concat(&present))
                            .concat(&self::ml_string::Text::from_units(vec![
                                44, 32, 78, 111, 110, 101, 32, 61, 62, 32,
                            ])))
                            .concat(&if absence
                                == (self::ml_string::Text::from_units(vec![110, 117, 108, 108]))
                            {
                                self::ml_string::Text::from_units(vec![48, 102, 54, 52])
                            } else {
                                undefined_value.clone()
                            }))
                            .concat(&self::ml_string::Text::from_units(vec![32, 125]))
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_state_relative_index(
    value: &self::ml_string::Text,
    length: &self::ml_string::Text,
) -> self::ml_string::Text {
    (((((self::ml_string::Text::from_units(vec![
        123, 32, 108, 101, 116, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 95, 118,
        97, 108, 117, 101, 58, 32, 102, 54, 52, 32, 61, 32,
    ]))
    .concat(value))
    .concat(&self::ml_string::Text::from_units(vec![
        59, 32, 108, 101, 116, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 32, 61, 32,
        105, 102, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 95, 118, 97, 108, 117,
        101, 46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 123, 32, 48, 102, 54, 52, 32, 125, 32,
        101, 108, 115, 101, 32, 123, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 95,
        118, 97, 108, 117, 101, 46, 116, 114, 117, 110, 99, 40, 41, 32, 125, 59, 32,
    ])))
    .concat(&self::ml_string::Text::from_units(vec![
        108, 101, 116, 32, 109, 108, 95, 98, 111, 117, 110, 100, 32, 61, 32,
    ])))
    .concat(length))
    .concat(&self::ml_string::Text::from_units(vec![
        32, 97, 115, 32, 102, 54, 52, 59, 32, 40, 105, 102, 32, 109, 108, 95, 114, 101, 108, 97,
        116, 105, 118, 101, 32, 60, 32, 48, 102, 54, 52, 32, 123, 32, 40, 109, 108, 95, 98, 111,
        117, 110, 100, 32, 43, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 41, 46, 99,
        108, 97, 109, 112, 40, 48, 102, 54, 52, 44, 32, 109, 108, 95, 98, 111, 117, 110, 100, 41,
        32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105,
        118, 101, 46, 109, 105, 110, 40, 109, 108, 95, 98, 111, 117, 110, 100, 41, 32, 125, 41, 32,
        97, 115, 32, 117, 115, 105, 122, 101, 32, 125,
    ]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_state_method_identity(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*operation) == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101])) {
        self::ml_string::Text::from_units(vec![
            114, 101, 112, 108, 97, 99, 101, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109,
            101, 110, 116, 115,
        ])
    } else if (*operation) == (self::ml_string::Text::from_units(vec![102, 105, 108, 108])) {
        self::ml_string::Text::from_units(vec![
            102, 105, 108, 108, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116,
            115,
        ])
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![99, 111, 112, 121, 87, 105, 116, 104, 105, 110]))
    {
        self::ml_string::Text::from_units(vec![
            99, 111, 112, 121, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116,
            115, 45, 119, 105, 116, 104, 105, 110,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_state_method_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        68, 101, 110, 115, 101, 32, 115, 104, 97, 114, 101, 100, 32, 97, 114, 114, 97, 121, 32,
        114, 97, 110, 103, 101, 32, 109, 117, 116, 97, 116, 105, 111, 110, 115, 32, 101, 118, 97,
        108, 117, 97, 116, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99,
        101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 101, 118, 101, 114, 121, 32, 97, 114, 103,
        117, 109, 101, 110, 116, 32, 111, 110, 99, 101, 32, 105, 110, 32, 111, 114, 100, 101, 114,
        44, 32, 100, 105, 115, 116, 105, 110, 103, 117, 105, 115, 104, 32, 109, 105, 115, 115, 105,
        110, 103, 32, 97, 110, 100, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 112, 97,
        114, 97, 109, 101, 116, 101, 114, 115, 44, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101,
        32, 78, 117, 109, 98, 101, 114, 32, 105, 110, 100, 105, 99, 101, 115, 32, 119, 105, 116,
        104, 111, 117, 116, 32, 110, 97, 114, 114, 111, 119, 105, 110, 103, 32, 98, 101, 102, 111,
        114, 101, 32, 98, 111, 117, 110, 100, 115, 44, 32, 114, 101, 116, 97, 105, 110, 32, 116,
        104, 101, 32, 115, 97, 109, 101, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 105, 100,
        101, 110, 116, 105, 116, 121, 32, 97, 110, 100, 32, 111, 114, 105, 103, 105, 110, 97, 108,
        32, 101, 108, 101, 109, 101, 110, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 115,
        44, 32, 97, 110, 100, 32, 99, 111, 112, 121, 32, 111, 118, 101, 114, 108, 97, 112, 112,
        105, 110, 103, 32, 114, 97, 110, 103, 101, 115, 32, 105, 110, 32, 116, 104, 101, 32, 114,
        101, 113, 117, 105, 114, 101, 100, 32, 100, 105, 114, 101, 99, 116, 105, 111, 110, 59, 32,
        115, 112, 108, 105, 99, 101, 32, 114, 101, 116, 117, 114, 110, 115, 32, 97, 32, 102, 114,
        101, 115, 104, 32, 114, 101, 109, 111, 118, 101, 100, 32, 97, 114, 114, 97, 121, 46,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_state_method_expression(
    target: &self::ml_string::Text,
    operation: &self::ml_string::Text,
    receiver: &self::ml_string::Text,
    arguments_: &[self::ml_string::Text],
    profiles: &[self::ml_string::Text],
    types: &[self::ml_string::Text],
    suffix: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (self::read_array_state_method_result(operation))
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
                    109, 108, 95, 97, 114, 114, 97, 121, 95, 111, 119, 110, 101, 114,
                ]))
                .concat(suffix);
                {
                    let output =
                        ((((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32]))
                            .concat(&owner))
                        .concat(&self::ml_string::Text::from_units(vec![32, 61, 32])))
                        .concat(receiver))
                        .concat(&self::ml_string::Text::from_units(vec![59, 32]));
                    {
                        let index = 0f64;
                        {
                            let output_2 = self::source_ml_source1_temporary_render_array_state_method_expression_loop10(arguments_, types, suffix, output, index);
                            {
                                let names = Vec::<self::ml_string::Text>::new();
                                {
                                    let index_2 = 0f64;
                                    {
                                        let names_2 = self::source_ml_source1_temporary_render_array_state_method_expression_loop11(arguments_, suffix, names, index_2);
                                        {
                                            let output_3 = output_2.concat(
                                                &((self::ml_string::Text::from_units(vec![
                                                    108, 101, 116, 32, 109, 108, 95, 97, 114, 114,
                                                    97, 121, 95, 108, 101, 110, 103, 116, 104, 32,
                                                    61, 32,
                                                ]))
                                                .concat(&owner))
                                                .concat(&self::ml_string::Text::from_units(vec![
                                                    46, 98, 111, 114, 114, 111, 119, 40, 41, 46,
                                                    108, 101, 110, 40, 41, 59, 32,
                                                ])),
                                            );
                                            if (*operation)
                                                == (self::ml_string::Text::from_units(vec![
                                                    115, 112, 108, 105, 99, 101,
                                                ]))
                                            {
                                                {
                                                    let begin = if (arguments_.len() as f64) == 0f64
                                                    {
                                                        self::ml_string::Text::from_units(vec![
                                                            48, 102, 54, 52,
                                                        ])
                                                    } else {
                                                        self::render_array_state_numeric_value(
                                                            &((self::ml_array::optional_at(
                                                                profiles,
                                                                self::ml_array::number_index(0f64),
                                                            ))
                                                            .unwrap_or_else(|| {
                                                                self::ml_string::Text::from_units(
                                                                    vec![],
                                                                )
                                                            })),
                                                            &((self::ml_array::optional_at(
                                                                &names_2,
                                                                self::ml_array::number_index(0f64),
                                                            ))
                                                            .unwrap_or_else(|| {
                                                                self::ml_string::Text::from_units(
                                                                    vec![],
                                                                )
                                                            })),
                                                            &(self::ml_string::Text::from_units(
                                                                vec![
                                                                    102, 54, 52, 58, 58, 78, 65, 78,
                                                                ],
                                                            )),
                                                        )
                                                    };
                                                    {
                                                        let output_4 = output_3.concat(&((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 61, 32])).concat(&self::render_array_state_relative_index(&(begin), &(self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104]))))).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                                                        {
                                                            let count = if (arguments_.len() as f64)
                                                                == 0f64
                                                            {
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        48, 117, 115, 105, 122, 101,
                                                                    ],
                                                                )
                                                            } else if (arguments_.len() as f64)
                                                                == 1f64
                                                            {
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        109, 108, 95, 97, 114, 114,
                                                                        97, 121, 95, 108, 101, 110,
                                                                        103, 116, 104, 32, 45, 32,
                                                                        109, 108, 95, 97, 114, 114,
                                                                        97, 121, 95, 115, 116, 97,
                                                                        114, 116,
                                                                    ],
                                                                )
                                                            } else {
                                                                ((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 108, 95, 100, 101, 108, 101, 116, 101, 95, 118, 97, 108, 117, 101, 58, 32, 102, 54, 52, 32, 61, 32])).concat(&self::render_array_state_numeric_value(&((self::ml_array::optional_at(profiles, self::ml_array::number_index(1f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &((self::ml_array::optional_at(&names_2, self::ml_array::number_index(1f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &(self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78]))))).concat(&self::ml_string::Text::from_units(vec![59, 32, 109, 108, 95, 100, 101, 108, 101, 116, 101, 95, 118, 97, 108, 117, 101, 46, 116, 114, 117, 110, 99, 40, 41, 46, 109, 97, 120, 40, 48, 102, 54, 52, 41, 46, 109, 105, 110, 40, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 45, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 41, 32, 97, 115, 32, 102, 54, 52, 41, 32, 97, 115, 32, 117, 115, 105, 122, 101, 32, 125]))
                                                            };
                                                            {
                                                                let inserted = Vec::<
                                                                    self::ml_string::Text,
                                                                >::new(
                                                                );
                                                                {
                                                                    let index_3 = 2f64;
                                                                    {
                                                                        let inserted_2 = self::source_ml_source1_temporary_render_array_state_method_expression_loop12(arguments_, &(names_2), inserted, index_3);
                                                                        (((((((((output_4.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 61, 32]))).concat(&count)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 115, 101, 114, 116, 101, 100, 32, 61, 32, 118, 101, 99, 33, 91]))).concat(&self::ml_string::join(&inserted_2, &self::ml_string::Text::from_units(vec![44, 32])))).concat(&self::ml_string::Text::from_units(vec![93, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 95, 32, 61, 32, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 45, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 41, 46, 99, 104, 101, 99, 107, 101, 100, 95, 97, 100, 100, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 115, 101, 114, 116, 101, 100, 46, 108, 101, 110, 40, 41, 41, 46, 102, 105, 108, 116, 101, 114, 40, 124, 108, 101, 110, 103, 116, 104, 124, 32, 42, 108, 101, 110, 103, 116, 104, 32, 60, 61, 32, 52, 50, 57, 52, 57, 54, 55, 50, 57, 53, 117, 115, 105, 122, 101, 41, 46, 101, 120, 112, 101, 99, 116, 40, 34, 73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 34, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 114, 101, 109, 111, 118, 101, 100, 58, 32, 86, 101, 99, 60, 95, 62, 32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 46, 115, 112, 108, 105, 99, 101, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 44, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 115, 101, 114, 116, 101, 100, 41, 46, 99, 111, 108, 108, 101, 99, 116, 40, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![115, 116, 100, 58, 58, 114, 99, 58, 58, 82, 99, 58, 58, 110, 101, 119, 40, 115, 116, 100, 58, 58, 99, 101, 108, 108, 58, 58, 82, 101, 102, 67, 101, 108, 108, 58, 58, 110, 101, 119, 40, 109, 108, 95, 114, 101, 109, 111, 118, 101, 100, 41, 41, 32, 125]))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let first = if (*operation)
                                                        == (self::ml_string::Text::from_units(
                                                            vec![102, 105, 108, 108],
                                                        )) {
                                                        1f64
                                                    } else {
                                                        0f64
                                                    };
                                                    {
                                                        let start_value = if (arguments_.len()
                                                            as f64)
                                                            <= first
                                                        {
                                                            self::ml_string::Text::from_units(vec![
                                                                48, 102, 54, 52,
                                                            ])
                                                        } else {
                                                            self::render_array_state_numeric_value(&((self::ml_array::optional_at(profiles, self::ml_array::number_index(first))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &((self::ml_array::optional_at(&names_2, self::ml_array::number_index(first))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &(self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78])))
                                                        };
                                                        {
                                                            let output_5 = output_3.concat(&((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 61, 32])).concat(&self::render_array_state_relative_index(&(start_value), &(self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104]))))).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                                                            {
                                                                let end_value = if (arguments_.len()
                                                                    as f64)
                                                                    <= 2f64
                                                                {
                                                                    self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 97, 115, 32, 102, 54, 52])
                                                                } else {
                                                                    self::render_array_state_numeric_value(&((self::ml_array::optional_at(profiles, self::ml_array::number_index(2f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &((self::ml_array::optional_at(&names_2, self::ml_array::number_index(2f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &(self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 97, 115, 32, 102, 54, 52])))
                                                                };
                                                                {
                                                                    let end = ((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 32, 61, 32])).concat(&self::render_array_state_relative_index(&(end_value), &(self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104]))))).concat(&self::ml_string::Text::from_units(vec![59, 32]));
                                                                    if (*operation) == (self::ml_string::Text::from_units(vec![102, 105, 108, 108])) {
                                                                            ((((((((output_5.concat(&end)).concat(&self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 117, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![105, 102, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 60, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 32, 123, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 93, 46, 102, 105, 108, 108, 40]))).concat(&(self::ml_array::optional_at(&names_2, self::ml_array::number_index(0f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![41, 59, 32, 125, 32, 125, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                                                                        } else {
                                                                            {
                                                                                let from_value = if (arguments_.len() as f64) <= 1f64 {
                                                                                    self::ml_string::Text::from_units(vec![48, 102, 54, 52])
                                                                                } else {
                                                                                    self::render_array_state_numeric_value(&((self::ml_array::optional_at(profiles, self::ml_array::number_index(1f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &((self::ml_array::optional_at(&names_2, self::ml_array::number_index(1f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))), &(self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78])))
                                                                                };
                                                                                (((((((((((output_5.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 61, 32]))).concat(&self::render_array_state_relative_index(&(from_value), &(self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104]))))).concat(&self::ml_string::Text::from_units(vec![59, 32]))).concat(&end)).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 61, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 46, 115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 115, 117, 98, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 41, 46, 109, 105, 110, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 45, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 117, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32, 105, 102, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 60, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 38, 38, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 60, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 123, 32]))).concat(&self::ml_string::Text::from_units(vec![102, 111, 114, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 32, 105, 110, 32, 40, 48, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 41, 46, 114, 101, 118, 40, 41, 32, 123, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 32, 61, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 46, 99, 108, 111, 110, 101, 40, 41, 59, 32, 125, 32]))).concat(&self::ml_string::Text::from_units(vec![125, 32, 101, 108, 115, 101, 32, 123, 32, 102, 111, 114, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 32, 105, 110, 32, 48, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 123, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 32, 61, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 46, 99, 108, 111, 110, 101, 40, 41, 59, 32, 125, 32, 125, 32, 125, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
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
pub fn read_collection_access_result(
    kind: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*kind) != (self::ml_string::Text::from_units(vec![109, 97, 112])))
        && ((*kind) != (self::ml_string::Text::from_units(vec![115, 101, 116])))
    {
        self::ml_string::Text::from_units(vec![])
    } else if (*operation) == (self::ml_string::Text::from_units(vec![104, 97, 115])) {
        self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
    } else if ((*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112])))
        && ((*operation) == (self::ml_string::Text::from_units(vec![103, 101, 116])))
    {
        self::ml_string::Text::from_units(vec![
            111, 112, 116, 105, 111, 110, 97, 108, 32, 118, 97, 108, 117, 101,
        ])
    } else if (*operation) == (self::ml_string::Text::from_units(vec![107, 101, 121, 115])) {
        self::ml_string::Text::from_units(vec![107, 101, 121, 115])
    } else if (*operation) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))
    {
        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))
    {
        self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_access_argument_count(operation: &self::ml_string::Text) -> f64 {
    if ((*operation) == (self::ml_string::Text::from_units(vec![103, 101, 116])))
        || ((*operation) == (self::ml_string::Text::from_units(vec![104, 97, 115])))
    {
        1f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_collection_size_expression(
    target: &self::ml_string::Text,
    object: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*target)
        == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))
    {
        ((self::ml_string::Text::from_units(vec![40])).concat(object)).concat(
            &self::ml_string::Text::from_units(vec![41, 46, 115, 105, 122, 101]),
        )
    } else if (*target) == (self::ml_string::Text::from_units(vec![82, 117, 115, 116])) {
        ((self::ml_string::Text::from_units(vec![40])).concat(object)).concat(
            &self::ml_string::Text::from_units(vec![
                46, 108, 101, 110, 40, 41, 32, 97, 115, 32, 102, 54, 52, 41,
            ]),
        )
    } else if (*target) == (self::ml_string::Text::from_units(vec![76, 101, 97, 110])) {
        ((self::ml_string::Text::from_units(vec![
            40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32,
        ]))
        .concat(object))
        .concat(&self::ml_string::Text::from_units(vec![
            46, 115, 105, 122, 101, 41,
        ]))
    } else if (*target) == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])) {
        ((self::ml_string::Text::from_units(vec![
            40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 111, 102, 95, 117, 105, 110, 116, 54,
            51, 32, 40, 85, 105, 110, 116, 54, 51, 46, 111, 102, 95, 90, 32, 40, 90, 46, 111, 102,
            95, 110, 97, 116, 32, 40, 76, 105, 115, 116, 46, 108, 101, 110, 103, 116, 104, 32,
        ]))
        .concat(object))
        .concat(&self::ml_string::Text::from_units(vec![41, 41, 41, 41]))
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}
