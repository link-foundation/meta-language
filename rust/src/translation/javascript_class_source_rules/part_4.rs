// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3;
use super::MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Result;
use super::find_java_script_class_group_end;
use super::ml_array;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_parameter_positions_loop2(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut close: f64,
    mut result: Vec<f64>,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Result {
    loop {
        return if at < close {
            {
                let ml_library_inequality_2 =
                    ((self::ml_array::optional_at(kinds, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                        != (self::ml_string::Text::from_units(vec![
                            105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                        ]));
                if ml_library_inequality_2 {
                    self::MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Result::MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Return(vec![-1f64, at])
                } else {
                    {
                        let result_2 = {
                            let source_ml_source1_concat_value80 = result;
                            {
                                let source_ml_source1_concat_value81 = vec![at];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value80,
                                    ),
                                    source_ml_source1_concat_value81,
                                )
                            }
                        };
                        {
                            let at_2 = at + 1f64;
                            {
                                let source_ml_source1_s82 = if ((self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(at_2),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                                    == (self::ml_string::Text::from_units(vec![61]))
                                {
                                    {
                                        let at_3 = at_2 + 1f64;
                                        {
                                            let value = (self::ml_array::optional_at(
                                                values,
                                                self::ml_array::number_index(at_3),
                                            ))
                                            .unwrap_or_else(|| {
                                                self::ml_string::Text::from_units(vec![])
                                            });
                                            if (value
                                                == (self::ml_string::Text::from_units(vec![43])))
                                                || (value
                                                    == (self::ml_string::Text::from_units(vec![
                                                        45,
                                                    ])))
                                            {
                                                {
                                                    let at_4 = at_3 + 1f64;
                                                    if (((self::ml_array::optional_at(
                                                        kinds,
                                                        self::ml_array::number_index(at_4),
                                                    ))
                                                    .unwrap_or_else(|| {
                                                        self::ml_string::Text::from_units(vec![])
                                                    })) != (self::ml_string::Text::from_units(
                                                        vec![110, 117, 109, 98, 101, 114],
                                                    ))) && !(self::ml_array::includes(
                                                        &vec![
                                                            self::ml_string::Text::from_units(
                                                                vec![78, 97, 78],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    73, 110, 102, 105, 110, 105,
                                                                    116, 121,
                                                                ],
                                                            ),
                                                        ],
                                                        &(self::ml_array::optional_at(
                                                            values,
                                                            self::ml_array::number_index(at_4),
                                                        ))
                                                        .unwrap_or_else(|| {
                                                            self::ml_string::Text::from_units(
                                                                vec![],
                                                            )
                                                        }),
                                                        0f64,
                                                        false,
                                                        |a, b| a == b,
                                                    )) {
                                                        self::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3Return(vec![-1f64, at_4])
                                                    } else {
                                                        {
                                                            let at_5 = at_4 + 1f64;
                                                            self::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3Next(at_5)
                                                        }
                                                    }
                                                }
                                            } else if ((((self::ml_array::optional_at(
                                                kinds,
                                                self::ml_array::number_index(at_3),
                                            ))
                                            .unwrap_or_else(|| {
                                                self::ml_string::Text::from_units(vec![])
                                            })) == (self::ml_string::Text::from_units(
                                                vec![110, 117, 109, 98, 101, 114],
                                            ))) || (((self::ml_array::optional_at(
                                                kinds,
                                                self::ml_array::number_index(at_3),
                                            ))
                                            .unwrap_or_else(|| {
                                                self::ml_string::Text::from_units(vec![])
                                            }))
                                                == (self::ml_string::Text::from_units(vec![
                                                    115, 116, 114, 105, 110, 103,
                                                ]))))
                                                || self::ml_array::includes(
                                                    &vec![
                                                        self::ml_string::Text::from_units(vec![
                                                            116, 114, 117, 101,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            102, 97, 108, 115, 101,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            110, 117, 108, 108,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            117, 110, 100, 101, 102, 105, 110, 101,
                                                            100,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            78, 97, 78,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            73, 110, 102, 105, 110, 105, 116, 121,
                                                        ]),
                                                    ],
                                                    &value.clone(),
                                                    0f64,
                                                    false,
                                                    |a, b| a == b,
                                                )
                                            {
                                                {
                                                    let at_6 = at_3 + 1f64;
                                                    self::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3Next(at_6)
                                                }
                                            } else if ((value
                                                == (self::ml_string::Text::from_units(vec![91])))
                                                && (((self::ml_array::optional_at(
                                                    values,
                                                    self::ml_array::number_index(at_3 + 1f64),
                                                ))
                                                .unwrap_or_else(|| {
                                                    self::ml_string::Text::from_units(vec![])
                                                })) == (self::ml_string::Text::from_units(
                                                    vec![93],
                                                ))))
                                                || ((value
                                                    == (self::ml_string::Text::from_units(vec![
                                                        123,
                                                    ])))
                                                    && (((self::ml_array::optional_at(
                                                        values,
                                                        self::ml_array::number_index(at_3 + 1f64),
                                                    ))
                                                    .unwrap_or_else(|| {
                                                        self::ml_string::Text::from_units(vec![])
                                                    })) == (self::ml_string::Text::from_units(
                                                        vec![125],
                                                    ))))
                                            {
                                                {
                                                    let at_7 = at_3 + 2f64;
                                                    self::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3Next(at_7)
                                                }
                                            } else {
                                                self::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3Return(vec![-1f64, at_3])
                                            }
                                        }
                                    }
                                } else {
                                    self::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3Next(at_2)
                                };
                                match source_ml_source1_s82 {
                                self::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3Next(at_8) => {
                                    if at_8 < close {
                                        {
let ml_library_inequality_1 = ((self::ml_array::optional_at(values, self::ml_array::number_index(at_8))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![44])) ;
if ml_library_inequality_1 {
                                            self::MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Result::MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Return(vec![-1f64, at_8])
                                        } else {
                                            {
                                                let at_9 = at_8 + 1f64;
                                                {
                                                    (result, at) = (result_2, at_9);
                                                    continue;
                                                }
                                            }
                                        }
}
                                    } else {
                                        {
                                            (result, at) = (result_2, at_8);
                                            continue;
                                        }
                                    }
                                }
                                self::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3Return(source_ml_temporary_result3) => {
                                    self::MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Result::MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Return(source_ml_temporary_result3)
                                }
                            }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Result::MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Done(result)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_render_java_script_closed_class_parameter_arguments_loop4(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    mut close: f64,
    mut result: Vec<self::ml_string::Text>,
    mut at: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if at < close {
            {
                let result_2 = {
                    let source_ml_source1_concat_value88 = result;
                    {
                        let source_ml_source1_concat_value89 = vec![{
                            let source_ml_source1_string_range_operand83 = source.clone();
                            {
                                let source_ml_source1_string_range_operand84 =
                                    (self::ml_array::optional_at(
                                        starts,
                                        self::ml_array::number_index(at),
                                    ))
                                    .unwrap_or_else(|| 0f64);
                                {
                                    let source_ml_source1_string_range_operand85 =
                                        (self::ml_array::optional_at(
                                            ends,
                                            self::ml_array::number_index(at),
                                        ))
                                        .unwrap_or_else(|| 0f64);
                                    {
                                        let source_ml_source1_string_range_position86 =
                                            source_ml_source1_string_range_operand84;
                                        {
                                            let source_ml_source1_string_range_position87 =
                                                source_ml_source1_string_range_operand85;
                                            source_ml_source1_string_range_operand83.slice(
                                                source_ml_source1_string_range_position86,
                                                source_ml_source1_string_range_position87,
                                            )
                                        }
                                    }
                                }
                            }
                        }];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source1_concat_value88,
                            ),
                            source_ml_source1_concat_value89,
                        )
                    }
                };
                {
                    let at_2 = at + 1f64;
                    {
                        let at_7 = if ((self::ml_array::optional_at(
                            values,
                            self::ml_array::number_index(at_2),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                            == (self::ml_string::Text::from_units(vec![61]))
                        {
                            {
                                let at_3 = at_2 + 1f64;
                                {
                                    let value = (self::ml_array::optional_at(
                                        values,
                                        self::ml_array::number_index(at_3),
                                    ))
                                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                                    if (value == (self::ml_string::Text::from_units(vec![91])))
                                        || (value == (self::ml_string::Text::from_units(vec![123])))
                                    {
                                        { at_3 + 2f64 }
                                    } else if (value
                                        == (self::ml_string::Text::from_units(vec![43])))
                                        || (value == (self::ml_string::Text::from_units(vec![45])))
                                    {
                                        { at_3 + 2f64 }
                                    } else {
                                        { at_3 + 1f64 }
                                    }
                                }
                            }
                        } else {
                            at_2
                        };
                        if at_7 < close {
                            {
                                let at_8 = at_7 + 1f64;
                                {
                                    (result, at) = (result_2, at_8);
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                (result, at) = (result_2, at_7);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptClosedClassInitializationStartLoop5Result {
    MlTemporaryFindJavaScriptClosedClassInitializationStartLoop5Done,
    MlTemporaryFindJavaScriptClosedClassInitializationStartLoop5Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_closed_class_initialization_start_loop5(
    values: &[self::ml_string::Text],
    mut first: f64,
    mut last: f64,
    mut depth: f64,
    mut at: f64,
) -> self::MlTemporaryFindJavaScriptClosedClassInitializationStartLoop5Result {
    loop {
        return if at < last {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let previous = (self::ml_array::optional_at(
                        values,
                        self::ml_array::number_index(at - 1f64),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    if ((value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115])))
                        && (previous != (self::ml_string::Text::from_units(vec![46]))))
                        && (previous != (self::ml_string::Text::from_units(vec![63, 46])))
                    {
                        {
                            let boundary = (((at).partial_cmp(&(first))
                                == Some(std::cmp::Ordering::Equal))
                                || (previous == (self::ml_string::Text::from_units(vec![59]))))
                                || (previous == (self::ml_string::Text::from_units(vec![125])));
                            self::MlTemporaryFindJavaScriptClosedClassInitializationStartLoop5Result::MlTemporaryFindJavaScriptClosedClassInitializationStartLoop5Return(if (((depth == 0f64) && boundary) && (((self::ml_array::optional_at(values, self::ml_array::number_index(at + 1f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![46])))) && (((self::ml_array::optional_at(values, self::ml_array::number_index(at + 3f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61]))) {
                                at
                            } else {
                                first
                            })
                        }
                    } else {
                        {
                            let depth_3 = if ((value
                                == (self::ml_string::Text::from_units(vec![40])))
                                || (value == (self::ml_string::Text::from_units(vec![91]))))
                                || (value == (self::ml_string::Text::from_units(vec![123])))
                            {
                                { depth + 1f64 }
                            } else {
                                depth
                            };
                            if ((value == (self::ml_string::Text::from_units(vec![41])))
                                || (value == (self::ml_string::Text::from_units(vec![93]))))
                                || (value == (self::ml_string::Text::from_units(vec![125])))
                            {
                                {
                                    let depth_4 = depth_3 - 1f64;
                                    {
                                        let at_2 = at + 1f64;
                                        {
                                            (depth, at) = (depth_4, at_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                {
                                    let at_3 = at + 1f64;
                                    {
                                        (depth, at) = (depth_3, at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptClosedClassInitializationStartLoop5Result::MlTemporaryFindJavaScriptClosedClassInitializationStartLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryJoin8 {
    MlTemporaryReadJavaScriptClosedClassDirectoryJoin8Next,
    MlTemporaryReadJavaScriptClosedClassDirectoryJoin8Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Done,
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop9(
    values: &[self::ml_string::Text],
    mut body_end: f64,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Result {
    loop {
        return if at < body_end {
            if ((self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]))
            {
                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Return(vec![-6f64, at])
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
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Done(Vec<f64>),
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Done(f64),
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Done,
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop12(
    values: &[self::ml_string::Text],
    mut group: f64,
    mut inner: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Result {
    loop {
        return if inner < group {
            if ((((self::ml_array::optional_at(values, self::ml_array::number_index(inner)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![116, 104, 105, 115])))
                && (((self::ml_array::optional_at(
                    values,
                    self::ml_array::number_index(inner - 1f64),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                    != (self::ml_string::Text::from_units(vec![46]))))
                && (((self::ml_array::optional_at(
                    values,
                    self::ml_array::number_index(inner - 1f64),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                    != (self::ml_string::Text::from_units(vec![63, 46])))
            {
                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![-5f64, inner])
            } else {
                {
                    let inner_2 = inner + 1f64;
                    {
                        inner = inner_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop11(
    values: &[self::ml_string::Text],
    mut body_end: f64,
    mut finish: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Result {
    loop {
        return if finish < body_end {
            {
                let value =
                    (self::ml_array::optional_at(values, self::ml_array::number_index(finish)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if value == (self::ml_string::Text::from_units(vec![59])) {
                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Done(finish)
                } else if value == (self::ml_string::Text::from_units(vec![44])) {
                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Return(vec![-5f64, finish])
                } else if ((value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115])))
                    && (((self::ml_array::optional_at(
                        values,
                        self::ml_array::number_index(finish - 1f64),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                        != (self::ml_string::Text::from_units(vec![46]))))
                    && (((self::ml_array::optional_at(
                        values,
                        self::ml_array::number_index(finish - 1f64),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                        != (self::ml_string::Text::from_units(vec![63, 46])))
                {
                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Return(vec![-5f64, finish])
                } else if ((value == (self::ml_string::Text::from_units(vec![40])))
                    || (value == (self::ml_string::Text::from_units(vec![91]))))
                    || (value == (self::ml_string::Text::from_units(vec![123])))
                {
                    {
                        let group = self::find_java_script_class_group_end(values, finish);
                        if (group < 0f64) || (group >= body_end) {
                            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Return(vec![-5f64, finish])
                        } else {
                            {
                                let inner = finish + 1f64;
                                {
                                    let source_ml_source1_s90 = self::source_ml_temporary_read_java_script_closed_class_directory_loop12(values, group, inner);
                                    match source_ml_source1_s90 {
                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Done => {
                                                        {
                                                            let finish_2 = group;
                                                            {
                                                                let finish_3 = finish_2 + 1f64;
                                                                {
                                                                    finish = finish_3;
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Return(source_ml_temporary_result12) => {
                                                        self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Return(source_ml_temporary_result12)
                                                    }
                                                }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let finish_4 = finish + 1f64;
                        {
                            finish = finish_4;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Done(finish)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop10Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop10Done,
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop10Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop10(
    values: &[self::ml_string::Text],
    fields: &[f64],
    mut cursor: f64,
    name: &self::ml_string::Text,
    mut previous: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop10Result {
    loop {
        return if previous < (fields.len() as f64) {
            if ((self::ml_array::optional_at(
                values,
                self::ml_array::number_index(
                    (self::ml_array::optional_at(fields, self::ml_array::number_index(previous)))
                        .unwrap_or_else(|| 0f64),
                ),
            ))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (*name)
            {
                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop10Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop10Return(vec![-4f64, (cursor + 2f64)])
            } else {
                {
                    let previous_2 = previous + 3f64;
                    {
                        previous = previous_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop10Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop10Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop7(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut fields: Vec<f64>,
    names: &[self::ml_string::Text],
    mut body_start: f64,
    mut body_end: f64,
    mut cursor: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result {
    loop {
        return if (((((body_start >= 0f64) && (cursor < body_end))
            && (((self::ml_array::optional_at(values, self::ml_array::number_index(cursor)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))))
            && (((self::ml_array::optional_at(
                values,
                self::ml_array::number_index(cursor + 1f64),
            ))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![46]))))
            && (((self::ml_array::optional_at(
                kinds,
                self::ml_array::number_index(cursor + 2f64),
            ))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![
                    105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                ]))))
            && (((self::ml_array::optional_at(
                values,
                self::ml_array::number_index(cursor + 3f64),
            ))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![61])))
        {
            {
                let name = (self::ml_array::optional_at(
                    values,
                    self::ml_array::number_index(cursor + 2f64),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if (name == (self::ml_string::Text::from_units(vec![36])))
                    || self::ml_array::includes(
                        names,
                        &(self::ml_string::Text::from_units(vec![
                            105, 110, 115, 116, 97, 110, 99, 101, 58,
                        ]))
                        .concat(&name),
                        0f64,
                        false,
                        |a, b| a == b,
                    )
                {
                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Return(vec![-4f64, (cursor + 2f64)])
                } else {
                    {
                        let previous = 0f64;
                        {
                            let source_ml_source1_s91 = self::source_ml_temporary_read_java_script_closed_class_directory_loop10(values, &(fields), cursor, &(name), previous);
                            match source_ml_source1_s91 {
                                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop10Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop10Done => {
                                    {
                                        let start = cursor + 4f64;
                                        {
                                            let finish = start;
                                            {
                                                let source_ml_source1_s92 = self::source_ml_temporary_read_java_script_closed_class_directory_loop11(values, body_end, finish);
                                                match source_ml_source1_s92 {
                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Done(finish_2) => {
                                                        if (finish_2 == start) || (finish_2 >= body_end) {
                                                            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Return(vec![-5f64, start])
                                                        } else {
                                                            {
                                                                let fields_2 = {
                                                                    let source_ml_source1_concat_value93 = fields;
                                                                    {
                                                                        let source_ml_source1_concat_value94 = vec![(cursor + 2f64), start, finish_2];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value93), source_ml_source1_concat_value94)
                                                                    }
                                                                };
                                                                {
                                                                    let cursor_2 = finish_2 + 1f64;
                                                                    {
                                                                        (fields, cursor) = (fields_2, cursor_2);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop11Return(source_ml_temporary_result11) => {
                                                        self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Return(source_ml_temporary_result11)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop10Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop10Return(source_ml_temporary_result12) => {
                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Return(source_ml_temporary_result12)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Done(fields)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Done(
        Vec<f64>,
        Vec<f64>,
        Vec<self::ml_string::Text>,
    ),
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryJoin13 {
    MlTemporaryReadJavaScriptClosedClassDirectoryJoin13Next(Vec<f64>, Vec<f64>),
}
