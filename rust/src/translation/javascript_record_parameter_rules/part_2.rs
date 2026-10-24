// Generated from complete js/src/translation/javascript-record-parameter-rules.js. Do not edit by hand.
use super::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Result;
use super::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Result;
use super::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result;
use super::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result;
use super::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Result;
use super::MlSource1TemporaryReadArrayBindingFactFieldLoop5Result;
use super::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Result;
use super::accept_java_script_source_binding_name;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::refuse_required_boolean_fact;
use super::source_ml_source1_temporary_accept_java_script_array_binding_name_loop4;
use super::source_ml_source1_temporary_accept_java_script_record_parameter_binding_loop13;
use super::source_ml_source1_temporary_accept_java_script_record_parameter_bound_name_loop14;
use super::source_ml_source1_temporary_find_java_script_record_parameter_pattern_loop12;
use super::source_ml_source1_temporary_find_java_script_record_parameter_strict_directive_loop19;
use super::source_ml_source1_temporary_plan_java_script_array_binding_names_loop8;
use super::source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop15;
use super::source_ml_source1_temporary_read_array_binding_fact_field_loop5;
use super::source_ml_source1_temporary_read_array_binding_initializer_end_loop6;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required Boolean fact column position"
        );

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
    if ((((*kind)
        != (self::ml_string::Text::from_units(vec![
            105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
        ])))
        || ((*name) == (self::ml_string::Text::from_units(vec![]))))
        || duplicate)
        || (((name.length()) >= 3f64)
            && ((name.slice(0f64, 3f64))
                == (self::ml_string::Text::from_units(vec![109, 108, 95]))))
    {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s3 =
                    self::source_ml_source1_temporary_accept_java_script_array_binding_name_loop4(
                        name, reserved, index,
                    );
                match source_ml_source2_s3 {
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
                    let source_ml_source2_s4 =
                        self::source_ml_source1_temporary_read_array_binding_fact_field_loop5(
                            labels, targets, field, at, end,
                        );
                    match source_ml_source2_s4 {
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
                let source_ml_source2_s5 =
                    self::source_ml_source1_temporary_read_array_binding_initializer_end_loop6(
                        kinds, values, start, index, closes,
                    );
                match source_ml_source2_s5 {
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
                    let source_ml_source2_s6 = self::source_ml_source1_temporary_plan_java_script_array_binding_names_loop8(kinds, values, index, positions);
                    match source_ml_source2_s6 {
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
pub fn find_java_script_record_parameter_pattern(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
) -> f64 {
    if ((((kinds.len() as f64) != (values.len() as f64)) || !(self::ml_math::is_integer(start)))
        || (start < 0f64))
        || (start >= (kinds.len() as f64))
    {
        -1f64
    } else {
        {
            let depth = 0f64;
            {
                let binding = true;
                {
                    let at = start;
                    {
                        let source_ml_source2_s7 = self::source_ml_source1_temporary_find_java_script_record_parameter_pattern_loop12(kinds, values, depth, binding, at);
                        match source_ml_source2_s7 {
                            self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Done => {
                                -1f64
                            }
                            self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop12Return(source_ml_source1_temporary_result12) => {
                                source_ml_source1_temporary_result12
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
pub fn accept_java_script_record_parameter_binding(
    kind: &self::ml_string::Text,
    name: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
) -> bool {
    {
        let duplicate = false;
        {
            let index = 0f64;
            {
                let duplicate_2 = self::source_ml_source1_temporary_accept_java_script_record_parameter_binding_loop13(name, names, duplicate, index);
                self::accept_java_script_array_binding_name(kind, name, duplicate_2, reserved)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_record_parameters(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
) -> Vec<f64> {
    self::plan_java_script_record_parameter_bindings(kinds, values, start, reserved, false)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_source_record_parameters(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
) -> Vec<f64> {
    self::plan_java_script_record_parameter_bindings(kinds, values, start, reserved, true)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_parameter_bound_name(
    kind: &self::ml_string::Text,
    name: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    ordinary: bool,
) -> bool {
    if ordinary {
        {
            let duplicate = false;
            {
                let index = 0f64;
                {
                    let duplicate_2 = self::source_ml_source1_temporary_accept_java_script_record_parameter_bound_name_loop14(name, names, duplicate, index);
                    self::accept_java_script_source_binding_name(kind, name, duplicate_2, reserved)
                }
            }
        }
    } else {
        self::accept_java_script_record_parameter_binding(kind, name, names, reserved)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_record_parameter_bindings(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
    ordinary: bool,
) -> Vec<f64> {
    if ((((kinds.len() as f64) != (values.len() as f64)) || !(self::ml_math::is_integer(start)))
        || (start < 0f64))
        || (start >= (kinds.len() as f64))
    {
        vec![-1f64, start]
    } else {
        {
            let at = start;
            {
                let slot = 0f64;
                {
                    let names = Vec::<self::ml_string::Text>::new();
                    {
                        let instructions = Vec::<f64>::new();
                        {
                            let source_ml_source2_s8 = self::source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop15(kinds, values, reserved, ordinary, at, slot, names, instructions);
                            match source_ml_source2_s8 {
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Done(at_2, slot_2, instructions_2) => {
                                    if (at_2 >= (kinds.len() as f64)) || !(((self::read_required_text_fact(kinds, at_2)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_2)) == (self::ml_string::Text::from_units(vec![41])))) {
                                        vec![-1f64, at_2]
                                    } else {
                                        {
                                            let source_ml_source2_concat_value9 = vec![(at_2 + 1f64), slot_2];
                                            {
                                                let source_ml_source2_concat_value10 = instructions_2;
                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value9), source_ml_source2_concat_value10)
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Return(source_ml_source1_temporary_result15) => {
                                    source_ml_source1_temporary_result15
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
pub fn accept_java_script_record_parameter_profile(
    context: &self::ml_string::Text,
    asynchronous: bool,
    documentation_parameters: f64,
    bound_writes: bool,
    variable_redeclarations: bool,
) -> bool {
    (((((*context)
        == (self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 110, 97, 109, 101, 100, 32, 102, 117, 110,
            99, 116, 105, 111, 110, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110,
        ])))
        && !asynchronous)
        && (documentation_parameters == 0f64))
        && !bound_writes)
        && !variable_redeclarations
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_standard_builtin_owner(
    name: &self::ml_string::Text,
    shadowed: bool,
) -> bool {
    (((((((((((((((((*name)
        == (self::ml_string::Text::from_units(vec![77, 97, 112])))
        || ((*name) == (self::ml_string::Text::from_units(vec![83, 101, 116]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 110, 115, 111, 108, 101,
            ]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![
                82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
            ]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![
                84, 121, 112, 101, 69, 114, 114, 111, 114,
            ]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![78, 97, 78]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![73, 110, 102, 105, 110, 105, 116, 121]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![
                117, 110, 100, 101, 102, 105, 110, 101, 100,
            ]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![105, 115, 70, 105, 110, 105, 116, 101]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![105, 115, 78, 97, 78]))))
        && !shadowed
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_record_parameter_strict_directive(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    raws: &[self::ml_string::Text],
    line_breaks: &[bool],
    start: f64,
) -> f64 {
    if ((((((kinds.len() as f64) != (values.len() as f64))
        || ((kinds.len() as f64) != (raws.len() as f64)))
        || ((kinds.len() as f64) != (line_breaks.len() as f64)))
        || !(self::ml_math::is_integer(start)))
        || (start < 0f64))
        || (start >= (kinds.len() as f64))
    {
        -2f64
    } else {
        {
            let at = start;
            {
                let source_ml_source2_s11 = self::source_ml_source1_temporary_find_java_script_record_parameter_strict_directive_loop19(kinds, values, raws, line_breaks, at);
                match source_ml_source2_s11 {
                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Done => {
                        -2f64
                    }
                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Return(source_ml_source1_temporary_result19) => {
                        source_ml_source1_temporary_result19
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continues_java_script_directive_expression(
    kind: &self::ml_string::Text,
    value: &self::ml_string::Text,
) -> bool {
    if ((*kind) == (self::ml_string::Text::from_units(vec![116, 101, 109, 112, 108, 97, 116, 101])))
        || ((*kind)
            == (self::ml_string::Text::from_units(vec![
                105, 110, 116, 101, 114, 112, 111, 108, 97, 116, 105, 111, 110,
            ])))
    {
        true
    } else if (*kind)
        == (self::ml_string::Text::from_units(vec![
            105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
        ]))
    {
        ((*value) == (self::ml_string::Text::from_units(vec![105, 110])))
            || ((*value)
                == (self::ml_string::Text::from_units(vec![
                    105, 110, 115, 116, 97, 110, 99, 101, 111, 102,
                ])))
    } else {
        let ml_library_inequality_1 =
            (*kind) != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]));
        if ml_library_inequality_1 {
            false
        } else if (((((((((((*value) == (self::ml_string::Text::from_units(vec![40])))
            || ((*value) == (self::ml_string::Text::from_units(vec![91]))))
            || ((*value) == (self::ml_string::Text::from_units(vec![46]))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![63, 46])))
                || ((*value) == (self::ml_string::Text::from_units(vec![43]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![45])))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![42])))
                || ((*value) == (self::ml_string::Text::from_units(vec![47]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![37])))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![42, 42])))
                || ((*value) == (self::ml_string::Text::from_units(vec![60, 60]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![62, 62])))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![62, 62, 62])))
                || ((*value) == (self::ml_string::Text::from_units(vec![60]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![62])))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![60, 61])))
                || ((*value) == (self::ml_string::Text::from_units(vec![62, 61]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![61, 61])))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![33, 61])))
                || ((*value) == (self::ml_string::Text::from_units(vec![61, 61, 61]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![33, 61, 61])))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![38])))
                || ((*value) == (self::ml_string::Text::from_units(vec![94]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![124])))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![38, 38])))
                || ((*value) == (self::ml_string::Text::from_units(vec![124, 124]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![63, 63]))))
        {
            true
        } else {
            ((*value) == (self::ml_string::Text::from_units(vec![63])))
                || ((*value) == (self::ml_string::Text::from_units(vec![44])))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result {
    MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Done(f64, f64),
    MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_source_identifier_prefix_ordinal_loop1(
    name: &self::ml_string::Text,
    mut index: f64,
    mut ordinal: f64,
) -> self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result {
    loop {
        return if index < (name.length()) {
            {
                let unit =
                    (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 {
                        {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if (index_2 < 0f64) || (index_2 >= (text.length())) {
                                f64::NAN
                            } else {
                                ((text.slice(index_2, (index_2 + 1f64))).code_point_at(0f64))
                                    .clone()
                                    .unwrap_or_else(|| f64::NAN)
                            }
                        }
                    }))(name.clone(), index);
                if (unit < 48f64) || (unit > 57f64) {
                    self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Done(index, ordinal)
                } else {
                    {
                        let ordinal_2 = ((ordinal * 10f64) + unit) - 48f64;
                        if (self::ml_math::is_safe_integer(ordinal_2)) {
                            {
                                let index_2 = index + 1f64;
                                {
                                    (index, ordinal) = (index_2, ordinal_2);
                                    continue;
                                }
                            }
                        }
                        {
                            self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Return(-1f64)
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Done(index, ordinal)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_source_identifier_prefix_loop2(
    ordinals: &[f64],
    mut next: f64,
    mut previous: f64,
    mut index: f64,
) -> f64 {
    loop {
        return if index < (ordinals.len() as f64) {
            {
                let ordinal =
                    (self::ml_array::optional_at(ordinals, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64);
                {
                    assert!(
                        !((!(self::ml_math::is_safe_integer(ordinal)) || (ordinal < 1f64))
                            || (ordinal < previous)),
                        "{}",
                        "source identifier ordinals need a sorted positive integer column"
                    );

                    {
                        let previous_2 = ordinal;
                        if ordinal == next {
                            {
                                let next_2 = next + 1f64;
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        (next, previous, index) = (next_2, previous_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let index_3 = index + 1f64;
                                {
                                    (previous, index) = (previous_2, index_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            next
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Result {
    MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Done,
    MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_java_script_source_binding_name_loop3(
    name: &self::ml_string::Text,
    reserved: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Result {
    loop {
        return if index < (reserved.len() as f64) {
            {
                let word =
                    self::ml_array::optional_at(reserved, self::ml_array::number_index(index));
                if match (word.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    panic!(
                        "{}",
                        "source binding reserved names need a complete text column"
                    )
                } else if match (word.clone(), name.clone()) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                } {
                    self::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Result::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Return(false)
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Result::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Done
        };
    }
}
