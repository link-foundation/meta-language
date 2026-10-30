// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryRenderJavaScriptClosedClassRangeLoop21Result;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_java_script_closed_class_methods;
use super::read_java_script_closed_class_static_target;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_render_java_script_closed_class_range_loop21(
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
) -> self::MlTemporaryRenderJavaScriptClosedClassRangeLoop21Result {
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
                                    let source_ml_source1_string_range_operand149 = source.clone();
                                    {
                                        let source_ml_source1_string_range_operand150 = previous;
                                        {
                                            let source_ml_source1_string_range_operand151 =
                                                (self::ml_array::optional_at(
                                                    starts,
                                                    self::ml_array::number_index(at),
                                                ))
                                                .unwrap_or_else(|| previous);
                                            {
                                                let source_ml_source1_string_range_position152 =
                                                    source_ml_source1_string_range_operand150;
                                                {
                                                    let source_ml_source1_string_range_position153 =
                                                        source_ml_source1_string_range_operand151;
                                                    source_ml_source1_string_range_operand149.slice(
                                                        source_ml_source1_string_range_position152,
                                                        source_ml_source1_string_range_position153,
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
                                            let source_ml_source1_string_range_operand154 = source.clone();
                                            {
                                                let source_ml_source1_string_range_operand155 = previous;
                                                {
                                                    let source_ml_source1_string_range_operand156 = (self::ml_array::optional_at(starts, self::ml_array::number_index(at))).unwrap_or_else(|| previous);
                                                    {
                                                        let source_ml_source1_string_range_position157 = source_ml_source1_string_range_operand155;
                                                        {
                                                            let source_ml_source1_string_range_position158 = source_ml_source1_string_range_operand156;
                                                            source_ml_source1_string_range_operand154.slice(source_ml_source1_string_range_position157, source_ml_source1_string_range_position158)
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
            self::MlTemporaryRenderJavaScriptClosedClassRangeLoop21Result::MlTemporaryRenderJavaScriptClosedClassRangeLoop21Done(result.clone(), previous)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_methods_loop22(
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
                        let source_ml_source1_concat_value159 = result;
                        {
                            let source_ml_source1_concat_value160 = vec![
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
                                    source_ml_source1_concat_value159,
                                ),
                                source_ml_source1_concat_value160,
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
pub fn source_ml_temporary_read_java_script_closed_class_static_calls_loop25(
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
                            let source_ml_source1_concat_value163 = result;
                            {
                                let source_ml_source1_concat_value164 = vec![
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
                                        let source_ml_source1_string_argument161 = ordinal;
                                        self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                source_ml_source1_string_argument161,
                                            ),
                                        )
                                    }))
                                    .concat(&self::ml_string::Text::from_units(vec![95])))
                                    .concat(&{
                                        let source_ml_source1_string_argument162 = at;
                                        self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                source_ml_source1_string_argument162,
                                            ),
                                        )
                                    }),
                                ];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source1_concat_value163,
                                    ),
                                    source_ml_source1_concat_value164,
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
pub fn source_ml_temporary_read_java_script_closed_class_static_calls_loop24(
    values: &[self::ml_string::Text],
    prefix: &self::ml_string::Text,
    mut result: Vec<self::ml_string::Text>,
    mut ordinal: f64,
    source_ml_temporary_values23: &[Vec<f64>],
    mut source_ml_temporary_index23: self::ml::Big,
) -> Vec<self::ml_string::Text> {
    loop {
        if source_ml_temporary_index23
            < (self::ml::Big::from_u128(source_ml_temporary_values23.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values23,
                    source_ml_temporary_index23.to_index(),
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
                        let at = 9f64
                            + (3f64
                                * (self::ml_array::optional_at(
                                    &row,
                                    self::ml_array::number_index(7f64),
                                ))
                                .unwrap_or_else(|| 0f64));
                        {
                            let result_2 = self::source_ml_temporary_read_java_script_closed_class_static_calls_loop25(values, prefix, result, ordinal, &(row), &(name), at);
                            {
                                let ordinal_2 = ordinal + 1f64;
                                {
                                    let source_ml_temporary_index23_2 = source_ml_temporary_index23
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, ordinal, source_ml_temporary_index23) =
                                            (result_2, ordinal_2, source_ml_temporary_index23_2);
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
pub enum MlTemporaryReadJavaScriptClosedClassStaticTargetLoop26Result {
    MlTemporaryReadJavaScriptClosedClassStaticTargetLoop26Done,
    MlTemporaryReadJavaScriptClosedClassStaticTargetLoop26Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_static_target_loop26(
    calls: &[self::ml_string::Text],
    key: &self::ml_string::Text,
    mut index: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop26Result {
    loop {
        return if index < (calls.len() as f64) {
            if ((self::ml_array::optional_at(calls, self::ml_array::number_index(index)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (*key)
            {
                self::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop26Result::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop26Return((self::ml_array::optional_at(calls, self::ml_array::number_index(index + 1f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
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
            self::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop26Result::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop26Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorJoin31 {
    MlTemporaryReadJavaScriptClosedClassObservationErrorJoin31Next,
    MlTemporaryReadJavaScriptClosedClassObservationErrorJoin31Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop32Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop32Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop32Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop32(
    mut at: f64,
    source_ml_temporary_values28: &[Vec<f64>],
    mut source_ml_temporary_index28: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop32Result {
    loop {
        return if source_ml_temporary_index28
            < (self::ml::Big::from_u128(source_ml_temporary_values28.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values28,
                    source_ml_temporary_index28.to_index(),
                );
                if (at
                    > ((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64)))
                        .unwrap_or_else(|| 0f64)))
                    && (at
                        < ((self::ml_array::optional_at(&row, self::ml_array::number_index(1f64)))
                            .unwrap_or_else(|| 0f64)))
                {
                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop32Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop32Return(at)
                } else {
                    {
                        let source_ml_temporary_index28_2 =
                            source_ml_temporary_index28.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index28 = source_ml_temporary_index28_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop32Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop32Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop30(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    directory: &[Vec<f64>],
    static_calls: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    methods: &[self::ml_string::Text],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Result {
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
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Return(at)
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
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Return(at)
                            } else {
                                {
                                    let source_ml_source1_s166 = if value
                                        == (self::ml_string::Text::from_units(vec![
                                            102, 117, 110, 99, 116, 105, 111, 110,
                                        ])) {
                                        {
                                            let source_ml_temporary_values28 = directory.to_vec();
                                            {
                                                let source_ml_temporary_index28 =
                                                    self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_s167 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop32(at, &(source_ml_temporary_values28), source_ml_temporary_index28);
                                                    match source_ml_source1_s167 {
                                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop32Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop32Done => {
                                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin31::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin31Next
                                                            }
                                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop32Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop32Return(source_ml_temporary_result32) => {
                                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin31::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin31Return(source_ml_temporary_result32)
                                                            }
                                                        }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin31::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin31Next
                                    };
                                    match source_ml_source1_s166 {
                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin31::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin31Next => {
                                                if ((((kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) && self::ml_array::includes(names, &value.clone(), 0f64, false, |a, b| a == b)) && (previous != (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))) && ((previous != (self::ml_string::Text::from_units(vec![110, 101, 119]))) || (next != (self::ml_string::Text::from_units(vec![40]))))) && ((self::read_java_script_closed_class_static_target(values, names, static_calls, at)) == (self::ml_string::Text::from_units(vec![]))) {
                                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Return(at)
                                                } else if ((((value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))) && (previous != (self::ml_string::Text::from_units(vec![46])))) && (previous != (self::ml_string::Text::from_units(vec![63, 46])))) && (next != (self::ml_string::Text::from_units(vec![46])))) && (next != (self::ml_string::Text::from_units(vec![91]))) {
                                                        self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Return(at)
                                                    } else if ((previous == (self::ml_string::Text::from_units(vec![46]))) || (previous == (self::ml_string::Text::from_units(vec![63, 46])))) && ((self::ml_array::includes(methods, &value.clone(), 0f64, false, |a, b| a == b) && (next != (self::ml_string::Text::from_units(vec![40])))) || (value == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])))) {
                                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Return(at)
                                                        } else if ((previous == (self::ml_string::Text::from_units(vec![91]))) && (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) && self::ml_array::includes(methods, &{
                                                                let source_ml_source1_string_range_operand168 = value.clone();
                                                                {
                                                                    let source_ml_source1_string_range_operand169 = 1f64;
                                                                    {
                                                                        let source_ml_source1_string_range_position170 = source_ml_source1_string_range_operand169;
                                                                        {
                                                                            let source_ml_source1_string_range_position171 = f64::INFINITY;
                                                                            source_ml_source1_string_range_operand168.slice(source_ml_source1_string_range_position170, source_ml_source1_string_range_position171)
                                                                        }
                                                                    }
                                                                }
                                                            }, 0f64, false, |a, b| a == b) {
                                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Return(at)
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
                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin31::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin31Return(source_ml_temporary_result31) => {
                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Return(source_ml_temporary_result31)
                                            }
                                        }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop29Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop29Done(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
    ),
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop29Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop33Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop33Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop33Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop34Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop34Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop34Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop34(
    values: &[self::ml_string::Text],
    row: &[f64],
    mut at: f64,
    mut inner: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop34Result {
    loop {
        return if inner
            < ((self::ml_array::optional_at(row, self::ml_array::number_index(at + 4f64)))
                .unwrap_or_else(|| 0f64))
        {
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
                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop34Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop34Return(inner)
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
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop34Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop34Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop33(
    values: &[self::ml_string::Text],
    row: &[f64],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop33Result {
    loop {
        return if at < (row.len() as f64) {
            if ((self::ml_array::optional_at(row, self::ml_array::number_index(at)))
                .unwrap_or_else(|| 0f64))
                < 0f64
            {
                {
                    let inner =
                        (self::ml_array::optional_at(row, self::ml_array::number_index(at + 3f64)))
                            .unwrap_or_else(|| 0f64)
                            + 1f64;
                    {
                        let source_ml_source1_s172 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop34(values, row, at, inner);
                        match source_ml_source1_s172 {
                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop34Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop34Done => {
                                {
                                    let at_2 = at + 5f64;
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop34Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop34Return(source_ml_temporary_result34) => {
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop33Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop33Return(source_ml_temporary_result34)
                            }
                        }
                    }
                }
            } else {
                {
                    let at_3 = at + 5f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop33Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop33Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop29(
    values: &[self::ml_string::Text],
    mut names: Vec<self::ml_string::Text>,
    mut methods: Vec<self::ml_string::Text>,
    source_ml_temporary_values27: &[Vec<f64>],
    mut source_ml_temporary_index27: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop29Result {
    loop {
        return if source_ml_temporary_index27
            < (self::ml::Big::from_u128(source_ml_temporary_values27.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values27,
                    source_ml_temporary_index27.to_index(),
                );
                {
                    let names_2 = {
                        let source_ml_source1_concat_value173 = names;
                        {
                            let source_ml_source1_concat_value174 = vec![
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
                                    source_ml_source1_concat_value173,
                                ),
                                source_ml_source1_concat_value174,
                            )
                        }
                    };
                    {
                        let methods_2 = {
                            let source_ml_source1_concat_value175 = methods;
                            {
                                let source_ml_source1_concat_value176 =
                                    self::read_java_script_closed_class_methods(values, &(row));
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source1_concat_value175,
                                    ),
                                    source_ml_source1_concat_value176,
                                )
                            }
                        };
                        {
                            let at = 9f64
                                + (3f64
                                    * (self::ml_array::optional_at(
                                        &row,
                                        self::ml_array::number_index(7f64),
                                    ))
                                    .unwrap_or_else(|| 0f64));
                            {
                                let source_ml_source1_s177 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop33(values, &(row), at);
                                match source_ml_source1_s177 {
                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop33Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop33Done => {
                                        {
                                            let source_ml_temporary_index27_2 = source_ml_temporary_index27.add(&self::ml::Big::from_i128(1));
                                            {
                                                (names, methods, source_ml_temporary_index27) = (names_2, methods_2, source_ml_temporary_index27_2);
                                                continue;
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop33Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop33Return(source_ml_temporary_result33) => {
                                        self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop29Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop29Return(source_ml_temporary_result33)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop29Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop29Done(names, methods)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderJavaScriptClosedClassLoop36Result {
    MlTemporaryRenderJavaScriptClosedClassLoop36Done(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
}
