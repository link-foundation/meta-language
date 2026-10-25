// Generated from complete js/src/translation/record-callable-parameter-rules.js. Do not edit by hand.
use super::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result;
use super::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Result;
use super::accept_java_script_record_parameter_binding;
use super::accept_java_script_source_binding_name;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_accept_java_script_record_parameter_bound_name_loop28;
use super::source_ml_source1_temporary_find_java_script_record_parameter_strict_directive_loop36;
use super::source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop29;
use super::source_ml_source1_temporary_read_java_script_record_formal_default_binding_positions_loop34;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_source_record_parameters(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
) -> Vec<f64> {
    self::plan_java_script_record_parameter_bindings(kinds, values, start, reserved, true, false)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_source_record_parameters_with_defaults(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
) -> Vec<f64> {
    self::plan_java_script_record_parameter_bindings(kinds, values, start, reserved, true, true)
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
pub fn plan_java_script_record_parameter_bindings(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
    ordinary: bool,
    defaults: bool,
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
                            let source_ml_source2_s87 = self::source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop29(kinds, values, reserved, ordinary, defaults, at, slot, names, instructions);
                            match source_ml_source2_s87 {
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Done(at_2, slot_2, instructions_2) => {
                                    if (at_2 >= (kinds.len() as f64)) || !(((self::read_required_text_fact(kinds, at_2)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_2)) == (self::ml_string::Text::from_units(vec![41])))) {
                                        vec![-1f64, at_2]
                                    } else {
                                        {
                                            let source_ml_source2_concat_value88 = vec![(at_2 + 1f64), slot_2];
                                            {
                                                let source_ml_source2_concat_value89 = instructions_2;
                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value88), source_ml_source2_concat_value89)
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Return(source_ml_source1_temporary_result29) => {
                                    source_ml_source1_temporary_result29
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
                    self::source_ml_source1_temporary_read_java_script_record_formal_default_binding_positions_loop34(plan, cursor, names)
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
                let source_ml_source2_s90 = self::source_ml_source1_temporary_find_java_script_record_parameter_strict_directive_loop36(kinds, values, raws, line_breaks, at);
                match source_ml_source2_s90 {
                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Done => {
                        -2f64
                    }
                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Return(source_ml_source1_temporary_result36) => {
                        source_ml_source1_temporary_result36
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
pub fn read_java_script_record_constant_context(expression: bool) -> self::ml_string::Text {
    if expression {
        self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32,
            102, 117, 110, 99, 116, 105, 111, 110, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111,
            110,
        ])
    } else {
        self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32,
            97, 114, 114, 111, 119, 32, 102, 117, 110, 99, 116, 105, 111, 110,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_callable_parameter_profile(
    context: &self::ml_string::Text,
    asynchronous: bool,
    documentation_parameters: f64,
) -> bool {
    (((((*context)
        == (self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32,
            102, 117, 110, 99, 116, 105, 111, 110, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111,
            110,
        ])))
        || ((*context)
            == (self::ml_string::Text::from_units(vec![
                111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 115, 116, 97, 110, 116,
                32, 97, 114, 114, 111, 119, 32, 102, 117, 110, 99, 116, 105, 111, 110,
            ]))))
        || ((*context)
            == (self::ml_string::Text::from_units(vec![
                111, 114, 100, 105, 110, 97, 114, 121, 32, 110, 97, 109, 101, 115, 112, 97, 99,
                101, 32, 111, 98, 106, 101, 99, 116, 32, 109, 101, 116, 104, 111, 100,
            ]))))
        && !asynchronous)
        && (documentation_parameters == 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_record_callable_parameter_bindings(
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
                    let source_ml_source2_string_unit_operand91 = name.clone();
                    {
                        let source_ml_source2_string_unit_operand92 = index;
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
                                    let source_ml_source2_string_unit_operand98 = {
                                        let source_ml_source2_string_range_operand93 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand94 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand95 = index_2 + 1f64;
                                                {
                                                    let source_ml_source2_string_range_position96 = source_ml_source2_string_range_operand94;
                                                    {
                                                        let source_ml_source2_string_range_position97 = source_ml_source2_string_range_operand95;
                                                        source_ml_source2_string_range_operand93.slice(source_ml_source2_string_range_position96, source_ml_source2_string_range_position97)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand99 = 0f64;
                                        source_ml_source2_string_unit_operand98.code_point_at(source_ml_source2_string_unit_operand99)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source2_string_unit_operand91,
                            source_ml_source2_string_unit_operand92,
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
                if {
                    let source_ml_source2_comparison_operand100 = word.clone();
                    {
                        let source_ml_source2_comparison_operand101 = None::<()>;
                        match (
                            source_ml_source2_comparison_operand100,
                            source_ml_source2_comparison_operand101,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }
                    }
                } {
                    panic!(
                        "{}",
                        "source binding reserved names need a complete text column"
                    )
                } else if {
                    let source_ml_source2_comparison_operand102 = word.clone();
                    {
                        let source_ml_source2_comparison_operand103 = name.clone();
                        match (
                            source_ml_source2_comparison_operand102,
                            source_ml_source2_comparison_operand103,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Result {
    MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Done,
    MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_java_script_array_binding_name_loop4(
    name: &self::ml_string::Text,
    reserved: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Result {
    loop {
        return if index < (reserved.len() as f64) {
            if (self::read_required_text_fact(reserved, index)) == (*name) {
                self::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Result::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Return(false)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Result::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadArrayBindingFactFieldLoop5Result {
    MlSource1TemporaryReadArrayBindingFactFieldLoop5Done,
    MlSource1TemporaryReadArrayBindingFactFieldLoop5Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_array_binding_fact_field_loop5(
    labels: &[self::ml_string::Text],
    targets: &[f64],
    field: &self::ml_string::Text,
    mut at: f64,
    mut end: f64,
) -> self::MlSource1TemporaryReadArrayBindingFactFieldLoop5Result {
    loop {
        return if at < end {
            if (self::read_required_text_fact(labels, at)) == (*field) {
                self::MlSource1TemporaryReadArrayBindingFactFieldLoop5Result::MlSource1TemporaryReadArrayBindingFactFieldLoop5Return(self::read_required_number_fact(targets, at))
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadArrayBindingFactFieldLoop5Result::MlSource1TemporaryReadArrayBindingFactFieldLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadArrayBindingInitializerEndLoop6Result {
    MlSource1TemporaryReadArrayBindingInitializerEndLoop6Done,
    MlSource1TemporaryReadArrayBindingInitializerEndLoop6Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadArrayBindingInitializerEndJoin7 {
    MlSource1TemporaryReadArrayBindingInitializerEndJoin7Next(Vec<self::ml_string::Text>),
    MlSource1TemporaryReadArrayBindingInitializerEndJoin7Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_array_binding_initializer_end_loop6(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut start: f64,
    mut index: f64,
    mut closes: Vec<self::ml_string::Text>,
) -> self::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Result {
    loop {
        return if (index < (values.len() as f64)) && (index < (kinds.len() as f64)) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                {
                    let value = self::read_required_text_fact(values, index);
                    if kind == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                        self::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Result::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Return(-1f64)
                    } else {
                        {
                            let source_ml_source2_s106 = if kind
                                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))
                            {
                                if ((closes.len() as f64) == 0f64)
                                    && ((value == (self::ml_string::Text::from_units(vec![44])))
                                        || (value == (self::ml_string::Text::from_units(vec![93]))))
                                {
                                    self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Return(if index > start {
                                        index
                                    } else {
                                        -1f64
                                    })
                                } else if ((value == (self::ml_string::Text::from_units(vec![40])))
                                    || (value == (self::ml_string::Text::from_units(vec![91]))))
                                    || (value == (self::ml_string::Text::from_units(vec![123])))
                                {
                                    {
                                        let closes_2 = {
                                            let source_ml_source2_concat_value107 = closes;
                                            {
                                                let source_ml_source2_concat_value108 =
                                                    vec![if value
                                                        == (self::ml_string::Text::from_units(
                                                            vec![40],
                                                        )) {
                                                        self::ml_string::Text::from_units(vec![41])
                                                    } else if value
                                                        == (self::ml_string::Text::from_units(
                                                            vec![91],
                                                        ))
                                                    {
                                                        self::ml_string::Text::from_units(vec![93])
                                                    } else {
                                                        self::ml_string::Text::from_units(vec![125])
                                                    }];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<self::ml_string::Text>::new(),
                                                        source_ml_source2_concat_value107,
                                                    ),
                                                    source_ml_source2_concat_value108,
                                                )
                                            }
                                        };
                                        self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Next(closes_2)
                                    }
                                } else if ((value == (self::ml_string::Text::from_units(vec![41])))
                                    || (value == (self::ml_string::Text::from_units(vec![93]))))
                                    || (value == (self::ml_string::Text::from_units(vec![125])))
                                {
                                    if ((closes.len() as f64) == 0f64)
                                        || ((self::read_required_text_fact(
                                            &(closes),
                                            ((closes.len() as f64) - 1f64),
                                        )) != value)
                                    {
                                        self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Return(-1f64)
                                    } else {
                                        {
                                            let closes_3 = self::ml_array::slice(
                                                &closes,
                                                0f64,
                                                ((closes.len() as f64) - 1f64),
                                            );
                                            self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Next(closes_3)
                                        }
                                    }
                                } else {
                                    self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Next(closes)
                                }
                            } else {
                                self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Next(closes)
                            };
                            match source_ml_source2_s106 {
                                self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Next(closes_4) => {
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (index, closes) = (index_2, closes_4);
                                            continue;
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadArrayBindingInitializerEndJoin7::MlSource1TemporaryReadArrayBindingInitializerEndJoin7Return(source_ml_source1_temporary_result7) => {
                                    self::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Result::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Return(source_ml_source1_temporary_result7)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Result::MlSource1TemporaryReadArrayBindingInitializerEndLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result {
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Done,
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11 {
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11Next(f64),
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10 {
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Next(f64, Vec<f64>),
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9 {
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9Next(f64, bool),
}
