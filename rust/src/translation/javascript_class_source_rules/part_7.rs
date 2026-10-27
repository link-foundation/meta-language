// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryLowerJavaScriptClosedClassSourceLoop37Result;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_closed_class_directory;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop37(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    names: &[self::ml_string::Text],
    mut error: f64,
    mut error_at: f64,
    mut at: f64,
) -> self::MlTemporaryLowerJavaScriptClosedClassSourceLoop37Result {
    loop {
        return if at < (values.len() as f64) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let previous = (self::ml_array::optional_at(
                        values,
                        self::ml_array::number_index(at - 1f64),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    if (((previous != (self::ml_string::Text::from_units(vec![46])))
                        && (previous != (self::ml_string::Text::from_units(vec![63, 46]))))
                        && ((((value
                            == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))
                            || (value
                                == (self::ml_string::Text::from_units(vec![
                                    116, 104, 105, 115,
                                ]))))
                            || ((value
                                == (self::ml_string::Text::from_units(vec![
                                    115, 116, 97, 116, 105, 99,
                                ])))
                                && (((self::ml_array::optional_at(
                                    kinds,
                                    self::ml_array::number_index(at + 1f64),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                                    == (self::ml_string::Text::from_units(vec![
                                        105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                                    ])))))
                            || ((value
                                == (self::ml_string::Text::from_units(vec![110, 101, 119])))
                                && self::ml_array::includes(
                                    names,
                                    &(self::ml_array::optional_at(
                                        values,
                                        self::ml_array::number_index(at + 1f64),
                                    ))
                                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                ))))
                        && (({
                            let source_ml_source1_string_range_operand161 = source.clone();
                            {
                                let source_ml_source1_string_range_operand162 =
                                    (self::ml_array::optional_at(
                                        starts,
                                        self::ml_array::number_index(at),
                                    ))
                                    .unwrap_or_else(|| 0f64);
                                {
                                    let source_ml_source1_string_range_operand163 =
                                        (self::ml_array::optional_at(
                                            ends,
                                            self::ml_array::number_index(at),
                                        ))
                                        .unwrap_or_else(|| 0f64);
                                    {
                                        let source_ml_source1_string_range_position164 =
                                            source_ml_source1_string_range_operand162;
                                        {
                                            let source_ml_source1_string_range_position165 =
                                                source_ml_source1_string_range_operand163;
                                            source_ml_source1_string_range_operand161.slice(
                                                source_ml_source1_string_range_position164,
                                                source_ml_source1_string_range_position165,
                                            )
                                        }
                                    }
                                }
                            }
                        }) != value)
                    {
                        {
                            let error_2 = -9f64;
                            {
                                let error_at_2 = at;
                                self::MlTemporaryLowerJavaScriptClosedClassSourceLoop37Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop37Done(error_2, error_at_2)
                            }
                        }
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
            }
        } else {
            self::MlTemporaryLowerJavaScriptClosedClassSourceLoop37Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop37Done(error, error_at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceLoop32Result {
    MlTemporaryLowerJavaScriptClosedClassSourceLoop32Done(
        Vec<Vec<f64>>,
        Vec<self::ml_string::Text>,
        f64,
        f64,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop38(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut end: f64,
) -> f64 {
    loop {
        return if (end < (values.len() as f64))
            && (((self::ml_array::optional_at(values, self::ml_array::number_index(end)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                != (self::ml_string::Text::from_units(vec![59])))
        {
            if ((self::ml_array::optional_at(kinds, self::ml_array::number_index(end)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))
            {
                end
            } else {
                {
                    let end_2 = end + 1f64;
                    {
                        end = end_2;
                        continue;
                    }
                }
            }
        } else {
            end
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
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop32(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut directory: Vec<Vec<f64>>,
    mut names: Vec<self::ml_string::Text>,
    mut depth: f64,
    mut declarations_finished: bool,
    mut error: f64,
    mut error_at: f64,
    mut at: f64,
) -> self::MlTemporaryLowerJavaScriptClosedClassSourceLoop32Result {
    loop {
        return if (at < (values.len() as f64)) && (error == 0f64) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if ((((value == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))
                    && (((self::ml_array::optional_at(kinds, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                        == (self::ml_string::Text::from_units(vec![
                            105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
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
                        != (self::ml_string::Text::from_units(vec![63, 46]))))
                    && ((((self::ml_array::optional_at(
                        kinds,
                        self::ml_array::number_index(at + 1f64),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                        == (self::ml_string::Text::from_units(vec![
                            105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                        ])))
                        || (((self::ml_array::optional_at(
                            values,
                            self::ml_array::number_index(at + 1f64),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                            == (self::ml_string::Text::from_units(vec![123]))))
                {
                    if (directory.len() as f64) >= 64f64 {
                        {
                            let error_2 = -8f64;
                            {
                                let error_at_2 = at;
                                self::MlTemporaryLowerJavaScriptClosedClassSourceLoop32Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop32Done(directory, names, error_2, error_at_2)
                            }
                        }
                    } else if (depth != 0f64) || declarations_finished {
                        {
                            let error_3 = -7f64;
                            {
                                let error_at_3 = at;
                                self::MlTemporaryLowerJavaScriptClosedClassSourceLoop32Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop32Done(directory, names, error_3, error_at_3)
                            }
                        }
                    } else {
                        {
                            let row =
                                self::read_java_script_closed_class_directory(values, kinds, at);
                            if ((self::ml_array::optional_at(
                                &row,
                                self::ml_array::number_index(0f64),
                            ))
                            .unwrap_or_else(|| -1f64))
                                < 0f64
                            {
                                {
                                    let error_4 = (self::ml_array::optional_at(
                                        &row,
                                        self::ml_array::number_index(0f64),
                                    ))
                                    .unwrap_or_else(|| -1f64);
                                    {
                                        let error_at_4 = (self::ml_array::optional_at(
                                            &row,
                                            self::ml_array::number_index(1f64),
                                        ))
                                        .unwrap_or_else(|| at);
                                        self::MlTemporaryLowerJavaScriptClosedClassSourceLoop32Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop32Done(directory, names, error_4, error_at_4)
                                    }
                                }
                            } else {
                                {
                                    let name = (self::ml_array::optional_at(
                                        values,
                                        self::ml_array::number_index(at + 1f64),
                                    ))
                                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                                    if self::ml_array::includes(
                                        &names,
                                        &name.clone(),
                                        0f64,
                                        false,
                                        |a, b| a == b,
                                    ) {
                                        {
                                            let error_5 = -7f64;
                                            {
                                                let error_at_5 = at + 1f64;
                                                self::MlTemporaryLowerJavaScriptClosedClassSourceLoop32Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop32Done(directory, names, error_5, error_at_5)
                                            }
                                        }
                                    } else {
                                        {
                                            let names_2 = {
                                                let source_ml_source1_concat_value166 = names;
                                                {
                                                    let source_ml_source1_concat_value167 =
                                                        vec![name.clone()];
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<self::ml_string::Text>::new(),
                                                            source_ml_source1_concat_value166,
                                                        ),
                                                        source_ml_source1_concat_value167,
                                                    )
                                                }
                                            };
                                            {
                                                let directory_2 = {
                                                    let source_ml_source1_concat_value168 =
                                                        directory;
                                                    {
                                                        let source_ml_source1_concat_value169 =
                                                            vec![row.clone()];
                                                        self::ml_array::append(
                                                            self::ml_array::append(
                                                                Vec::<Vec<f64>>::new(),
                                                                source_ml_source1_concat_value168,
                                                            ),
                                                            source_ml_source1_concat_value169,
                                                        )
                                                    }
                                                };
                                                {
                                                    let at_2 = (self::ml_array::optional_at(
                                                        &row,
                                                        self::ml_array::number_index(1f64),
                                                    ))
                                                    .unwrap_or_else(|| at);
                                                    {
                                                        let at_3 = at_2 + 1f64;
                                                        {
                                                            (directory, names, at) =
                                                                (directory_2, names_2, at_3);
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
                } else if (depth == 0f64)
                    && (value
                        == (self::ml_string::Text::from_units(vec![105, 109, 112, 111, 114, 116])))
                {
                    {
                        let end = at;
                        {
                            let end_2 = self::source_ml_temporary_lower_java_script_closed_class_source_loop38(values, kinds, end);
                            if (self::ml_array::includes(
                                &vec![
                                    self::ml_string::Text::from_units(vec![
                                        35, 110, 111, 100, 101, 58, 97, 115, 115, 101, 114, 116,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        35, 110, 111, 100, 101, 58, 97, 115, 115, 101, 114, 116,
                                        47, 115, 116, 114, 105, 99, 116,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        35, 97, 115, 115, 101, 114, 116,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        35, 97, 115, 115, 101, 114, 116, 47, 115, 116, 114, 105,
                                        99, 116,
                                    ]),
                                ],
                                &(self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(end_2),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                                0f64,
                                false,
                                |a, b| a == b,
                            )) {
                                {
                                    let at_4 = end_2;
                                    {
                                        let at_5 = at_4 + 1f64;
                                        {
                                            at = at_5;
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                {
                                    let error_6 = -7f64;
                                    {
                                        let error_at_6 = at;
                                        self::MlTemporaryLowerJavaScriptClosedClassSourceLoop32Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop32Done(directory, names, error_6, error_at_6)
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let declarations_finished_3 = if ((depth == 0f64)
                            && (value != (self::ml_string::Text::from_units(vec![59]))))
                            && (value
                                != (self::ml_string::Text::from_units(vec![
                                    35, 117, 115, 101, 32, 115, 116, 114, 105, 99, 116,
                                ]))) {
                            { true }
                        } else {
                            declarations_finished
                        };
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
                                        let at_6 = at + 1f64;
                                        {
                                            (depth, declarations_finished, at) =
                                                (depth_4, declarations_finished_3, at_6);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                {
                                    let at_7 = at + 1f64;
                                    {
                                        (depth, declarations_finished, at) =
                                            (depth_3, declarations_finished_3, at_7);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryLowerJavaScriptClosedClassSourceLoop32Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop32Done(directory, names, error, error_at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryProjectJavaScriptClassSourceIntervalLoop39Result {
    MlTemporaryProjectJavaScriptClassSourceIntervalLoop39Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_project_java_script_class_source_interval_loop39(
    intervals: &[f64],
    mut first: f64,
    mut last: f64,
    mut low: f64,
    mut high: f64,
    mut at: f64,
) -> self::MlTemporaryProjectJavaScriptClassSourceIntervalLoop39Result {
    loop {
        if at < (intervals.len() as f64) {
            {
                let start =
                    (self::ml_array::optional_at(intervals, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| 0f64);
                {
                    let end = (self::ml_array::optional_at(
                        intervals,
                        self::ml_array::number_index(at + 1f64),
                    ))
                    .unwrap_or_else(|| 0f64);
                    if ((end < first) || (start > last)) || (start == end) {
                        {
                            let at_2 = at + 4f64;
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let original_start = (self::ml_array::optional_at(
                                intervals,
                                self::ml_array::number_index(at + 2f64),
                            ))
                            .unwrap_or_else(|| 0f64);
                            {
                                let original_end = (self::ml_array::optional_at(
                                    intervals,
                                    self::ml_array::number_index(at + 3f64),
                                ))
                                .unwrap_or_else(|| 0f64);
                                {
                                    let low_2 = if low < 0f64 {
                                        original_start
                                    } else {
                                        self::ml_math::min(low, original_start)
                                    };
                                    {
                                        let high_2 = self::ml_math::max(high, original_end);
                                        {
                                            let at_3 = at + 4f64;
                                            {
                                                (low, high, at) = (low_2, high_2, at_3);
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
        return {
            self::MlTemporaryProjectJavaScriptClassSourceIntervalLoop39Result::MlTemporaryProjectJavaScriptClassSourceIntervalLoop39Done(low, high)
        };
    }
}
