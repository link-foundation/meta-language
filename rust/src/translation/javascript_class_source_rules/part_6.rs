// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop21Result;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_java_script_closed_class_methods;
use super::render_java_script_closed_class;
use super::render_java_script_closed_class_parameter_arguments;
use super::render_java_script_closed_class_range;

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop25Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop25Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop25Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop26Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop26Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop26Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop26(
    values: &[self::ml_string::Text],
    row: &[f64],
    mut at: f64,
    mut inner: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop26Result {
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
                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop26Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop26Return(inner)
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
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop26Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop26Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop25(
    values: &[self::ml_string::Text],
    row: &[f64],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop25Result {
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
                        let source_ml_source1_s124 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop26(values, row, at, inner);
                        match source_ml_source1_s124 {
                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop26Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop26Done => {
                                {
                                    let at_2 = at + 5f64;
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop26Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop26Return(source_ml_temporary_result26) => {
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop25Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop25Return(source_ml_temporary_result26)
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
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop25Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop25Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop21(
    values: &[self::ml_string::Text],
    mut names: Vec<self::ml_string::Text>,
    mut methods: Vec<self::ml_string::Text>,
    source_ml_temporary_values19: &[Vec<f64>],
    mut source_ml_temporary_index19: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop21Result {
    loop {
        return if source_ml_temporary_index19
            < (self::ml::Big::from_u128(source_ml_temporary_values19.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values19,
                    source_ml_temporary_index19.to_index(),
                );
                {
                    let names_2 = {
                        let source_ml_source1_concat_value125 = names;
                        {
                            let source_ml_source1_concat_value126 = vec![
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
                                    source_ml_source1_concat_value125,
                                ),
                                source_ml_source1_concat_value126,
                            )
                        }
                    };
                    {
                        let methods_2 = {
                            let source_ml_source1_concat_value127 = methods;
                            {
                                let source_ml_source1_concat_value128 =
                                    self::read_java_script_closed_class_methods(values, &(row));
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source1_concat_value127,
                                    ),
                                    source_ml_source1_concat_value128,
                                )
                            }
                        };
                        {
                            let at = 8f64
                                + (3f64
                                    * (self::ml_array::optional_at(
                                        &row,
                                        self::ml_array::number_index(7f64),
                                    ))
                                    .unwrap_or_else(|| 0f64));
                            {
                                let source_ml_source1_s129 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop25(values, &(row), at);
                                match source_ml_source1_s129 {
                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop25Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop25Done => {
                                        {
                                            let source_ml_temporary_index19_2 = source_ml_temporary_index19.add(&self::ml::Big::from_i128(1));
                                            {
                                                (names, methods, source_ml_temporary_index19) = (names_2, methods_2, source_ml_temporary_index19_2);
                                                continue;
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop25Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop25Return(source_ml_temporary_result25) => {
                                        self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop21Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop21Return(source_ml_temporary_result25)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop21Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop21Done(names, methods)
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
pub fn source_ml_temporary_render_java_script_closed_class_loop27(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    row: &[f64],
    class_names: &[self::ml_string::Text],
    static_calls: &[self::ml_string::Text],
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
                        static_calls,
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
pub enum MlTemporaryRenderJavaScriptClosedClassLoop28Result {
    MlTemporaryRenderJavaScriptClosedClassLoop28Done(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_render_java_script_closed_class_loop28(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    row: &[f64],
    prefix: &self::ml_string::Text,
    mut ordinal: f64,
    class_names: &[self::ml_string::Text],
    static_calls: &[self::ml_string::Text],
    receiver: &self::ml_string::Text,
    mut stubs: self::ml_string::Text,
    mut fields: self::ml_string::Text,
    mut assignments: self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryRenderJavaScriptClosedClassLoop28Result {
    loop {
        if at < (row.len() as f64) {
            {
                let member = (self::ml_array::optional_at(row, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| 0f64);
                {
                    let position = if member < 0f64 {
                        (-member) - 1f64
                    } else {
                        member
                    };
                    {
                        let method = (self::ml_array::optional_at(
                            values,
                            self::ml_array::number_index(position),
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
                                (self::ml_array::optional_at(
                                    row,
                                    self::ml_array::number_index(at + 2f64),
                                ))
                                .unwrap_or_else(|| 0f64),
                                &(self::ml_string::Text::from_units(vec![])),
                                class_names,
                                static_calls,
                            );
                            {
                                let arguments_ =
                                    self::render_java_script_closed_class_parameter_arguments(
                                        source,
                                        values,
                                        starts,
                                        ends,
                                        (self::ml_array::optional_at(
                                            row,
                                            self::ml_array::number_index(at + 1f64),
                                        ))
                                        .unwrap_or_else(|| 0f64),
                                        (self::ml_array::optional_at(
                                            row,
                                            self::ml_array::number_index(at + 2f64),
                                        ))
                                        .unwrap_or_else(|| 0f64),
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
                                        &(if member < 0f64 {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            receiver.clone()
                                        }),
                                        class_names,
                                        static_calls,
                                    );
                                    {
                                        let stub = (((prefix.concat(
                                            &self::ml_string::Text::from_units(vec![
                                                99, 108, 97, 115, 115, 95, 117, 110, 105, 110, 105,
                                                116, 105, 97, 108, 105, 122, 101, 100, 95,
                                            ]),
                                        ))
                                        .concat(&{
                                            let source_ml_source1_string_argument130 = ordinal;
                                            self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(
                                                    source_ml_source1_string_argument130,
                                                ),
                                            )
                                        }))
                                        .concat(&self::ml_string::Text::from_units(vec![95])))
                                        .concat(&{
                                            let source_ml_source1_string_argument131 = at;
                                            self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(
                                                    source_ml_source1_string_argument131,
                                                ),
                                            )
                                        });
                                        {
                                            let declaration = if member < 0f64 {
                                                position - 1f64
                                            } else {
                                                position
                                            };
                                            {
                                                let documentation = {
                                                    let source_ml_source1_string_range_operand132 =
                                                        source.clone();
                                                    {
                                                        let source_ml_source1_string_range_operand133 =
                                                            (self::ml_array::optional_at(
                                                                ends,
                                                                self::ml_array::number_index(
                                                                    declaration - 1f64,
                                                                ),
                                                            ))
                                                            .unwrap_or_else(|| 0f64);
                                                        {
                                                            let source_ml_source1_string_range_operand134 =
                                                                (self::ml_array::optional_at(
                                                                    starts,
                                                                    self::ml_array::number_index(
                                                                        declaration,
                                                                    ),
                                                                ))
                                                                .unwrap_or_else(|| 0f64);
                                                            {
                                                                let source_ml_source1_string_range_position135 = source_ml_source1_string_range_operand133;
                                                                {
                                                                    let source_ml_source1_string_range_position136 = source_ml_source1_string_range_operand134;
                                                                    source_ml_source1_string_range_operand132.slice(source_ml_source1_string_range_position135, source_ml_source1_string_range_position136)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                if member < 0f64 {
                                                    {
                                                        let stubs_2 = stubs.concat(&((((((((((((self::ml_string::Text::from_units(vec![10])).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(prefix)).concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 115, 116, 97, 116, 105, 99, 95]))).concat(&{
                                                            let source_ml_source1_string_argument137 = ordinal;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument137))
                                                        })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                            let source_ml_source1_string_argument138 = at;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument138))
                                                        })).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&args)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 32]))).concat(&body)).concat(&self::ml_string::Text::from_units(vec![32, 125, 10])));
                                                        {
                                                            let at_2 = at + 5f64;
                                                            {
                                                                (stubs, at) = (stubs_2, at_2);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                                {
                                                    {
                                                        let environment = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 95]))).concat(&{
                                                            let source_ml_source1_string_argument139 = ordinal;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument139))
                                                        })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                            let source_ml_source1_string_argument140 = at;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument140))
                                                        });
                                                        {
                                                            let stubs_3 = stubs.concat(&((((((((((self::ml_string::Text::from_units(vec![10])).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(&stub)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&args)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 32, 114, 101, 116, 117, 114, 110, 32]))).concat(&stub)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&arguments_)).concat(&self::ml_string::Text::from_units(vec![41, 59, 32, 125, 10])));
                                                            {
                                                                let stubs_4 = stubs_3.concat(&((((((((self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32])).concat(&environment)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(receiver)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 32, 114, 101, 116, 117, 114, 110, 32, 40]))).concat(&args)).concat(&self::ml_string::Text::from_units(vec![41, 32, 61, 62, 32, 123, 32]))).concat(&body)).concat(&self::ml_string::Text::from_units(vec![32, 125, 59, 32, 125, 10])));
                                                                {
                                                                    let fields_2 = fields.concat(&(((self::ml_string::Text::from_units(vec![44, 32])).concat(&method)).concat(&self::ml_string::Text::from_units(vec![58, 32]))).concat(&stub));
                                                                    {
                                                                        let assignments_2 = assignments.concat(&((((((((self::ml_string::Text::from_units(vec![10])).concat(receiver)).concat(&self::ml_string::Text::from_units(vec![46]))).concat(&method)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&environment)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(receiver)).concat(&self::ml_string::Text::from_units(vec![41, 59, 10])));
                                                                        {
                                                                            let at_3 = at + 5f64;
                                                                            {
                                                                                (
                                                                                    stubs,
                                                                                    fields,
                                                                                    assignments,
                                                                                    at,
                                                                                ) = (
                                                                                    stubs_4,
                                                                                    fields_2,
                                                                                    assignments_2,
                                                                                    at_3,
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
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryRenderJavaScriptClosedClassLoop28Result::MlTemporaryRenderJavaScriptClosedClassLoop28Done(stubs.clone(), fields.clone(), assignments.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop30(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut syntax: Vec<self::ml_string::Text>,
    mut at: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if at < (values.len() as f64) {
            {
                let syntax_2 = {
                    let source_ml_source1_concat_value141 = syntax;
                    {
                        let source_ml_source1_concat_value142 =
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
                                source_ml_source1_concat_value141,
                            ),
                            source_ml_source1_concat_value142,
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
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop35(
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
pub enum MlTemporaryLowerJavaScriptClosedClassSourceLoop34Result {
    MlTemporaryLowerJavaScriptClosedClassSourceLoop34Done(
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
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop34(
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
    source_ml_temporary_values29: &[Vec<f64>],
    mut source_ml_temporary_index29: self::ml::Big,
) -> self::MlTemporaryLowerJavaScriptClosedClassSourceLoop34Result {
    loop {
        if source_ml_temporary_index29
            < (self::ml::Big::from_u128(source_ml_temporary_values29.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values29,
                    source_ml_temporary_index29.to_index(),
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
                                let source_ml_source1_string_range_operand143 = source.clone();
                                {
                                    let source_ml_source1_string_range_operand144 = previous;
                                    {
                                        let source_ml_source1_string_range_operand145 =
                                            (self::ml_array::optional_at(
                                                starts,
                                                self::ml_array::number_index(first),
                                            ))
                                            .unwrap_or_else(|| previous);
                                        {
                                            let source_ml_source1_string_range_position146 =
                                                source_ml_source1_string_range_operand144;
                                            {
                                                let source_ml_source1_string_range_position147 =
                                                    source_ml_source1_string_range_operand145;
                                                source_ml_source1_string_range_operand143.slice(
                                                    source_ml_source1_string_range_position146,
                                                    source_ml_source1_string_range_position147,
                                                )
                                            }
                                        }
                                    }
                                }
                            };
                            {
                                let intervals_2 = {
                                    let source_ml_source1_concat_value148 = intervals;
                                    {
                                        let source_ml_source1_concat_value149 = vec![
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
                                                source_ml_source1_concat_value148,
                                            ),
                                            source_ml_source1_concat_value149,
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
                                        );
                                        {
                                            let intervals_3 = {
                                                let source_ml_source1_concat_value150 = intervals_2;
                                                {
                                                    let source_ml_source1_concat_value151 = vec![
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
                                                            source_ml_source1_concat_value150,
                                                        ),
                                                        source_ml_source1_concat_value151,
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
                                                            let source_ml_source1_concat_value153 =
                                                                brands;
                                                            {
                                                                let source_ml_source1_concat_value154 = vec![(prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95]))).concat(&{
                                                                    let source_ml_source1_string_argument152 = ordinal;
                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument152))
                                                                })];
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value153), source_ml_source1_concat_value154)
                                                            }
                                                        };
                                                        {
                                                            let ordinal_2 = ordinal + 1f64;
                                                            {
                                                                let source_ml_temporary_index29_2 =
                                                                    source_ml_temporary_index29.add(
                                                                        &self::ml::Big::from_i128(
                                                                            1,
                                                                        ),
                                                                    );
                                                                {
                                                                    (text, intervals, previous, ordinal, brands, source_ml_temporary_index29) = (text_3, intervals_3, previous_2, ordinal_2, brands_2, source_ml_temporary_index29_2);
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
            self::MlTemporaryLowerJavaScriptClosedClassSourceLoop34Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop34Done(text.clone(), intervals, previous, brands)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceJoin33 {
    MlTemporaryLowerJavaScriptClosedClassSourceJoin33Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceJoin32 {
    MlTemporaryLowerJavaScriptClosedClassSourceJoin32Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceLoop36Result {
    MlTemporaryLowerJavaScriptClosedClassSourceLoop36Done(f64, f64),
}
