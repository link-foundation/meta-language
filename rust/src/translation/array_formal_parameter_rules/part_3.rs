// Generated from complete js/src/translation/array-formal-parameter-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result;
use super::accept_java_script_source_binding_name;
use super::find_java_script_formal_default_expression_end;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_required_text_fact;

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
                    let source_ml_source2_string_unit_operand66 = name.clone();
                    {
                        let source_ml_source2_string_unit_operand67 = index;
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
                                    let source_ml_source2_string_unit_operand73 = {
                                        let source_ml_source2_string_range_operand68 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand69 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand70 = index_2 + 1f64;
                                                {
                                                    let source_ml_source2_string_range_position71 = source_ml_source2_string_range_operand69;
                                                    {
                                                        let source_ml_source2_string_range_position72 = source_ml_source2_string_range_operand70;
                                                        source_ml_source2_string_range_operand68.slice(source_ml_source2_string_range_position71, source_ml_source2_string_range_position72)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand74 = 0f64;
                                        source_ml_source2_string_unit_operand73.code_point_at(source_ml_source2_string_unit_operand74)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source2_string_unit_operand66,
                            source_ml_source2_string_unit_operand67,
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
                    let source_ml_source2_comparison_operand75 = word.clone();
                    {
                        let source_ml_source2_comparison_operand76 = None::<()>;
                        match (
                            source_ml_source2_comparison_operand75,
                            source_ml_source2_comparison_operand76,
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
                    let source_ml_source2_comparison_operand77 = word.clone();
                    {
                        let source_ml_source2_comparison_operand78 = name.clone();
                        match (
                            source_ml_source2_comparison_operand77,
                            source_ml_source2_comparison_operand78,
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Result {
    MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Done,
    MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_array_formal_pattern_loop4(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut depth: f64,
    mut binding: bool,
    mut index: f64,
) -> self::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Result {
    loop {
        return if index < (kinds.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                {
                    let value = self::read_required_text_fact(values, index);
                    if kind == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                        self::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Result::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Return(-1f64)
                    } else {
                        if kind != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))
                        {
                            {
                                let binding_3 = if depth == 0f64 { { false } } else { binding };
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        (binding, index) = (binding_3, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if (depth == 0f64)
                                && (value == (self::ml_string::Text::from_units(vec![41])))
                            {
                                self::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Result::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Return(-1f64)
                            } else if ((depth == 0f64) && binding)
                                && (value == (self::ml_string::Text::from_units(vec![91])))
                            {
                                self::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Result::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Return(index)
                            } else {
                                if ((value == (self::ml_string::Text::from_units(vec![40])))
                                    || (value == (self::ml_string::Text::from_units(vec![91]))))
                                    || (value == (self::ml_string::Text::from_units(vec![123])))
                                {
                                    {
                                        let depth_2 = depth + 1f64;
                                        {
                                            let binding_4 = false;
                                            {
                                                let index_3 = index + 1f64;
                                                {
                                                    (depth, binding, index) =
                                                        (depth_2, binding_4, index_3);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                                {
                                    if ((value == (self::ml_string::Text::from_units(vec![41])))
                                        || (value == (self::ml_string::Text::from_units(vec![93]))))
                                        || (value == (self::ml_string::Text::from_units(vec![125])))
                                    {
                                        {
                                            let depth_3 = depth - 1f64;
                                            if depth_3 < 0f64 {
                                                self::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Result::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Return(-1f64)
                                            } else {
                                                {
                                                    let index_4 = index + 1f64;
                                                    {
                                                        (depth, index) = (depth_3, index_4);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if depth == 0f64 {
                                            {
                                                let binding_5 = value
                                                    == (self::ml_string::Text::from_units(vec![
                                                        44,
                                                    ]));
                                                {
                                                    let index_5 = index + 1f64;
                                                    {
                                                        (binding, index) = (binding_5, index_5);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            {
                                                let index_6 = index + 1f64;
                                                {
                                                    index = index_6;
                                                    continue;
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
        } else {
            self::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Result::MlSource1TemporaryFindJavaScriptArrayFormalPatternLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Result {
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Done(f64, f64, Vec<f64>),
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6 {
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6Next(
        f64,
        Vec<self::ml_string::Text>,
        Vec<f64>,
    ),
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Result {
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Done(
        f64,
        Vec<self::ml_string::Text>,
        Vec<f64>,
    ),
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop8(
    names: &[self::ml_string::Text],
    name: &self::ml_string::Text,
    mut duplicate: bool,
    mut previous: f64,
) -> bool {
    loop {
        if previous < (names.len() as f64) {
            if (self::read_required_text_fact(names, previous)) == (*name) {
                {
                    let duplicate_2 = true;
                    {
                        let previous_2 = previous + 1f64;
                        {
                            (duplicate, previous) = (duplicate_2, previous_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let previous_3 = previous + 1f64;
                    {
                        previous = previous_3;
                        continue;
                    }
                }
            }
        }
        return { duplicate };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9 {
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9Next(f64, f64, f64),
    MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop7(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut defaults: bool,
    mut index: f64,
    mut names: Vec<self::ml_string::Text>,
    mut position: f64,
    mut bindings: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Result {
    loop {
        return if (index < (kinds.len() as f64))
            && !(((self::read_required_text_fact(kinds, index))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, index))
                    == (self::ml_string::Text::from_units(vec![93]))))
        {
            if ((self::read_required_text_fact(kinds, index))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, index))
                    == (self::ml_string::Text::from_units(vec![44])))
            {
                {
                    let position_2 = position + 1f64;
                    {
                        let index_2 = index + 1f64;
                        {
                            (index, position) = (index_2, position_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let rest = ((self::read_required_text_fact(kinds, index))
                        == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                        && ((self::read_required_text_fact(values, index))
                            == (self::ml_string::Text::from_units(vec![46, 46, 46])));
                    {
                        let index_4 = if rest { { index + 1f64 } } else { index };
                        if index_4 >= (kinds.len() as f64) {
                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Return(vec![-1f64, index_4])
                        } else {
                            {
                                let token = index_4;
                                {
                                    let name = self::read_required_text_fact(values, index_4);
                                    {
                                        let token_kind =
                                            self::read_required_text_fact(kinds, index_4);
                                        if (token_kind
                                            == (self::ml_string::Text::from_units(vec![
                                                112, 117, 110, 99, 116,
                                            ])))
                                            && ((name
                                                == (self::ml_string::Text::from_units(vec![91])))
                                                || (name
                                                    == (self::ml_string::Text::from_units(vec![
                                                        123,
                                                    ]))))
                                        {
                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Return(vec![-2f64, index_4])
                                        } else {
                                            {
                                                let duplicate = false;
                                                {
                                                    let previous = 0f64;
                                                    {
                                                        let duplicate_2 = self::source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop8(&(names), &(name), duplicate, previous);
                                                        if (self::accept_java_script_source_binding_name(&(token_kind), &(name), duplicate_2, reserved)) {
                                                            {
                                                                let names_2 = {
                                                                    let source_ml_source2_concat_value79 = names;
                                                                    {
                                                                        let source_ml_source2_concat_value80 = vec![name.clone()];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value79), source_ml_source2_concat_value80)
                                                                    }
                                                                };
                                                                {
                                                                    let index_5 = index_4 + 1f64;
                                                                    {
                                                                        let position_3 = position + 1f64;
                                                                        {
                                                                            let default_start = -1f64;
                                                                            {
                                                                                let default_end = -1f64;
                                                                                if index_5 >= (kinds.len() as f64) {
                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Return(vec![-1f64, index_5])
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_source2_s81 = if ((self::read_required_text_fact(kinds, index_5)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_5)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                                                            if defaults {if rest {
                                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9Return(vec![-5f64, index_5])
                                                                                                } else {
                                                                                                    {
                                                                                                        let default_start_2 = index_5 + 1f64;
                                                                                                        {
                                                                                                            let default_end_2 = self::find_java_script_formal_default_expression_end(kinds, values, default_start_2);
                                                                                                            if default_end_2 < 0f64 {
                                                                                                                self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9Return(vec![-1f64, default_start_2])
                                                                                                            } else {
                                                                                                                {
                                                                                                                    let index_6 = default_end_2;
                                                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9Next(index_6, default_start_2, default_end_2)
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }} else {
                                                                                                self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9Return(vec![-3f64, index_5])
                                                                                            }
                                                                                        } else {
                                                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9Next(index_5, default_start, default_end)
                                                                                        };
                                                                                        match source_ml_source2_s81 {
                                                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9Next(index_7, default_start_3, default_end_3) => {
                                                                                                {
                                                                                                    let bindings_2 = {
                                                                                                        let source_ml_source2_concat_value82 = bindings;
                                                                                                        {
                                                                                                            let source_ml_source2_concat_value83 = if defaults {
                                                                                                                vec![if rest {
                                                                                                                    1f64
                                                                                                                } else {
                                                                                                                    0f64
                                                                                                                }, (position_3 - 1f64), token, default_start_3, default_end_3]
                                                                                                            } else {
                                                                                                                vec![if rest {
                                                                                                                    1f64
                                                                                                                } else {
                                                                                                                    0f64
                                                                                                                }, (position_3 - 1f64), token]
                                                                                                            };
                                                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value82), source_ml_source2_concat_value83)
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let next_kind = self::read_required_text_fact(kinds, index_7);
                                                                                                        {
                                                                                                            let next = self::read_required_text_fact(values, index_7);
                                                                                                            if (next_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (next == (self::ml_string::Text::from_units(vec![61]))) {
                                                                                                                self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Return(vec![-3f64, index_7])
                                                                                                            } else if rest && !((next_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (next == (self::ml_string::Text::from_units(vec![93])))) {
                                                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Return(vec![-5f64, index_7])
                                                                                                                } else {
                                                                                                                    if (next_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (next == (self::ml_string::Text::from_units(vec![44]))) {
                                                                                                                        {
                                                                                                                            let index_8 = index_7 + 1f64;
                                                                                                                            {
                                                                                                                                (index, names, position, bindings) = (index_8, names_2, position_3, bindings_2);
                                                                                                                                continue;
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
{
                                                                                                                        if ((next_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (next == (self::ml_string::Text::from_units(vec![93])))) {
                                                                                                                            {
                                                                                                                                (index, names, position, bindings) = (index_7, names_2, position_3, bindings_2);
                                                                                                                                continue;
                                                                                                                            }
                                                                                                                        }
{
                                                                                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Return(vec![-1f64, index_7])
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin9Return(source_ml_source1_temporary_result9) => {
                                                                                                self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Return(source_ml_source1_temporary_result9)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Return(vec![-4f64, index_4])
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
        } else {
            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Done(index, names, bindings)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop10(
    names: &[self::ml_string::Text],
    value: &self::ml_string::Text,
    mut duplicate: bool,
    mut previous: f64,
) -> bool {
    loop {
        if previous < (names.len() as f64) {
            if (self::read_required_text_fact(names, previous)) == (*value) {
                {
                    let duplicate_2 = true;
                    {
                        let previous_2 = previous + 1f64;
                        {
                            (duplicate, previous) = (duplicate_2, previous_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let previous_3 = previous + 1f64;
                    {
                        previous = previous_3;
                        continue;
                    }
                }
            }
        }
        return { duplicate };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop5(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut defaults: bool,
    mut index: f64,
    mut slot: f64,
    mut names: Vec<self::ml_string::Text>,
    mut instructions: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Result {
    loop {
        return if (index < (kinds.len() as f64))
            && !(((self::read_required_text_fact(kinds, index))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, index))
                    == (self::ml_string::Text::from_units(vec![41]))))
        {
            {
                let kind = self::read_required_text_fact(kinds, index);
                {
                    let value = self::read_required_text_fact(values, index);
                    {
                        let source_ml_source2_s92 = if (kind
                            == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                            && (value == (self::ml_string::Text::from_units(vec![91])))
                        {
                            {
                                let open = index;
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        let position = 0f64;
                                        {
                                            let bindings = Vec::<f64>::new();
                                            {
                                                let source_ml_source2_s93 = self::source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop7(kinds, values, reserved, defaults, index_2, names, position, bindings);
                                                match source_ml_source2_s93 {
                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Done(index_3, names_2, bindings_2) => {
                                                        if index_3 >= (kinds.len() as f64) {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6Return(vec![-1f64, index_3])
                                                        } else {
                                                            {
                                                                let instructions_2 = {
                                                                    let source_ml_source2_concat_value94 = instructions;
                                                                    {
                                                                        let source_ml_source2_concat_value95 = vec![2f64, slot, open, index_3, ((bindings_2.len() as f64) / if defaults {
                                                                            5f64
                                                                        } else {
                                                                            3f64
                                                                        })];
                                                                        {
                                                                            let source_ml_source2_concat_value96 = bindings_2;
                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value94), source_ml_source2_concat_value95), source_ml_source2_concat_value96)
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let index_4 = index_3 + 1f64;
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6Next(index_4, names_2, instructions_2)
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Return(source_ml_source1_temporary_result7) => {
                                                        self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6Return(source_ml_source1_temporary_result7)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else if (kind
                            == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                            && (value == (self::ml_string::Text::from_units(vec![123])))
                        {
                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6Return(vec![-2f64, index])
                        } else {
                            {
                                let duplicate = false;
                                {
                                    let previous = 0f64;
                                    {
                                        let duplicate_2 = self::source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop10(&(names), &(value), duplicate, previous);
                                        if (self::accept_java_script_source_binding_name(
                                            &(kind),
                                            &(value),
                                            duplicate_2,
                                            reserved,
                                        )) {
                                            {
                                                let names_3 = {
                                                    let source_ml_source2_concat_value97 = names;
                                                    {
                                                        let source_ml_source2_concat_value98 =
                                                            vec![value.clone()];
                                                        self::ml_array::append(
                                                            self::ml_array::append(
                                                                Vec::<self::ml_string::Text>::new(),
                                                                source_ml_source2_concat_value97,
                                                            ),
                                                            source_ml_source2_concat_value98,
                                                        )
                                                    }
                                                };
                                                {
                                                    let instructions_3 = {
                                                        let source_ml_source2_concat_value99 =
                                                            instructions;
                                                        {
                                                            let source_ml_source2_concat_value100 =
                                                                vec![0f64, slot, index];
                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value99), source_ml_source2_concat_value100)
                                                        }
                                                    };
                                                    {
                                                        let index_5 = index + 1f64;
                                                        self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6Next(index_5, names_3, instructions_3)
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6Return(vec![-6f64, index])
                                        }
                                    }
                                }
                            }
                        };
                        match source_ml_source2_s92 {
                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6Next(index_6, names_4, instructions_4) => {
                                {
                                    let slot_2 = slot + 1f64;
                                    if index_6 >= (kinds.len() as f64) {
                                        self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Return(vec![-1f64, index_6])
                                    } else {
                                        {
                                            let delimiter_kind = self::read_required_text_fact(kinds, index_6);
                                            {
                                                let delimiter_value = self::read_required_text_fact(values, index_6);
                                                if (delimiter_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (delimiter_value == (self::ml_string::Text::from_units(vec![61]))) {
                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Return(vec![-3f64, index_6])
                                                } else {
                                                    if (delimiter_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (delimiter_value == (self::ml_string::Text::from_units(vec![44]))) {
                                                        {
                                                            let index_7 = index_6 + 1f64;
                                                            {
                                                                (index, slot, names, instructions) = (index_7, slot_2, names_4, instructions_4);
                                                                continue;
                                                            }
                                                        }
                                                    }
{
                                                        if ((delimiter_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (delimiter_value == (self::ml_string::Text::from_units(vec![41])))) {
                                                            {
                                                                (index, slot, names, instructions) = (index_6, slot_2, names_4, instructions_4);
                                                                continue;
                                                            }
                                                        }
{
                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Return(vec![-1f64, index_6])
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6Return(source_ml_source1_temporary_result6) => {
                                self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Return(source_ml_source1_temporary_result6)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Done(index, slot, instructions)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptFormalDefaultExpressionEndLoop11Result {
    MlSource1TemporaryFindJavaScriptFormalDefaultExpressionEndLoop11Done,
    MlSource1TemporaryFindJavaScriptFormalDefaultExpressionEndLoop11Return(f64),
}
