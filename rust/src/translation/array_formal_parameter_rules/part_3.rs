// Generated from complete js/src/translation/array-formal-parameter-rules.js. Do not edit by hand.
use super::MlSource1TemporaryAcceptJavaScriptSourceBindingNameLoop3Result;
use super::accept_java_script_source_binding_name;
use super::ml_array;
use super::ml_string;
use super::read_required_text_fact;

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
                    let source_ml_source2_comparison_operand74 = word.clone();
                    {
                        let source_ml_source2_comparison_operand75 = None::<()>;
                        match (
                            source_ml_source2_comparison_operand74,
                            source_ml_source2_comparison_operand75,
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
                    let source_ml_source2_comparison_operand76 = word.clone();
                    {
                        let source_ml_source2_comparison_operand77 = name.clone();
                        match (
                            source_ml_source2_comparison_operand76,
                            source_ml_source2_comparison_operand77,
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
pub enum MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop5Result {
    MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop5Done(f64, f64, Vec<f64>),
    MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop5Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6 {
    MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6Next(
        f64,
        Vec<self::ml_string::Text>,
        Vec<f64>,
    ),
    MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Result {
    MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Done(
        f64,
        Vec<self::ml_string::Text>,
        Vec<f64>,
    ),
    MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_array_formal_parameters_loop8(
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_array_formal_parameters_loop7(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut index: f64,
    mut names: Vec<self::ml_string::Text>,
    mut position: f64,
    mut bindings: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Result {
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
                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Return(vec![-1f64, index_4])
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
                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Return(vec![-2f64, index_4])
                                        } else {
                                            {
                                                let duplicate = false;
                                                {
                                                    let previous = 0f64;
                                                    {
                                                        let duplicate_2 = self::source_ml_source1_temporary_plan_java_script_array_formal_parameters_loop8(&(names), &(name), duplicate, previous);
                                                        if (self::accept_java_script_source_binding_name(&(token_kind), &(name), duplicate_2, reserved)) {
                                                            {
                                                                let names_2 = {
                                                                    let source_ml_source2_concat_value78 = names;
                                                                    {
                                                                        let source_ml_source2_concat_value79 = vec![name.clone()];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value78), source_ml_source2_concat_value79)
                                                                    }
                                                                };
                                                                {
                                                                    let bindings_2 = {
                                                                        let source_ml_source2_concat_value80 = bindings;
                                                                        {
                                                                            let source_ml_source2_concat_value81 = vec![if rest {
                                                                                1f64
                                                                            } else {
                                                                                0f64
                                                                            }, position, token];
                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value80), source_ml_source2_concat_value81)
                                                                        }
                                                                    };
                                                                    {
                                                                        let index_5 = index_4 + 1f64;
                                                                        {
                                                                            let position_3 = position + 1f64;
                                                                            if index_5 >= (kinds.len() as f64) {
                                                                                self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Return(vec![-1f64, index_5])
                                                                            } else {
                                                                                {
                                                                                    let next_kind = self::read_required_text_fact(kinds, index_5);
                                                                                    {
                                                                                        let next = self::read_required_text_fact(values, index_5);
                                                                                        if (next_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (next == (self::ml_string::Text::from_units(vec![61]))) {
                                                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Return(vec![-3f64, index_5])
                                                                                        } else if rest && !((next_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (next == (self::ml_string::Text::from_units(vec![93])))) {
                                                                                                self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Return(vec![-5f64, index_5])
                                                                                            } else {
                                                                                                if (next_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (next == (self::ml_string::Text::from_units(vec![44]))) {
                                                                                                    {
                                                                                                        let index_6 = index_5 + 1f64;
                                                                                                        {
                                                                                                            (index, names, position, bindings) = (index_6, names_2, position_3, bindings_2);
                                                                                                            continue;
                                                                                                        }
                                                                                                    }
                                                                                                }
{
                                                                                                    if ((next_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (next == (self::ml_string::Text::from_units(vec![93])))) {
                                                                                                        {
                                                                                                            (index, names, position, bindings) = (index_5, names_2, position_3, bindings_2);
                                                                                                            continue;
                                                                                                        }
                                                                                                    }
{
                                                                                                        self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Return(vec![-1f64, index_5])
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
                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Return(vec![-4f64, index_4])
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
            self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Done(index, names, bindings)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_array_formal_parameters_loop9(
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
pub fn source_ml_source1_temporary_plan_java_script_array_formal_parameters_loop5(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut index: f64,
    mut slot: f64,
    mut names: Vec<self::ml_string::Text>,
    mut instructions: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop5Result {
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
                        let source_ml_source2_s90 = if (kind
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
                                                let source_ml_source2_s91 = self::source_ml_source1_temporary_plan_java_script_array_formal_parameters_loop7(kinds, values, reserved, index_2, names, position, bindings);
                                                match source_ml_source2_s91 {
                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Done(index_3, names_2, bindings_2) => {
                                                        if index_3 >= (kinds.len() as f64) {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6Return(vec![-1f64, index_3])
                                                        } else {
                                                            {
                                                                let instructions_2 = {
                                                                    let source_ml_source2_concat_value92 = instructions;
                                                                    {
                                                                        let source_ml_source2_concat_value93 = vec![2f64, slot, open, index_3, ((bindings_2.len() as f64) / 3f64)];
                                                                        {
                                                                            let source_ml_source2_concat_value94 = bindings_2;
                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value92), source_ml_source2_concat_value93), source_ml_source2_concat_value94)
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let index_4 = index_3 + 1f64;
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6Next(index_4, names_2, instructions_2)
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop7Return(source_ml_source1_temporary_result7) => {
                                                        self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6Return(source_ml_source1_temporary_result7)
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
                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6Return(vec![-2f64, index])
                        } else {
                            {
                                let duplicate = false;
                                {
                                    let previous = 0f64;
                                    {
                                        let duplicate_2 = self::source_ml_source1_temporary_plan_java_script_array_formal_parameters_loop9(&(names), &(value), duplicate, previous);
                                        if (self::accept_java_script_source_binding_name(
                                            &(kind),
                                            &(value),
                                            duplicate_2,
                                            reserved,
                                        )) {
                                            {
                                                let names_3 = {
                                                    let source_ml_source2_concat_value95 = names;
                                                    {
                                                        let source_ml_source2_concat_value96 =
                                                            vec![value.clone()];
                                                        self::ml_array::append(
                                                            self::ml_array::append(
                                                                Vec::<self::ml_string::Text>::new(),
                                                                source_ml_source2_concat_value95,
                                                            ),
                                                            source_ml_source2_concat_value96,
                                                        )
                                                    }
                                                };
                                                {
                                                    let instructions_3 = {
                                                        let source_ml_source2_concat_value97 =
                                                            instructions;
                                                        {
                                                            let source_ml_source2_concat_value98 =
                                                                vec![0f64, slot, index];
                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value97), source_ml_source2_concat_value98)
                                                        }
                                                    };
                                                    {
                                                        let index_5 = index + 1f64;
                                                        self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6Next(index_5, names_3, instructions_3)
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6Return(vec![-6f64, index])
                                        }
                                    }
                                }
                            }
                        };
                        match source_ml_source2_s90 {
                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6Next(index_6, names_4, instructions_4) => {
                                {
                                    let slot_2 = slot + 1f64;
                                    if index_6 >= (kinds.len() as f64) {
                                        self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop5Result::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop5Return(vec![-1f64, index_6])
                                    } else {
                                        {
                                            let delimiter_kind = self::read_required_text_fact(kinds, index_6);
                                            {
                                                let delimiter_value = self::read_required_text_fact(values, index_6);
                                                if (delimiter_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (delimiter_value == (self::ml_string::Text::from_units(vec![61]))) {
                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop5Result::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop5Return(vec![-3f64, index_6])
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
                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop5Result::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop5Return(vec![-1f64, index_6])
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParametersJoin6Return(source_ml_source1_temporary_result6) => {
                                self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop5Result::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop5Return(source_ml_source1_temporary_result6)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop5Result::MlSource1TemporaryPlanJavaScriptArrayFormalParametersLoop5Done(index, slot, instructions)
        };
    }
}
