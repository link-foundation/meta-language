// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryLowerJavaScriptClosedClassSourceLoop60Result;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_java_script_closed_class_directory;
use super::render_java_script_closed_class;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop62;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop63;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop64(
    values: &[self::ml_string::Text],
    row: &[f64],
    mut own_fields: Vec<self::ml_string::Text>,
    mut field: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if field
            < (9f64
                + (3f64
                    * (self::ml_array::optional_at(row, self::ml_array::number_index(7f64)))
                        .unwrap_or_else(|| 0f64)))
        {
            {
                let position =
                    (self::ml_array::optional_at(row, self::ml_array::number_index(field)))
                        .unwrap_or_else(|| 0f64);
                {
                    let name = (self::ml_array::optional_at(
                        values,
                        self::ml_array::number_index(if position < 0f64 {
                            (-position) - 1f64
                        } else {
                            position
                        }),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    if !(self::ml_array::includes(
                        &own_fields,
                        &name.clone(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )) {
                        {
                            let own_fields_2 = {
                                let source_ml_source1_concat_value279 = own_fields;
                                {
                                    let source_ml_source1_concat_value280 = vec![name.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::ml_string::Text>::new(),
                                            source_ml_source1_concat_value279,
                                        ),
                                        source_ml_source1_concat_value280,
                                    )
                                }
                            };
                            {
                                let field_2 = field + 3f64;
                                {
                                    (own_fields, field) = (own_fields_2, field_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let field_3 = field + 3f64;
                            {
                                field = field_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { own_fields };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop65(
    values: &[self::ml_string::Text],
    prefix: &self::ml_string::Text,
    mut ordinal: f64,
    mut prototype: bool,
    row: &[f64],
    mut own_methods: Vec<self::ml_string::Text>,
    mut method: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if method < (row.len() as f64) {
            if ((self::ml_array::optional_at(row, self::ml_array::number_index(method)))
                .unwrap_or_else(|| -1f64))
                >= 0f64
            {
                {
                    let own_methods_2 = {
                        let source_ml_source1_concat_value283 = own_methods;
                        {
                            let source_ml_source1_concat_value284 = vec![
                                (self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(
                                        (self::ml_array::optional_at(
                                            row,
                                            self::ml_array::number_index(method),
                                        ))
                                        .unwrap_or_else(|| 0f64),
                                    ),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                                (((prefix.concat(&if prototype {
                                    self::ml_string::Text::from_units(vec![
                                        99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95,
                                    ])
                                } else {
                                    self::ml_string::Text::from_units(vec![
                                        99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100,
                                        95, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116,
                                        95,
                                    ])
                                }))
                                .concat(&{
                                    let source_ml_source1_string_argument281 = ordinal;
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                                        source_ml_source1_string_argument281,
                                    ))
                                }))
                                .concat(&self::ml_string::Text::from_units(vec![95])))
                                .concat(&{
                                    let source_ml_source1_string_argument282 = method;
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                                        source_ml_source1_string_argument282,
                                    ))
                                }),
                            ];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<self::ml_string::Text>::new(),
                                    source_ml_source1_concat_value283,
                                ),
                                source_ml_source1_concat_value284,
                            )
                        }
                    };
                    {
                        let method_2 = method + 5f64;
                        {
                            (own_methods, method) = (own_methods_2, method_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let method_3 = method + 5f64;
                    {
                        method = method_3;
                        continue;
                    }
                }
            }
        }
        return { own_methods };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop60(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    prefix: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    static_calls: &[self::ml_string::Text],
    mut text: self::ml_string::Text,
    mut intervals: Vec<f64>,
    mut previous: f64,
    mut ordinal: f64,
    mut brands: Vec<self::ml_string::Text>,
    mut public_fields: Vec<self::ml_string::Text>,
    mut prototype_methods: Vec<self::ml_string::Text>,
    mut owner_rows: Vec<Vec<self::ml_string::Text>>,
    mut prototype: bool,
    source_ml_temporary_values54: &[Vec<f64>],
    mut source_ml_temporary_index54: self::ml::Big,
) -> self::MlTemporaryLowerJavaScriptClosedClassSourceLoop60Result {
    loop {
        if source_ml_temporary_index54
            < (self::ml::Big::from_u128(source_ml_temporary_values54.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values54,
                    source_ml_temporary_index54.to_index(),
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
                                let source_ml_source1_string_range_operand285 = source.clone();
                                {
                                    let source_ml_source1_string_range_operand286 = previous;
                                    {
                                        let source_ml_source1_string_range_operand287 =
                                            (self::ml_array::optional_at(
                                                starts,
                                                self::ml_array::number_index(first),
                                            ))
                                            .unwrap_or_else(|| previous);
                                        {
                                            let source_ml_source1_string_range_position288 =
                                                source_ml_source1_string_range_operand286;
                                            {
                                                let source_ml_source1_string_range_position289 =
                                                    source_ml_source1_string_range_operand287;
                                                source_ml_source1_string_range_operand285.slice(
                                                    source_ml_source1_string_range_position288,
                                                    source_ml_source1_string_range_position289,
                                                )
                                            }
                                        }
                                    }
                                }
                            };
                            {
                                let intervals_2 = {
                                    let source_ml_source1_concat_value290 = intervals;
                                    {
                                        let source_ml_source1_concat_value291 = vec![
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
                                                source_ml_source1_concat_value290,
                                            ),
                                            source_ml_source1_concat_value291,
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
                                            static_calls,
                                            prototype,
                                        );
                                        {
                                            let intervals_3 = {
                                                let source_ml_source1_concat_value292 = intervals_2;
                                                {
                                                    let source_ml_source1_concat_value293 = vec![
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
                                                            source_ml_source1_concat_value292,
                                                        ),
                                                        source_ml_source1_concat_value293,
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
                                                        let brand = (prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95]))).concat(&{
                                                            let source_ml_source1_string_argument294 = ordinal;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument294))
                                                        });
                                                        {
                                                            let brands_2 = {
                                                                let source_ml_source1_concat_value295 =
                                                                    brands;
                                                                {
                                                                    let source_ml_source1_concat_value296 =
                                                                        vec![brand.clone()];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value295), source_ml_source1_concat_value296)
                                                                }
                                                            };
                                                            {
                                                                let prototype_methods_3 =
                                                                    if prototype {
                                                                        {
                                                                            let method = 9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).unwrap_or_else(|| 0f64));
                                                                            {
                                                                                self::source_ml_temporary_lower_java_script_closed_class_source_loop62(values, prefix, ordinal, prototype_methods, &(row), &(brand), method)
                                                                            }
                                                                        }
                                                                    } else {
                                                                        prototype_methods
                                                                    };
                                                                {
                                                                    let field = 9f64;
                                                                    {
                                                                        let public_fields_2 = self::source_ml_temporary_lower_java_script_closed_class_source_loop63(values, public_fields, &(row), &(brand), field);
                                                                        {
                                                                            let own_fields = Vec::<self::ml_string::Text>::new();
                                                                            {
                                                                                let own_methods = Vec::<self::ml_string::Text>::new();
                                                                                {
                                                                                    let field_2 =
                                                                                        9f64;
                                                                                    {
                                                                                        let own_fields_2 = self::source_ml_temporary_lower_java_script_closed_class_source_loop64(values, &(row), own_fields, field_2);
                                                                                        {
                                                                                            let method_2 = 9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).unwrap_or_else(|| 0f64));
                                                                                            {
                                                                                                let own_methods_2 = self::source_ml_temporary_lower_java_script_closed_class_source_loop65(values, prefix, ordinal, prototype, &(row), own_methods, method_2);
                                                                                                {
                                                                                                    let owner_rows_2 = {
                                                                                                        let source_ml_source1_concat_value302 = owner_rows;
                                                                                                        {
                                                                                                            let source_ml_source1_concat_value303 = vec![{
                                                                                                                let source_ml_source1_concat_value299 = vec![(self::ml_array::optional_at(values, self::ml_array::number_index(first + 1f64))).unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), brand.clone(), if prototype {
                                                                                                                    self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 104, 101, 108, 112, 101, 114, 115])
                                                                                                                } else {
                                                                                                                    self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115])
                                                                                                                }, {
                                                                                                                    let source_ml_source1_string_argument297 = own_fields_2.len() as f64;
                                                                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument297))
                                                                                                                }, {
                                                                                                                    let source_ml_source1_string_argument298 = (own_methods_2.len() as f64) / 2f64;
                                                                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument298))
                                                                                                                }];
                                                                                                                {
                                                                                                                    let source_ml_source1_concat_value300 = own_fields_2;
                                                                                                                    {
                                                                                                                        let source_ml_source1_concat_value301 = own_methods_2;
                                                                                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value299), source_ml_source1_concat_value300), source_ml_source1_concat_value301)
                                                                                                                    }
                                                                                                                }
                                                                                                            }];
                                                                                                            self::ml_array::append(self::ml_array::append(Vec::<Vec<self::ml_string::Text>>::new(), source_ml_source1_concat_value302), source_ml_source1_concat_value303)
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let ordinal_2 = ordinal + 1f64;
                                                                                                        {
                                                                                                            let source_ml_temporary_index54_2 = source_ml_temporary_index54.add(&self::ml::Big::from_i128(1));
                                                                                                            {
                                                                                                                (text, intervals, previous, ordinal, brands, public_fields, prototype_methods, owner_rows, source_ml_temporary_index54) = (text_3, intervals_3, previous_2, ordinal_2, brands_2, public_fields_2, prototype_methods_3, owner_rows_2, source_ml_temporary_index54_2);
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
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryLowerJavaScriptClosedClassSourceLoop60Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop60Done(text.clone(), intervals, previous, brands, public_fields, prototype_methods, owner_rows)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceJoin59 {
    MlTemporaryLowerJavaScriptClosedClassSourceJoin59Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceJoin58 {
    MlTemporaryLowerJavaScriptClosedClassSourceJoin58Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceJoin57 {
    MlTemporaryLowerJavaScriptClosedClassSourceJoin57Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceLoop66Result {
    MlTemporaryLowerJavaScriptClosedClassSourceLoop66Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop66(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    names: &[self::ml_string::Text],
    mut error: f64,
    mut error_at: f64,
    mut at: f64,
) -> self::MlTemporaryLowerJavaScriptClosedClassSourceLoop66Result {
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
                            let source_ml_source1_string_range_operand309 = source.clone();
                            {
                                let source_ml_source1_string_range_operand310 =
                                    (self::ml_array::optional_at(
                                        starts,
                                        self::ml_array::number_index(at),
                                    ))
                                    .unwrap_or_else(|| 0f64);
                                {
                                    let source_ml_source1_string_range_operand311 =
                                        (self::ml_array::optional_at(
                                            ends,
                                            self::ml_array::number_index(at),
                                        ))
                                        .unwrap_or_else(|| 0f64);
                                    {
                                        let source_ml_source1_string_range_position312 =
                                            source_ml_source1_string_range_operand310;
                                        {
                                            let source_ml_source1_string_range_position313 =
                                                source_ml_source1_string_range_operand311;
                                            source_ml_source1_string_range_operand309.slice(
                                                source_ml_source1_string_range_position312,
                                                source_ml_source1_string_range_position313,
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
                                self::MlTemporaryLowerJavaScriptClosedClassSourceLoop66Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop66Done(error_2, error_at_2)
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
            self::MlTemporaryLowerJavaScriptClosedClassSourceLoop66Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop66Done(error, error_at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceLoop56Result {
    MlTemporaryLowerJavaScriptClosedClassSourceLoop56Done(
        Vec<Vec<f64>>,
        Vec<self::ml_string::Text>,
        f64,
        f64,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop67(
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
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop56(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut directory: Vec<Vec<f64>>,
    mut names: Vec<self::ml_string::Text>,
    mut depth: f64,
    mut declarations_finished: bool,
    mut error: f64,
    mut error_at: f64,
    mut at: f64,
) -> self::MlTemporaryLowerJavaScriptClosedClassSourceLoop56Result {
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
                                self::MlTemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error_2, error_at_2)
                            }
                        }
                    } else if (depth != 0f64) || declarations_finished {
                        {
                            let error_3 = -7f64;
                            {
                                let error_at_3 = at;
                                self::MlTemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error_3, error_at_3)
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
                                        self::MlTemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error_4, error_at_4)
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
                                                self::MlTemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error_5, error_at_5)
                                            }
                                        }
                                    } else {
                                        {
                                            let names_2 = {
                                                let source_ml_source1_concat_value314 = names;
                                                {
                                                    let source_ml_source1_concat_value315 =
                                                        vec![name.clone()];
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<self::ml_string::Text>::new(),
                                                            source_ml_source1_concat_value314,
                                                        ),
                                                        source_ml_source1_concat_value315,
                                                    )
                                                }
                                            };
                                            {
                                                let directory_2 = {
                                                    let source_ml_source1_concat_value316 =
                                                        directory;
                                                    {
                                                        let source_ml_source1_concat_value317 =
                                                            vec![row.clone()];
                                                        self::ml_array::append(
                                                            self::ml_array::append(
                                                                Vec::<Vec<f64>>::new(),
                                                                source_ml_source1_concat_value316,
                                                            ),
                                                            source_ml_source1_concat_value317,
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
                            let end_2 = self::source_ml_temporary_lower_java_script_closed_class_source_loop67(values, kinds, end);
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
                                        self::MlTemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error_6, error_at_6)
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
            self::MlTemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error, error_at)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result {
    MlTemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Done,
    MlTemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Return(bool),
}
