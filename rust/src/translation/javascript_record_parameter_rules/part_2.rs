// Generated from complete js/src/translation/javascript-record-parameter-rules.js. Do not edit by hand.
use super::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Result;
use super::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Result;
use super::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result;
use super::MlSource1TemporaryReadArrayBindingFactFieldLoop5Result;
use super::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Result;
use super::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop12Result;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::source_ml_source1_temporary_accept_java_script_array_binding_name_loop4;
use super::source_ml_source1_temporary_accept_java_script_source_binding_name_loop3;
use super::source_ml_source1_temporary_plan_java_script_array_binding_names_loop8;
use super::source_ml_source1_temporary_read_array_binding_fact_field_loop5;
use super::source_ml_source1_temporary_read_array_binding_initializer_end_loop6;
use super::source_ml_source1_temporary_read_java_script_binding_default_term_loop12;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_source_cell_parameter(
    prefix: &self::ml_string::Text,
    name: &self::ml_string::Text,
    reference: bool,
) -> self::ml_string::Text {
    if reference {
        (prefix.concat(&self::ml_string::Text::from_units(vec![
            99, 101, 108, 108, 95,
        ])))
        .concat(name)
    } else {
        name.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_source_binding_name(
    kind: &self::ml_string::Text,
    name: &self::ml_string::Text,
    duplicate: bool,
    reserved: &[self::ml_string::Text],
) -> bool {
    if (((*kind)
        != (self::ml_string::Text::from_units(vec![
            105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
        ])))
        || ((*name) == (self::ml_string::Text::from_units(vec![]))))
        || duplicate
    {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s36 =
                    self::source_ml_source1_temporary_accept_java_script_source_binding_name_loop3(
                        name, reserved, index,
                    );
                match source_ml_source2_s36 {
                    self::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Result::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Done => {
                        true
                    }
                    self::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Result::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Return(source_ml_source1_temporary_result3) => {
                        source_ml_source1_temporary_result3
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (self::ml_string::Text::from_units(vec![
            109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78,
            117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118,
            97, 108, 117, 101,
        ]))
        .0,
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> self::ml_string::Text {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (self::ml_string::Text::from_units(vec![
            109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116,
            101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108,
            117, 101,
        ]))
        .0,
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (self::ml_string::Text::from_units(vec![
            109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66,
            111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32,
            118, 97, 108, 117, 101,
        ]))
        .0,
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
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32,
                78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110,
                32, 112, 111, 115, 105, 116, 105, 111, 110,
            ]))
            .0,
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
pub fn read_required_text_fact(
    values: &[self::ml_string::Text],
    index: f64,
) -> self::ml_string::Text {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32,
                116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112,
                111, 115, 105, 116, 105, 111, 110,
            ]))
            .0,
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
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32,
                66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109,
                110, 32, 112, 111, 115, 105, 116, 105, 111, 110,
            ]))
            .0,
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
pub fn read_java_script_array_binding_form(token: &self::ml_string::Text) -> self::ml_string::Text {
    if (*token) == (self::ml_string::Text::from_units(vec![93])) {
        self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])
    } else if (*token) == (self::ml_string::Text::from_units(vec![44])) {
        self::ml_string::Text::from_units(vec![101, 108, 105, 115, 105, 111, 110])
    } else if (*token) == (self::ml_string::Text::from_units(vec![46, 46, 46])) {
        self::ml_string::Text::from_units(vec![114, 101, 115, 116])
    } else if (*token) == (self::ml_string::Text::from_units(vec![91])) {
        self::ml_string::Text::from_units(vec![110, 101, 115, 116, 101, 100])
    } else {
        self::ml_string::Text::from_units(vec![110, 97, 109, 101])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_rest_binding_error(
    following: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*following) == (self::ml_string::Text::from_units(vec![61])) {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 97, 114, 114, 97, 121, 32, 114, 101, 115, 116, 32, 98, 105, 110, 100, 105,
            110, 103, 32, 99, 97, 110, 110, 111, 116, 32, 104, 97, 118, 101, 32, 97, 32, 100, 101,
            102, 97, 117, 108, 116,
        ])
    } else if (*following) == (self::ml_string::Text::from_units(vec![44])) {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 97, 114, 114, 97, 121, 32, 114, 101, 115, 116, 32, 98, 105, 110, 100, 105,
            110, 103, 32, 109, 117, 115, 116, 32, 98, 101, 32, 108, 97, 115, 116, 32, 119, 105,
            116, 104, 111, 117, 116, 32, 97, 32, 116, 114, 97, 105, 108, 105, 110, 103, 32, 99,
            111, 109, 109, 97,
        ])
    } else if (*following) == (self::ml_string::Text::from_units(vec![93])) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 97, 114, 114, 97, 121, 32, 114, 101, 115, 116, 32, 98, 105, 110, 100, 105,
            110, 103, 32, 110, 101, 101, 100, 115, 32, 116, 104, 101, 32, 99, 108, 111, 115, 105,
            110, 103, 32, 98, 114, 97, 99, 107, 101, 116,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_array_binding_name(
    kind: &self::ml_string::Text,
    name: &self::ml_string::Text,
    duplicate: bool,
    reserved: &[self::ml_string::Text],
) -> bool {
    if (((*kind)
        != (self::ml_string::Text::from_units(vec![
            105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
        ])))
        || ((*name) == (self::ml_string::Text::from_units(vec![]))))
        || duplicate
    {
        false
    } else if ((name.length()) >= 3f64)
        && (({
            let source_ml_source2_string_range_operand42 = name.clone();
            {
                let source_ml_source2_string_range_operand43 = 0f64;
                {
                    let source_ml_source2_string_range_operand44 = 3f64;
                    {
                        let source_ml_source2_string_range_position45 =
                            source_ml_source2_string_range_operand43;
                        {
                            let source_ml_source2_string_range_position46 =
                                source_ml_source2_string_range_operand44;
                            source_ml_source2_string_range_operand42.slice(
                                source_ml_source2_string_range_position45,
                                source_ml_source2_string_range_position46,
                            )
                        }
                    }
                }
            }
        }) == (self::ml_string::Text::from_units(vec![109, 108, 95])))
    {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s47 =
                    self::source_ml_source1_temporary_accept_java_script_array_binding_name_loop4(
                        name, reserved, index,
                    );
                match source_ml_source2_s47 {
                        self::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Result::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Result::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Return(source_ml_source1_temporary_result4) => {
                            source_ml_source1_temporary_result4
                        }
                    }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_binding_value_form(
    rest: bool,
    default_value: bool,
) -> self::ml_string::Text {
    if rest {
        self::ml_string::Text::from_units(vec![115, 117, 102, 102, 105, 120])
    } else if default_value {
        self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])
    } else {
        self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_array_rest_receiver(kind: &self::ml_string::Text) -> bool {
    (*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_java_script_array_binding_offset(
    literal: bool,
    safe_integer: bool,
    negative: bool,
) -> bool {
    (literal && safe_integer) && !negative
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_array_binding_offset_field(field: &self::ml_string::Text) -> bool {
    ((((*field) == (self::ml_string::Text::from_units(vec![107])))
        || ((*field) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))))
        || ((*field)
            == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 105, 118, 101]))))
        || ((*field) == (self::ml_string::Text::from_units(vec![115, 112, 97, 110])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_array_binding_field(
    kind: &self::ml_string::Text,
    field: &self::ml_string::Text,
) -> bool {
    if ((*kind)
        != (self::ml_string::Text::from_units(vec![
            97, 114, 114, 97, 121, 66, 105, 110, 100, 105, 110, 103, 82, 101, 115, 116,
        ])))
        && ((*kind)
            != (self::ml_string::Text::from_units(vec![
                97, 114, 114, 97, 121, 66, 105, 110, 100, 105, 110, 103, 68, 101, 102, 97, 117,
                108, 116,
            ])))
    {
        false
    } else if (((*field) == (self::ml_string::Text::from_units(vec![107])))
        || ((*field) == (self::ml_string::Text::from_units(vec![115, 112, 97, 110]))))
        || ((*field)
            == (self::ml_string::Text::from_units(vec![
                112, 97, 114, 101, 110, 116, 104, 101, 115, 105, 122, 101, 100,
            ])))
    {
        true
    } else if (*kind)
        == (self::ml_string::Text::from_units(vec![
            97, 114, 114, 97, 121, 66, 105, 110, 100, 105, 110, 103, 82, 101, 115, 116,
        ]))
    {
        ((*field) == (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])))
            || ((*field) == (self::ml_string::Text::from_units(vec![115, 116, 97, 114, 116])))
    } else {
        ((*kind)
            == (self::ml_string::Text::from_units(vec![
                97, 114, 114, 97, 121, 66, 105, 110, 100, 105, 110, 103, 68, 101, 102, 97, 117,
                108, 116,
            ])))
            && (((*field) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))
                || ((*field)
                    == (self::ml_string::Text::from_units(vec![
                        102, 97, 108, 108, 98, 97, 99, 107,
                    ]))))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_default_operation(
    language: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    absence: &self::ml_string::Text,
    element: &self::ml_string::Text,
    same_fallback: bool,
) -> self::ml_string::Text {
    {
        let ml_library_inequality_1 = (*language)
            != (self::ml_string::Text::from_units(vec![
                74, 97, 118, 97, 83, 99, 114, 105, 112, 116,
            ]));
        if ml_library_inequality_1 {
            self::ml_string::Text::from_units(vec![
                114, 101, 102, 117, 115, 101, 45, 108, 97, 110, 103, 117, 97, 103, 101,
            ])
        } else if ((*kind)
            != (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
            || ((*absence)
                != (self::ml_string::Text::from_units(vec![
                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                ])))
        {
            self::ml_string::Text::from_units(vec![
                114, 101, 102, 117, 115, 101, 45, 97, 98, 115, 101, 110, 99, 101,
            ])
        } else if ((((((*element)
            != (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))
            && ((*element) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))))
            && ((*element)
                != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))
            && ((*element) != (self::ml_string::Text::from_units(vec![105, 110, 116]))))
            && ((*element) != (self::ml_string::Text::from_units(vec![110, 97, 116]))))
            && ((*element) != (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
        {
            self::ml_string::Text::from_units(vec![
                114, 101, 102, 117, 115, 101, 45, 101, 108, 101, 109, 101, 110, 116,
            ])
        } else if same_fallback {
            self::ml_string::Text::from_units(vec![99, 111, 97, 108, 101, 115, 99, 101])
        } else {
            self::ml_string::Text::from_units(vec![
                114, 101, 102, 117, 115, 101, 45, 102, 97, 108, 108, 98, 97, 99, 107,
            ])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_java_script_array_default_inference(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    resolved: f64,
    fallback: f64,
    result: f64,
) -> Vec<f64> {
    {
        let kind = self::read_array_binding_fact_field(
            starts,
            counts,
            labels,
            targets,
            resolved,
            &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
        );
        if kind < 0f64 {
            vec![-1f64]
        } else {
            {
                let value = self::read_required_text_fact(texts, kind);
                if value == (self::ml_string::Text::from_units(vec![118, 97, 114])) {
                    Vec::<f64>::new()
                } else {
                    {
                        let absence = self::read_array_binding_fact_field(
                            starts,
                            counts,
                            labels,
                            targets,
                            resolved,
                            &(self::ml_string::Text::from_units(vec![
                                97, 98, 115, 101, 110, 99, 101,
                            ])),
                        );
                        if ((value
                            != (self::ml_string::Text::from_units(vec![
                                111, 112, 116, 105, 111, 110,
                            ])))
                            || (absence < 0f64))
                            || ((self::read_required_text_fact(texts, absence))
                                != (self::ml_string::Text::from_units(vec![
                                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                                ])))
                        {
                            vec![-1f64]
                        } else {
                            {
                                let element = self::read_array_binding_fact_field(
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    resolved,
                                    &(self::ml_string::Text::from_units(vec![
                                        101, 108, 101, 109, 101, 110, 116,
                                    ])),
                                );
                                if element < 0f64 {
                                    vec![-1f64]
                                } else {
                                    vec![element, fallback, result, element]
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
pub fn read_array_binding_fact_field(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    field: &self::ml_string::Text,
) -> f64 {
    if ((node < 0f64) || (node >= (starts.len() as f64))) || (node >= (counts.len() as f64)) {
        -1f64
    } else {
        {
            let at = self::read_required_number_fact(starts, node);
            {
                let end = at + self::read_required_number_fact(counts, node);
                {
                    let source_ml_source2_s48 =
                        self::source_ml_source1_temporary_read_array_binding_fact_field_loop5(
                            labels, targets, field, at, end,
                        );
                    match source_ml_source2_s48 {
                        self::MlSource1TemporaryReadArrayBindingFactFieldLoop5Result::MlSource1TemporaryReadArrayBindingFactFieldLoop5Done => {
                            -1f64
                        }
                        self::MlSource1TemporaryReadArrayBindingFactFieldLoop5Result::MlSource1TemporaryReadArrayBindingFactFieldLoop5Return(source_ml_source1_temporary_result5) => {
                            source_ml_source1_temporary_result5
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_binding_initializer_end(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
) -> f64 {
    {
        let index = start;
        {
            let closes = Vec::<self::ml_string::Text>::new();
            {
                let source_ml_source2_s49 =
                    self::source_ml_source1_temporary_read_array_binding_initializer_end_loop6(
                        kinds, values, start, index, closes,
                    );
                match source_ml_source2_s49 {
                    self::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Result::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Done => {
                        -1f64
                    }
                    self::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Result::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Return(source_ml_source1_temporary_result6) => {
                        source_ml_source1_temporary_result6
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_array_binding_names(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
) -> Vec<f64> {
    if ((((start < 0f64) || ((start % 1f64) != 0f64)) || (start >= (kinds.len() as f64)))
        || (start >= (values.len() as f64)))
        || (((self::read_required_text_fact(kinds, start))
            != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
            || ((self::read_required_text_fact(values, start))
                != (self::ml_string::Text::from_units(vec![91]))))
    {
        vec![-1f64]
    } else {
        {
            let index = start + 1f64;
            {
                let positions = Vec::<f64>::new();
                {
                    let source_ml_source2_s50 = self::source_ml_source1_temporary_plan_java_script_array_binding_names_loop8(kinds, values, index, positions);
                    match source_ml_source2_s50 {
                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Done => {
                                vec![-1f64]
                            }
                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(source_ml_source1_temporary_result8) => {
                                source_ml_source1_temporary_result8
                            }
                        }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_binding_default_form(
    language: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    absence: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let ml_library_inequality_1 = (*language)
            != (self::ml_string::Text::from_units(vec![
                74, 97, 118, 97, 83, 99, 114, 105, 112, 116,
            ]));
        if ml_library_inequality_1 {
            self::ml_string::Text::from_units(vec![
                114, 101, 102, 117, 115, 101, 45, 108, 97, 110, 103, 117, 97, 103, 101,
            ])
        } else if ((*kind) == (self::ml_string::Text::from_units(vec![118, 97, 114])))
            || ((*kind)
                == (self::ml_string::Text::from_units(vec![108, 105, 116, 101, 114, 97, 108])))
        {
            self::ml_string::Text::from_units(vec![100, 101, 102, 101, 114])
        } else if ((*kind)
            == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
            && ((*absence) != (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))
        {
            self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116])
        } else {
            self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_binding_default_result_absence(
    left: &self::ml_string::Text,
    right_kind: &self::ml_string::Text,
    right: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let null_value = (((*left)
            == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))
            || ((*left) == (self::ml_string::Text::from_units(vec![98, 111, 116, 104]))))
            || (((*right_kind)
                == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
                && (((*right) == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))
                    || ((*right) == (self::ml_string::Text::from_units(vec![98, 111, 116, 104])))));
        {
            let undefined_value = ((*right_kind)
                == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))
                || (((*right_kind)
                    == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
                    && (((*right)
                        == (self::ml_string::Text::from_units(vec![
                            117, 110, 100, 101, 102, 105, 110, 101, 100,
                        ])))
                        || ((*right)
                            == (self::ml_string::Text::from_units(vec![98, 111, 116, 104])))));
            if null_value {
                if undefined_value {
                    self::ml_string::Text::from_units(vec![98, 111, 116, 104])
                } else {
                    self::ml_string::Text::from_units(vec![110, 117, 108, 108])
                }
            } else if undefined_value {
                self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_binding_default_absence_code(absence: &self::ml_string::Text) -> f64 {
    if (*absence) == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])) {
        1f64
    } else if (*absence)
        == (self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))
    {
        2f64
    } else if (*absence) == (self::ml_string::Text::from_units(vec![98, 111, 116, 104])) {
        3f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_binding_default_absence_name(code: f64) -> self::ml_string::Text {
    if code == 1f64 {
        self::ml_string::Text::from_units(vec![110, 117, 108, 108])
    } else if code == 2f64 {
        self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
    } else if code == 3f64 {
        self::ml_string::Text::from_units(vec![98, 111, 116, 104])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_binding_default_inference(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    value: f64,
    fallback: f64,
) -> Vec<f64> {
    {
        let kind = self::read_array_binding_fact_field(
            starts,
            counts,
            labels,
            targets,
            value,
            &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
        );
        if kind < 0f64 {
            vec![-1f64]
        } else if (self::read_required_text_fact(texts, kind))
            == (self::ml_string::Text::from_units(vec![118, 97, 114]))
        {
            Vec::<f64>::new()
        } else {
            {
                let left = self::read_java_script_binding_default_term(
                    texts, starts, counts, labels, targets, value,
                );
                {
                    let right = self::read_java_script_binding_default_term(
                        texts, starts, counts, labels, targets, fallback,
                    );
                    {
                        let element = self::read_required_number_fact(&(left), 0f64);
                        {
                            let other = self::read_required_number_fact(&(right), 0f64);
                            if (element < 0f64) || (other < 0f64) {
                                vec![-1f64]
                            } else {
                                {
                                    let code = self::read_required_number_fact(&(left), 1f64);
                                    {
                                        let right_code =
                                            self::read_required_number_fact(&(right), 1f64);
                                        if code == 0f64 {
                                            vec![0f64, value]
                                        } else if code == 1f64 {
                                            vec![1f64, element]
                                        } else {
                                            {
                                                let first_kind =
                                                    self::read_array_binding_fact_field(
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        element,
                                                        &(self::ml_string::Text::from_units(vec![
                                                            107, 105, 110, 100,
                                                        ])),
                                                    );
                                                {
                                                    let other_kind =
                                                        self::read_array_binding_fact_field(
                                                            starts,
                                                            counts,
                                                            labels,
                                                            targets,
                                                            other,
                                                            &(self::ml_string::Text::from_units(
                                                                vec![107, 105, 110, 100],
                                                            )),
                                                        );
                                                    if (first_kind < 0f64) || (other_kind < 0f64) {
                                                        vec![-1f64]
                                                    } else {
                                                        {
                                                            let first_unit = (self::read_required_text_fact(texts, first_kind)) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]));
                                                            {
                                                                let second_unit = (self::read_required_text_fact(texts, other_kind)) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]));
                                                                {
                                                                    let output = self::read_java_script_binding_default_absence_code(&(self::read_java_script_binding_default_result_absence(&(self::read_java_script_binding_default_absence_name(code)), &(if right_code == 0f64 {
                                                                                self::read_required_text_fact(texts, other_kind)
                                                                            } else {
                                                                                self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])
                                                                            }), &(self::read_java_script_binding_default_absence_name(right_code)))));
                                                                    {
                                                                        let base = if first_unit {
                                                                            other
                                                                        } else {
                                                                            element
                                                                        };
                                                                        if first_unit || second_unit
                                                                        {
                                                                            vec![output, base]
                                                                        } else {
                                                                            vec![
                                                                                output, base,
                                                                                element, other,
                                                                            ]
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
pub fn read_java_script_binding_default_term(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    value: f64,
) -> Vec<f64> {
    {
        let node = value;
        {
            let absence = self::ml_string::Text::from_units(vec![]);
            {
                let kind = self::read_array_binding_fact_field(
                    starts,
                    counts,
                    labels,
                    targets,
                    node,
                    &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
                );
                {
                    let source_ml_source2_s51 = self::source_ml_source1_temporary_read_java_script_binding_default_term_loop12(texts, starts, counts, labels, targets, node, absence, kind);
                    match source_ml_source2_s51 {
                        self::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop12Result::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop12Done(node_2, absence_2, kind_2) => {
                            if kind_2 < 0f64 {
                                vec![-1f64, 0f64]
                            } else {
                                vec![node_2, self::read_java_script_binding_default_absence_code(&(absence_2))]
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
pub fn read_java_script_binding_default_lift_form(
    kind: &self::ml_string::Text,
    element: &self::ml_string::Text,
    absence: &self::ml_string::Text,
    drop_undefined: bool,
) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])) {
        self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
    } else {
        let ml_library_inequality_2 =
            (*kind) != (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]));
        if ml_library_inequality_2 {
            self::ml_string::Text::from_units(vec![112, 114, 101, 115, 101, 110, 116])
        } else {
            let ml_library_inequality_1 =
                (*element) != (self::ml_string::Text::from_units(vec![117, 110, 105, 116]));
            if ml_library_inequality_1 {
                self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
            } else if drop_undefined {
                self::ml_string::Text::from_units(vec![110, 117, 108, 108])
            } else if (*absence) == (self::ml_string::Text::from_units(vec![98, 111, 116, 104])) {
                self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 99, 101, 115])
            } else {
                absence.clone()
            }
        }
    }
}
