// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptClosedClassDirectoryJoin12;
use super::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Result;
use super::find_java_script_class_group_end;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_java_script_closed_class_parameter_positions;
use super::read_java_script_closed_class_static_target;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop5(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut end: f64,
    mut constructor: Vec<f64>,
    mut methods: Vec<f64>,
    mut names: Vec<self::ml_string::Text>,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Result {
    loop {
        return if at < end {
            if ((self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![59]))
            {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                {
                    let static_member = ((((self::ml_array::optional_at(
                        values,
                        self::ml_array::number_index(at),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                        == (self::ml_string::Text::from_units(vec![115, 116, 97, 116, 105, 99])))
                        && (((self::ml_array::optional_at(
                            kinds,
                            self::ml_array::number_index(at + 1f64),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                            == (self::ml_string::Text::from_units(vec![
                                105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                            ]))))
                        && (((self::ml_array::optional_at(
                            values,
                            self::ml_array::number_index(at + 2f64),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                            == (self::ml_string::Text::from_units(vec![40])));
                    {
                        let position = if static_member { at + 1f64 } else { at };
                        {
                            let name = (self::ml_array::optional_at(
                                values,
                                self::ml_array::number_index(position),
                            ))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                            {
                                let owner = (if static_member {
                                    self::ml_string::Text::from_units(vec![
                                        115, 116, 97, 116, 105, 99, 58,
                                    ])
                                } else {
                                    self::ml_string::Text::from_units(vec![
                                        105, 110, 115, 116, 97, 110, 99, 101, 58,
                                    ])
                                })
                                .concat(&name);
                                if ((((((((((((self::ml_array::optional_at(
                                    kinds,
                                    self::ml_array::number_index(position),
                                ))
                                .unwrap_or_else(|| {
                                    self::ml_string::Text::from_units(vec![])
                                })) != (self::ml_string::Text::from_units(
                                    vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114],
                                ))) || (name
                                    == (self::ml_string::Text::from_units(vec![
                                        115, 116, 97, 116, 105, 99,
                                    ]))))
                                    || (name
                                        == (self::ml_string::Text::from_units(vec![
                                            103, 101, 116,
                                        ]))))
                                    || (name
                                        == (self::ml_string::Text::from_units(vec![
                                            115, 101, 116,
                                        ]))))
                                    || (name
                                        == (self::ml_string::Text::from_units(vec![
                                            97, 115, 121, 110, 99,
                                        ]))))
                                    || (name
                                        == (self::ml_string::Text::from_units(vec![
                                            95, 95, 112, 114, 111, 116, 111, 95, 95,
                                        ]))))
                                    || (name
                                        == (self::ml_string::Text::from_units(vec![
                                            112, 114, 111, 116, 111, 116, 121, 112, 101,
                                        ]))))
                                    || (name == (self::ml_string::Text::from_units(vec![36]))))
                                    || (static_member
                                        && (name
                                            == (self::ml_string::Text::from_units(vec![
                                                99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114,
                                            ])))))
                                    || self::ml_array::includes(
                                        &names,
                                        &owner.clone(),
                                        0f64,
                                        false,
                                        |a, b| a == b,
                                    ))
                                    || (((self::ml_array::optional_at(
                                        values,
                                        self::ml_array::number_index(position + 1f64),
                                    ))
                                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                                        != (self::ml_string::Text::from_units(vec![40])))
                                {
                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Return(vec![-2f64, at])
                                } else {
                                    {
                                        let parameters = self::find_java_script_class_group_end(
                                            values,
                                            (position + 1f64),
                                        );
                                        if (parameters < 0f64)
                                            || (((self::ml_array::optional_at(
                                                values,
                                                self::ml_array::number_index(parameters + 1f64),
                                            ))
                                            .unwrap_or_else(|| {
                                                self::ml_string::Text::from_units(vec![])
                                            })) != (self::ml_string::Text::from_units(vec![
                                                123,
                                            ])))
                                        {
                                            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Return(vec![-2f64, at])
                                        } else {
                                            {
                                                let positions = self::read_java_script_closed_class_parameter_positions(values, kinds, (position + 1f64), parameters);
                                                if ((self::ml_array::optional_at(
                                                    &positions,
                                                    self::ml_array::number_index(0f64),
                                                ))
                                                .unwrap_or_else(|| 0f64))
                                                    < 0f64
                                                {
                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Return(vec![-3f64, (self::ml_array::optional_at(&positions, self::ml_array::number_index(1f64))).unwrap_or_else(|| at)])
                                                } else {
                                                    {
                                                        let body =
                                                            self::find_java_script_class_group_end(
                                                                values,
                                                                (parameters + 1f64),
                                                            );
                                                        if (body < 0f64) || (body > end) {
                                                            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Return(vec![-2f64, at])
                                                        } else {
                                                            {
                                                                let names_2 = {
                                                                    let source_ml_source1_concat_value94 =
                                                                        names;
                                                                    {
                                                                        let source_ml_source1_concat_value95 =
                                                                            vec![owner.clone()];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value94), source_ml_source1_concat_value95)
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_s98 = if (name == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))) && !static_member {
                                                                        {
                                                                            let constructor_2 = vec![position, (position + 1f64), parameters, (parameters + 1f64), body];
                                                                            self::MlTemporaryReadJavaScriptClosedClassDirectoryJoin12::MlTemporaryReadJavaScriptClosedClassDirectoryJoin12Next(constructor_2, methods)
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let methods_2 = {
                                                                                let source_ml_source1_concat_value99 = methods;
                                                                                {
                                                                                    let source_ml_source1_concat_value100 = vec![if static_member {
                                                                                        (-position) - 1f64
                                                                                    } else {
                                                                                        position
                                                                                    }, (position + 1f64), parameters, (parameters + 1f64), body];
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value99), source_ml_source1_concat_value100)
                                                                                }
                                                                            };
                                                                            self::MlTemporaryReadJavaScriptClosedClassDirectoryJoin12::MlTemporaryReadJavaScriptClosedClassDirectoryJoin12Next(constructor, methods_2)
                                                                        }
                                                                    };
                                                                    match source_ml_source1_s98 {
                                                                        self::MlTemporaryReadJavaScriptClosedClassDirectoryJoin12::MlTemporaryReadJavaScriptClosedClassDirectoryJoin12Next(constructor_3, methods_3) => {
                                                                            {
                                                                                let at_3 = body + 1f64;
                                                                                {
                                                                                    (constructor, methods, names, at) = (constructor_3, methods_3, names_2, at_3);
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
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Done(constructor, methods, names)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryRenderJavaScriptClosedClassRangeLoop13Result {
    MlTemporaryRenderJavaScriptClosedClassRangeLoop13Done(self::ml_string::Text, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_render_java_script_closed_class_range_loop13(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    mut last: f64,
    receiver: &self::ml_string::Text,
    class_names: &[self::ml_string::Text],
    static_calls: &[self::ml_string::Text],
    mut result: self::ml_string::Text,
    mut previous: f64,
    mut at: f64,
) -> self::MlTemporaryRenderJavaScriptClosedClassRangeLoop13Result {
    loop {
        if at < last {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let target = self::read_java_script_closed_class_static_target(
                        values,
                        class_names,
                        static_calls,
                        at,
                    );
                    if target != (self::ml_string::Text::from_units(vec![])) {
                        {
                            let result_2 = result.concat(
                                &({
                                    let source_ml_source1_string_range_operand101 = source.clone();
                                    {
                                        let source_ml_source1_string_range_operand102 = previous;
                                        {
                                            let source_ml_source1_string_range_operand103 =
                                                (self::ml_array::optional_at(
                                                    starts,
                                                    self::ml_array::number_index(at),
                                                ))
                                                .unwrap_or_else(|| previous);
                                            {
                                                let source_ml_source1_string_range_position104 =
                                                    source_ml_source1_string_range_operand102;
                                                {
                                                    let source_ml_source1_string_range_position105 =
                                                        source_ml_source1_string_range_operand103;
                                                    source_ml_source1_string_range_operand101.slice(
                                                        source_ml_source1_string_range_position104,
                                                        source_ml_source1_string_range_position105,
                                                    )
                                                }
                                            }
                                        }
                                    }
                                })
                                .concat(&target),
                            );
                            {
                                let previous_2 = (self::ml_array::optional_at(
                                    ends,
                                    self::ml_array::number_index(at + 2f64),
                                ))
                                .unwrap_or_else(|| previous);
                                {
                                    let at_2 = at + 2f64;
                                    {
                                        let at_3 = at_2 + 1f64;
                                        {
                                            (result, previous, at) = (result_2, previous_2, at_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    {
                        {
                            let this_reference = ((((*receiver)
                                != (self::ml_string::Text::from_units(vec![])))
                                && (value
                                    == (self::ml_string::Text::from_units(vec![
                                        116, 104, 105, 115,
                                    ]))))
                                && (((self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(at - 1f64),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                                    != (self::ml_string::Text::from_units(vec![46]))))
                                && (((self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(at - 1f64),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                                    != (self::ml_string::Text::from_units(vec![63, 46])));
                            {
                                let construction = ((value
                                    == (self::ml_string::Text::from_units(vec![110, 101, 119])))
                                    && self::ml_array::includes(
                                        class_names,
                                        &(self::ml_array::optional_at(
                                            values,
                                            self::ml_array::number_index(at + 1f64),
                                        ))
                                        .unwrap_or_else(
                                            || self::ml_string::Text::from_units(vec![]),
                                        ),
                                        0f64,
                                        false,
                                        |a, b| a == b,
                                    ))
                                    && (((self::ml_array::optional_at(
                                        values,
                                        self::ml_array::number_index(at + 2f64),
                                    ))
                                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                                        == (self::ml_string::Text::from_units(vec![40])));
                                if this_reference || construction {
                                    {
                                        let result_3 = result.concat(&({
                                            let source_ml_source1_string_range_operand106 = source.clone();
                                            {
                                                let source_ml_source1_string_range_operand107 = previous;
                                                {
                                                    let source_ml_source1_string_range_operand108 = (self::ml_array::optional_at(starts, self::ml_array::number_index(at))).unwrap_or_else(|| previous);
                                                    {
                                                        let source_ml_source1_string_range_position109 = source_ml_source1_string_range_operand107;
                                                        {
                                                            let source_ml_source1_string_range_position110 = source_ml_source1_string_range_operand108;
                                                            source_ml_source1_string_range_operand106.slice(source_ml_source1_string_range_position109, source_ml_source1_string_range_position110)
                                                        }
                                                    }
                                                }
                                            }
                                        }).concat(&if this_reference {
                                            receiver.clone()
                                        } else {
                                            self::ml_string::Text::from_units(vec![])
                                        }));
                                        {
                                            let previous_3 = (self::ml_array::optional_at(
                                                ends,
                                                self::ml_array::number_index(at),
                                            ))
                                            .unwrap_or_else(|| previous);
                                            {
                                                let at_4 = at + 1f64;
                                                {
                                                    (result, previous, at) =
                                                        (result_3, previous_3, at_4);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                                {
                                    {
                                        let at_5 = at + 1f64;
                                        {
                                            at = at_5;
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
        return {
            self::MlTemporaryRenderJavaScriptClosedClassRangeLoop13Result::MlTemporaryRenderJavaScriptClosedClassRangeLoop13Done(result.clone(), previous)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_methods_loop14(
    values: &[self::ml_string::Text],
    directory: &[f64],
    mut result: Vec<self::ml_string::Text>,
    mut at: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if at < (directory.len() as f64) {
            if ((self::ml_array::optional_at(directory, self::ml_array::number_index(at)))
                .unwrap_or_else(|| -1f64))
                >= 0f64
            {
                {
                    let result_2 = {
                        let source_ml_source1_concat_value111 = result;
                        {
                            let source_ml_source1_concat_value112 = vec![
                                (self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(
                                        (self::ml_array::optional_at(
                                            directory,
                                            self::ml_array::number_index(at),
                                        ))
                                        .unwrap_or_else(|| 0f64),
                                    ),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                            ];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<self::ml_string::Text>::new(),
                                    source_ml_source1_concat_value111,
                                ),
                                source_ml_source1_concat_value112,
                            )
                        }
                    };
                    {
                        let at_2 = at + 5f64;
                        {
                            (result, at) = (result_2, at_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let at_3 = at + 5f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_static_calls_loop17(
    values: &[self::ml_string::Text],
    prefix: &self::ml_string::Text,
    mut result: Vec<self::ml_string::Text>,
    mut ordinal: f64,
    row: &[f64],
    name: &self::ml_string::Text,
    mut at: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if at < (row.len() as f64) {
            {
                let position = (self::ml_array::optional_at(row, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| 0f64);
                if position < 0f64 {
                    {
                        let result_2 = {
                            let source_ml_source1_concat_value115 = result;
                            {
                                let source_ml_source1_concat_value116 = vec![
                                    (name.concat(&self::ml_string::Text::from_units(vec![46])))
                                        .concat(
                                            &(self::ml_array::optional_at(
                                                values,
                                                self::ml_array::number_index((-position) - 1f64),
                                            ))
                                            .unwrap_or_else(|| {
                                                self::ml_string::Text::from_units(vec![])
                                            }),
                                        ),
                                    (((prefix.concat(&self::ml_string::Text::from_units(vec![
                                        99, 108, 97, 115, 115, 95, 115, 116, 97, 116, 105, 99, 95,
                                    ])))
                                    .concat(&{
                                        let source_ml_source1_string_argument113 = ordinal;
                                        self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                source_ml_source1_string_argument113,
                                            ),
                                        )
                                    }))
                                    .concat(&self::ml_string::Text::from_units(vec![95])))
                                    .concat(&{
                                        let source_ml_source1_string_argument114 = at;
                                        self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                source_ml_source1_string_argument114,
                                            ),
                                        )
                                    }),
                                ];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source1_concat_value115,
                                    ),
                                    source_ml_source1_concat_value116,
                                )
                            }
                        };
                        {
                            let at_2 = at + 5f64;
                            {
                                (result, at) = (result_2, at_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let at_3 = at + 5f64;
                        {
                            at = at_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_static_calls_loop16(
    values: &[self::ml_string::Text],
    prefix: &self::ml_string::Text,
    mut result: Vec<self::ml_string::Text>,
    mut ordinal: f64,
    source_ml_temporary_values15: &[Vec<f64>],
    mut source_ml_temporary_index15: self::ml::Big,
) -> Vec<self::ml_string::Text> {
    loop {
        if source_ml_temporary_index15
            < (self::ml::Big::from_u128(source_ml_temporary_values15.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values15,
                    source_ml_temporary_index15.to_index(),
                );
                {
                    let name = (self::ml_array::optional_at(
                        values,
                        self::ml_array::number_index(
                            (self::ml_array::optional_at(&row, self::ml_array::number_index(0f64)))
                                .unwrap_or_else(|| 0f64)
                                + 1f64,
                        ),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    {
                        let at = 8f64
                            + (3f64
                                * (self::ml_array::optional_at(
                                    &row,
                                    self::ml_array::number_index(7f64),
                                ))
                                .unwrap_or_else(|| 0f64));
                        {
                            let result_2 = self::source_ml_temporary_read_java_script_closed_class_static_calls_loop17(values, prefix, result, ordinal, &(row), &(name), at);
                            {
                                let ordinal_2 = ordinal + 1f64;
                                {
                                    let source_ml_temporary_index15_2 = source_ml_temporary_index15
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, ordinal, source_ml_temporary_index15) =
                                            (result_2, ordinal_2, source_ml_temporary_index15_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptClosedClassStaticTargetLoop18Result {
    MlTemporaryReadJavaScriptClosedClassStaticTargetLoop18Done,
    MlTemporaryReadJavaScriptClosedClassStaticTargetLoop18Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_static_target_loop18(
    calls: &[self::ml_string::Text],
    key: &self::ml_string::Text,
    mut index: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop18Result {
    loop {
        return if index < (calls.len() as f64) {
            if ((self::ml_array::optional_at(calls, self::ml_array::number_index(index)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (*key)
            {
                self::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop18Result::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop18Return((self::ml_array::optional_at(calls, self::ml_array::number_index(index + 1f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
            } else {
                {
                    let index_2 = index + 2f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop18Result::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop18Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorJoin23 {
    MlTemporaryReadJavaScriptClosedClassObservationErrorJoin23Next,
    MlTemporaryReadJavaScriptClosedClassObservationErrorJoin23Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop24Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop24Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop24Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop24(
    mut at: f64,
    source_ml_temporary_values20: &[Vec<f64>],
    mut source_ml_temporary_index20: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop24Result {
    loop {
        return if source_ml_temporary_index20
            < (self::ml::Big::from_u128(source_ml_temporary_values20.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values20,
                    source_ml_temporary_index20.to_index(),
                );
                if (at
                    > ((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64)))
                        .unwrap_or_else(|| 0f64)))
                    && (at
                        < ((self::ml_array::optional_at(&row, self::ml_array::number_index(1f64)))
                            .unwrap_or_else(|| 0f64)))
                {
                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop24Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop24Return(at)
                } else {
                    {
                        let source_ml_temporary_index20_2 =
                            source_ml_temporary_index20.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index20 = source_ml_temporary_index20_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop24Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop24Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop22(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    directory: &[Vec<f64>],
    static_calls: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    methods: &[self::ml_string::Text],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Result {
    loop {
        return if at < (values.len() as f64) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let kind =
                        (self::ml_array::optional_at(kinds, self::ml_array::number_index(at)))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    {
                        let previous = (self::ml_array::optional_at(
                            values,
                            self::ml_array::number_index(at - 1f64),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        {
                            let next = (self::ml_array::optional_at(
                                values,
                                self::ml_array::number_index(at + 1f64),
                            ))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                            if (kind
                                == (self::ml_string::Text::from_units(vec![
                                    105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                                ])))
                                && self::ml_array::includes(
                                    &vec![
                                        self::ml_string::Text::from_units(vec![
                                            79, 98, 106, 101, 99, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            82, 101, 102, 108, 101, 99, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![74, 83, 79, 78]),
                                        self::ml_string::Text::from_units(vec![
                                            83, 121, 109, 98, 111, 108,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            80, 114, 111, 120, 121,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            70, 117, 110, 99, 116, 105, 111, 110,
                                        ]),
                                        self::ml_string::Text::from_units(vec![101, 118, 97, 108]),
                                        self::ml_string::Text::from_units(vec![
                                            97, 114, 103, 117, 109, 101, 110, 116, 115,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            115, 117, 112, 101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            101, 120, 112, 111, 114, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            100, 101, 101, 112, 69, 113, 117, 97, 108,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            110, 111, 116, 68, 101, 101, 112, 69, 113, 117, 97, 108,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            100, 101, 101, 112, 83, 116, 114, 105, 99, 116, 69,
                                            113, 117, 97, 108,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            110, 111, 116, 68, 101, 101, 112, 83, 116, 114, 105,
                                            99, 116, 69, 113, 117, 97, 108,
                                        ]),
                                    ],
                                    &value.clone(),
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                )
                            {
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Return(at)
                            } else if ((value
                                == (self::ml_string::Text::from_units(vec![46, 46, 46])))
                                || (value
                                    == (self::ml_string::Text::from_units(vec![
                                        95, 95, 112, 114, 111, 116, 111, 95, 95,
                                    ]))))
                                || (value
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 116, 111, 116, 121, 112, 101,
                                    ])))
                            {
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Return(at)
                            } else {
                                {
                                    let source_ml_source1_s118 = if value
                                        == (self::ml_string::Text::from_units(vec![
                                            102, 117, 110, 99, 116, 105, 111, 110,
                                        ])) {
                                        {
                                            let source_ml_temporary_values20 = directory.to_vec();
                                            {
                                                let source_ml_temporary_index20 =
                                                    self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_s119 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop24(at, &(source_ml_temporary_values20), source_ml_temporary_index20);
                                                    match source_ml_source1_s119 {
                                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop24Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop24Done => {
                                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin23::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin23Next
                                                            }
                                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop24Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop24Return(source_ml_temporary_result24) => {
                                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin23::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin23Return(source_ml_temporary_result24)
                                                            }
                                                        }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin23::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin23Next
                                    };
                                    match source_ml_source1_s118 {
                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin23::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin23Next => {
                                                if ((((kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) && self::ml_array::includes(names, &value.clone(), 0f64, false, |a, b| a == b)) && (previous != (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))) && ((previous != (self::ml_string::Text::from_units(vec![110, 101, 119]))) || (next != (self::ml_string::Text::from_units(vec![40]))))) && ((self::read_java_script_closed_class_static_target(values, names, static_calls, at)) == (self::ml_string::Text::from_units(vec![]))) {
                                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Return(at)
                                                } else if ((((value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))) && (previous != (self::ml_string::Text::from_units(vec![46])))) && (previous != (self::ml_string::Text::from_units(vec![63, 46])))) && (next != (self::ml_string::Text::from_units(vec![46])))) && (next != (self::ml_string::Text::from_units(vec![91]))) {
                                                        self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Return(at)
                                                    } else if ((previous == (self::ml_string::Text::from_units(vec![46]))) || (previous == (self::ml_string::Text::from_units(vec![63, 46])))) && ((self::ml_array::includes(methods, &value.clone(), 0f64, false, |a, b| a == b) && (next != (self::ml_string::Text::from_units(vec![40])))) || (value == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])))) {
                                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Return(at)
                                                        } else if ((previous == (self::ml_string::Text::from_units(vec![91]))) && (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) && self::ml_array::includes(methods, &{
                                                                let source_ml_source1_string_range_operand120 = value.clone();
                                                                {
                                                                    let source_ml_source1_string_range_operand121 = 1f64;
                                                                    {
                                                                        let source_ml_source1_string_range_position122 = source_ml_source1_string_range_operand121;
                                                                        {
                                                                            let source_ml_source1_string_range_position123 = f64::INFINITY;
                                                                            source_ml_source1_string_range_operand120.slice(source_ml_source1_string_range_position122, source_ml_source1_string_range_position123)
                                                                        }
                                                                    }
                                                                }
                                                            }, 0f64, false, |a, b| a == b) {
                                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Return(at)
                                                            } else {
                                                                {
                                                                    let at_2 = at + 1f64;
                                                                    {
                                                                        at = at_2;
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                            }
                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin23::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin23Return(source_ml_temporary_result23) => {
                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Return(source_ml_temporary_result23)
                                            }
                                        }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop21Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop21Done(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
    ),
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop21Return(f64),
}
