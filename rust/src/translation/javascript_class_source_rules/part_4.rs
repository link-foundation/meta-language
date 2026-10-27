// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptClosedClassDirectoryJoin10;
use super::MlTemporaryReadJavaScriptClosedClassDirectoryLoop2Result;
use super::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Result;
use super::find_java_script_class_group_end;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_closed_class_methods;
use super::render_java_script_closed_class_range;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop9(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut parameters: f64,
    mut parameter: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Result {
    loop {
        return if parameter < parameters {
            if (((self::ml_array::optional_at(kinds, self::ml_array::number_index(parameter)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                != (self::ml_string::Text::from_units(vec![
                    105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                ])))
                || (((parameter + 1f64) < parameters)
                    && (((self::ml_array::optional_at(
                        values,
                        self::ml_array::number_index(parameter + 1f64),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                        != (self::ml_string::Text::from_units(vec![44]))))
            {
                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Return(vec![-3f64, parameter])
            } else {
                {
                    let parameter_2 = parameter + 2f64;
                    {
                        parameter = parameter_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop2(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut end: f64,
    mut constructor: Vec<f64>,
    mut methods: Vec<f64>,
    mut names: Vec<self::ml_string::Text>,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop2Result {
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
                    let name =
                        (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    if (((((((((((self::ml_array::optional_at(
                        kinds,
                        self::ml_array::number_index(at),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                        != (self::ml_string::Text::from_units(vec![
                            105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                        ])))
                        || (name
                            == (self::ml_string::Text::from_units(vec![
                                115, 116, 97, 116, 105, 99,
                            ]))))
                        || (name == (self::ml_string::Text::from_units(vec![103, 101, 116]))))
                        || (name == (self::ml_string::Text::from_units(vec![115, 101, 116]))))
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
                        || self::ml_array::includes(&names, &name.clone(), 0f64, false, |a, b| {
                            a == b
                        }))
                        || (((self::ml_array::optional_at(
                            values,
                            self::ml_array::number_index(at + 1f64),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                            != (self::ml_string::Text::from_units(vec![40])))
                    {
                        self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop2Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop2Return(vec![-2f64, at])
                    } else {
                        {
                            let parameters =
                                self::find_java_script_class_group_end(values, (at + 1f64));
                            if (parameters < 0f64)
                                || (((self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(parameters + 1f64),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                                    != (self::ml_string::Text::from_units(vec![123])))
                            {
                                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop2Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop2Return(vec![-2f64, at])
                            } else {
                                {
                                    let parameter = at + 2f64;
                                    {
                                        let source_ml_source1_s76 = self::source_ml_temporary_read_java_script_closed_class_directory_loop9(values, kinds, parameters, parameter);
                                        match source_ml_source1_s76 {
                                            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Done => {
                                                {
                                                    let body = self::find_java_script_class_group_end(values, (parameters + 1f64));
                                                    if (body < 0f64) || (body > end) {
                                                        self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop2Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop2Return(vec![-2f64, at])
                                                    } else {
                                                        {
                                                            let names_2 = {
                                                                let source_ml_source1_concat_value77 = names;
                                                                {
                                                                    let source_ml_source1_concat_value78 = vec![name.clone()];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value77), source_ml_source1_concat_value78)
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_s81 = if name == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])) {
                                                                    {
                                                                        let constructor_2 = vec![at, (at + 1f64), parameters, (parameters + 1f64), body];
                                                                        self::MlTemporaryReadJavaScriptClosedClassDirectoryJoin10::MlTemporaryReadJavaScriptClosedClassDirectoryJoin10Next(constructor_2, methods)
                                                                    }
                                                                } else {
                                                                    {
                                                                        let methods_2 = {
                                                                            let source_ml_source1_concat_value82 = methods;
                                                                            {
                                                                                let source_ml_source1_concat_value83 = vec![at, (at + 1f64), parameters, (parameters + 1f64), body];
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value82), source_ml_source1_concat_value83)
                                                                            }
                                                                        };
                                                                        self::MlTemporaryReadJavaScriptClosedClassDirectoryJoin10::MlTemporaryReadJavaScriptClosedClassDirectoryJoin10Next(constructor, methods_2)
                                                                    }
                                                                };
                                                                match source_ml_source1_s81 {
                                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryJoin10::MlTemporaryReadJavaScriptClosedClassDirectoryJoin10Next(constructor_3, methods_3) => {
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
                                            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Return(source_ml_temporary_result10) => {
                                                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop2Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop2Return(source_ml_temporary_result10)
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
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop2Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop2Done(constructor, methods, names)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryRenderJavaScriptClosedClassRangeLoop11Result {
    MlTemporaryRenderJavaScriptClosedClassRangeLoop11Done(self::ml_string::Text, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_render_java_script_closed_class_range_loop11(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    mut last: f64,
    receiver: &self::ml_string::Text,
    class_names: &[self::ml_string::Text],
    mut result: self::ml_string::Text,
    mut previous: f64,
    mut at: f64,
) -> self::MlTemporaryRenderJavaScriptClosedClassRangeLoop11Result {
    loop {
        if at < last {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let this_reference = ((((*receiver)
                        != (self::ml_string::Text::from_units(vec![])))
                        && (value
                            == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))))
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
                        let construction = (value
                            == (self::ml_string::Text::from_units(vec![110, 101, 119])))
                            && self::ml_array::includes(
                                class_names,
                                &(self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(at + 1f64),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                                0f64,
                                false,
                                |a, b| a == b,
                            );
                        if this_reference || construction {
                            {
                                let result_2 = result.concat(&({
                                    let source_ml_source1_string_range_operand84 = source.clone();
                                    {
                                        let source_ml_source1_string_range_operand85 = previous;
                                        {
                                            let source_ml_source1_string_range_operand86 = (self::ml_array::optional_at(starts, self::ml_array::number_index(at))).unwrap_or_else(|| previous);
                                            {
                                                let source_ml_source1_string_range_position87 = source_ml_source1_string_range_operand85;
                                                {
                                                    let source_ml_source1_string_range_position88 = source_ml_source1_string_range_operand86;
                                                    source_ml_source1_string_range_operand84.slice(source_ml_source1_string_range_position87, source_ml_source1_string_range_position88)
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
                                    let previous_2 = (self::ml_array::optional_at(
                                        ends,
                                        self::ml_array::number_index(at),
                                    ))
                                    .unwrap_or_else(|| previous);
                                    {
                                        let at_2 = at + 1f64;
                                        {
                                            (result, previous, at) = (result_2, previous_2, at_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                        {
                            {
                                let at_3 = at + 1f64;
                                {
                                    at = at_3;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryRenderJavaScriptClosedClassRangeLoop11Result::MlTemporaryRenderJavaScriptClosedClassRangeLoop11Done(result.clone(), previous)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_methods_loop12(
    values: &[self::ml_string::Text],
    directory: &[f64],
    mut result: Vec<self::ml_string::Text>,
    mut at: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if at < (directory.len() as f64) {
            {
                let result_2 = {
                    let source_ml_source1_concat_value89 = result;
                    {
                        let source_ml_source1_concat_value90 = vec![
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
                                source_ml_source1_concat_value89,
                            ),
                            source_ml_source1_concat_value90,
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
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorJoin17 {
    MlTemporaryReadJavaScriptClosedClassObservationErrorJoin17Next,
    MlTemporaryReadJavaScriptClosedClassObservationErrorJoin17Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop18Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop18Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop18Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop18(
    mut at: f64,
    source_ml_temporary_values14: &[Vec<f64>],
    mut source_ml_temporary_index14: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop18Result {
    loop {
        return if source_ml_temporary_index14
            < (self::ml::Big::from_u128(source_ml_temporary_values14.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values14,
                    source_ml_temporary_index14.to_index(),
                );
                if (at
                    > ((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64)))
                        .unwrap_or_else(|| 0f64)))
                    && (at
                        < ((self::ml_array::optional_at(&row, self::ml_array::number_index(1f64)))
                            .unwrap_or_else(|| 0f64)))
                {
                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop18Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop18Return(at)
                } else {
                    {
                        let source_ml_temporary_index14_2 =
                            source_ml_temporary_index14.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index14 = source_ml_temporary_index14_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop18Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop18Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop16(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    directory: &[Vec<f64>],
    names: &[self::ml_string::Text],
    methods: &[self::ml_string::Text],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Result {
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
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Return(at)
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
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Return(at)
                            } else {
                                {
                                    let source_ml_source1_s92 = if value
                                        == (self::ml_string::Text::from_units(vec![
                                            102, 117, 110, 99, 116, 105, 111, 110,
                                        ])) {
                                        {
                                            let source_ml_temporary_values14 = directory.to_vec();
                                            {
                                                let source_ml_temporary_index14 =
                                                    self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_s93 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop18(at, &(source_ml_temporary_values14), source_ml_temporary_index14);
                                                    match source_ml_source1_s93 {
                                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop18Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop18Done => {
                                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin17::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin17Next
                                                            }
                                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop18Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop18Return(source_ml_temporary_result18) => {
                                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin17::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin17Return(source_ml_temporary_result18)
                                                            }
                                                        }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin17::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin17Next
                                    };
                                    match source_ml_source1_s92 {
                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin17::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin17Next => {
                                                if (((kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) && self::ml_array::includes(names, &value.clone(), 0f64, false, |a, b| a == b)) && (previous != (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))) && ((previous != (self::ml_string::Text::from_units(vec![110, 101, 119]))) || (next != (self::ml_string::Text::from_units(vec![40])))) {
                                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Return(at)
                                                } else if ((((value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))) && (previous != (self::ml_string::Text::from_units(vec![46])))) && (previous != (self::ml_string::Text::from_units(vec![63, 46])))) && (next != (self::ml_string::Text::from_units(vec![46])))) && (next != (self::ml_string::Text::from_units(vec![91]))) {
                                                        self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Return(at)
                                                    } else if ((previous == (self::ml_string::Text::from_units(vec![46]))) || (previous == (self::ml_string::Text::from_units(vec![63, 46])))) && ((self::ml_array::includes(methods, &value.clone(), 0f64, false, |a, b| a == b) && (next != (self::ml_string::Text::from_units(vec![40])))) || (value == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])))) {
                                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Return(at)
                                                        } else if ((previous == (self::ml_string::Text::from_units(vec![91]))) && (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) && self::ml_array::includes(methods, &{
                                                                let source_ml_source1_string_range_operand94 = value.clone();
                                                                {
                                                                    let source_ml_source1_string_range_operand95 = 1f64;
                                                                    {
                                                                        let source_ml_source1_string_range_position96 = source_ml_source1_string_range_operand95;
                                                                        {
                                                                            let source_ml_source1_string_range_position97 = f64::INFINITY;
                                                                            source_ml_source1_string_range_operand94.slice(source_ml_source1_string_range_position96, source_ml_source1_string_range_position97)
                                                                        }
                                                                    }
                                                                }
                                                            }, 0f64, false, |a, b| a == b) {
                                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Return(at)
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
                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin17::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin17Return(source_ml_temporary_result17) => {
                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Return(source_ml_temporary_result17)
                                            }
                                        }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop16Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop15Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop15Done(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop15(
    values: &[self::ml_string::Text],
    mut names: Vec<self::ml_string::Text>,
    mut methods: Vec<self::ml_string::Text>,
    source_ml_temporary_values13: &[Vec<f64>],
    mut source_ml_temporary_index13: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop15Result {
    loop {
        if source_ml_temporary_index13
            < (self::ml::Big::from_u128(source_ml_temporary_values13.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values13,
                    source_ml_temporary_index13.to_index(),
                );
                {
                    let names_2 = {
                        let source_ml_source1_concat_value98 = names;
                        {
                            let source_ml_source1_concat_value99 = vec![
                                (self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(
                                        (self::ml_array::optional_at(
                                            &row,
                                            self::ml_array::number_index(0f64),
                                        ))
                                        .unwrap_or_else(|| 0f64)
                                            + 1f64,
                                    ),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                            ];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<self::ml_string::Text>::new(),
                                    source_ml_source1_concat_value98,
                                ),
                                source_ml_source1_concat_value99,
                            )
                        }
                    };
                    {
                        let methods_2 = {
                            let source_ml_source1_concat_value100 = methods;
                            {
                                let source_ml_source1_concat_value101 =
                                    self::read_java_script_closed_class_methods(values, &(row));
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source1_concat_value100,
                                    ),
                                    source_ml_source1_concat_value101,
                                )
                            }
                        };
                        {
                            let source_ml_temporary_index13_2 =
                                source_ml_temporary_index13.add(&self::ml::Big::from_i128(1));
                            {
                                (names, methods, source_ml_temporary_index13) =
                                    (names_2, methods_2, source_ml_temporary_index13_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop15Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop15Done(names, methods)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_render_java_script_closed_class_loop19(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    row: &[f64],
    class_names: &[self::ml_string::Text],
    mut methods: f64,
    mut fields: self::ml_string::Text,
    mut at: f64,
) -> self::ml_string::Text {
    loop {
        if at < methods {
            {
                let field = (self::ml_array::optional_at(
                    values,
                    self::ml_array::number_index(
                        (self::ml_array::optional_at(row, self::ml_array::number_index(at)))
                            .unwrap_or_else(|| 0f64),
                    ),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let value = self::render_java_script_closed_class_range(
                        source,
                        values,
                        starts,
                        ends,
                        (self::ml_array::optional_at(row, self::ml_array::number_index(at + 1f64)))
                            .unwrap_or_else(|| 0f64),
                        (self::ml_array::optional_at(row, self::ml_array::number_index(at + 2f64)))
                            .unwrap_or_else(|| 0f64),
                        &(self::ml_string::Text::from_units(vec![])),
                        class_names,
                    );
                    {
                        let fields_2 = fields.concat(
                            &((((self::ml_string::Text::from_units(vec![44, 32])).concat(&field))
                                .concat(&self::ml_string::Text::from_units(vec![58, 32, 40])))
                            .concat(&value))
                            .concat(&self::ml_string::Text::from_units(vec![41])),
                        );
                        {
                            let at_2 = at + 3f64;
                            {
                                (fields, at) = (fields_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { fields.clone() };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderJavaScriptClosedClassLoop20Result {
    MlTemporaryRenderJavaScriptClosedClassLoop20Done(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
}
