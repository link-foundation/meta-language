// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryRenderJavaScriptClosedClassJoin45;
use super::MlTemporaryRenderJavaScriptClosedClassJoin46;
use super::MlTemporaryRenderJavaScriptClosedClassLoop43Result;
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
pub fn source_ml_temporary_render_java_script_closed_class_loop43(
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
) -> self::MlTemporaryRenderJavaScriptClosedClassLoop43Result {
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
                            let source_ml_source1_s248 = if position < 0f64 {
                                {
                                    let initializer = (((prefix.concat(
                                        &self::ml_string::Text::from_units(vec![
                                            105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 95,
                                            99, 108, 97, 115, 115, 95, 102, 105, 101, 108, 100, 95,
                                        ]),
                                    ))
                                    .concat(&{
                                        let source_ml_source1_string_argument249 = ordinal;
                                        self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                source_ml_source1_string_argument249,
                                            ),
                                        )
                                    }))
                                    .concat(&self::ml_string::Text::from_units(vec![95])))
                                    .concat(&{
                                        let source_ml_source1_string_argument250 = at;
                                        self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                source_ml_source1_string_argument250,
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
                                            let source_ml_source1_string_argument251 = ordinal;
                                            self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(
                                                    source_ml_source1_string_argument251,
                                                ),
                                            )
                                        }))
                                        .concat(&self::ml_string::Text::from_units(vec![95])))
                                        .concat(&{
                                            let source_ml_source1_string_argument252 = at;
                                            self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(
                                                    source_ml_source1_string_argument252,
                                                ),
                                            )
                                        });
                                        {
                                            let documentation = self::render_java_script_closed_class_field_documentation(&({
                                                let source_ml_source1_string_range_operand253 = source.clone();
                                                {
                                                    let source_ml_source1_string_range_operand254 = (self::ml_array::optional_at(ends, self::ml_array::number_index((-position) - 2f64))).unwrap_or_else(|| 0f64);
                                                    {
                                                        let source_ml_source1_string_range_operand255 = (self::ml_array::optional_at(starts, self::ml_array::number_index((-position) - 1f64))).unwrap_or_else(|| 0f64);
                                                        {
                                                            let source_ml_source1_string_range_position256 = source_ml_source1_string_range_operand254;
                                                            {
                                                                let source_ml_source1_string_range_position257 = source_ml_source1_string_range_operand255;
                                                                source_ml_source1_string_range_operand253.slice(source_ml_source1_string_range_position256, source_ml_source1_string_range_position257)
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
                                                                        let source_ml_source1_string_argument258 = ordinal;
                                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument258))
                                                                    })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                                        let source_ml_source1_string_argument259 = assigned;
                                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument259))
                                                                    });
                                                                    self::MlTemporaryRenderJavaScriptClosedClassJoin45::MlTemporaryRenderJavaScriptClosedClassJoin45Next(stubs_2.clone(), initializations_2.clone(), value_3)
                                                                }
                                                            } else {
                                                                self::MlTemporaryRenderJavaScriptClosedClassJoin45::MlTemporaryRenderJavaScriptClosedClassJoin45Next(stubs_2.clone(), initializations_2.clone(), value_2)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlTemporaryRenderJavaScriptClosedClassJoin45::MlTemporaryRenderJavaScriptClosedClassJoin45Next(stubs.clone(), initializations.clone(), value.clone())
                            };
                            match source_ml_source1_s248 {
                                self::MlTemporaryRenderJavaScriptClosedClassJoin45::MlTemporaryRenderJavaScriptClosedClassJoin45Next(stubs_3, initializations_3, value_4) => {
                                    {
                                        let source_ml_source1_s262 = if position >= 0f64 {
                                            {
                                                let capture_2 = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 97, 115, 115, 105, 103, 110, 101, 100, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101, 95]))).concat(&{
                                                    let source_ml_source1_string_argument263 = ordinal;
                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument263))
                                                })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                    let source_ml_source1_string_argument264 = at;
                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument264))
                                                });
                                                {
                                                    let constructor_fields_2 = constructor_fields.concat(&((((self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 32])).concat(&capture_2)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 40]))).concat(&value_4)).concat(&self::ml_string::Text::from_units(vec![41, 59, 10])));
                                                    {
                                                        let value_5 = capture_2.clone();
                                                        self::MlTemporaryRenderJavaScriptClosedClassJoin46::MlTemporaryRenderJavaScriptClosedClassJoin46Next(constructor_fields_2, value_5)
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlTemporaryRenderJavaScriptClosedClassJoin46::MlTemporaryRenderJavaScriptClosedClassJoin46Next(constructor_fields.clone(), value_4.clone())
                                        };
                                        match source_ml_source1_s262 {
                                            self::MlTemporaryRenderJavaScriptClosedClassJoin46::MlTemporaryRenderJavaScriptClosedClassJoin46Next(constructor_fields_3, value_6) => {
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
                                                            let source_ml_source1_concat_value265 = field_names;
                                                            {
                                                                let source_ml_source1_concat_value266 = vec![field.clone()];
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value265), source_ml_source1_concat_value266)
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
            self::MlTemporaryRenderJavaScriptClosedClassLoop43Result::MlTemporaryRenderJavaScriptClosedClassLoop43Done(stubs.clone(), initializations.clone(), constructor_fields.clone(), fields.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_require_java_script_closed_class_prototype_methods_loop49(
    values: &[self::ml_string::Text],
    mut names: Vec<self::ml_string::Text>,
    source_ml_temporary_values47: &[Vec<f64>],
    mut source_ml_temporary_index47: self::ml::Big,
) -> Vec<self::ml_string::Text> {
    loop {
        if source_ml_temporary_index47
            < (self::ml::Big::from_u128(source_ml_temporary_values47.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values47,
                    source_ml_temporary_index47.to_index(),
                );
                {
                    let names_2 = {
                        let source_ml_source1_concat_value267 = names;
                        {
                            let source_ml_source1_concat_value268 = vec![
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
                                    source_ml_source1_concat_value267,
                                ),
                                source_ml_source1_concat_value268,
                            )
                        }
                    };
                    {
                        let source_ml_temporary_index47_2 =
                            source_ml_temporary_index47.add(&self::ml::Big::from_i128(1));
                        {
                            (names, source_ml_temporary_index47) =
                                (names_2, source_ml_temporary_index47_2);
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
pub enum MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result {
    MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Done,
    MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result {
    MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Done,
    MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result {
    MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Done,
    MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_require_java_script_closed_class_prototype_methods_loop52(
    values: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    row: &[f64],
    mut method: f64,
    mut at: f64,
) -> self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result {
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
                self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Return(true)
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
            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_require_java_script_closed_class_prototype_methods_loop51(
    values: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    row: &[f64],
    mut method: f64,
) -> self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result {
    loop {
        return if method < (row.len() as f64) {
            if ((self::ml_array::optional_at(row, self::ml_array::number_index(method)))
                .unwrap_or_else(|| -1f64))
                >= 0f64
            {
                {
                    let at = (self::ml_array::optional_at(
                        row,
                        self::ml_array::number_index(method + 3f64),
                    ))
                    .unwrap_or_else(|| 0f64)
                        + 1f64;
                    {
                        let source_ml_source1_s269 = self::source_ml_temporary_require_java_script_closed_class_prototype_methods_loop52(values, names, row, method, at);
                        match source_ml_source1_s269 {
                            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Done => {
                                {
                                    let method_2 = method + 5f64;
                                    {
                                        method = method_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Return(source_ml_temporary_result52) => {
                                self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Return(source_ml_temporary_result52)
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
            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_require_java_script_closed_class_prototype_methods_loop50(
    values: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    source_ml_temporary_values48: &[Vec<f64>],
    mut source_ml_temporary_index48: self::ml::Big,
) -> self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result {
    loop {
        return if source_ml_temporary_index48
            < (self::ml::Big::from_u128(source_ml_temporary_values48.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values48,
                    source_ml_temporary_index48.to_index(),
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
                        let source_ml_source1_s270 = self::source_ml_temporary_require_java_script_closed_class_prototype_methods_loop51(values, names, &(row), method);
                        match source_ml_source1_s270 {
                            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Done => {
                                {
                                    let source_ml_temporary_index48_2 = source_ml_temporary_index48.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index48 = source_ml_temporary_index48_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Return(source_ml_temporary_result51) => {
                                self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Return(source_ml_temporary_result51)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptClosedClassMethodTargetLoop53Result {
    MlTemporaryReadJavaScriptClosedClassMethodTargetLoop53Done(self::ml_string::Text),
    MlTemporaryReadJavaScriptClosedClassMethodTargetLoop53Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_method_target_loop53(
    methods: &[self::ml_string::Text],
    receiver_fields: &[self::ml_string::Text],
    method: &self::ml_string::Text,
    mut target: self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassMethodTargetLoop53Result {
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
                        self::MlTemporaryReadJavaScriptClosedClassMethodTargetLoop53Result::MlTemporaryReadJavaScriptClosedClassMethodTargetLoop53Return(self::ml_string::Text::from_units(vec![]))
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
            self::MlTemporaryReadJavaScriptClosedClassMethodTargetLoop53Result::MlTemporaryReadJavaScriptClosedClassMethodTargetLoop53Done(target.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop55(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut syntax: Vec<self::ml_string::Text>,
    mut at: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if at < (values.len() as f64) {
            {
                let syntax_2 = {
                    let source_ml_source1_concat_value271 = syntax;
                    {
                        let source_ml_source1_concat_value272 =
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
                                source_ml_source1_concat_value271,
                            ),
                            source_ml_source1_concat_value272,
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
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop61(
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
pub enum MlTemporaryLowerJavaScriptClosedClassSourceLoop60Result {
    MlTemporaryLowerJavaScriptClosedClassSourceLoop60Done(
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
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop62(
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
                        let source_ml_source1_concat_value275 = prototype_methods;
                        {
                            let source_ml_source1_concat_value276 = vec![
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
                                    let source_ml_source1_string_argument273 = ordinal;
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                                        source_ml_source1_string_argument273,
                                    ))
                                }))
                                .concat(&self::ml_string::Text::from_units(vec![95])))
                                .concat(&{
                                    let source_ml_source1_string_argument274 = method;
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                                        source_ml_source1_string_argument274,
                                    ))
                                }),
                            ];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<self::ml_string::Text>::new(),
                                    source_ml_source1_concat_value275,
                                ),
                                source_ml_source1_concat_value276,
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
pub fn source_ml_temporary_lower_java_script_closed_class_source_loop63(
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
                        let source_ml_source1_concat_value277 = public_fields;
                        {
                            let source_ml_source1_concat_value278 = vec![
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
                                    source_ml_source1_concat_value277,
                                ),
                                source_ml_source1_concat_value278,
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
