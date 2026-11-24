// Generated from complete js/src/translation/whole-parameter-default-rules.js. Do not edit by hand.
use super::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6;
use super::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop5Result;
use super::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Result;
use super::accept_java_script_source_binding_name;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop7;
use super::source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop10;

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
                        let source_ml_source2_s127 = if (kind
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
                                                let source_ml_source2_s128 = self::source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop7(kinds, values, reserved, defaults, index_2, names, position, bindings);
                                                match source_ml_source2_s128 {
                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop7Done(index_3, names_2, bindings_2) => {
                                                        if index_3 >= (kinds.len() as f64) {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin6Return(vec![-1f64, index_3])
                                                        } else {
                                                            {
                                                                let instructions_2 = {
                                                                    let source_ml_source2_concat_value129 = instructions;
                                                                    {
                                                                        let source_ml_source2_concat_value130 = vec![2f64, slot, open, index_3, ((bindings_2.len() as f64) / if defaults {
                                                                            5f64
                                                                        } else {
                                                                            3f64
                                                                        })];
                                                                        {
                                                                            let source_ml_source2_concat_value131 = bindings_2;
                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value129), source_ml_source2_concat_value130), source_ml_source2_concat_value131)
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
                                                    let source_ml_source2_concat_value132 = names;
                                                    {
                                                        let source_ml_source2_concat_value133 =
                                                            vec![value.clone()];
                                                        self::ml_array::append(
                                                            self::ml_array::append(
                                                                Vec::<self::ml_string::Text>::new(),
                                                                source_ml_source2_concat_value132,
                                                            ),
                                                            source_ml_source2_concat_value133,
                                                        )
                                                    }
                                                };
                                                {
                                                    let instructions_3 = {
                                                        let source_ml_source2_concat_value134 =
                                                            instructions;
                                                        {
                                                            let source_ml_source2_concat_value135 =
                                                                vec![0f64, slot, index];
                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value134), source_ml_source2_concat_value135)
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
                        match source_ml_source2_s127 {
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
pub enum MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Result {
    MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Done,
    MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_pattern_default_expression_end_loop11(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut start: f64,
    closing: &self::ml_string::Text,
    mut stack: Vec<self::ml_string::Text>,
    mut index: f64,
) -> self::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Result {
    loop {
        return if index < (kinds.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                if kind == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                    self::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Result::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Return(-1f64)
                } else {
                    if kind != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let value = self::read_required_text_fact(values, index);
                            if ((stack.len() as f64) == 0f64)
                                && ((value == (self::ml_string::Text::from_units(vec![44])))
                                    || (value == (*closing)))
                            {
                                self::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Result::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Return(if (index).partial_cmp(&(start)) == Some(std::cmp::Ordering::Equal) {
                                    -1f64
                                } else {
                                    index
                                })
                            } else {
                                if ((value == (self::ml_string::Text::from_units(vec![40])))
                                    || (value == (self::ml_string::Text::from_units(vec![91]))))
                                    || (value == (self::ml_string::Text::from_units(vec![123])))
                                {
                                    {
                                        let stack_2 = {
                                            let source_ml_source2_concat_value136 = stack;
                                            {
                                                let source_ml_source2_concat_value137 =
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
                                                        source_ml_source2_concat_value136,
                                                    ),
                                                    source_ml_source2_concat_value137,
                                                )
                                            }
                                        };
                                        {
                                            let index_3 = index + 1f64;
                                            {
                                                (stack, index) = (stack_2, index_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    if ((value == (self::ml_string::Text::from_units(vec![41])))
                                        || (value == (self::ml_string::Text::from_units(vec![93]))))
                                        || (value == (self::ml_string::Text::from_units(vec![125])))
                                    {
                                        if ((stack.len() as f64) == 0f64)
                                            || ((self::read_required_text_fact(
                                                &(stack),
                                                ((stack.len() as f64) - 1f64),
                                            )) != value)
                                        {
                                            self::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Result::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Return(-1f64)
                                        } else {
                                            {
                                                let stack_3 =
                                                    self::ml_array::slice(&stack, 0f64, -1f64);
                                                {
                                                    let index_4 = index + 1f64;
                                                    {
                                                        (stack, index) = (stack_3, index_4);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let index_5 = index + 1f64;
                                            {
                                                index = index_5;
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
        } else {
            self::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Result::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop13Result {
    MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop13Done(f64, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_formal_default_binding_positions_loop13(
    plan: &[f64],
    mut cursor: f64,
    mut names: Vec<f64>,
    mut count: f64,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop13Result {
    loop {
        if index < count {
            {
                let names_2 = {
                    let source_ml_source2_concat_value138 = names;
                    {
                        let source_ml_source2_concat_value139 =
                            vec![self::read_required_number_fact(plan, (cursor + 2f64))];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value138,
                            ),
                            source_ml_source2_concat_value139,
                        )
                    }
                };
                {
                    let cursor_2 = cursor + 5f64;
                    {
                        let index_2 = index + 1f64;
                        {
                            (cursor, names, index) = (cursor_2, names_2, index_2);
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop13Result::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop13Done(cursor, names)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_formal_default_binding_positions_loop12(
    plan: &[f64],
    mut cursor: f64,
    mut names: Vec<f64>,
) -> Vec<f64> {
    loop {
        return if cursor < (plan.len() as f64) {
            {
                let form = self::read_required_number_fact(plan, cursor);
                {
                    let cursor_2 = cursor + 2f64;
                    if form == 0f64 {
                        {
                            let names_2 = {
                                let source_ml_source2_concat_value140 = names;
                                {
                                    let source_ml_source2_concat_value141 =
                                        vec![self::read_required_number_fact(plan, cursor_2)];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value140,
                                        ),
                                        source_ml_source2_concat_value141,
                                    )
                                }
                            };
                            {
                                let cursor_3 = cursor_2 + 1f64;
                                {
                                    (cursor, names) = (cursor_3, names_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if form == 2f64 {
                            {
                                let count =
                                    self::read_required_number_fact(plan, (cursor_2 + 2f64));
                                {
                                    let cursor_4 = cursor_2 + 3f64;
                                    {
                                        let index = 0f64;
                                        {
                                            let source_ml_source2_s142 = self::source_ml_source1_temporary_read_java_script_array_formal_default_binding_positions_loop13(plan, cursor_4, names, count, index);
                                            match source_ml_source2_s142 {
                                                self::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop13Result::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop13Done(cursor_5, names_3) => {
                                                    {
                                                        (cursor, names) = (cursor_5, names_3);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("RangeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 97, 114, 114, 97, 121, 32, 100, 101, 102, 97, 117,
                                    108, 116, 32, 98, 105, 110, 100, 105, 110, 103, 32, 99, 101,
                                    110, 115, 117, 115, 32, 110, 101, 101, 100, 115, 32, 97, 99,
                                    116, 117, 97, 108, 32, 102, 111, 114, 109, 97, 108, 32, 105,
                                    110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    }
                }
            }
        } else {
            names
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPartitionJavaScriptFormalInitializerBindingPositionsLoop14Result {
    MlSource1TemporaryPartitionJavaScriptFormalInitializerBindingPositionsLoop14Done(
        Vec<f64>,
        Vec<f64>,
        bool,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_partition_java_script_formal_initializer_binding_positions_loop14(
    positions: &[f64],
    mut binding: f64,
    mut available: Vec<f64>,
    mut uninitialized: Vec<f64>,
    mut found: bool,
    mut index: f64,
) -> self::MlSource1TemporaryPartitionJavaScriptFormalInitializerBindingPositionsLoop14Result {
    loop {
        if index < (positions.len() as f64) {
            {
                let position = self::read_required_number_fact(positions, index);
                {
                    let found_3 =
                        if (position).partial_cmp(&(binding)) == Some(std::cmp::Ordering::Equal) {
                            { true }
                        } else {
                            found
                        };
                    if position < binding {
                        {
                            let available_2 = {
                                let source_ml_source2_concat_value143 = available;
                                {
                                    let source_ml_source2_concat_value144 = vec![position];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value143,
                                        ),
                                        source_ml_source2_concat_value144,
                                    )
                                }
                            };
                            {
                                let index_2 = index + 1f64;
                                {
                                    (available, found, index) = (available_2, found_3, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let uninitialized_2 = {
                                let source_ml_source2_concat_value145 = uninitialized;
                                {
                                    let source_ml_source2_concat_value146 = vec![position];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value145,
                                        ),
                                        source_ml_source2_concat_value146,
                                    )
                                }
                            };
                            {
                                let index_3 = index + 1f64;
                                {
                                    (uninitialized, found, index) =
                                        (uninitialized_2, found_3, index_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryPartitionJavaScriptFormalInitializerBindingPositionsLoop14Result::MlSource1TemporaryPartitionJavaScriptFormalInitializerBindingPositionsLoop14Done(available, uninitialized, found)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop15Result {
    MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop15Done,
    MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop15Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop16Result {
    MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop16Done,
    MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop16Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_array_formal_initializer_assignment_loop16(
    assignments: &[self::ml_string::Text],
    parameters: &[self::ml_string::Text],
    mut index: f64,
    mut parameter: f64,
) -> self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop16Result {
    loop {
        return if parameter < (parameters.len() as f64) {
            if (self::read_required_text_fact(assignments, index))
                == (self::read_required_text_fact(parameters, parameter))
            {
                self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop16Result::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop16Return(index)
            } else {
                {
                    let parameter_2 = parameter + 1f64;
                    {
                        parameter = parameter_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop16Result::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop16Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_array_formal_initializer_assignment_loop15(
    assignments: &[self::ml_string::Text],
    parameters: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop15Result {
    loop {
        return if index < (assignments.len() as f64) {
            {
                let parameter = 0f64;
                {
                    let source_ml_source2_s147 = self::source_ml_source1_temporary_find_java_script_array_formal_initializer_assignment_loop16(assignments, parameters, index, parameter);
                    match source_ml_source2_s147 {
                        self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop16Result::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop16Done => {
                            {
                                let index_2 = index + 1f64;
                                {
                                    index = index_2;
                                    continue;
                                }
                            }
                        }
                        self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop16Result::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop16Return(source_ml_source1_temporary_result16) => {
                            self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop15Result::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop15Return(source_ml_source1_temporary_result16)
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop15Result::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop15Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result {
    MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Done(f64, Vec<f64>, f64),
    MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_object_binding_tree_loop20(
    mut rows: Vec<f64>,
    children: &[f64],
    mut owner: f64,
    mut count: f64,
    mut base: f64,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < count {
            {
                let offset = 2f64 + (index * 8f64);
                {
                    let old = self::read_required_number_fact(children, (offset + 1f64));
                    {
                        let parent = if old == 0f64 {
                            owner
                        } else {
                            (base + old) - 1f64
                        };
                        {
                            let rows_2 = {
                                let source_ml_source2_concat_value148 = rows;
                                {
                                    let source_ml_source2_concat_value149 = vec![
                                        self::read_required_number_fact(children, offset),
                                        parent,
                                    ];
                                    {
                                        let source_ml_source2_concat_value150 =
                                            self::ml_array::slice(
                                                children,
                                                (offset + 2f64),
                                                (offset + 8f64),
                                            );
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source2_concat_value148,
                                                ),
                                                source_ml_source2_concat_value149,
                                            ),
                                            source_ml_source2_concat_value150,
                                        )
                                    }
                                }
                            };
                            {
                                let index_2 = index + 1f64;
                                {
                                    (rows, index) = (rows_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { rows };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin19 {
    MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin19Next(f64, f64, f64),
    MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin19Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18 {
    MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18Next(f64, f64, f64, f64, Vec<f64>),
    MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18Return(Vec<f64>),
}
