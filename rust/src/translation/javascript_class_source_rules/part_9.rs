// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryRenderJavaScriptClosedClassJoin47;
use super::MlTemporaryRenderJavaScriptClosedClassJoin48;
use super::MlTemporaryRenderJavaScriptClosedClassLoop45Result;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_java_script_closed_class_assigned_field_position;
use super::render_java_script_closed_class_field_documentation;
use super::render_java_script_closed_class_range;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_render_java_script_closed_class_loop45(
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
) -> self::MlTemporaryRenderJavaScriptClosedClassLoop45Result {
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
                            let source_ml_source1_s259 = if position < 0f64 {
                                {
                                    let initializer = (((prefix.concat(
                                        &self::ml_string::Text::from_units(vec![
                                            105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 95,
                                            99, 108, 97, 115, 115, 95, 102, 105, 101, 108, 100, 95,
                                        ]),
                                    ))
                                    .concat(&{
                                        let source_ml_source1_string_argument260 = ordinal;
                                        self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                source_ml_source1_string_argument260,
                                            ),
                                        )
                                    }))
                                    .concat(&self::ml_string::Text::from_units(vec![95])))
                                    .concat(&{
                                        let source_ml_source1_string_argument261 = at;
                                        self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                source_ml_source1_string_argument261,
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
                                            let source_ml_source1_string_argument262 = ordinal;
                                            self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(
                                                    source_ml_source1_string_argument262,
                                                ),
                                            )
                                        }))
                                        .concat(&self::ml_string::Text::from_units(vec![95])))
                                        .concat(&{
                                            let source_ml_source1_string_argument263 = at;
                                            self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(
                                                    source_ml_source1_string_argument263,
                                                ),
                                            )
                                        });
                                        {
                                            let documentation = self::render_java_script_closed_class_field_documentation(&({
                                                let source_ml_source1_string_range_operand264 = source.clone();
                                                {
                                                    let source_ml_source1_string_range_operand265 = (self::ml_array::optional_at(ends, self::ml_array::number_index((-position) - 2f64))).unwrap_or_else(|| 0f64);
                                                    {
                                                        let source_ml_source1_string_range_operand266 = (self::ml_array::optional_at(starts, self::ml_array::number_index((-position) - 1f64))).unwrap_or_else(|| 0f64);
                                                        {
                                                            let source_ml_source1_string_range_position267 = source_ml_source1_string_range_operand265;
                                                            {
                                                                let source_ml_source1_string_range_position268 = source_ml_source1_string_range_operand266;
                                                                source_ml_source1_string_range_operand264.slice(source_ml_source1_string_range_position267, source_ml_source1_string_range_position268)
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
                                                                        let source_ml_source1_string_argument269 = ordinal;
                                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument269))
                                                                    })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                                        let source_ml_source1_string_argument270 = assigned;
                                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument270))
                                                                    });
                                                                    self::MlTemporaryRenderJavaScriptClosedClassJoin47::MlTemporaryRenderJavaScriptClosedClassJoin47Next(stubs_2.clone(), initializations_2.clone(), value_3)
                                                                }
                                                            } else {
                                                                self::MlTemporaryRenderJavaScriptClosedClassJoin47::MlTemporaryRenderJavaScriptClosedClassJoin47Next(stubs_2.clone(), initializations_2.clone(), value_2)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlTemporaryRenderJavaScriptClosedClassJoin47::MlTemporaryRenderJavaScriptClosedClassJoin47Next(stubs.clone(), initializations.clone(), value.clone())
                            };
                            match source_ml_source1_s259 {
                                self::MlTemporaryRenderJavaScriptClosedClassJoin47::MlTemporaryRenderJavaScriptClosedClassJoin47Next(stubs_3, initializations_3, value_4) => {
                                    {
                                        let source_ml_source1_s273 = if position >= 0f64 {
                                            {
                                                let capture_2 = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 97, 115, 115, 105, 103, 110, 101, 100, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101, 95]))).concat(&{
                                                    let source_ml_source1_string_argument274 = ordinal;
                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument274))
                                                })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                    let source_ml_source1_string_argument275 = at;
                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument275))
                                                });
                                                {
                                                    let constructor_fields_2 = constructor_fields.concat(&((((self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 32])).concat(&capture_2)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 40]))).concat(&value_4)).concat(&self::ml_string::Text::from_units(vec![41, 59, 10])));
                                                    {
                                                        let value_5 = capture_2.clone();
                                                        self::MlTemporaryRenderJavaScriptClosedClassJoin48::MlTemporaryRenderJavaScriptClosedClassJoin48Next(constructor_fields_2, value_5)
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlTemporaryRenderJavaScriptClosedClassJoin48::MlTemporaryRenderJavaScriptClosedClassJoin48Next(constructor_fields.clone(), value_4.clone())
                                        };
                                        match source_ml_source1_s273 {
                                            self::MlTemporaryRenderJavaScriptClosedClassJoin48::MlTemporaryRenderJavaScriptClosedClassJoin48Next(constructor_fields_3, value_6) => {
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
                                                            let source_ml_source1_concat_value276 = field_names;
                                                            {
                                                                let source_ml_source1_concat_value277 = vec![field.clone()];
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value276), source_ml_source1_concat_value277)
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
            self::MlTemporaryRenderJavaScriptClosedClassLoop45Result::MlTemporaryRenderJavaScriptClosedClassLoop45Done(stubs.clone(), initializations.clone(), constructor_fields.clone(), fields.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_require_java_script_closed_class_prototype_methods_loop51(
    values: &[self::ml_string::Text],
    mut names: Vec<self::ml_string::Text>,
    source_ml_temporary_values49: &[Vec<f64>],
    mut source_ml_temporary_index49: self::ml::Big,
) -> Vec<self::ml_string::Text> {
    loop {
        if source_ml_temporary_index49
            < (self::ml::Big::from_u128(source_ml_temporary_values49.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values49,
                    source_ml_temporary_index49.to_index(),
                );
                {
                    let names_2 = {
                        let source_ml_source1_concat_value278 = names;
                        {
                            let source_ml_source1_concat_value279 = vec![
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
                                    source_ml_source1_concat_value278,
                                ),
                                source_ml_source1_concat_value279,
                            )
                        }
                    };
                    {
                        let source_ml_temporary_index49_2 =
                            source_ml_temporary_index49.add(&self::ml::Big::from_i128(1));
                        {
                            (names, source_ml_temporary_index49) =
                                (names_2, source_ml_temporary_index49_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { names };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result {
    MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Done,
    MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop53Result {
    MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop53Done,
    MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop53Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop55Result {
    MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop55Done,
    MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop55Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_require_java_script_closed_class_prototype_methods_loop55(
    values: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    row: &[f64],
    mut method: f64,
    mut at: f64,
) -> self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop55Result {
    loop {
        return if at
            < ((self::ml_array::optional_at(row, self::ml_array::number_index(method + 4f64)))
                .unwrap_or_else(|| 0f64))
        {
            if ((((self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![110, 101, 119])))
                && self::ml_array::includes(
                    names,
                    &(self::ml_array::optional_at(values, self::ml_array::number_index(at + 1f64)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                    0f64,
                    false,
                    |a, b| a == b,
                ))
                && (((self::ml_array::optional_at(
                    values,
                    self::ml_array::number_index(at + 2f64),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                    == (self::ml_string::Text::from_units(vec![40])))
            {
                self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop55Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop55Return(true)
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
            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop55Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop55Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop54Result {
    MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop54Done,
    MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop54Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_require_java_script_closed_class_prototype_methods_loop54(
    values: &[self::ml_string::Text],
    row: &[f64],
    mut method: f64,
    mut at: f64,
) -> self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop54Result {
    loop {
        return if at
            < ((self::ml_array::optional_at(row, self::ml_array::number_index(method + 2f64)))
                .unwrap_or_else(|| 0f64))
        {
            if ((self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![61]))
            {
                self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop54Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop54Return(true)
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
            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop54Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop54Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_require_java_script_closed_class_prototype_methods_loop53(
    values: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    row: &[f64],
    mut method: f64,
) -> self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop53Result {
    loop {
        return if method < (row.len() as f64) {
            if ((self::ml_array::optional_at(row, self::ml_array::number_index(method)))
                .unwrap_or_else(|| -1f64))
                >= 0f64
            {
                {
                    let at = (self::ml_array::optional_at(
                        row,
                        self::ml_array::number_index(method + 1f64),
                    ))
                    .unwrap_or_else(|| 0f64)
                        + 1f64;
                    {
                        let source_ml_source1_s280 = self::source_ml_temporary_require_java_script_closed_class_prototype_methods_loop54(values, row, method, at);
                        match source_ml_source1_s280 {
                            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop54Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop54Done => {
                                {
                                    let at_2 = (self::ml_array::optional_at(row, self::ml_array::number_index(method + 3f64))).unwrap_or_else(|| 0f64) + 1f64;
                                    {
                                        let source_ml_source1_s281 = self::source_ml_temporary_require_java_script_closed_class_prototype_methods_loop55(values, names, row, method, at_2);
                                        match source_ml_source1_s281 {
                                            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop55Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop55Done => {
                                                {
                                                    let method_2 = method + 5f64;
                                                    {
                                                        method = method_2;
                                                        continue;
                                                    }
                                                }
                                            }
                                            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop55Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop55Return(source_ml_temporary_result55) => {
                                                self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop53Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop53Return(source_ml_temporary_result55)
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop54Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop54Return(source_ml_temporary_result55_2) => {
                                self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop53Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop53Return(source_ml_temporary_result55_2)
                            }
                        }
                    }
                }
            } else {
                {
                    let method_3 = method + 5f64;
                    {
                        method = method_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop53Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop53Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_require_java_script_closed_class_prototype_methods_loop52(
    values: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    source_ml_temporary_values50: &[Vec<f64>],
    mut source_ml_temporary_index50: self::ml::Big,
) -> self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result {
    loop {
        return if source_ml_temporary_index50
            < (self::ml::Big::from_u128(source_ml_temporary_values50.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values50,
                    source_ml_temporary_index50.to_index(),
                );
                {
                    let method = 9f64
                        + (3f64
                            * (self::ml_array::optional_at(
                                &row,
                                self::ml_array::number_index(7f64),
                            ))
                            .unwrap_or_else(|| 0f64));
                    {
                        let source_ml_source1_s282 = self::source_ml_temporary_require_java_script_closed_class_prototype_methods_loop53(values, names, &(row), method);
                        match source_ml_source1_s282 {
                            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop53Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop53Done => {
                                {
                                    let source_ml_temporary_index50_2 = source_ml_temporary_index50.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index50 = source_ml_temporary_index50_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop53Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop53Return(source_ml_temporary_result53) => {
                                self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Return(source_ml_temporary_result53)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptClosedClassMethodTargetLoop56Result {
    MlTemporaryReadJavaScriptClosedClassMethodTargetLoop56Done(self::ml_string::Text),
    MlTemporaryReadJavaScriptClosedClassMethodTargetLoop56Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_method_target_loop56(
    methods: &[self::ml_string::Text],
    receiver_fields: &[self::ml_string::Text],
    method: &self::ml_string::Text,
    mut target: self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassMethodTargetLoop56Result {
    loop {
        return if at < (methods.len() as f64) {
            if self::ml_array::includes(
                receiver_fields,
                &(self::ml_array::optional_at(methods, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                0f64,
                false,
                |a, b| a == b,
            ) && (((self::ml_array::optional_at(
                methods,
                self::ml_array::number_index(at + 1f64),
            ))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (*method))
            {
                {
                    let ml_library_inequality_1 =
                        target != (self::ml_string::Text::from_units(vec![]));
                    if ml_library_inequality_1 {
                        self::MlTemporaryReadJavaScriptClosedClassMethodTargetLoop56Result::MlTemporaryReadJavaScriptClosedClassMethodTargetLoop56Return(self::ml_string::Text::from_units(vec![]))
                    } else {
                        {
                            let target_2 = (self::ml_array::optional_at(
                                methods,
                                self::ml_array::number_index(at + 2f64),
                            ))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                            {
                                let at_2 = at + 3f64;
                                {
                                    (target, at) = (target_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            } else {
                {
                    let at_3 = at + 3f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassMethodTargetLoop56Result::MlTemporaryReadJavaScriptClosedClassMethodTargetLoop56Done(target.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop58(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut syntax: Vec<self::ml_string::Text>,
    mut at: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if at < (values.len() as f64) {
            {
                let syntax_2 = {
                    let source_ml_source1_concat_value283 = syntax;
                    {
                        let source_ml_source1_concat_value284 =
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
                                source_ml_source1_concat_value283,
                            ),
                            source_ml_source1_concat_value284,
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
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop64(
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
pub enum MlTemporaryLowerJavaScriptClosedClassSourceLoop63Result {
    MlTemporaryLowerJavaScriptClosedClassSourceLoop63Done(
        self::ml_string::Text,
        Vec<f64>,
        f64,
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        Vec<Vec<self::ml_string::Text>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop65(
    values: &[self::ml_string::Text],
    prefix: &self::ml_string::Text,
    mut ordinal: f64,
    mut prototype_methods: Vec<self::ml_string::Text>,
    row: &[f64],
    brand: &self::ml_string::Text,
    mut method: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if method < (row.len() as f64) {
            if ((self::ml_array::optional_at(row, self::ml_array::number_index(method)))
                .unwrap_or_else(|| -1f64))
                >= 0f64
            {
                {
                    let prototype_methods_2 = {
                        let source_ml_source1_concat_value287 = prototype_methods;
                        {
                            let source_ml_source1_concat_value288 = vec![
                                brand.clone(),
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
                                (((prefix.concat(&self::ml_string::Text::from_units(vec![
                                    99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95,
                                ])))
                                .concat(&{
                                    let source_ml_source1_string_argument285 = ordinal;
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                                        source_ml_source1_string_argument285,
                                    ))
                                }))
                                .concat(&self::ml_string::Text::from_units(vec![95])))
                                .concat(&{
                                    let source_ml_source1_string_argument286 = method;
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                                        source_ml_source1_string_argument286,
                                    ))
                                }),
                            ];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<self::ml_string::Text>::new(),
                                    source_ml_source1_concat_value287,
                                ),
                                source_ml_source1_concat_value288,
                            )
                        }
                    };
                    {
                        let method_2 = method + 5f64;
                        {
                            (prototype_methods, method) = (prototype_methods_2, method_2);
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
        return { prototype_methods };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop66(
    values: &[self::ml_string::Text],
    mut public_fields: Vec<self::ml_string::Text>,
    row: &[f64],
    brand: &self::ml_string::Text,
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
                    let public_fields_2 = {
                        let source_ml_source1_concat_value289 = public_fields;
                        {
                            let source_ml_source1_concat_value290 = vec![
                                brand.clone(),
                                (self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(if position < 0f64 {
                                        (-position) - 1f64
                                    } else {
                                        position
                                    }),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                            ];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<self::ml_string::Text>::new(),
                                    source_ml_source1_concat_value289,
                                ),
                                source_ml_source1_concat_value290,
                            )
                        }
                    };
                    {
                        let field_2 = field + 3f64;
                        {
                            (public_fields, field) = (public_fields_2, field_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { public_fields };
    }
}
