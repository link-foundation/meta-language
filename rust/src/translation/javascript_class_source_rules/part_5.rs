// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3;
use super::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result;
use super::find_java_script_class_group_end;
use super::ml_array;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_parameter_positions_with_collections_loop2(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut close: f64,
    mut collections: bool,
    mut result: Vec<f64>,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result {
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
                    self::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Return(vec![-1f64, at])
                } else {
                    {
                        let result_2 = {
                            let source_ml_source1_concat_value141 = result;
                            {
                                let source_ml_source1_concat_value142 = vec![at];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value141,
                                    ),
                                    source_ml_source1_concat_value142,
                                )
                            }
                        };
                        {
                            let at_2 = at + 1f64;
                            {
                                let source_ml_source1_s143 = if ((self::ml_array::optional_at(
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
                                                        self::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Return(vec![-1f64, at_4])
                                                    } else {
                                                        {
                                                            let at_5 = at_4 + 1f64;
                                                            self::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_5)
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
                                                    self::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_6)
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
                                                    self::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_7)
                                                }
                                            } else if (((collections
                                                && (value
                                                    == (self::ml_string::Text::from_units(
                                                        vec![110, 101, 119],
                                                    ))))
                                                && self::ml_array::includes(
                                                    &vec![
                                                        self::ml_string::Text::from_units(vec![
                                                            83, 101, 116,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            77, 97, 112,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            87, 101, 97, 107, 77, 97, 112,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            87, 101, 97, 107, 83, 101, 116,
                                                        ]),
                                                    ],
                                                    &(self::ml_array::optional_at(
                                                        values,
                                                        self::ml_array::number_index(at_3 + 1f64),
                                                    ))
                                                    .unwrap_or_else(|| {
                                                        self::ml_string::Text::from_units(vec![])
                                                    }),
                                                    0f64,
                                                    false,
                                                    |a, b| a == b,
                                                ))
                                                && (((self::ml_array::optional_at(
                                                    values,
                                                    self::ml_array::number_index(at_3 + 2f64),
                                                ))
                                                .unwrap_or_else(|| {
                                                    self::ml_string::Text::from_units(vec![])
                                                })) == (self::ml_string::Text::from_units(
                                                    vec![40],
                                                ))))
                                                && (((self::ml_array::optional_at(
                                                    values,
                                                    self::ml_array::number_index(at_3 + 3f64),
                                                ))
                                                .unwrap_or_else(|| {
                                                    self::ml_string::Text::from_units(vec![])
                                                })) == (self::ml_string::Text::from_units(
                                                    vec![41],
                                                )))
                                            {
                                                {
                                                    let at_8 = at_3 + 4f64;
                                                    self::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_8)
                                                }
                                            } else {
                                                self::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Return(vec![-1f64, at_3])
                                            }
                                        }
                                    }
                                } else {
                                    self::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_2)
                                };
                                match source_ml_source1_s143 {
                                self::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_9) => {
                                    if at_9 < close {
                                        {
let ml_library_inequality_1 = ((self::ml_array::optional_at(values, self::ml_array::number_index(at_9))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![44])) ;
if ml_library_inequality_1 {
                                            self::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Return(vec![-1f64, at_9])
                                        } else {
                                            {
                                                let at_10 = at_9 + 1f64;
                                                {
                                                    (result, at) = (result_2, at_10);
                                                    continue;
                                                }
                                            }
                                        }
}
                                    } else {
                                        {
                                            (result, at) = (result_2, at_9);
                                            continue;
                                        }
                                    }
                                }
                                self::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Return(source_ml_temporary_result3) => {
                                    self::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Return(source_ml_temporary_result3)
                                }
                            }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result::MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Done(result)
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
                    let source_ml_source1_concat_value149 = result;
                    {
                        let source_ml_source1_concat_value150 = vec![{
                            let source_ml_source1_string_range_operand144 = source.clone();
                            {
                                let source_ml_source1_string_range_operand145 =
                                    (self::ml_array::optional_at(
                                        starts,
                                        self::ml_array::number_index(at),
                                    ))
                                    .unwrap_or_else(|| 0f64);
                                {
                                    let source_ml_source1_string_range_operand146 =
                                        (self::ml_array::optional_at(
                                            ends,
                                            self::ml_array::number_index(at),
                                        ))
                                        .unwrap_or_else(|| 0f64);
                                    {
                                        let source_ml_source1_string_range_position147 =
                                            source_ml_source1_string_range_operand145;
                                        {
                                            let source_ml_source1_string_range_position148 =
                                                source_ml_source1_string_range_operand146;
                                            source_ml_source1_string_range_operand144.slice(
                                                source_ml_source1_string_range_position147,
                                                source_ml_source1_string_range_position148,
                                            )
                                        }
                                    }
                                }
                            }
                        }];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source1_concat_value149,
                            ),
                            source_ml_source1_concat_value150,
                        )
                    }
                };
                {
                    let at_2 = at + 1f64;
                    {
                        let at_8 = if ((self::ml_array::optional_at(
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
                                    } else if (((value
                                        == (self::ml_string::Text::from_units(vec![
                                            110, 101, 119,
                                        ])))
                                        && self::ml_array::includes(
                                            &vec![
                                                self::ml_string::Text::from_units(vec![
                                                    83, 101, 116,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    77, 97, 112,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    87, 101, 97, 107, 77, 97, 112,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    87, 101, 97, 107, 83, 101, 116,
                                                ]),
                                            ],
                                            &(self::ml_array::optional_at(
                                                values,
                                                self::ml_array::number_index(at_3 + 1f64),
                                            ))
                                            .unwrap_or_else(|| {
                                                self::ml_string::Text::from_units(vec![])
                                            }),
                                            0f64,
                                            false,
                                            |a, b| a == b,
                                        ))
                                        && (((self::ml_array::optional_at(
                                            values,
                                            self::ml_array::number_index(at_3 + 2f64),
                                        ))
                                        .unwrap_or_else(|| {
                                            self::ml_string::Text::from_units(vec![])
                                        })) == (self::ml_string::Text::from_units(vec![40]))))
                                        && (((self::ml_array::optional_at(
                                            values,
                                            self::ml_array::number_index(at_3 + 3f64),
                                        ))
                                        .unwrap_or_else(|| {
                                            self::ml_string::Text::from_units(vec![])
                                        })) == (self::ml_string::Text::from_units(vec![41])))
                                    {
                                        { at_3 + 4f64 }
                                    } else {
                                        { at_3 + 1f64 }
                                    }
                                }
                            }
                        } else {
                            at_2
                        };
                        if at_8 < close {
                            {
                                let at_9 = at_8 + 1f64;
                                {
                                    (result, at) = (result_2, at_9);
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                (result, at) = (result_2, at_8);
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
                                    let source_ml_source1_s151 = self::source_ml_temporary_find_java_script_closed_class_field_end_loop7(values, end, inner);
                                    match source_ml_source1_s151 {
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
                        let source_ml_source1_string_unit_operand152 = comment.clone();
                        {
                            let source_ml_source1_string_unit_operand153 = at - 1f64;
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
                                                let source_ml_source1_string_range_operand154 =
                                                    text.clone();
                                                {
                                                    let source_ml_source1_string_range_operand155 =
                                                        index;
                                                    {
                                                        let source_ml_source1_string_range_operand156 =
                                                            index + 1f64;
                                                        {
                                                            let source_ml_source1_string_range_position157 = source_ml_source1_string_range_operand155;
                                                            {
                                                                let source_ml_source1_string_range_position158 = source_ml_source1_string_range_operand156;
                                                                source_ml_source1_string_range_operand154.slice(source_ml_source1_string_range_position157, source_ml_source1_string_range_position158)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                            ))(
                                source_ml_source1_string_unit_operand152,
                                source_ml_source1_string_unit_operand153,
                            )
                        }
                    }
                };
                {
                    let after = {
                        let source_ml_source1_string_unit_operand159 = comment.clone();
                        {
                            let source_ml_source1_string_unit_operand160 = at + 5f64;
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
                                                let source_ml_source1_string_range_operand161 =
                                                    text.clone();
                                                {
                                                    let source_ml_source1_string_range_operand162 =
                                                        index;
                                                    {
                                                        let source_ml_source1_string_range_operand163 =
                                                            index + 1f64;
                                                        {
                                                            let source_ml_source1_string_range_position164 = source_ml_source1_string_range_operand162;
                                                            {
                                                                let source_ml_source1_string_range_position165 = source_ml_source1_string_range_operand163;
                                                                source_ml_source1_string_range_operand161.slice(source_ml_source1_string_range_position164, source_ml_source1_string_range_position165)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                            ))(
                                source_ml_source1_string_unit_operand159,
                                source_ml_source1_string_unit_operand160,
                            )
                        }
                    };
                    {
                        let source_ml_source1_s171 = if self::ml_array::includes(
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
                                    let source_ml_source1_string_range_operand172 = comment.clone();
                                    {
                                        let source_ml_source1_string_range_operand173 = previous;
                                        {
                                            let source_ml_source1_string_range_operand174 = at;
                                            {
                                                let source_ml_source1_string_range_position175 = source_ml_source1_string_range_operand173;
                                                {
                                                    let source_ml_source1_string_range_position176 = source_ml_source1_string_range_operand174;
                                                    source_ml_source1_string_range_operand172.slice(source_ml_source1_string_range_position175, source_ml_source1_string_range_position176)
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
                        match source_ml_source1_s171 {
                            self::MlTemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9::MlTemporaryRenderJavaScriptClosedClassFieldDocumentationJoin9Next(result_3, previous_3) => {
                                {
                                    let at_2 = {
                                        let source_ml_source1_string_search_operand177 = comment.clone();
                                        {
                                            let source_ml_source1_string_search_operand178 = self::ml_string::Text::from_units(vec![64, 116, 121, 112, 101]);
                                            {
                                                let source_ml_source1_string_search_operand179 = at + 5f64;
                                                {
                                                    let source_ml_source1_string_search_text180 = source_ml_source1_string_search_operand178;
                                                    {
                                                        let source_ml_source1_string_search_position181 = source_ml_source1_string_search_operand179;
                                                        source_ml_source1_string_search_operand177.index_of(&source_ml_source1_string_search_text180, source_ml_source1_string_search_position181)
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
