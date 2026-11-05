// Generated from complete js/src/translation/record-closure-parameter-rules.js. Do not edit by hand.
use super::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result;
use super::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result;
use super::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Result;
use super::accept_java_script_record_parameter_binding;
use super::accept_java_script_source_binding_name;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_array_formal_default_scope_boundary;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_accept_java_script_record_parameter_bound_name_loop28;
use super::source_ml_source1_temporary_find_java_script_record_parameter_strict_directive_loop38;
use super::source_ml_source1_temporary_plan_java_script_object_pattern_bindings_loop29;
use super::source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop32;
use super::source_ml_source1_temporary_read_java_script_object_pattern_binding_positions_loop35;
use super::source_ml_source1_temporary_read_java_script_record_formal_default_binding_positions_loop36;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_record_parameters(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
) -> Vec<f64> {
    self::plan_java_script_record_parameter_bindings(
        kinds, values, start, reserved, false, false, false,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_source_record_parameters(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
) -> Vec<f64> {
    self::plan_java_script_record_parameter_bindings(
        kinds, values, start, reserved, true, false, false,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_source_record_parameters_with_defaults(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
) -> Vec<f64> {
    self::plan_java_script_record_parameter_bindings(
        kinds, values, start, reserved, true, true, false,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_source_record_parameters_with_rest(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
) -> Vec<f64> {
    self::plan_java_script_record_parameter_bindings(
        kinds, values, start, reserved, true, true, true,
    )
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
                    let duplicate_2 = self::source_ml_source1_temporary_accept_java_script_record_parameter_bound_name_loop28(name, names, duplicate, index);
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
pub fn plan_java_script_object_pattern_bindings(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
    previous: &[self::ml_string::Text],
    profile: f64,
) -> Vec<f64> {
    if ((((((((kinds.len() as f64) != (values.len() as f64))
        || !(self::ml_math::is_integer(start)))
        || (start < 0f64))
        || (start >= (kinds.len() as f64)))
        || (profile < 0f64))
        || (profile > 3f64))
        || !(self::ml_math::is_integer(profile)))
        || (((self::read_required_text_fact(kinds, start))
            != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
            || ((self::read_required_text_fact(values, start))
                != (self::ml_string::Text::from_units(vec![123]))))
    {
        vec![-1f64, start]
    } else {
        {
            let at = start + 1f64;
            {
                let names = previous.to_vec();
                {
                    let pairs = Vec::<f64>::new();
                    {
                        let source_ml_source2_s87 = self::source_ml_source1_temporary_plan_java_script_object_pattern_bindings_loop29(kinds, values, reserved, profile, at, names, pairs);
                        match source_ml_source2_s87 {
                                self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Done(at_2, pairs_2) => {
                                    if (pairs_2.len() as f64) == 0f64 {
                                        vec![-4f64, start]
                                    } else if (at_2 >= (kinds.len() as f64)) || !(((self::read_required_text_fact(kinds, at_2)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_2)) == (self::ml_string::Text::from_units(vec![125])))) {
                                            vec![-1f64, at_2]
                                        } else {
                                            {
                                                let source_ml_source2_concat_value88 = vec![(at_2 + 1f64), ((pairs_2.len() as f64) / if profile >= 2f64 {
                                                    4f64
                                                } else {
                                                    2f64
                                                })];
                                                {
                                                    let source_ml_source2_concat_value89 = pairs_2;
                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value88), source_ml_source2_concat_value89)
                                                }
                                            }
                                        }
                                }
                                self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Return(source_ml_source1_temporary_result29) => {
                                    source_ml_source1_temporary_result29
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
pub fn plan_java_script_record_parameter_bindings(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
    ordinary: bool,
    defaults: bool,
    rest: bool,
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
                            let source_ml_source2_s90 = self::source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop32(kinds, values, reserved, ordinary, defaults, rest, at, slot, names, instructions);
                            match source_ml_source2_s90 {
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Done(at_2, slot_2, instructions_2) => {
                                    if (at_2 >= (kinds.len() as f64)) || !(((self::read_required_text_fact(kinds, at_2)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_2)) == (self::ml_string::Text::from_units(vec![41])))) {
                                        vec![-1f64, at_2]
                                    } else {
                                        {
                                            let source_ml_source2_concat_value91 = vec![(at_2 + 1f64), slot_2];
                                            {
                                                let source_ml_source2_concat_value92 = instructions_2;
                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value91), source_ml_source2_concat_value92)
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Return(source_ml_source1_temporary_result32) => {
                                    source_ml_source1_temporary_result32
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
pub fn read_java_script_pattern_initializer_scope_boundary(context: &self::ml_string::Text) -> f64 {
    if (*context)
        == (self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 111, 98, 106, 101, 99, 116, 32, 98, 105,
            110, 100, 105, 110, 103, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110,
        ]))
    {
        0f64
    } else {
        self::read_java_script_array_formal_default_scope_boundary(context)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_pattern_initializer_subject(
    context: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*context)
        == (self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 111, 98, 106, 101, 99, 116, 32, 98, 105,
            110, 100, 105, 110, 103, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110,
        ]))
    {
        self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103])
    } else {
        self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 101, 116, 101, 114])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_pattern_initializer_reference_reason(
    context: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*context)
        == (self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 111, 98, 106, 101, 99, 116, 32, 98, 105,
            110, 100, 105, 110, 103, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            116, 104, 101, 32, 99, 117, 114, 114, 101, 110, 116, 32, 98, 105, 110, 100, 105, 110,
            103, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 112, 114, 111, 102,
            105, 108, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 110, 105, 116, 105,
            97, 108, 105, 122, 101, 100, 32, 101, 97, 114, 108, 105, 101, 114, 32, 108, 101, 120,
            105, 99, 97, 108, 32, 98, 105, 110, 100, 105, 110, 103, 115,
        ])
    } else {
        self::ml_string::Text::from_units(vec![
            116, 104, 101, 32, 99, 117, 114, 114, 101, 110, 116, 32, 100, 101, 102, 97, 117, 108,
            116, 32, 112, 114, 111, 102, 105, 108, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115,
            32, 97, 32, 99, 108, 111, 115, 101, 100, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114,
            32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 101, 110,
            118, 105, 114, 111, 110, 109, 101, 110, 116,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_pattern_initializer_assignment_reason(
    context: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*context)
        == (self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 111, 98, 106, 101, 99, 116, 32, 98, 105,
            110, 100, 105, 110, 103, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            119, 114, 105, 116, 101, 115, 32, 100, 117, 114, 105, 110, 103, 32, 98, 105, 110, 100,
            105, 110, 103, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110,
            32, 110, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100,
            32, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 32, 105, 110, 116, 101, 114,
            112, 114, 101, 116, 97, 116, 105, 111, 110,
        ])
    } else {
        self::ml_string::Text::from_units(vec![
            119, 114, 105, 116, 101, 115, 32, 100, 117, 114, 105, 110, 103, 32, 112, 97, 114, 97,
            109, 101, 116, 101, 114, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105,
            111, 110, 32, 110, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114,
            101, 100, 32, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 32, 105, 110, 116,
            101, 114, 112, 114, 101, 116, 97, 116, 105, 111, 110,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_object_binding_statement(
    mode: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*mode) == (self::ml_string::Text::from_units(vec![108, 101, 116])) {
        self::ml_string::Text::from_units(vec![108, 101, 116])
    } else if (*mode) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116])) {
        self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_object_binding_name(
    kind: &self::ml_string::Text,
    name: &self::ml_string::Text,
    reserved: &[self::ml_string::Text],
) -> bool {
    self::accept_java_script_source_binding_name(kind, name, false, reserved)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_object_pattern_binding_positions(plan: &[f64]) -> Vec<f64> {
    if ((plan.len() as f64) < 2f64) || ((self::read_required_number_fact(plan, 0f64)) < 0f64) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 98, 105, 110, 100, 105, 110, 103, 32,
                99, 101, 110, 115, 117, 115, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32,
                97, 100, 109, 105, 116, 116, 101, 100, 32, 100, 101, 102, 97, 117, 108, 116, 45,
                101, 110, 97, 98, 108, 101, 100, 32, 112, 97, 116, 116, 101, 114, 110, 32, 112,
                108, 97, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        {
            let count = self::read_required_number_fact(plan, 1f64);
            {
                let positions = Vec::<f64>::new();
                {
                    let index = 0f64;
                    {
                        self::source_ml_source1_temporary_read_java_script_object_pattern_binding_positions_loop35(plan, count, positions, index)
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
pub fn read_java_script_record_formal_default_binding_positions(plan: &[f64]) -> Vec<f64> {
    if ((plan.len() as f64) < 2f64) || ((self::read_required_number_fact(plan, 0f64)) < 0f64) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                97, 32, 114, 101, 99, 111, 114, 100, 32, 105, 110, 105, 116, 105, 97, 108, 105,
                122, 101, 114, 32, 99, 101, 110, 115, 117, 115, 32, 110, 101, 101, 100, 115, 32,
                97, 110, 32, 97, 100, 109, 105, 116, 116, 101, 100, 32, 99, 111, 109, 112, 108,
                101, 116, 101, 32, 112, 108, 97, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        {
            let cursor = 2f64;
            {
                let names = Vec::<f64>::new();
                {
                    self::source_ml_source1_temporary_read_java_script_record_formal_default_binding_positions_loop36(plan, cursor, names)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_formal_default_operation() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        114, 101, 97, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 99, 111, 114, 100,
        70, 111, 114, 109, 97, 108, 68, 101, 102, 97, 117, 108, 116,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_formal_default_source(
    language: &self::ml_string::Text,
    count: f64,
    spread: bool,
    key: &self::ml_string::Text,
) -> bool {
    ((((*language)
        == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116])))
        && (count == 3f64))
        && !spread)
        && ((*key) == (self::ml_string::Text::from_units(vec![115, 116, 114])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_formal_default_field_form(
    kind: &self::ml_string::Text,
    open: bool,
    present: bool,
    name: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*kind) == (self::ml_string::Text::from_units(vec![118, 97, 114])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![108, 105, 116, 101, 114, 97, 108])))
    {
        self::ml_string::Text::from_units(vec![119, 97, 105, 116])
    } else {
        let ml_library_inequality_1 =
            (*kind) != (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]));
        if ml_library_inequality_1 {
            self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100])
        } else if present {
            self::ml_string::Text::from_units(vec![112, 114, 101, 115, 101, 110, 116])
        } else if open {
            self::ml_string::Text::from_units(vec![119, 97, 105, 116])
        } else if ((((((((((((*name)
            == (self::ml_string::Text::from_units(vec![
                95, 95, 112, 114, 111, 116, 111, 95, 95,
            ])))
            || ((*name)
                == (self::ml_string::Text::from_units(vec![
                    99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114,
                ]))))
            || ((*name)
                == (self::ml_string::Text::from_units(vec![
                    104, 97, 115, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121,
                ]))))
            || ((*name)
                == (self::ml_string::Text::from_units(vec![
                    105, 115, 80, 114, 111, 116, 111, 116, 121, 112, 101, 79, 102,
                ]))))
            || ((*name)
                == (self::ml_string::Text::from_units(vec![
                    112, 114, 111, 112, 101, 114, 116, 121, 73, 115, 69, 110, 117, 109, 101, 114,
                    97, 98, 108, 101,
                ]))))
            || ((*name)
                == (self::ml_string::Text::from_units(vec![
                    116, 111, 76, 111, 99, 97, 108, 101, 83, 116, 114, 105, 110, 103,
                ]))))
            || ((*name)
                == (self::ml_string::Text::from_units(vec![
                    116, 111, 83, 116, 114, 105, 110, 103,
                ]))))
            || ((*name)
                == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102]))))
            || ((*name)
                == (self::ml_string::Text::from_units(vec![
                    95, 95, 100, 101, 102, 105, 110, 101, 71, 101, 116, 116, 101, 114, 95, 95,
                ]))))
            || ((*name)
                == (self::ml_string::Text::from_units(vec![
                    95, 95, 100, 101, 102, 105, 110, 101, 83, 101, 116, 116, 101, 114, 95, 95,
                ]))))
            || ((*name)
                == (self::ml_string::Text::from_units(vec![
                    95, 95, 108, 111, 111, 107, 117, 112, 71, 101, 116, 116, 101, 114, 95, 95,
                ]))))
            || ((*name)
                == (self::ml_string::Text::from_units(vec![
                    95, 95, 108, 111, 111, 107, 117, 112, 83, 101, 116, 116, 101, 114, 95, 95,
                ])))
        {
            self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101])
        } else {
            self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_standard_builtin_owner(
    name: &self::ml_string::Text,
    shadowed: bool,
) -> bool {
    (((((((((((((((((((*name)
        == (self::ml_string::Text::from_units(vec![77, 97, 112])))
        || ((*name) == (self::ml_string::Text::from_units(vec![83, 101, 116]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 110, 115, 111, 108, 101,
            ]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![
                66, 111, 111, 108, 101, 97, 110,
            ]))))
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
                let source_ml_source2_s93 = self::source_ml_source1_temporary_find_java_script_record_parameter_strict_directive_loop38(kinds, values, raws, line_breaks, at);
                match source_ml_source2_s93 {
                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Done => {
                        -2f64
                    }
                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Return(source_ml_source1_temporary_result38) => {
                        source_ml_source1_temporary_result38
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_parameter_binding_statement(written: bool) -> self::ml_string::Text {
    if written {
        self::ml_string::Text::from_units(vec![108, 101, 116])
    } else {
        self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_closure_context(
    method: bool,
    expression: bool,
) -> self::ml_string::Text {
    if method {
        self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 111, 98, 106, 101, 99, 116, 32, 109, 101,
            116, 104, 111, 100, 32, 99, 108, 111, 115, 117, 114, 101,
        ])
    } else if expression {
        self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32,
            101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 99, 108, 111, 115, 117, 114, 101,
        ])
    } else {
        self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 114, 114, 111, 119, 32, 102, 117, 110,
            99, 116, 105, 111, 110, 32, 99, 108, 111, 115, 117, 114, 101,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_closure_parameter_profile(
    context: &self::ml_string::Text,
    asynchronous: bool,
    documentation_parameters: f64,
) -> bool {
    (((((*context)
        == (self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 111, 98, 106, 101, 99, 116, 32, 109, 101,
            116, 104, 111, 100, 32, 99, 108, 111, 115, 117, 114, 101,
        ])))
        || ((*context)
            == (self::ml_string::Text::from_units(vec![
                111, 114, 100, 105, 110, 97, 114, 121, 32, 102, 117, 110, 99, 116, 105, 111, 110,
                32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 99, 108, 111, 115, 117,
                114, 101,
            ]))))
        || ((*context)
            == (self::ml_string::Text::from_units(vec![
                111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 114, 114, 111, 119, 32, 102, 117,
                110, 99, 116, 105, 111, 110, 32, 99, 108, 111, 115, 117, 114, 101,
            ]))))
        && !asynchronous)
        && (documentation_parameters == 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_record_closure_parameter_bindings(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
) -> Vec<f64> {
    self::plan_java_script_source_record_parameters(kinds, values, start, reserved)
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
                let unit = {
                    let source_ml_source2_string_unit_operand94 = name.clone();
                    {
                        let source_ml_source2_string_unit_operand95 = index;
                        (std::rc::Rc::new(
                            move |text: self::ml_string::Text, position: f64| -> f64 {
                                {
                                    let index_2 = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index_2 < 0f64) || (index_2 >= (text.length())) {
                                        f64::NAN
                                    } else {
                                        ({
                                    let source_ml_source2_string_unit_operand101 = {
                                        let source_ml_source2_string_range_operand96 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand97 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand98 = index_2 + 1f64;
                                                {
                                                    let source_ml_source2_string_range_position99 = source_ml_source2_string_range_operand97;
                                                    {
                                                        let source_ml_source2_string_range_position100 = source_ml_source2_string_range_operand98;
                                                        source_ml_source2_string_range_operand96.slice(source_ml_source2_string_range_position99, source_ml_source2_string_range_position100)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand102 = 0f64;
                                        source_ml_source2_string_unit_operand101.code_point_at(source_ml_source2_string_unit_operand102)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source2_string_unit_operand94,
                            source_ml_source2_string_unit_operand95,
                        )
                    }
                };
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
