// Generated from complete js/src/translation/recursive-binding-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop13Result;
use super::accept_java_script_source_binding_name;
use super::find_java_script_pattern_default_expression_end;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_object_binding_tree;
use super::read_required_number_fact;
use super::read_required_text_fact;

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
                    let source_ml_source2_concat_value133 = names;
                    {
                        let source_ml_source2_concat_value134 =
                            vec![self::read_required_number_fact(plan, (cursor + 2f64))];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value133,
                            ),
                            source_ml_source2_concat_value134,
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
                                let source_ml_source2_concat_value135 = names;
                                {
                                    let source_ml_source2_concat_value136 =
                                        vec![self::read_required_number_fact(plan, cursor_2)];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value135,
                                        ),
                                        source_ml_source2_concat_value136,
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
                                            let source_ml_source2_s137 = self::source_ml_source1_temporary_read_java_script_array_formal_default_binding_positions_loop13(plan, cursor_4, names, count, index);
                                            match source_ml_source2_s137 {
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
                                let source_ml_source2_concat_value138 = available;
                                {
                                    let source_ml_source2_concat_value139 = vec![position];
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
                                let source_ml_source2_concat_value140 = uninitialized;
                                {
                                    let source_ml_source2_concat_value141 = vec![position];
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
                    let source_ml_source2_s142 = self::source_ml_source1_temporary_find_java_script_array_formal_initializer_assignment_loop16(assignments, parameters, index, parameter);
                    match source_ml_source2_s142 {
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
                                let source_ml_source2_concat_value143 = rows;
                                {
                                    let source_ml_source2_concat_value144 = vec![
                                        self::read_required_number_fact(children, offset),
                                        parent,
                                    ];
                                    {
                                        let source_ml_source2_concat_value145 =
                                            self::ml_array::slice(
                                                children,
                                                (offset + 2f64),
                                                (offset + 8f64),
                                            );
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source2_concat_value143,
                                                ),
                                                source_ml_source2_concat_value144,
                                            ),
                                            source_ml_source2_concat_value145,
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_object_binding_tree_loop17(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut depth: f64,
    mut maximum_depth: f64,
    mut arrays: bool,
    mut at: f64,
    mut rows: Vec<f64>,
    mut properties: f64,
) -> self::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result {
    loop {
        return if (at < (kinds.len() as f64))
            && !(((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![125]))))
        {
            {
                let property = at;
                if ((self::read_required_text_fact(kinds, at))
                    == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                    && ((self::read_required_text_fact(values, at))
                        == (self::ml_string::Text::from_units(vec![46, 46, 46])))
                {
                    {
                        let binding = at + 1f64;
                        if binding >= (kinds.len() as f64) {
                            self::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Return(vec![-1f64, binding])
                        } else if (self::accept_java_script_source_binding_name(
                            &(self::read_required_text_fact(kinds, binding)),
                            &(self::read_required_text_fact(values, binding)),
                            false,
                            reserved,
                        )) {
                            {
                                let at_2 = binding + 1f64;
                                if ((at_2 >= (kinds.len() as f64))
                                    || ((self::read_required_text_fact(kinds, at_2))
                                        != (self::ml_string::Text::from_units(vec![
                                            112, 117, 110, 99, 116,
                                        ]))))
                                    || ((self::read_required_text_fact(values, at_2))
                                        != (self::ml_string::Text::from_units(vec![125])))
                                {
                                    self::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Return(vec![-6f64, at_2])
                                } else {
                                    {
                                        let rows_2 = {
                                            let source_ml_source2_concat_value146 = rows;
                                            {
                                                let source_ml_source2_concat_value147 = vec![
                                                    2f64, 0f64, property, binding, -1f64, -1f64,
                                                    -1f64, -1f64,
                                                ];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value146,
                                                    ),
                                                    source_ml_source2_concat_value147,
                                                )
                                            }
                                        };
                                        {
                                            let properties_2 = properties + 1f64;
                                            self::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Done(at_2, rows_2, properties_2)
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Return(vec![-8f64, binding])
                        }
                    }
                } else {
                    let ml_library_inequality_1 = (self::read_required_text_fact(kinds, at))
                        != (self::ml_string::Text::from_units(vec![
                            105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                        ]));
                    if ml_library_inequality_1 {
                        self::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Return(vec![-7f64, at])
                    } else {
                        {
                            let at_3 = at + 1f64;
                            if at_3 >= (kinds.len() as f64) {
                                self::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Return(vec![-1f64, at_3])
                            } else {
                                {
                                    let binding_2 = property;
                                    {
                                        let open = -1f64;
                                        {
                                            let close = -1f64;
                                            {
                                                let children = Vec::<f64>::new();
                                                {
                                                    let source_ml_source2_s148 = if ((self::read_required_text_fact(kinds, at_3)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_3)) == (self::ml_string::Text::from_units(vec![58]))) {
                                                        {
                                                            let at_4 = at_3 + 1f64;
                                                            if at_4 >= (kinds.len() as f64) {
                                                                self::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18Return(vec![-1f64, at_4])
                                                            } else if ((self::read_required_text_fact(kinds, at_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![123]))) || (arrays && ((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![91]))))) {
                                                                    {
                                                                        let open_2 = at_4;
                                                                        {
                                                                            let children_2 = self::read_java_script_object_binding_tree(kinds, values, at_4, reserved, (depth + 1f64), maximum_depth, arrays);
                                                                            if (self::read_required_number_fact(&(children_2), 0f64)) < 0f64 {
                                                                                self::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18Return(children_2)
                                                                            } else {
                                                                                {
                                                                                    let at_5 = self::read_required_number_fact(&(children_2), 0f64);
                                                                                    {
                                                                                        let close_2 = at_5 - 1f64;
                                                                                        {
                                                                                            let binding_3 = -1f64;
                                                                                            self::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18Next(at_5, binding_3, open_2, close_2, children_2)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else if ((self::read_required_text_fact(kinds, at_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![91]))) {
                                                                        self::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18Return(vec![-5f64, at_4])
                                                                    } else {
                                                                        {
                                                                            let binding_4 = at_4;
                                                                            {
                                                                                let at_6 = at_4 + 1f64;
                                                                                self::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18Next(at_6, binding_4, open, close, children)
                                                                            }
                                                                        }
                                                                    }
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18Next(at_3, binding_2, open, close, children)
                                                    };
                                                    match source_ml_source2_s148 {
                                                        self::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18Next(at_7, binding_5, open_3, close_3, children_3) => {
                                                            if (binding_5 >= 0f64) && !(self::accept_java_script_source_binding_name(&(self::read_required_text_fact(kinds, binding_5)), &(self::read_required_text_fact(values, binding_5)), false, reserved)) {
                                                                self::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Return(vec![-8f64, binding_5])
                                                            } else if at_7 >= (kinds.len() as f64) {
                                                                    self::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Return(vec![-1f64, at_7])
                                                                } else {
                                                                    {
                                                                        let first = -1f64;
                                                                        {
                                                                            let last = -1f64;
                                                                            {
                                                                                let source_ml_source2_s149 = if ((self::read_required_text_fact(kinds, at_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_7)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                                                    {
                                                                                        let first_2 = at_7 + 1f64;
                                                                                        {
                                                                                            let last_2 = self::find_java_script_pattern_default_expression_end(kinds, values, first_2, &(self::ml_string::Text::from_units(vec![125])));
                                                                                            if last_2 < 0f64 {
                                                                                                self::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin19::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin19Return(vec![-1f64, first_2])
                                                                                            } else {
                                                                                                {
                                                                                                    let at_8 = last_2;
                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin19::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin19Next(at_8, first_2, last_2)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin19::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin19Next(at_7, first, last)
                                                                                };
                                                                                match source_ml_source2_s149 {
                                                                                    self::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin19::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin19Next(at_9, first_3, last_3) => {
                                                                                        {
                                                                                            let owner = (rows.len() as f64) / 8f64;
                                                                                            {
                                                                                                let rows_3 = {
                                                                                                    let source_ml_source2_concat_value150 = rows;
                                                                                                    {
                                                                                                        let source_ml_source2_concat_value151 = vec![1f64, 0f64, property, binding_5, first_3, last_3, open_3, close_3];
                                                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value150), source_ml_source2_concat_value151)
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let properties_3 = properties + 1f64;
                                                                                                    {
                                                                                                        let rows_5 = {
let ml_library_condition_2 =  { let ml_value = children_3.len() as f64; ml_value != 0f64 && !ml_value.is_nan() };
if ml_library_condition_2 {
                                                                                                            {
                                                                                                                let count = self::read_required_number_fact(&(children_3), 1f64);
                                                                                                                {
                                                                                                                    let base = (rows_3.len() as f64) / 8f64;
                                                                                                                    {
                                                                                                                        let index = 1f64;
                                                                                                                        {
                                                                                                                            self::source_ml_source1_temporary_read_java_script_object_binding_tree_loop20(rows_3, &(children_3), owner, count, base, index)
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        } else {
                                                                                                            rows_3
                                                                                                        }
};
                                                                                                        if ((self::read_required_text_fact(kinds, at_9)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_9)) == (self::ml_string::Text::from_units(vec![44]))) {
                                                                                                            {
                                                                                                                let at_10 = at_9 + 1f64;
                                                                                                                {
                                                                                                                    (at, rows, properties) = (at_10, rows_5, properties_3);
                                                                                                                    continue;
                                                                                                                }
                                                                                                            }
                                                                                                        }
{
                                                                                                            if (((self::read_required_text_fact(kinds, at_9)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_9)) == (self::ml_string::Text::from_units(vec![125])))) {
                                                                                                                {
                                                                                                                    (at, rows, properties) = (at_9, rows_5, properties_3);
                                                                                                                    continue;
                                                                                                                }
                                                                                                            }
{
                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Return(vec![-1f64, at_9])
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin19::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin19Return(source_ml_source1_temporary_result20) => {
                                                                                        self::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Return(source_ml_source1_temporary_result20)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18::MlSource1TemporaryReadJavaScriptObjectBindingTreeJoin18Return(source_ml_source1_temporary_result20_2) => {
                                                            self::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Return(source_ml_source1_temporary_result20_2)
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
            self::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Done(at, rows, properties)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_recursive_object_binding_positions_loop21(
    plan: &[f64],
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (self::read_required_number_fact(plan, 1f64)) {
            {
                let binding =
                    self::read_required_number_fact(plan, ((2f64 + (index * 8f64)) + 3f64));
                if binding >= 0f64 {
                    {
                        let result_2 = {
                            let source_ml_source2_concat_value152 = result;
                            {
                                let source_ml_source2_concat_value153 = vec![binding];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value152,
                                    ),
                                    source_ml_source2_concat_value153,
                                )
                            }
                        };
                        {
                            let index_2 = index + 1f64;
                            {
                                (result, index) = (result_2, index_2);
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
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecursiveObjectBindingsLoop22Result {
    MlSource1TemporaryPlanJavaScriptRecursiveObjectBindingsLoop22Done,
    MlSource1TemporaryPlanJavaScriptRecursiveObjectBindingsLoop22Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_recursive_object_bindings_loop22(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    positions: &[f64],
    mut names: Vec<self::ml_string::Text>,
    mut index: f64,
) -> self::MlSource1TemporaryPlanJavaScriptRecursiveObjectBindingsLoop22Result {
    loop {
        return if index < (positions.len() as f64) {
            {
                let position = self::read_required_number_fact(positions, index);
                {
                    let name = self::read_required_text_fact(values, position);
                    if (self::accept_java_script_source_binding_name(
                        &(self::read_required_text_fact(kinds, position)),
                        &(name),
                        self::ml_array::includes(&names, &name.clone(), 0f64, false, |a, b| a == b),
                        reserved,
                    )) {
                        {
                            let names_2 = {
                                let source_ml_source2_concat_value154 = names;
                                {
                                    let source_ml_source2_concat_value155 = vec![name.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::ml_string::Text>::new(),
                                            source_ml_source2_concat_value154,
                                        ),
                                        source_ml_source2_concat_value155,
                                    )
                                }
                            };
                            {
                                let index_2 = index + 1f64;
                                {
                                    (names, index) = (names_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        self::MlSource1TemporaryPlanJavaScriptRecursiveObjectBindingsLoop22Result::MlSource1TemporaryPlanJavaScriptRecursiveObjectBindingsLoop22Return(vec![-8f64, position])
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptRecursiveObjectBindingsLoop22Result::MlSource1TemporaryPlanJavaScriptRecursiveObjectBindingsLoop22Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_recursive_object_excluded_positions_loop23(
    plan: &[f64],
    mut owner: f64,
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (self::read_required_number_fact(plan, 1f64)) {
            {
                let at = 2f64 + (index * 8f64);
                if ((self::read_required_number_fact(plan, at)) == 1f64)
                    && ((self::read_required_number_fact(plan, (at + 1f64))) == owner)
                {
                    {
                        let result_2 = {
                            let source_ml_source2_concat_value156 = result;
                            {
                                let source_ml_source2_concat_value157 =
                                    vec![self::read_required_number_fact(plan, (at + 2f64))];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value156,
                                    ),
                                    source_ml_source2_concat_value157,
                                )
                            }
                        };
                        {
                            let index_2 = index + 1f64;
                            {
                                (result, index) = (result_2, index_2);
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
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptRecursiveObjectInitializerBindingLoop24Result {
    MlSource1TemporaryReadJavaScriptRecursiveObjectInitializerBindingLoop24Done,
    MlSource1TemporaryReadJavaScriptRecursiveObjectInitializerBindingLoop24Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_recursive_object_initializer_binding_loop24(
    mut open: f64,
    mut close: f64,
    positions: &[f64],
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptRecursiveObjectInitializerBindingLoop24Result {
    loop {
        return if index < (positions.len() as f64) {
            {
                let position = self::read_required_number_fact(positions, index);
                if (position > open) && (position < close) {
                    self::MlSource1TemporaryReadJavaScriptRecursiveObjectInitializerBindingLoop24Result::MlSource1TemporaryReadJavaScriptRecursiveObjectInitializerBindingLoop24Return(position)
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
            self::MlSource1TemporaryReadJavaScriptRecursiveObjectInitializerBindingLoop24Result::MlSource1TemporaryReadJavaScriptRecursiveObjectInitializerBindingLoop24Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryContainsJavaScriptRecursiveObjectBindingLoop25Result {
    MlSource1TemporaryContainsJavaScriptRecursiveObjectBindingLoop25Done,
    MlSource1TemporaryContainsJavaScriptRecursiveObjectBindingLoop25Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_contains_java_script_recursive_object_binding_loop25(
    plan: &[f64],
    mut index: f64,
) -> self::MlSource1TemporaryContainsJavaScriptRecursiveObjectBindingLoop25Result {
    loop {
        return if index < (self::read_required_number_fact(plan, 1f64)) {
            if (self::read_required_number_fact(plan, ((2f64 + (index * 8f64)) + 6f64))) >= 0f64 {
                self::MlSource1TemporaryContainsJavaScriptRecursiveObjectBindingLoop25Result::MlSource1TemporaryContainsJavaScriptRecursiveObjectBindingLoop25Return(true)
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
            self::MlSource1TemporaryContainsJavaScriptRecursiveObjectBindingLoop25Result::MlSource1TemporaryContainsJavaScriptRecursiveObjectBindingLoop25Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Result {
    MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Done(f64, f64, Vec<f64>),
    MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27 {
    MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27Next(
        f64,
        Vec<self::ml_string::Text>,
        Vec<f64>,
    ),
    MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27Return(Vec<f64>),
}
