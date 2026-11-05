// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::find_java_script_class_group_end;
use super::ml_array;
use super::ml_string;

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Result {
    MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Done(Vec<f64>),
    MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3 {
    MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3Next(f64),
    MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3Return(Vec<f64>),
}

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
                            let source_ml_source1_concat_value92 = result;
                            {
                                let source_ml_source1_concat_value93 = vec![at];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value92,
                                    ),
                                    source_ml_source1_concat_value93,
                                )
                            }
                        };
                        {
                            let at_2 = at + 1f64;
                            {
                                let source_ml_source1_s94 = if ((self::ml_array::optional_at(
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
                                match source_ml_source1_s94 {
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
                    let source_ml_source1_concat_value100 = result;
                    {
                        let source_ml_source1_concat_value101 = vec![{
                            let source_ml_source1_string_range_operand95 = source.clone();
                            {
                                let source_ml_source1_string_range_operand96 =
                                    (self::ml_array::optional_at(
                                        starts,
                                        self::ml_array::number_index(at),
                                    ))
                                    .unwrap_or_else(|| 0f64);
                                {
                                    let source_ml_source1_string_range_operand97 =
                                        (self::ml_array::optional_at(
                                            ends,
                                            self::ml_array::number_index(at),
                                        ))
                                        .unwrap_or_else(|| 0f64);
                                    {
                                        let source_ml_source1_string_range_position98 =
                                            source_ml_source1_string_range_operand96;
                                        {
                                            let source_ml_source1_string_range_position99 =
                                                source_ml_source1_string_range_operand97;
                                            source_ml_source1_string_range_operand95.slice(
                                                source_ml_source1_string_range_position98,
                                                source_ml_source1_string_range_position99,
                                            )
                                        }
                                    }
                                }
                            }
                        }];
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
pub enum MlTemporaryFindJavaScriptClosedClassFieldEndLoop6Result {
    MlTemporaryFindJavaScriptClosedClassFieldEndLoop6Done,
    MlTemporaryFindJavaScriptClosedClassFieldEndLoop6Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptClosedClassFieldEndLoop7Result {
    MlTemporaryFindJavaScriptClosedClassFieldEndLoop7Done,
    MlTemporaryFindJavaScriptClosedClassFieldEndLoop7Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_closed_class_field_end_loop7(
    values: &[self::ml_string::Text],
    mut end: f64,
    mut inner: f64,
) -> self::MlTemporaryFindJavaScriptClosedClassFieldEndLoop7Result {
    loop {
        return if inner < end {
            {
                let token =
                    (self::ml_array::optional_at(values, self::ml_array::number_index(inner)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if self::ml_array::includes(
                    &vec![
                        self::ml_string::Text::from_units(vec![116, 104, 105, 115]),
                        self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]),
                        self::ml_string::Text::from_units(vec![
                            97, 114, 103, 117, 109, 101, 110, 116, 115,
                        ]),
                        self::ml_string::Text::from_units(vec![97, 119, 97, 105, 116]),
                        self::ml_string::Text::from_units(vec![121, 105, 101, 108, 100]),
                        self::ml_string::Text::from_units(vec![61, 62]),
                        self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]),
                    ],
                    &token,
                    0f64,
                    false,
                    |a, b| a == b,
                ) {
                    self::MlTemporaryFindJavaScriptClosedClassFieldEndLoop7Result::MlTemporaryFindJavaScriptClosedClassFieldEndLoop7Return(-1f64)
                } else {
                    {
                        let inner_2 = inner + 1f64;
                        {
                            inner = inner_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptClosedClassFieldEndLoop7Result::MlTemporaryFindJavaScriptClosedClassFieldEndLoop7Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_closed_class_field_end_loop6(
    values: &[self::ml_string::Text],
    mut last: f64,
    mut at: f64,
) -> self::MlTemporaryFindJavaScriptClosedClassFieldEndLoop6Result {
    loop {
        return if at < last {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if value == (self::ml_string::Text::from_units(vec![59])) {
                    self::MlTemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlTemporaryFindJavaScriptClosedClassFieldEndLoop6Return(at)
                } else if (((((((value == (self::ml_string::Text::from_units(vec![44])))
                    || (value
                        == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))))
                    || (value
                        == (self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]))))
                    || (value
                        == (self::ml_string::Text::from_units(vec![
                            97, 114, 103, 117, 109, 101, 110, 116, 115,
                        ]))))
                    || (value
                        == (self::ml_string::Text::from_units(vec![97, 119, 97, 105, 116]))))
                    || (value
                        == (self::ml_string::Text::from_units(vec![121, 105, 101, 108, 100]))))
                    || (value == (self::ml_string::Text::from_units(vec![61, 62]))))
                    || (value == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))
                {
                    self::MlTemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlTemporaryFindJavaScriptClosedClassFieldEndLoop6Return(-1f64)
                } else if ((value == (self::ml_string::Text::from_units(vec![40])))
                    || (value == (self::ml_string::Text::from_units(vec![91]))))
                    || (value == (self::ml_string::Text::from_units(vec![123])))
                {
                    {
                        let end = self::find_java_script_class_group_end(values, at);
                        if (end < 0f64) || (end >= last) {
                            self::MlTemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlTemporaryFindJavaScriptClosedClassFieldEndLoop6Return(-1f64)
                        } else {
                            {
                                let inner = at + 1f64;
                                {
                                    let source_ml_source1_s102 = self::source_ml_temporary_find_java_script_closed_class_field_end_loop7(values, end, inner);
                                    match source_ml_source1_s102 {
                                                self::MlTemporaryFindJavaScriptClosedClassFieldEndLoop7Result::MlTemporaryFindJavaScriptClosedClassFieldEndLoop7Done => {
                                                    {
                                                        let at_2 = end;
                                                        {
                                                            let at_3 = at_2 + 1f64;
                                                            {
                                                                at = at_3;
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryFindJavaScriptClosedClassFieldEndLoop7Result::MlTemporaryFindJavaScriptClosedClassFieldEndLoop7Return(source_ml_temporary_result7) => {
                                                    self::MlTemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlTemporaryFindJavaScriptClosedClassFieldEndLoop6Return(source_ml_temporary_result7)
                                                }
                                            }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let at_4 = at + 1f64;
                        {
                            at = at_4;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptClosedClassFieldEndLoop6Result::MlTemporaryFindJavaScriptClosedClassFieldEndLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Result {
    MlTemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Done(self::ml_string::Text, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9 {
    MlTemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9Next(self::ml_string::Text, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_render_java_script_closed_class_field_documentation_loop8(
    comment: &self::ml_string::Text,
    mut result: self::ml_string::Text,
    mut previous: f64,
    mut at: f64,
) -> self::MlTemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Result {
    loop {
        return if at >= 0f64 {
            {
                let before = if at == 0f64 {
                    self::ml_string::Text::from_units(vec![32])
                } else {
                    {
                        let source_ml_source1_string_unit_operand103 = comment.clone();
                        {
                            let source_ml_source1_string_unit_operand104 = at - 1f64;
                            (std::rc::Rc::new(
                                move |text: self::ml_string::Text,
                                      position: f64|
                                      -> self::ml_string::Text {
                                    {
                                        let index = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if (index < 0f64) || (index >= (text.length())) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            {
                                                let source_ml_source1_string_range_operand105 =
                                                    text.clone();
                                                {
                                                    let source_ml_source1_string_range_operand106 =
                                                        index;
                                                    {
                                                        let source_ml_source1_string_range_operand107 =
                                                            index + 1f64;
                                                        {
                                                            let source_ml_source1_string_range_position108 = source_ml_source1_string_range_operand106;
                                                            {
                                                                let source_ml_source1_string_range_position109 = source_ml_source1_string_range_operand107;
                                                                source_ml_source1_string_range_operand105.slice(source_ml_source1_string_range_position108, source_ml_source1_string_range_position109)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                            ))(
                                source_ml_source1_string_unit_operand103,
                                source_ml_source1_string_unit_operand104,
                            )
                        }
                    }
                };
                {
                    let after = {
                        let source_ml_source1_string_unit_operand110 = comment.clone();
                        {
                            let source_ml_source1_string_unit_operand111 = at + 5f64;
                            (std::rc::Rc::new(
                                move |text: self::ml_string::Text,
                                      position: f64|
                                      -> self::ml_string::Text {
                                    {
                                        let index = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if (index < 0f64) || (index >= (text.length())) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            {
                                                let source_ml_source1_string_range_operand112 =
                                                    text.clone();
                                                {
                                                    let source_ml_source1_string_range_operand113 =
                                                        index;
                                                    {
                                                        let source_ml_source1_string_range_operand114 =
                                                            index + 1f64;
                                                        {
                                                            let source_ml_source1_string_range_position115 = source_ml_source1_string_range_operand113;
                                                            {
                                                                let source_ml_source1_string_range_position116 = source_ml_source1_string_range_operand114;
                                                                source_ml_source1_string_range_operand112.slice(source_ml_source1_string_range_position115, source_ml_source1_string_range_position116)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                            ))(
                                source_ml_source1_string_unit_operand110,
                                source_ml_source1_string_unit_operand111,
                            )
                        }
                    };
                    {
                        let source_ml_source1_s122 = if self::ml_array::includes(
                            &vec![
                                self::ml_string::Text::from_units(vec![32]),
                                self::ml_string::Text::from_units(vec![9]),
                                self::ml_string::Text::from_units(vec![13]),
                                self::ml_string::Text::from_units(vec![10]),
                                self::ml_string::Text::from_units(vec![42]),
                            ],
                            &before,
                            0f64,
                            false,
                            |a, b| a == b,
                        ) && self::ml_array::includes(
                            &vec![
                                self::ml_string::Text::from_units(vec![32]),
                                self::ml_string::Text::from_units(vec![9]),
                                self::ml_string::Text::from_units(vec![13]),
                                self::ml_string::Text::from_units(vec![10]),
                                self::ml_string::Text::from_units(vec![123]),
                                self::ml_string::Text::from_units(vec![]),
                            ],
                            &after,
                            0f64,
                            false,
                            |a, b| a == b,
                        ) {
                            {
                                let result_2 = result.concat(&({
                                    let source_ml_source1_string_range_operand123 = comment.clone();
                                    {
                                        let source_ml_source1_string_range_operand124 = previous;
                                        {
                                            let source_ml_source1_string_range_operand125 = at;
                                            {
                                                let source_ml_source1_string_range_position126 = source_ml_source1_string_range_operand124;
                                                {
                                                    let source_ml_source1_string_range_position127 = source_ml_source1_string_range_operand125;
                                                    source_ml_source1_string_range_operand123.slice(source_ml_source1_string_range_position126, source_ml_source1_string_range_position127)
                                                }
                                            }
                                        }
                                    }
                                }).concat(&self::ml_string::Text::from_units(vec![64, 114, 101, 116, 117, 114, 110, 115])));
                                {
                                    let previous_2 = at + 5f64;
                                    self::MlTemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9::MlTemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9Next(result_2, previous_2)
                                }
                            }
                        } else {
                            self::MlTemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9::MlTemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9Next(result.clone(), previous)
                        };
                        match source_ml_source1_s122 {
                            self::MlTemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9::MlTemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9Next(result_3, previous_3) => {
                                {
                                    let at_2 = {
                                        let source_ml_source1_string_search_operand128 = comment.clone();
                                        {
                                            let source_ml_source1_string_search_operand129 = self::ml_string::Text::from_units(vec![64, 116, 121, 112, 101]);
                                            {
                                                let source_ml_source1_string_search_operand130 = at + 5f64;
                                                {
                                                    let source_ml_source1_string_search_text131 = source_ml_source1_string_search_operand129;
                                                    {
                                                        let source_ml_source1_string_search_position132 = source_ml_source1_string_search_operand130;
                                                        source_ml_source1_string_search_operand128.index_of(&source_ml_source1_string_search_text131, source_ml_source1_string_search_position132)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        (result, previous, at) = (result_3.clone(), previous_3, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Result::MlTemporaryRenderJavaScriptClosedClassFieldDocumentationLoop8Done(result.clone(), previous)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_prefix_end_loop10(
    row: &[f64],
    mut end: f64,
    mut fields: f64,
    mut at: f64,
) -> f64 {
    loop {
        if at < (9f64 + (fields * 3f64)) {
            if ((self::ml_array::optional_at(row, self::ml_array::number_index(at)))
                .unwrap_or_else(|| -1f64))
                >= 0f64
            {
                {
                    let end_2 =
                        (self::ml_array::optional_at(row, self::ml_array::number_index(at + 2f64)))
                            .unwrap_or_else(|| 0f64)
                            + 1f64;
                    {
                        let at_2 = at + 3f64;
                        {
                            (end, at) = (end_2, at_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let at_3 = at + 3f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        }
        return { end };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Result {
    MlTemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Done,
    MlTemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_assigned_field_position_loop11(
    values: &[self::ml_string::Text],
    row: &[f64],
    name: &self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Result {
    loop {
        return if at
            < (9f64
                + (3f64
                    * (self::ml_array::optional_at(row, self::ml_array::number_index(7f64)))
                        .unwrap_or_else(|| 0f64)))
        {
            if (((self::ml_array::optional_at(row, self::ml_array::number_index(at)))
                .unwrap_or_else(|| -1f64))
                >= 0f64)
                && (((self::ml_array::optional_at(
                    values,
                    self::ml_array::number_index(
                        (self::ml_array::optional_at(row, self::ml_array::number_index(at)))
                            .unwrap_or_else(|| 0f64),
                    ),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                    == (*name))
            {
                self::MlTemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Result::MlTemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Return(at)
            } else {
                {
                    let at_2 = at + 3f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Result::MlTemporaryReadJavaScriptClosedClassAssignedFieldPositionLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryJoin14 {
    MlTemporaryReadJavaScriptClosedClassDirectoryJoin14Next,
    MlTemporaryReadJavaScriptClosedClassDirectoryJoin14Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop15Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop15Done,
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop15Return(Vec<f64>),
}
