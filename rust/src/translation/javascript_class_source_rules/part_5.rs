// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryRenderJavaScriptClosedClassLoop20Result;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::render_java_script_closed_class;
use super::render_java_script_closed_class_range;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_render_java_script_closed_class_loop20(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    row: &[f64],
    prefix: &self::ml_string::Text,
    mut ordinal: f64,
    class_names: &[self::ml_string::Text],
    receiver: &self::ml_string::Text,
    mut stubs: self::ml_string::Text,
    mut fields: self::ml_string::Text,
    mut assignments: self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryRenderJavaScriptClosedClassLoop20Result {
    loop {
        if at < (row.len() as f64) {
            {
                let method = (self::ml_array::optional_at(
                    values,
                    self::ml_array::number_index(
                        (self::ml_array::optional_at(row, self::ml_array::number_index(at)))
                            .unwrap_or_else(|| 0f64),
                    ),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let args = self::render_java_script_closed_class_range(
                        source,
                        values,
                        starts,
                        ends,
                        ((self::ml_array::optional_at(
                            row,
                            self::ml_array::number_index(at + 1f64),
                        ))
                        .unwrap_or_else(|| 0f64)
                            + 1f64),
                        (self::ml_array::optional_at(row, self::ml_array::number_index(at + 2f64)))
                            .unwrap_or_else(|| 0f64),
                        &(self::ml_string::Text::from_units(vec![])),
                        class_names,
                    );
                    {
                        let body = self::render_java_script_closed_class_range(
                            source,
                            values,
                            starts,
                            ends,
                            ((self::ml_array::optional_at(
                                row,
                                self::ml_array::number_index(at + 3f64),
                            ))
                            .unwrap_or_else(|| 0f64)
                                + 1f64),
                            (self::ml_array::optional_at(
                                row,
                                self::ml_array::number_index(at + 4f64),
                            ))
                            .unwrap_or_else(|| 0f64),
                            receiver,
                            class_names,
                        );
                        {
                            let stub =
                                (((prefix.concat(&self::ml_string::Text::from_units(vec![
                                    99, 108, 97, 115, 115, 95, 117, 110, 105, 110, 105, 116, 105,
                                    97, 108, 105, 122, 101, 100, 95,
                                ])))
                                .concat(&{
                                    let source_ml_source1_string_argument102 = ordinal;
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                                        source_ml_source1_string_argument102,
                                    ))
                                }))
                                .concat(&self::ml_string::Text::from_units(vec![95])))
                                .concat(&{
                                    let source_ml_source1_string_argument103 = at;
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                                        source_ml_source1_string_argument103,
                                    ))
                                });
                            {
                                let declaration = (self::ml_array::optional_at(
                                    row,
                                    self::ml_array::number_index(at),
                                ))
                                .unwrap_or_else(|| 0f64);
                                {
                                    let documentation = {
                                        let source_ml_source1_string_range_operand104 =
                                            source.clone();
                                        {
                                            let source_ml_source1_string_range_operand105 =
                                                (self::ml_array::optional_at(
                                                    ends,
                                                    self::ml_array::number_index(
                                                        declaration - 1f64,
                                                    ),
                                                ))
                                                .unwrap_or_else(|| 0f64);
                                            {
                                                let source_ml_source1_string_range_operand106 =
                                                    (self::ml_array::optional_at(
                                                        starts,
                                                        self::ml_array::number_index(declaration),
                                                    ))
                                                    .unwrap_or_else(|| 0f64);
                                                {
                                                    let source_ml_source1_string_range_position107 =
                                                        source_ml_source1_string_range_operand105;
                                                    {
                                                        let source_ml_source1_string_range_position108 = source_ml_source1_string_range_operand106;
                                                        source_ml_source1_string_range_operand104.slice(source_ml_source1_string_range_position107, source_ml_source1_string_range_position108)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let stubs_2 = stubs.concat(
                                            &((((((((((self::ml_string::Text::from_units(
                                                vec![10],
                                            ))
                                            .concat(&documentation))
                                            .concat(&self::ml_string::Text::from_units(
                                                vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32],
                                            )))
                                            .concat(&stub))
                                            .concat(&self::ml_string::Text::from_units(
                                                vec![40],
                                            )))
                                            .concat(&args))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                41, 32, 123, 32, 114, 101, 116, 117, 114, 110, 32,
                                            ])))
                                            .concat(&stub))
                                            .concat(&self::ml_string::Text::from_units(vec![40])))
                                            .concat(&args))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                41, 59, 32, 125, 10,
                                            ])),
                                        );
                                        {
                                            let fields_2 = fields.concat(
                                                &(((self::ml_string::Text::from_units(vec![
                                                    44, 32,
                                                ]))
                                                .concat(&method))
                                                .concat(&self::ml_string::Text::from_units(vec![
                                                    58, 32,
                                                ])))
                                                .concat(&stub),
                                            );
                                            {
                                                let assignments_2 = assignments.concat(&((((((((self::ml_string::Text::from_units(vec![10])).concat(receiver)).concat(&self::ml_string::Text::from_units(vec![46]))).concat(&method)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 40]))).concat(&args)).concat(&self::ml_string::Text::from_units(vec![41, 32, 61, 62, 32, 123, 32]))).concat(&body)).concat(&self::ml_string::Text::from_units(vec![32, 125, 59, 10])));
                                                {
                                                    let at_2 = at + 5f64;
                                                    {
                                                        (stubs, fields, assignments, at) = (
                                                            stubs_2,
                                                            fields_2,
                                                            assignments_2,
                                                            at_2,
                                                        );
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
        return {
            self::MlTemporaryRenderJavaScriptClosedClassLoop20Result::MlTemporaryRenderJavaScriptClosedClassLoop20Done(stubs.clone(), fields.clone(), assignments.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop22(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut syntax: Vec<self::ml_string::Text>,
    mut at: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if at < (values.len() as f64) {
            {
                let syntax_2 = {
                    let source_ml_source1_concat_value109 = syntax;
                    {
                        let source_ml_source1_concat_value110 =
                            vec![if (((self::ml_array::optional_at(
                                kinds,
                                self::ml_array::number_index(at),
                            ))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                                == (self::ml_string::Text::from_units(vec![
                                    105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                                ])))
                                || (((self::ml_array::optional_at(
                                    kinds,
                                    self::ml_array::number_index(at),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 117, 110, 99, 116,
                                    ])))
                            {
                                (self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(at),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))
                            } else {
                                (self::ml_string::Text::from_units(vec![35])).concat(
                                    &(self::ml_array::optional_at(
                                        values,
                                        self::ml_array::number_index(at),
                                    ))
                                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                                )
                            }];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source1_concat_value109,
                            ),
                            source_ml_source1_concat_value110,
                        )
                    }
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (syntax, at) = (syntax_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { syntax };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop27(
    values: &[self::ml_string::Text],
    starts: &[f64],
    mut previous: f64,
    mut first: f64,
) -> f64 {
    loop {
        if (first < (values.len() as f64))
            && (((self::ml_array::optional_at(starts, self::ml_array::number_index(first)))
                .unwrap_or_else(|| 0f64))
                < previous)
        {
            {
                let first_2 = first + 1f64;
                {
                    first = first_2;
                    continue;
                }
            }
        }
        return { first };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceLoop26Result {
    MlTemporaryLowerJavaScriptClosedClassSourceLoop26Done(
        self::ml_string::Text,
        Vec<f64>,
        f64,
        Vec<self::ml_string::Text>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop26(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    prefix: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    mut text: self::ml_string::Text,
    mut intervals: Vec<f64>,
    mut previous: f64,
    mut ordinal: f64,
    mut brands: Vec<self::ml_string::Text>,
    source_ml_temporary_values21: &[Vec<f64>],
    mut source_ml_temporary_index21: self::ml::Big,
) -> self::MlTemporaryLowerJavaScriptClosedClassSourceLoop26Result {
    loop {
        if source_ml_temporary_index21
            < (self::ml::Big::from_u128(source_ml_temporary_values21.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values21,
                    source_ml_temporary_index21.to_index(),
                );
                {
                    let first =
                        (self::ml_array::optional_at(&row, self::ml_array::number_index(0f64)))
                            .unwrap_or_else(|| 0f64);
                    {
                        let last =
                            (self::ml_array::optional_at(&row, self::ml_array::number_index(1f64)))
                                .unwrap_or_else(|| 0f64);
                        {
                            let before = {
                                let source_ml_source1_string_range_operand111 = source.clone();
                                {
                                    let source_ml_source1_string_range_operand112 = previous;
                                    {
                                        let source_ml_source1_string_range_operand113 =
                                            (self::ml_array::optional_at(
                                                starts,
                                                self::ml_array::number_index(first),
                                            ))
                                            .unwrap_or_else(|| previous);
                                        {
                                            let source_ml_source1_string_range_position114 =
                                                source_ml_source1_string_range_operand112;
                                            {
                                                let source_ml_source1_string_range_position115 =
                                                    source_ml_source1_string_range_operand113;
                                                source_ml_source1_string_range_operand111.slice(
                                                    source_ml_source1_string_range_position114,
                                                    source_ml_source1_string_range_position115,
                                                )
                                            }
                                        }
                                    }
                                }
                            };
                            {
                                let intervals_2 = {
                                    let source_ml_source1_concat_value116 = intervals;
                                    {
                                        let source_ml_source1_concat_value117 = vec![
                                            text.length(),
                                            (text.length() + before.length()),
                                            previous,
                                            (self::ml_array::optional_at(
                                                starts,
                                                self::ml_array::number_index(first),
                                            ))
                                            .unwrap_or_else(|| previous),
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source1_concat_value116,
                                            ),
                                            source_ml_source1_concat_value117,
                                        )
                                    }
                                };
                                {
                                    let text_2 = text.concat(&before);
                                    {
                                        let replacement = self::render_java_script_closed_class(
                                            source,
                                            values,
                                            starts,
                                            ends,
                                            &(row),
                                            prefix,
                                            ordinal,
                                            names,
                                        );
                                        {
                                            let intervals_3 = {
                                                let source_ml_source1_concat_value118 = intervals_2;
                                                {
                                                    let source_ml_source1_concat_value119 = vec![
                                                        text_2.length(),
                                                        (text_2.length() + replacement.length()),
                                                        (self::ml_array::optional_at(
                                                            starts,
                                                            self::ml_array::number_index(first),
                                                        ))
                                                        .unwrap_or_else(|| 0f64),
                                                        (self::ml_array::optional_at(
                                                            ends,
                                                            self::ml_array::number_index(last),
                                                        ))
                                                        .unwrap_or_else(|| 0f64),
                                                    ];
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<f64>::new(),
                                                            source_ml_source1_concat_value118,
                                                        ),
                                                        source_ml_source1_concat_value119,
                                                    )
                                                }
                                            };
                                            {
                                                let text_3 = text_2.concat(&replacement);
                                                {
                                                    let previous_2 = (self::ml_array::optional_at(
                                                        ends,
                                                        self::ml_array::number_index(last),
                                                    ))
                                                    .unwrap_or_else(|| previous);
                                                    {
                                                        let brands_2 = {
                                                            let source_ml_source1_concat_value121 =
                                                                brands;
                                                            {
                                                                let source_ml_source1_concat_value122 = vec![(prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95]))).concat(&{
                                                                    let source_ml_source1_string_argument120 = ordinal;
                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument120))
                                                                })];
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value121), source_ml_source1_concat_value122)
                                                            }
                                                        };
                                                        {
                                                            let ordinal_2 = ordinal + 1f64;
                                                            {
                                                                let source_ml_temporary_index21_2 =
                                                                    source_ml_temporary_index21.add(
                                                                        &self::ml::Big::from_i128(
                                                                            1,
                                                                        ),
                                                                    );
                                                                {
                                                                    (text, intervals, previous, ordinal, brands, source_ml_temporary_index21) = (text_3, intervals_3, previous_2, ordinal_2, brands_2, source_ml_temporary_index21_2);
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
        return {
            self::MlTemporaryLowerJavaScriptClosedClassSourceLoop26Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop26Done(text.clone(), intervals, previous, brands)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceJoin25 {
    MlTemporaryLowerJavaScriptClosedClassSourceJoin25Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceJoin24 {
    MlTemporaryLowerJavaScriptClosedClassSourceJoin24Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceLoop28Result {
    MlTemporaryLowerJavaScriptClosedClassSourceLoop28Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop28(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    names: &[self::ml_string::Text],
    mut error: f64,
    mut error_at: f64,
    mut at: f64,
) -> self::MlTemporaryLowerJavaScriptClosedClassSourceLoop28Result {
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
                        && (((value
                            == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))
                            || (value
                                == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))))
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
                            let source_ml_source1_string_range_operand128 = source.clone();
                            {
                                let source_ml_source1_string_range_operand129 =
                                    (self::ml_array::optional_at(
                                        starts,
                                        self::ml_array::number_index(at),
                                    ))
                                    .unwrap_or_else(|| 0f64);
                                {
                                    let source_ml_source1_string_range_operand130 =
                                        (self::ml_array::optional_at(
                                            ends,
                                            self::ml_array::number_index(at),
                                        ))
                                        .unwrap_or_else(|| 0f64);
                                    {
                                        let source_ml_source1_string_range_position131 =
                                            source_ml_source1_string_range_operand129;
                                        {
                                            let source_ml_source1_string_range_position132 =
                                                source_ml_source1_string_range_operand130;
                                            source_ml_source1_string_range_operand128.slice(
                                                source_ml_source1_string_range_position131,
                                                source_ml_source1_string_range_position132,
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
                                self::MlTemporaryLowerJavaScriptClosedClassSourceLoop28Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop28Done(error_2, error_at_2)
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
            self::MlTemporaryLowerJavaScriptClosedClassSourceLoop28Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop28Done(error, error_at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceLoop23Result {
    MlTemporaryLowerJavaScriptClosedClassSourceLoop23Done(
        Vec<Vec<f64>>,
        Vec<self::ml_string::Text>,
        f64,
        f64,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop29(
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
