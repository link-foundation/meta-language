// Generated from complete js/src/translation/record-callable-parameter-rules.js. Do not edit by hand.
use super::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15;
use super::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop14Result;
use super::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Result;
use super::accept_java_script_source_binding_name;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop16;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop19(
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
pub fn source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop14(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut defaults: bool,
    mut index: f64,
    mut slot: f64,
    mut names: Vec<self::ml_string::Text>,
    mut instructions: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop14Result {
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
                        let source_ml_source2_s142 = if (kind
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
                                                let source_ml_source2_s143 = self::source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop16(kinds, values, reserved, defaults, index_2, names, position, bindings);
                                                match source_ml_source2_s143 {
                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Done(index_3, names_2, bindings_2) => {
                                                        if index_3 >= (kinds.len() as f64) {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15Return(vec![-1f64, index_3])
                                                        } else {
                                                            {
                                                                let instructions_2 = {
                                                                    let source_ml_source2_concat_value144 = instructions;
                                                                    {
                                                                        let source_ml_source2_concat_value145 = vec![2f64, slot, open, index_3, ((bindings_2.len() as f64) / if defaults {
                                                                            5f64
                                                                        } else {
                                                                            3f64
                                                                        })];
                                                                        {
                                                                            let source_ml_source2_concat_value146 = bindings_2;
                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value144), source_ml_source2_concat_value145), source_ml_source2_concat_value146)
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let index_4 = index_3 + 1f64;
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15Next(index_4, names_2, instructions_2)
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop16Return(source_ml_source1_temporary_result16) => {
                                                        self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15Return(source_ml_source1_temporary_result16)
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
                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15Return(vec![-2f64, index])
                        } else {
                            {
                                let duplicate = false;
                                {
                                    let previous = 0f64;
                                    {
                                        let duplicate_2 = self::source_ml_source1_temporary_plan_java_script_array_formal_parameter_bindings_loop19(&(names), &(value), duplicate, previous);
                                        if (self::accept_java_script_source_binding_name(
                                            &(kind),
                                            &(value),
                                            duplicate_2,
                                            reserved,
                                        )) {
                                            {
                                                let names_3 = {
                                                    let source_ml_source2_concat_value147 = names;
                                                    {
                                                        let source_ml_source2_concat_value148 =
                                                            vec![value.clone()];
                                                        self::ml_array::append(
                                                            self::ml_array::append(
                                                                Vec::<self::ml_string::Text>::new(),
                                                                source_ml_source2_concat_value147,
                                                            ),
                                                            source_ml_source2_concat_value148,
                                                        )
                                                    }
                                                };
                                                {
                                                    let instructions_3 = {
                                                        let source_ml_source2_concat_value149 =
                                                            instructions;
                                                        {
                                                            let source_ml_source2_concat_value150 =
                                                                vec![0f64, slot, index];
                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value149), source_ml_source2_concat_value150)
                                                        }
                                                    };
                                                    {
                                                        let index_5 = index + 1f64;
                                                        self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15Next(index_5, names_3, instructions_3)
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15Return(vec![-6f64, index])
                                        }
                                    }
                                }
                            }
                        };
                        match source_ml_source2_s142 {
                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15Next(index_6, names_4, instructions_4) => {
                                {
                                    let slot_2 = slot + 1f64;
                                    if index_6 >= (kinds.len() as f64) {
                                        self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop14Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop14Return(vec![-1f64, index_6])
                                    } else {
                                        {
                                            let delimiter_kind = self::read_required_text_fact(kinds, index_6);
                                            {
                                                let delimiter_value = self::read_required_text_fact(values, index_6);
                                                if (delimiter_kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (delimiter_value == (self::ml_string::Text::from_units(vec![61]))) {
                                                    self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop14Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop14Return(vec![-3f64, index_6])
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
                                                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop14Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop14Return(vec![-1f64, index_6])
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsJoin15Return(source_ml_source1_temporary_result15) => {
                                self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop14Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop14Return(source_ml_source1_temporary_result15)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop14Result::MlSource1TemporaryPlanJavaScriptArrayFormalParameterBindingsLoop14Done(index, slot, instructions)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop20Result {
    MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop20Done,
    MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop20Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_pattern_default_expression_end_loop20(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut start: f64,
    closing: &self::ml_string::Text,
    mut stack: Vec<self::ml_string::Text>,
    mut index: f64,
) -> self::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop20Result {
    loop {
        return if index < (kinds.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                if kind == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                    self::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop20Result::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop20Return(-1f64)
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
                                self::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop20Result::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop20Return(if (index).partial_cmp(&(start)) == Some(std::cmp::Ordering::Equal) {
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
                                            let source_ml_source2_concat_value151 = stack;
                                            {
                                                let source_ml_source2_concat_value152 =
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
                                                        source_ml_source2_concat_value151,
                                                    ),
                                                    source_ml_source2_concat_value152,
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
                                            self::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop20Result::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop20Return(-1f64)
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
            self::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop20Result::MlSource1TemporaryFindJavaScriptPatternDefaultExpressionEndLoop20Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop22Result {
    MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop22Done(f64, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_formal_default_binding_positions_loop22(
    plan: &[f64],
    mut cursor: f64,
    mut names: Vec<f64>,
    mut count: f64,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop22Result {
    loop {
        if index < count {
            {
                let names_2 = {
                    let source_ml_source2_concat_value153 = names;
                    {
                        let source_ml_source2_concat_value154 =
                            vec![self::read_required_number_fact(plan, (cursor + 2f64))];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value153,
                            ),
                            source_ml_source2_concat_value154,
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
            self::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop22Result::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop22Done(cursor, names)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_formal_default_binding_positions_loop21(
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
                                let source_ml_source2_concat_value155 = names;
                                {
                                    let source_ml_source2_concat_value156 =
                                        vec![self::read_required_number_fact(plan, cursor_2)];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value155,
                                        ),
                                        source_ml_source2_concat_value156,
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
                                            let source_ml_source2_s157 = self::source_ml_source1_temporary_read_java_script_array_formal_default_binding_positions_loop22(plan, cursor_4, names, count, index);
                                            match source_ml_source2_s157 {
                                                self::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop22Result::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop22Done(cursor_5, names_3) => {
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
pub enum MlSource1TemporaryPartitionJavaScriptFormalInitializerBindingPositionsLoop23Result {
    MlSource1TemporaryPartitionJavaScriptFormalInitializerBindingPositionsLoop23Done(
        Vec<f64>,
        Vec<f64>,
        bool,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_partition_java_script_formal_initializer_binding_positions_loop23(
    positions: &[f64],
    mut binding: f64,
    mut available: Vec<f64>,
    mut uninitialized: Vec<f64>,
    mut found: bool,
    mut index: f64,
) -> self::MlSource1TemporaryPartitionJavaScriptFormalInitializerBindingPositionsLoop23Result {
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
                                let source_ml_source2_concat_value158 = available;
                                {
                                    let source_ml_source2_concat_value159 = vec![position];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value158,
                                        ),
                                        source_ml_source2_concat_value159,
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
                                let source_ml_source2_concat_value160 = uninitialized;
                                {
                                    let source_ml_source2_concat_value161 = vec![position];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value160,
                                        ),
                                        source_ml_source2_concat_value161,
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
            self::MlSource1TemporaryPartitionJavaScriptFormalInitializerBindingPositionsLoop23Result::MlSource1TemporaryPartitionJavaScriptFormalInitializerBindingPositionsLoop23Done(available, uninitialized, found)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop24Result {
    MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop24Done,
    MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop24Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop25Result {
    MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop25Done,
    MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop25Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_array_formal_initializer_assignment_loop25(
    assignments: &[self::ml_string::Text],
    parameters: &[self::ml_string::Text],
    mut index: f64,
    mut parameter: f64,
) -> self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop25Result {
    loop {
        return if parameter < (parameters.len() as f64) {
            if (self::read_required_text_fact(assignments, index))
                == (self::read_required_text_fact(parameters, parameter))
            {
                self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop25Result::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop25Return(index)
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
            self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop25Result::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop25Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_array_formal_initializer_assignment_loop24(
    assignments: &[self::ml_string::Text],
    parameters: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop24Result {
    loop {
        return if index < (assignments.len() as f64) {
            {
                let parameter = 0f64;
                {
                    let source_ml_source2_s162 = self::source_ml_source1_temporary_find_java_script_array_formal_initializer_assignment_loop25(assignments, parameters, index, parameter);
                    match source_ml_source2_s162 {
                        self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop25Result::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop25Done => {
                            {
                                let index_2 = index + 1f64;
                                {
                                    index = index_2;
                                    continue;
                                }
                            }
                        }
                        self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop25Result::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop25Return(source_ml_source1_temporary_result25) => {
                            self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop24Result::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop24Return(source_ml_source1_temporary_result25)
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop24Result::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop24Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Result {
    MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Done,
    MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_record_parameter_pattern_loop26(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut depth: f64,
    mut binding: bool,
    mut at: f64,
) -> self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Result {
    loop {
        return if at < (kinds.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, at);
                if kind == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                    self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Return(-1f64)
                } else {
                    if kind != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) {
                        {
                            let binding_3 = if depth == 0f64 { { false } } else { binding };
                            {
                                let at_2 = at + 1f64;
                                {
                                    (binding, at) = (binding_3, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let value = self::read_required_text_fact(values, at);
                            if (depth == 0f64)
                                && (value == (self::ml_string::Text::from_units(vec![41])))
                            {
                                self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Return(-1f64)
                            } else if ((depth == 0f64) && binding)
                                && (value == (self::ml_string::Text::from_units(vec![123])))
                            {
                                self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Return(at)
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
                                                let at_3 = at + 1f64;
                                                {
                                                    (depth, binding, at) =
                                                        (depth_2, binding_4, at_3);
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
                                                self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Return(-1f64)
                                            } else {
                                                {
                                                    let at_4 = at + 1f64;
                                                    {
                                                        (depth, at) = (depth_3, at_4);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if (depth == 0f64)
                                            && (value
                                                == (self::ml_string::Text::from_units(vec![44])))
                                        {
                                            {
                                                let binding_5 = true;
                                                {
                                                    let at_5 = at + 1f64;
                                                    {
                                                        (binding, at) = (binding_5, at_5);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            if (depth == 0f64)
                                                && (value
                                                    != (self::ml_string::Text::from_units(vec![
                                                        44,
                                                    ])))
                                            {
                                                {
                                                    let binding_6 = false;
                                                    {
                                                        let at_6 = at + 1f64;
                                                        {
                                                            (binding, at) = (binding_6, at_6);
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                            {
                                                {
                                                    let at_7 = at + 1f64;
                                                    {
                                                        at = at_7;
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
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_java_script_record_parameter_binding_loop27(
    name: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    mut duplicate: bool,
    mut index: f64,
) -> bool {
    loop {
        if index < (names.len() as f64) {
            if (self::read_required_text_fact(names, index)) == (*name) {
                {
                    let duplicate_2 = true;
                    {
                        let index_2 = index + 1f64;
                        {
                            (duplicate, index) = (duplicate_2, index_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
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
pub fn source_ml_source1_temporary_accept_java_script_record_parameter_bound_name_loop28(
    name: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    mut duplicate: bool,
    mut index: f64,
) -> bool {
    loop {
        if index < (names.len() as f64) {
            if (self::read_required_text_fact(names, index)) == (*name) {
                {
                    let duplicate_2 = true;
                    {
                        let index_2 = index + 1f64;
                        {
                            (duplicate, index) = (duplicate_2, index_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        }
        return { duplicate };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result {
    MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Done(f64, Vec<f64>),
    MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31 {
    MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31Next(f64, f64, f64),
    MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30 {
    MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30Next(f64, f64),
    MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30Return(Vec<f64>),
}
