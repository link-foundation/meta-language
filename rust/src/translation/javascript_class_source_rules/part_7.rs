// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryRenderJavaScriptClosedClassLoop36Result;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_java_script_closed_class_assigned_field_position;
use super::render_java_script_closed_class;
use super::render_java_script_closed_class_field_documentation;
use super::render_java_script_closed_class_parameter_arguments;
use super::render_java_script_closed_class_range;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_render_java_script_closed_class_loop36(
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
) -> self::MlTemporaryRenderJavaScriptClosedClassLoop36Result {
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
                                            let source_ml_source1_string_argument178 = ordinal;
                                            self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(
                                                    source_ml_source1_string_argument178,
                                                ),
                                            )
                                        }))
                                        .concat(&self::ml_string::Text::from_units(vec![95])))
                                        .concat(&{
                                            let source_ml_source1_string_argument179 = at;
                                            self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(
                                                    source_ml_source1_string_argument179,
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
                                                    let source_ml_source1_string_range_operand180 =
                                                        source.clone();
                                                    {
                                                        let source_ml_source1_string_range_operand181 =
                                                            (self::ml_array::optional_at(
                                                                ends,
                                                                self::ml_array::number_index(
                                                                    declaration - 1f64,
                                                                ),
                                                            ))
                                                            .unwrap_or_else(|| 0f64);
                                                        {
                                                            let source_ml_source1_string_range_operand182 =
                                                                (self::ml_array::optional_at(
                                                                    starts,
                                                                    self::ml_array::number_index(
                                                                        declaration,
                                                                    ),
                                                                ))
                                                                .unwrap_or_else(|| 0f64);
                                                            {
                                                                let source_ml_source1_string_range_position183 = source_ml_source1_string_range_operand181;
                                                                {
                                                                    let source_ml_source1_string_range_position184 = source_ml_source1_string_range_operand182;
                                                                    source_ml_source1_string_range_operand180.slice(source_ml_source1_string_range_position183, source_ml_source1_string_range_position184)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                if member < 0f64 {
                                                    {
                                                        let stubs_2 = stubs.concat(&((((((((((((self::ml_string::Text::from_units(vec![10])).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(prefix)).concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 115, 116, 97, 116, 105, 99, 95]))).concat(&{
                                                            let source_ml_source1_string_argument185 = ordinal;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument185))
                                                        })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                            let source_ml_source1_string_argument186 = at;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument186))
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
                                                            let source_ml_source1_string_argument187 = ordinal;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument187))
                                                        })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                            let source_ml_source1_string_argument188 = at;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument188))
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
            self::MlTemporaryRenderJavaScriptClosedClassLoop36Result::MlTemporaryRenderJavaScriptClosedClassLoop36Done(stubs.clone(), fields.clone(), assignments.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderJavaScriptClosedClassLoop35Result {
    MlTemporaryRenderJavaScriptClosedClassLoop35Done(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderJavaScriptClosedClassJoin38 {
    MlTemporaryRenderJavaScriptClosedClassJoin38Next(self::ml_string::Text, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderJavaScriptClosedClassJoin37 {
    MlTemporaryRenderJavaScriptClosedClassJoin37Next(
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
pub fn source_ml_temporary_render_java_script_closed_class_loop35(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    row: &[f64],
    prefix: &self::ml_string::Text,
    mut ordinal: f64,
    class_names: &[self::ml_string::Text],
    static_calls: &[self::ml_string::Text],
    mut methods: f64,
    mut stubs: self::ml_string::Text,
    mut initializations: self::ml_string::Text,
    mut constructor_fields: self::ml_string::Text,
    mut field_names: Vec<self::ml_string::Text>,
    mut fields: self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryRenderJavaScriptClosedClassLoop35Result {
    loop {
        return if at < methods {
            {
                let position = (self::ml_array::optional_at(row, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| 0f64);
                {
                    let field = (self::ml_array::optional_at(
                        values,
                        self::ml_array::number_index(if position < 0f64 {
                            (-position) - 1f64
                        } else {
                            position
                        }),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    {
                        let value = if ((self::ml_array::optional_at(
                            row,
                            self::ml_array::number_index(at + 1f64),
                        ))
                        .unwrap_or_else(|| 0f64))
                            == ((self::ml_array::optional_at(
                                row,
                                self::ml_array::number_index(at + 2f64),
                            ))
                            .unwrap_or_else(|| 0f64))
                        {
                            self::ml_string::Text::from_units(vec![
                                40, 118, 111, 105, 100, 32, 48, 41,
                            ])
                        } else {
                            self::render_java_script_closed_class_range(
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
                                &(self::ml_string::Text::from_units(vec![])),
                                class_names,
                                static_calls,
                            )
                        };
                        {
                            let source_ml_source1_s200 = if position < 0f64 {
                                {
                                    let initializer = (((prefix.concat(
                                        &self::ml_string::Text::from_units(vec![
                                            105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 95,
                                            99, 108, 97, 115, 115, 95, 102, 105, 101, 108, 100, 95,
                                        ]),
                                    ))
                                    .concat(&{
                                        let source_ml_source1_string_argument201 = ordinal;
                                        self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                source_ml_source1_string_argument201,
                                            ),
                                        )
                                    }))
                                    .concat(&self::ml_string::Text::from_units(vec![95])))
                                    .concat(&{
                                        let source_ml_source1_string_argument202 = at;
                                        self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                source_ml_source1_string_argument202,
                                            ),
                                        )
                                    });
                                    {
                                        let capture = (((prefix.concat(
                                            &self::ml_string::Text::from_units(vec![
                                                99, 108, 97, 115, 115, 95, 102, 105, 101, 108, 100,
                                                95, 118, 97, 108, 117, 101, 95,
                                            ]),
                                        ))
                                        .concat(&{
                                            let source_ml_source1_string_argument203 = ordinal;
                                            self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(
                                                    source_ml_source1_string_argument203,
                                                ),
                                            )
                                        }))
                                        .concat(&self::ml_string::Text::from_units(vec![95])))
                                        .concat(&{
                                            let source_ml_source1_string_argument204 = at;
                                            self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(
                                                    source_ml_source1_string_argument204,
                                                ),
                                            )
                                        });
                                        {
                                            let documentation = self::render_java_script_closed_class_field_documentation(&({
                                                let source_ml_source1_string_range_operand205 = source.clone();
                                                {
                                                    let source_ml_source1_string_range_operand206 = (self::ml_array::optional_at(ends, self::ml_array::number_index((-position) - 2f64))).unwrap_or_else(|| 0f64);
                                                    {
                                                        let source_ml_source1_string_range_operand207 = (self::ml_array::optional_at(starts, self::ml_array::number_index((-position) - 1f64))).unwrap_or_else(|| 0f64);
                                                        {
                                                            let source_ml_source1_string_range_position208 = source_ml_source1_string_range_operand206;
                                                            {
                                                                let source_ml_source1_string_range_position209 = source_ml_source1_string_range_operand207;
                                                                source_ml_source1_string_range_operand205.slice(source_ml_source1_string_range_position208, source_ml_source1_string_range_position209)
                                                            }
                                                        }
                                                    }
                                                }
                                            }));
                                            {
                                                let stubs_2 = stubs.concat(
                                                    &((((((self::ml_string::Text::from_units(
                                                        vec![10],
                                                    ))
                                                    .concat(&documentation))
                                                    .concat(
                                                        &self::ml_string::Text::from_units(vec![
                                                            10, 102, 117, 110, 99, 116, 105, 111,
                                                            110, 32,
                                                        ]),
                                                    ))
                                                    .concat(&initializer))
                                                    .concat(&self::ml_string::Text::from_units(
                                                        vec![
                                                            40, 41, 32, 123, 32, 114, 101, 116,
                                                            117, 114, 110, 32, 40,
                                                        ],
                                                    )))
                                                    .concat(&value))
                                                    .concat(&self::ml_string::Text::from_units(
                                                        vec![41, 59, 32, 125, 10],
                                                    )),
                                                );
                                                {
                                                    let initializations_2 = initializations.concat(
                                                        &((((self::ml_string::Text::from_units(
                                                            vec![99, 111, 110, 115, 116, 32],
                                                        ))
                                                        .concat(&capture))
                                                        .concat(
                                                            &self::ml_string::Text::from_units(
                                                                vec![32, 61, 32],
                                                            ),
                                                        ))
                                                        .concat(&initializer))
                                                        .concat(
                                                            &self::ml_string::Text::from_units(
                                                                vec![40, 41, 59, 10],
                                                            ),
                                                        ),
                                                    );
                                                    {
                                                        let value_2 = capture.clone();
                                                        {
                                                            let assigned = self::read_java_script_closed_class_assigned_field_position(values, row, &(field));
                                                            if assigned >= 0f64 {
                                                                {
                                                                    let value_3 = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 97, 115, 115, 105, 103, 110, 101, 100, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101, 95]))).concat(&{
                                                                        let source_ml_source1_string_argument210 = ordinal;
                                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument210))
                                                                    })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                                        let source_ml_source1_string_argument211 = assigned;
                                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument211))
                                                                    });
                                                                    self::MlTemporaryRenderJavaScriptClosedClassJoin37::MlTemporaryRenderJavaScriptClosedClassJoin37Next(stubs_2.clone(), initializations_2.clone(), value_3)
                                                                }
                                                            } else {
                                                                self::MlTemporaryRenderJavaScriptClosedClassJoin37::MlTemporaryRenderJavaScriptClosedClassJoin37Next(stubs_2.clone(), initializations_2.clone(), value_2)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlTemporaryRenderJavaScriptClosedClassJoin37::MlTemporaryRenderJavaScriptClosedClassJoin37Next(stubs.clone(), initializations.clone(), value.clone())
                            };
                            match source_ml_source1_s200 {
                                self::MlTemporaryRenderJavaScriptClosedClassJoin37::MlTemporaryRenderJavaScriptClosedClassJoin37Next(stubs_3, initializations_3, value_4) => {
                                    {
                                        let source_ml_source1_s214 = if position >= 0f64 {
                                            {
                                                let capture_2 = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 97, 115, 115, 105, 103, 110, 101, 100, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101, 95]))).concat(&{
                                                    let source_ml_source1_string_argument215 = ordinal;
                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument215))
                                                })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                    let source_ml_source1_string_argument216 = at;
                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument216))
                                                });
                                                {
                                                    let constructor_fields_2 = constructor_fields.concat(&((((self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 32])).concat(&capture_2)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 40]))).concat(&value_4)).concat(&self::ml_string::Text::from_units(vec![41, 59, 10])));
                                                    {
                                                        let value_5 = capture_2.clone();
                                                        self::MlTemporaryRenderJavaScriptClosedClassJoin38::MlTemporaryRenderJavaScriptClosedClassJoin38Next(constructor_fields_2, value_5)
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlTemporaryRenderJavaScriptClosedClassJoin38::MlTemporaryRenderJavaScriptClosedClassJoin38Next(constructor_fields.clone(), value_4.clone())
                                        };
                                        match source_ml_source1_s214 {
                                            self::MlTemporaryRenderJavaScriptClosedClassJoin38::MlTemporaryRenderJavaScriptClosedClassJoin38Next(constructor_fields_3, value_6) => {
                                                if self::ml_array::includes(&field_names, &field.clone(), 0f64, false, |a, b| a == b) {
                                                    {
                                                        let at_2 = at + 3f64;
                                                        {
                                                            (stubs, initializations, constructor_fields, at) = (stubs_3.clone(), initializations_3.clone(), constructor_fields_3.clone(), at_2);
                                                            continue;
                                                        }
                                                    }
                                                }
{
                                                    {
                                                        let field_names_2 = {
                                                            let source_ml_source1_concat_value217 = field_names;
                                                            {
                                                                let source_ml_source1_concat_value218 = vec![field.clone()];
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value217), source_ml_source1_concat_value218)
                                                            }
                                                        };
                                                        {
                                                            let fields_2 = fields.concat(&((((self::ml_string::Text::from_units(vec![44, 32])).concat(&field)).concat(&self::ml_string::Text::from_units(vec![58, 32, 40]))).concat(&value_6)).concat(&self::ml_string::Text::from_units(vec![41])));
                                                            {
                                                                let at_3 = at + 3f64;
                                                                {
                                                                    (stubs, initializations, constructor_fields, field_names, fields, at) = (stubs_3.clone(), initializations_3.clone(), constructor_fields_3.clone(), field_names_2, fields_2, at_3);
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
        } else {
            self::MlTemporaryRenderJavaScriptClosedClassLoop35Result::MlTemporaryRenderJavaScriptClosedClassLoop35Done(stubs.clone(), initializations.clone(), constructor_fields.clone(), fields.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop40(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut syntax: Vec<self::ml_string::Text>,
    mut at: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if at < (values.len() as f64) {
            {
                let syntax_2 = {
                    let source_ml_source1_concat_value219 = syntax;
                    {
                        let source_ml_source1_concat_value220 =
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
                                source_ml_source1_concat_value219,
                            ),
                            source_ml_source1_concat_value220,
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
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop45(
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
pub enum MlTemporaryLowerJavaScriptClosedClassSourceLoop44Result {
    MlTemporaryLowerJavaScriptClosedClassSourceLoop44Done(
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
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop44(
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
    source_ml_temporary_values39: &[Vec<f64>],
    mut source_ml_temporary_index39: self::ml::Big,
) -> self::MlTemporaryLowerJavaScriptClosedClassSourceLoop44Result {
    loop {
        if source_ml_temporary_index39
            < (self::ml::Big::from_u128(source_ml_temporary_values39.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values39,
                    source_ml_temporary_index39.to_index(),
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
                                let source_ml_source1_string_range_operand221 = source.clone();
                                {
                                    let source_ml_source1_string_range_operand222 = previous;
                                    {
                                        let source_ml_source1_string_range_operand223 =
                                            (self::ml_array::optional_at(
                                                starts,
                                                self::ml_array::number_index(first),
                                            ))
                                            .unwrap_or_else(|| previous);
                                        {
                                            let source_ml_source1_string_range_position224 =
                                                source_ml_source1_string_range_operand222;
                                            {
                                                let source_ml_source1_string_range_position225 =
                                                    source_ml_source1_string_range_operand223;
                                                source_ml_source1_string_range_operand221.slice(
                                                    source_ml_source1_string_range_position224,
                                                    source_ml_source1_string_range_position225,
                                                )
                                            }
                                        }
                                    }
                                }
                            };
                            {
                                let intervals_2 = {
                                    let source_ml_source1_concat_value226 = intervals;
                                    {
                                        let source_ml_source1_concat_value227 = vec![
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
                                                source_ml_source1_concat_value226,
                                            ),
                                            source_ml_source1_concat_value227,
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
                                                let source_ml_source1_concat_value228 = intervals_2;
                                                {
                                                    let source_ml_source1_concat_value229 = vec![
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
                                                            source_ml_source1_concat_value228,
                                                        ),
                                                        source_ml_source1_concat_value229,
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
                                                            let source_ml_source1_concat_value231 =
                                                                brands;
                                                            {
                                                                let source_ml_source1_concat_value232 = vec![(prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95]))).concat(&{
                                                                    let source_ml_source1_string_argument230 = ordinal;
                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument230))
                                                                })];
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value231), source_ml_source1_concat_value232)
                                                            }
                                                        };
                                                        {
                                                            let ordinal_2 = ordinal + 1f64;
                                                            {
                                                                let source_ml_temporary_index39_2 =
                                                                    source_ml_temporary_index39.add(
                                                                        &self::ml::Big::from_i128(
                                                                            1,
                                                                        ),
                                                                    );
                                                                {
                                                                    (text, intervals, previous, ordinal, brands, source_ml_temporary_index39) = (text_3, intervals_3, previous_2, ordinal_2, brands_2, source_ml_temporary_index39_2);
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
            self::MlTemporaryLowerJavaScriptClosedClassSourceLoop44Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop44Done(text.clone(), intervals, previous, brands)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceJoin43 {
    MlTemporaryLowerJavaScriptClosedClassSourceJoin43Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceJoin42 {
    MlTemporaryLowerJavaScriptClosedClassSourceJoin42Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptClosedClassSourceLoop46Result {
    MlTemporaryLowerJavaScriptClosedClassSourceLoop46Done(f64, f64),
}
