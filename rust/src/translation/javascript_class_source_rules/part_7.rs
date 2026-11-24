// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryRenderJavaScriptClosedClassRangeLoop21Result;
use super::accept_java_script_closed_class_function_export;
use super::find_java_script_class_group_end;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_java_script_closed_class_assigned_field_position;
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
                                    let source_ml_source1_string_range_operand188 = source.clone();
                                    {
                                        let source_ml_source1_string_range_operand189 = previous;
                                        {
                                            let source_ml_source1_string_range_operand190 =
                                                (self::ml_array::optional_at(
                                                    starts,
                                                    self::ml_array::number_index(at),
                                                ))
                                                .unwrap_or_else(|| previous);
                                            {
                                                let source_ml_source1_string_range_position191 =
                                                    source_ml_source1_string_range_operand189;
                                                {
                                                    let source_ml_source1_string_range_position192 =
                                                        source_ml_source1_string_range_operand190;
                                                    source_ml_source1_string_range_operand188.slice(
                                                        source_ml_source1_string_range_position191,
                                                        source_ml_source1_string_range_position192,
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
                                            let source_ml_source1_string_range_operand193 = source.clone();
                                            {
                                                let source_ml_source1_string_range_operand194 = previous;
                                                {
                                                    let source_ml_source1_string_range_operand195 = (self::ml_array::optional_at(starts, self::ml_array::number_index(at))).unwrap_or_else(|| previous);
                                                    {
                                                        let source_ml_source1_string_range_position196 = source_ml_source1_string_range_operand194;
                                                        {
                                                            let source_ml_source1_string_range_position197 = source_ml_source1_string_range_operand195;
                                                            source_ml_source1_string_range_operand193.slice(source_ml_source1_string_range_position196, source_ml_source1_string_range_position197)
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
                        let source_ml_source1_concat_value198 = result;
                        {
                            let source_ml_source1_concat_value199 = vec![
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
                                    source_ml_source1_concat_value198,
                                ),
                                source_ml_source1_concat_value199,
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
                            let source_ml_source1_concat_value202 = result;
                            {
                                let source_ml_source1_concat_value203 = vec![
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
                                        let source_ml_source1_string_argument200 = ordinal;
                                        self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                source_ml_source1_string_argument200,
                                            ),
                                        )
                                    }))
                                    .concat(&self::ml_string::Text::from_units(vec![95])))
                                    .concat(&{
                                        let source_ml_source1_string_argument201 = at;
                                        self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                source_ml_source1_string_argument201,
                                            ),
                                        )
                                    }),
                                ];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source1_concat_value202,
                                    ),
                                    source_ml_source1_concat_value203,
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop29(
    values: &[self::ml_string::Text],
    mut names: Vec<self::ml_string::Text>,
    source_ml_temporary_values27: &[Vec<f64>],
    mut source_ml_temporary_index27: self::ml::Big,
) -> Vec<self::ml_string::Text> {
    loop {
        if source_ml_temporary_index27
            < (self::ml::Big::from_u128(source_ml_temporary_values27.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values27,
                    source_ml_temporary_index27.to_index(),
                );
                {
                    let names_2 = {
                        let source_ml_source1_concat_value204 = names;
                        {
                            let source_ml_source1_concat_value205 = vec![
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
                                    source_ml_source1_concat_value204,
                                ),
                                source_ml_source1_concat_value205,
                            )
                        }
                    };
                    {
                        let source_ml_temporary_index27_2 =
                            source_ml_temporary_index27.add(&self::ml::Big::from_i128(1));
                        {
                            (names, source_ml_temporary_index27) =
                                (names_2, source_ml_temporary_index27_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { names };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result {
    MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Done,
    MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop31(
    row: &[f64],
    mut parameters: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (row.len() as f64) {
            {
                let parameters_2 = {
                    let source_ml_source1_concat_value206 = parameters;
                    {
                        let source_ml_source1_concat_value207 = vec![
                            (self::ml_array::optional_at(
                                row,
                                self::ml_array::number_index(at + 1f64),
                            ))
                            .unwrap_or_else(|| -1f64),
                            (self::ml_array::optional_at(
                                row,
                                self::ml_array::number_index(at + 2f64),
                            ))
                            .unwrap_or_else(|| -1f64),
                        ];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source1_concat_value206,
                            ),
                            source_ml_source1_concat_value207,
                        )
                    }
                };
                {
                    let at_2 = at + 5f64;
                    {
                        (parameters, at) = (parameters_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { parameters };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result {
    MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Done,
    MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result {
    MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Done,
    MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop33(
    values: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    parameters: &[f64],
    mut pair: f64,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result {
    loop {
        return if at
            < ((self::ml_array::optional_at(parameters, self::ml_array::number_index(pair + 1f64)))
                .unwrap_or_else(|| -1f64))
        {
            if (((((self::ml_array::optional_at(
                values,
                self::ml_array::number_index(at - 1f64),
            ))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![61])))
                && (((self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                    == (self::ml_string::Text::from_units(vec![110, 101, 119]))))
                && self::ml_array::includes(
                    &vec![
                        self::ml_string::Text::from_units(vec![83, 101, 116]),
                        self::ml_string::Text::from_units(vec![77, 97, 112]),
                        self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]),
                        self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116]),
                    ],
                    &(self::ml_array::optional_at(values, self::ml_array::number_index(at + 1f64)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                    0f64,
                    false,
                    |a, b| a == b,
                ))
                && self::ml_array::includes(
                    names,
                    &(self::ml_array::optional_at(values, self::ml_array::number_index(at + 1f64)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                    0f64,
                    false,
                    |a, b| a == b,
                )
            {
                self::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Return(at + 1f64)
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
            self::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop32(
    values: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    parameters: &[f64],
    mut pair: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result {
    loop {
        return if pair < (parameters.len() as f64) {
            {
                let at =
                    (self::ml_array::optional_at(parameters, self::ml_array::number_index(pair)))
                        .unwrap_or_else(|| -1f64)
                        + 1f64;
                {
                    let source_ml_source1_s208 = self::source_ml_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop33(values, names, parameters, pair, at);
                    match source_ml_source1_s208 {
                        self::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Done => {
                            {
                                let pair_2 = pair + 2f64;
                                {
                                    pair = pair_2;
                                    continue;
                                }
                            }
                        }
                        self::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Result::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop33Return(source_ml_temporary_result33) => {
                            self::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Return(source_ml_temporary_result33)
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop30(
    values: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    source_ml_temporary_values28: &[Vec<f64>],
    mut source_ml_temporary_index28: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result {
    loop {
        return if source_ml_temporary_index28
            < (self::ml::Big::from_u128(source_ml_temporary_values28.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values28,
                    source_ml_temporary_index28.to_index(),
                );
                {
                    let parameters = vec![
                        (self::ml_array::optional_at(&row, self::ml_array::number_index(3f64)))
                            .unwrap_or_else(|| -1f64),
                        (self::ml_array::optional_at(&row, self::ml_array::number_index(4f64)))
                            .unwrap_or_else(|| -1f64),
                    ];
                    {
                        let at = 9f64
                            + (3f64
                                * (self::ml_array::optional_at(
                                    &row,
                                    self::ml_array::number_index(7f64),
                                ))
                                .unwrap_or_else(|| 0f64));
                        {
                            let parameters_2 = self::source_ml_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop31(&(row), parameters, at);
                            {
                                let pair = 0f64;
                                {
                                    let source_ml_source1_s209 = self::source_ml_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop32(values, names, &(parameters_2), pair);
                                    match source_ml_source1_s209 {
                                        self::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Done => {
                                            {
                                                let source_ml_temporary_index28_2 = source_ml_temporary_index28.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index28 = source_ml_temporary_index28_2;
                                                    continue;
                                                }
                                            }
                                        }
                                        self::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Result::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop32Return(source_ml_temporary_result32) => {
                                            self::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Return(source_ml_temporary_result32)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_function_exports_loop34(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut result: Vec<self::ml_string::Text>,
    mut at: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if at < (values.len() as f64) {
            if (((self::ml_array::optional_at(kinds, self::ml_array::number_index(at)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![
                    105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                ])))
                && self::accept_java_script_closed_class_function_export(values, kinds, at)
            {
                {
                    let result_2 = {
                        let source_ml_source1_concat_value210 = result;
                        {
                            let source_ml_source1_concat_value211 = vec![
                                (self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(at + 2f64),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                            ];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<self::ml_string::Text>::new(),
                                    source_ml_source1_concat_value210,
                                ),
                                source_ml_source1_concat_value211,
                            )
                        }
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (result, at) = (result_2, at_2);
                            continue;
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
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptClosedClassCollectionSizeLoop36Result {
    MlTemporaryAcceptJavaScriptClosedClassCollectionSizeLoop36Done,
    MlTemporaryAcceptJavaScriptClosedClassCollectionSizeLoop36Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_closed_class_collection_size_loop36(
    values: &[self::ml_string::Text],
    mut at: f64,
    source_ml_temporary_values35: &[Vec<f64>],
    mut source_ml_temporary_index35: self::ml::Big,
) -> self::MlTemporaryAcceptJavaScriptClosedClassCollectionSizeLoop36Result {
    loop {
        return if source_ml_temporary_index35
            < (self::ml::Big::from_u128(source_ml_temporary_values35.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values35,
                    source_ml_temporary_index35.to_index(),
                );
                if (at
                    <= ((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64)))
                        .unwrap_or_else(|| 0f64)))
                    || (at
                        >= ((self::ml_array::optional_at(
                            &row,
                            self::ml_array::number_index(1f64),
                        ))
                        .unwrap_or_else(|| 0f64)))
                {
                    {
                        let source_ml_temporary_index35_2 =
                            source_ml_temporary_index35.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index35 = source_ml_temporary_index35_2;
                            continue;
                        }
                    }
                }
                {
                    {
                        let position = self::read_java_script_closed_class_assigned_field_position(
                            values,
                            &(row),
                            &((self::ml_array::optional_at(
                                values,
                                self::ml_array::number_index(at - 2f64),
                            ))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))),
                        );
                        if position < 0f64 {
                            self::MlTemporaryAcceptJavaScriptClosedClassCollectionSizeLoop36Result::MlTemporaryAcceptJavaScriptClosedClassCollectionSizeLoop36Return(false)
                        } else {
                            {
                                let start = (self::ml_array::optional_at(
                                    &row,
                                    self::ml_array::number_index(position + 1f64),
                                ))
                                .unwrap_or_else(|| 0f64);
                                {
                                    let end = (self::ml_array::optional_at(
                                        &row,
                                        self::ml_array::number_index(position + 2f64),
                                    ))
                                    .unwrap_or_else(|| 0f64);
                                    self::MlTemporaryAcceptJavaScriptClosedClassCollectionSizeLoop36Result::MlTemporaryAcceptJavaScriptClosedClassCollectionSizeLoop36Return((((((self::ml_array::optional_at(values, self::ml_array::number_index(start))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![110, 101, 119]))) && self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![77, 97, 112])], &(self::ml_array::optional_at(values, self::ml_array::number_index(start + 1f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) && (((self::ml_array::optional_at(values, self::ml_array::number_index(start + 2f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![40])))) && ((self::find_java_script_class_group_end(values, (start + 2f64))) == (end - 1f64)))
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptJavaScriptClosedClassCollectionSizeLoop36Result::MlTemporaryAcceptJavaScriptClosedClassCollectionSizeLoop36Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorJoin41 {
    MlTemporaryReadJavaScriptClosedClassObservationErrorJoin41Next,
    MlTemporaryReadJavaScriptClosedClassObservationErrorJoin41Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Return(f64),
}
