// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result;
use super::MlTemporaryReadJavaScriptClosedClassMethodTargetLoop53Result;
use super::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop37Result;
use super::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Result;
use super::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop26Result;
use super::MlTemporaryRenderJavaScriptClosedClassLoop43Result;
use super::MlTemporaryRenderJavaScriptClosedClassLoop44Result;
use super::MlTemporaryRenderJavaScriptClosedClassRangeLoop21Result;
use super::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_string;
use super::ml_string_number;
use super::read_java_script_closed_class_prefix_end;
use super::source_ml_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop29;
use super::source_ml_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop30;
use super::source_ml_temporary_read_java_script_closed_class_function_exports_loop34;
use super::source_ml_temporary_read_java_script_closed_class_method_target_loop53;
use super::source_ml_temporary_read_java_script_closed_class_methods_loop22;
use super::source_ml_temporary_read_java_script_closed_class_observation_error_loop37;
use super::source_ml_temporary_read_java_script_closed_class_observation_error_loop38;
use super::source_ml_temporary_read_java_script_closed_class_static_calls_loop24;
use super::source_ml_temporary_read_java_script_closed_class_static_target_loop26;
use super::source_ml_temporary_render_java_script_closed_class_loop43;
use super::source_ml_temporary_render_java_script_closed_class_loop44;
use super::source_ml_temporary_render_java_script_closed_class_range_loop21;
use super::source_ml_temporary_require_java_script_closed_class_prototype_methods_loop49;
use super::source_ml_temporary_require_java_script_closed_class_prototype_methods_loop50;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn render_java_script_closed_class_range(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    first: f64,
    last: f64,
    receiver: &self::ml_string::Text,
    class_names: &[self::ml_string::Text],
    static_calls: &[self::ml_string::Text],
) -> self::ml_string::Text {
    if first >= last {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let result = self::ml_string::Text::from_units(vec![]);
            {
                let previous =
                    (self::ml_array::optional_at(starts, self::ml_array::number_index(first)))
                        .unwrap_or_else(|| 0f64);
                {
                    let at = first;
                    {
                        let source_ml_source1_s20 =
                            self::source_ml_temporary_render_java_script_closed_class_range_loop21(
                                source,
                                values,
                                starts,
                                ends,
                                last,
                                receiver,
                                class_names,
                                static_calls,
                                result,
                                previous,
                                at,
                            );
                        match source_ml_source1_s20 {
                            self::MlTemporaryRenderJavaScriptClosedClassRangeLoop21Result::MlTemporaryRenderJavaScriptClosedClassRangeLoop21Done(result_2, previous_2) => {
                                result_2.concat(&{
                                    let source_ml_source1_string_range_operand21 = source.clone();
                                    {
                                        let source_ml_source1_string_range_operand22 = previous_2;
                                        {
                                            let source_ml_source1_string_range_operand23 = (self::ml_array::optional_at(ends, self::ml_array::number_index(last - 1f64))).unwrap_or_else(|| previous_2);
                                            {
                                                let source_ml_source1_string_range_position24 = source_ml_source1_string_range_operand22;
                                                {
                                                    let source_ml_source1_string_range_position25 = source_ml_source1_string_range_operand23;
                                                    source_ml_source1_string_range_operand21.slice(source_ml_source1_string_range_position24, source_ml_source1_string_range_position25)
                                                }
                                            }
                                        }
                                    }
                                })
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_methods(
    values: &[self::ml_string::Text],
    directory: &[f64],
) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let first = 9f64
                + (3f64
                    * (self::ml_array::optional_at(directory, self::ml_array::number_index(7f64)))
                        .unwrap_or_else(|| 0f64));
            {
                let at = first;
                {
                    self::source_ml_temporary_read_java_script_closed_class_methods_loop22(
                        values, directory, result, at,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_static_calls(
    values: &[self::ml_string::Text],
    directory: &[Vec<f64>],
    prefix: &self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let ordinal = 0f64;
            {
                let source_ml_temporary_values23 = directory.to_vec();
                {
                    let source_ml_temporary_index23 = self::ml::Big::from_i128(0);
                    {
                        self::source_ml_temporary_read_java_script_closed_class_static_calls_loop24(
                            values,
                            prefix,
                            result,
                            ordinal,
                            &(source_ml_temporary_values23),
                            source_ml_temporary_index23,
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_static_target(
    values: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    calls: &[self::ml_string::Text],
    at: f64,
) -> self::ml_string::Text {
    {
        let previous =
            (self::ml_array::optional_at(values, self::ml_array::number_index(at - 1f64)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
        if ((((!(self::ml_array::includes(
            names,
            &(self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
            0f64,
            false,
            |a, b| a == b,
        )) || (previous == (self::ml_string::Text::from_units(vec![110, 101, 119]))))
            || (previous == (self::ml_string::Text::from_units(vec![46]))))
            || (previous == (self::ml_string::Text::from_units(vec![63, 46]))))
            || (((self::ml_array::optional_at(values, self::ml_array::number_index(at + 1f64)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                != (self::ml_string::Text::from_units(vec![46]))))
            || (((self::ml_array::optional_at(values, self::ml_array::number_index(at + 3f64)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                != (self::ml_string::Text::from_units(vec![40])))
        {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let key =
                    (((self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                    .concat(&self::ml_string::Text::from_units(vec![46])))
                    .concat(
                        &(self::ml_array::optional_at(
                            values,
                            self::ml_array::number_index(at + 2f64),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                    );
                {
                    let index = 0f64;
                    {
                        let source_ml_source1_s26 = self::source_ml_temporary_read_java_script_closed_class_static_target_loop26(calls, &(key), index);
                        match source_ml_source1_s26 {
                            self::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop26Result::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop26Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop26Result::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop26Return(source_ml_temporary_result26) => {
                                source_ml_temporary_result26.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_collection_default_class_owner_error(
    values: &[self::ml_string::Text],
    directory: &[Vec<f64>],
) -> f64 {
    {
        let names = Vec::<self::ml_string::Text>::new();
        {
            let source_ml_temporary_values27 = directory.to_vec();
            {
                let source_ml_temporary_index27 = self::ml::Big::from_i128(0);
                {
                    let names_2 = self::source_ml_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop29(values, names, &(source_ml_temporary_values27), source_ml_temporary_index27);
                    {
                        let source_ml_temporary_values28 = directory.to_vec();
                        {
                            let source_ml_temporary_index28 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source1_s27 = self::source_ml_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop30(values, &(names_2), &(source_ml_temporary_values28), source_ml_temporary_index28);
                                match source_ml_source1_s27 {
                                    self::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Done => {
                                        -1f64
                                    }
                                    self::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result::MlTemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Return(source_ml_temporary_result30) => {
                                        source_ml_temporary_result30
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_closed_class_function_export(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    at: f64,
) -> bool {
    {
        let previous =
            (self::ml_array::optional_at(values, self::ml_array::number_index(at - 1f64)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
        ((((((((self::ml_array::optional_at(values, self::ml_array::number_index(at)))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
            == (self::ml_string::Text::from_units(vec![101, 120, 112, 111, 114, 116])))
            && (previous != (self::ml_string::Text::from_units(vec![46]))))
            && (previous != (self::ml_string::Text::from_units(vec![63, 46]))))
            && (((self::ml_array::optional_at(values, self::ml_array::number_index(at + 1f64)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![
                    102, 117, 110, 99, 116, 105, 111, 110,
                ]))))
            && (((self::ml_array::optional_at(kinds, self::ml_array::number_index(at + 2f64)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![
                    105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                ]))))
            && (((self::ml_array::optional_at(values, self::ml_array::number_index(at + 3f64)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![40]))))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_function_exports(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let at = 0f64;
            {
                self::source_ml_temporary_read_java_script_closed_class_function_exports_loop34(
                    values, kinds, result, at,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_observation_error(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    directory: &[Vec<f64>],
    static_calls: &[self::ml_string::Text],
) -> f64 {
    {
        let names = Vec::<self::ml_string::Text>::new();
        {
            let methods = Vec::<self::ml_string::Text>::new();
            {
                let source_ml_temporary_values35 = directory.to_vec();
                {
                    let source_ml_temporary_index35 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s28 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop37(values, names, methods, &(source_ml_temporary_values35), source_ml_temporary_index35);
                        match source_ml_source1_s28 {
                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop37Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop37Done(names_2, methods_2) => {
                                {
                                    let at = 0f64;
                                    {
                                        let source_ml_source1_s29 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop38(values, kinds, directory, static_calls, &(names_2), &(methods_2), at);
                                        match source_ml_source1_s29 {
                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Done => {
                                                -1f64
                                            }
                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(source_ml_temporary_result38) => {
                                                source_ml_temporary_result38
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop37Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop37Return(source_ml_temporary_result40) => {
                                source_ml_temporary_result40
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn render_java_script_closed_class(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    row: &[f64],
    prefix: &self::ml_string::Text,
    ordinal: f64,
    class_names: &[self::ml_string::Text],
    static_calls: &[self::ml_string::Text],
    prototype: bool,
) -> self::ml_string::Text {
    {
        let name = (self::ml_array::optional_at(
            values,
            self::ml_array::number_index(
                (self::ml_array::optional_at(row, self::ml_array::number_index(0f64)))
                    .unwrap_or_else(|| 0f64)
                    + 1f64,
            ),
        ))
        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
        {
            let receiver = (prefix.concat(&self::ml_string::Text::from_units(vec![
                99, 108, 97, 115, 115, 95, 114, 101, 99, 101, 105, 118, 101, 114, 95,
            ])))
            .concat(&{
                let source_ml_source1_string_argument30 = ordinal;
                self::ml_string::Text::from_text(&self::ml_number::js_number(
                    source_ml_source1_string_argument30,
                ))
            });
            {
                let brand = (prefix.concat(&self::ml_string::Text::from_units(vec![
                    99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95,
                ])))
                .concat(&{
                    let source_ml_source1_string_argument31 = ordinal;
                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                        source_ml_source1_string_argument31,
                    ))
                });
                {
                    let brand_value = (prefix.concat(&self::ml_string::Text::from_units(vec![
                        99, 108, 97, 115, 115, 95, 98, 114, 97, 110, 100, 95, 118, 97, 108, 117,
                        101, 95,
                    ])))
                    .concat(&{
                        let source_ml_source1_string_argument32 = ordinal;
                        self::ml_string::Text::from_text(&self::ml_number::js_number(
                            source_ml_source1_string_argument32,
                        ))
                    });
                    {
                        let field_count =
                            (self::ml_array::optional_at(row, self::ml_array::number_index(7f64)))
                                .unwrap_or_else(|| 0f64);
                        {
                            let methods = 9f64 + (field_count * 3f64);
                            {
                                let parameter_first = (self::ml_array::optional_at(
                                    row,
                                    self::ml_array::number_index(3f64),
                                ))
                                .unwrap_or_else(|| -1f64)
                                    + 1f64;
                                {
                                    let parameter_last = (self::ml_array::optional_at(
                                        row,
                                        self::ml_array::number_index(4f64),
                                    ))
                                    .unwrap_or_else(|| -1f64);
                                    {
                                        let parameters = if parameter_last < 0f64 {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::render_java_script_closed_class_range(
                                                source,
                                                values,
                                                starts,
                                                ends,
                                                parameter_first,
                                                parameter_last,
                                                &(self::ml_string::Text::from_units(vec![])),
                                                class_names,
                                                static_calls,
                                            )
                                        };
                                        {
                                            let stubs =
                                                ((self::ml_string::Text::from_units(vec![
                                                    10, 102, 117, 110, 99, 116, 105, 111, 110, 32,
                                                ]))
                                                .concat(&brand_value))
                                                .concat(&self::ml_string::Text::from_units(vec![
                                                    40, 41, 32, 123, 32, 114, 101, 116, 117, 114,
                                                    110, 59, 32, 125, 10,
                                                ]));
                                            {
                                                let initializations =
                                                    self::ml_string::Text::from_units(vec![]);
                                                {
                                                    let constructor_fields =
                                                        self::ml_string::Text::from_units(vec![]);
                                                    {
                                                        let field_names =
                                                            Vec::<self::ml_string::Text>::new();
                                                        {
                                                            let fields = ((brand.concat(
                                                                &self::ml_string::Text::from_units(
                                                                    vec![58, 32],
                                                                ),
                                                            ))
                                                            .concat(&brand_value))
                                                            .concat(
                                                                &self::ml_string::Text::from_units(
                                                                    vec![40, 41],
                                                                ),
                                                            );
                                                            {
                                                                let at = 9f64;
                                                                {
                                                                    let source_ml_source1_s33 = self::source_ml_temporary_render_java_script_closed_class_loop43(source, values, starts, ends, row, prefix, ordinal, class_names, static_calls, methods, stubs, initializations, constructor_fields, field_names, fields, at);
                                                                    match source_ml_source1_s33 {
                                                                        self::MlTemporaryRenderJavaScriptClosedClassLoop43Result::MlTemporaryRenderJavaScriptClosedClassLoop43Done(stubs_2, initializations_2, constructor_fields_2, fields_2) => {
                                                                            {
                                                                                let assignments = self::ml_string::Text::from_units(vec![]);
                                                                                {
                                                                                    let at_2 = methods;
                                                                                    {
                                                                                        let source_ml_source1_s34 = self::source_ml_temporary_render_java_script_closed_class_loop44(source, values, starts, ends, row, prefix, ordinal, class_names, static_calls, prototype, &(receiver), stubs_2.clone(), fields_2.clone(), assignments, at_2);
                                                                                        match source_ml_source1_s34 {
                                                                                            self::MlTemporaryRenderJavaScriptClosedClassLoop44Result::MlTemporaryRenderJavaScriptClosedClassLoop44Done(stubs_3, fields_3, assignments_2) => {
                                                                                                {
                                                                                                    let constructor_body = (self::ml_array::optional_at(row, self::ml_array::number_index(5f64))).unwrap_or_else(|| -1f64);
                                                                                                    {
                                                                                                        let constructor_end = (self::ml_array::optional_at(row, self::ml_array::number_index(6f64))).unwrap_or_else(|| -1f64);
                                                                                                        {
                                                                                                            let initialization = (self::ml_array::optional_at(row, self::ml_array::number_index(8f64))).unwrap_or_else(|| -1f64);
                                                                                                            {
                                                                                                                let prologue = if constructor_body < 0f64 {
                                                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                                                } else {
                                                                                                                    self::render_java_script_closed_class_range(source, values, starts, ends, (constructor_body + 1f64), initialization, &(self::ml_string::Text::from_units(vec![])), class_names, static_calls)
                                                                                                                };
                                                                                                                {
                                                                                                                    let remainder = if constructor_body < 0f64 {
                                                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                                                    } else {
                                                                                                                        self::render_java_script_closed_class_range(source, values, starts, ends, self::read_java_script_closed_class_prefix_end(row), constructor_end, &(receiver), class_names, static_calls)
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let constructor_name = (self::ml_array::optional_at(row, self::ml_array::number_index(2f64))).unwrap_or_else(|| -1f64);
                                                                                                                        {
                                                                                                                            let documentation = if constructor_name < 0f64 {
                                                                                                                                self::ml_string::Text::from_units(vec![])
                                                                                                                            } else {
                                                                                                                                {
                                                                                                                                    let source_ml_source1_string_range_operand35 = source.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source1_string_range_operand36 = (self::ml_array::optional_at(ends, self::ml_array::number_index(constructor_name - 1f64))).unwrap_or_else(|| 0f64);
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_string_range_operand37 = (self::ml_array::optional_at(starts, self::ml_array::number_index(constructor_name))).unwrap_or_else(|| 0f64);
                                                                                                                                            {
                                                                                                                                                let source_ml_source1_string_range_position38 = source_ml_source1_string_range_operand36;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source1_string_range_position39 = source_ml_source1_string_range_operand37;
                                                                                                                                                    source_ml_source1_string_range_operand35.slice(source_ml_source1_string_range_position38, source_ml_source1_string_range_position39)
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            };
                                                                                                                            ((((((((((((((((((((stubs_3.concat(&self::ml_string::Text::from_units(vec![10]))).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(&name)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&parameters)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 10]))).concat(&initializations_2)).concat(&prologue)).concat(&self::ml_string::Text::from_units(vec![10]))).concat(&constructor_fields_2)).concat(&self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 123, 32]))).concat(&fields_3)).concat(&self::ml_string::Text::from_units(vec![32, 125, 59, 10]))).concat(&assignments_2)).concat(&remainder)).concat(&self::ml_string::Text::from_units(vec![10, 114, 101, 116, 117, 114, 110, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![59, 10, 125, 10]))
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
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_java_script_closed_class_prototype_methods(
    values: &[self::ml_string::Text],
    directory: &[Vec<f64>],
) -> bool {
    {
        let names = Vec::<self::ml_string::Text>::new();
        {
            let source_ml_temporary_values47 = directory.to_vec();
            {
                let source_ml_temporary_index47 = self::ml::Big::from_i128(0);
                {
                    let names_2 = self::source_ml_temporary_require_java_script_closed_class_prototype_methods_loop49(values, names, &(source_ml_temporary_values47), source_ml_temporary_index47);
                    {
                        let source_ml_temporary_values48 = directory.to_vec();
                        {
                            let source_ml_temporary_index48 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source1_s40 = self::source_ml_temporary_require_java_script_closed_class_prototype_methods_loop50(values, &(names_2), &(source_ml_temporary_values48), source_ml_temporary_index48);
                                match source_ml_source1_s40 {
                                    self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Done => {
                                        false
                                    }
                                    self::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result::MlTemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Return(source_ml_temporary_result50) => {
                                        source_ml_temporary_result50
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_method_target(
    methods: &[self::ml_string::Text],
    receiver_fields: &[self::ml_string::Text],
    method: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let ml_library_inequality_1 = ((methods.len() as f64) % 3f64) != 0f64;
        if ml_library_inequality_1 {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let target = self::ml_string::Text::from_units(vec![]);
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s41 = self::source_ml_temporary_read_java_script_closed_class_method_target_loop53(methods, receiver_fields, method, target, at);
                        match source_ml_source1_s41 {
                        self::MlTemporaryReadJavaScriptClosedClassMethodTargetLoop53Result::MlTemporaryReadJavaScriptClosedClassMethodTargetLoop53Done(target_2) => {
                            target_2
                        }
                        self::MlTemporaryReadJavaScriptClosedClassMethodTargetLoop53Result::MlTemporaryReadJavaScriptClosedClassMethodTargetLoop53Return(source_ml_temporary_result53) => {
                            source_ml_temporary_result53
                        }
                    }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_closed_class_method_definition(
    brand: &self::ml_string::Text,
    method: &self::ml_string::Text,
    helper: &self::ml_string::Text,
    parameter: &self::ml_string::Text,
    receiver_fields: &[self::ml_string::Text],
    constructor: bool,
) -> bool {
    {
        let marker = {
            let source_ml_source1_string_search_operand42 = brand.clone();
            {
                let source_ml_source1_string_search_operand43 =
                    self::ml_string::Text::from_units(vec![
                        99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95,
                    ]);
                {
                    let source_ml_source1_string_search_text44 =
                        source_ml_source1_string_search_operand43;
                    {
                        let source_ml_source1_string_search_position45 = f64::NAN;
                        source_ml_source1_string_search_operand42.last_index_of(
                            &source_ml_source1_string_search_text44,
                            source_ml_source1_string_search_position45,
                        )
                    }
                }
            }
        };
        if marker < 0f64 {
            false
        } else {
            {
                let prefix = {
                    let source_ml_source1_string_range_operand46 = brand.clone();
                    {
                        let source_ml_source1_string_range_operand47 = 0f64;
                        {
                            let source_ml_source1_string_range_operand48 = marker;
                            {
                                let source_ml_source1_string_range_position49 =
                                    source_ml_source1_string_range_operand47;
                                {
                                    let source_ml_source1_string_range_position50 =
                                        source_ml_source1_string_range_operand48;
                                    source_ml_source1_string_range_operand46.slice(
                                        source_ml_source1_string_range_position49,
                                        source_ml_source1_string_range_position50,
                                    )
                                }
                            }
                        }
                    }
                };
                {
                    let ordinal = {
                        let source_ml_source1_string_range_operand51 = brand.clone();
                        {
                            let source_ml_source1_string_range_operand52 = marker + 15f64;
                            {
                                let source_ml_source1_string_range_position53 =
                                    source_ml_source1_string_range_operand52;
                                {
                                    let source_ml_source1_string_range_position54 = f64::INFINITY;
                                    source_ml_source1_string_range_operand51.slice(
                                        source_ml_source1_string_range_position53,
                                        source_ml_source1_string_range_position54,
                                    )
                                }
                            }
                        }
                    };
                    ((((((ordinal != (self::ml_string::Text::from_units(vec![])))
                        && ((*method) != (self::ml_string::Text::from_units(vec![]))))
                        && helper.starts_with(
                            &((prefix.concat(&self::ml_string::Text::from_units(vec![
                                99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95,
                            ])))
                            .concat(&ordinal))
                            .concat(&self::ml_string::Text::from_units(vec![95])),
                        ))
                        && ((*parameter)
                            == ((prefix.concat(&self::ml_string::Text::from_units(vec![
                                99, 108, 97, 115, 115, 95, 114, 101, 99, 101, 105, 118, 101, 114,
                                95,
                            ])))
                            .concat(&ordinal))))
                        && self::ml_array::includes(
                            receiver_fields,
                            brand,
                            0f64,
                            false,
                            |a, b| a == b,
                        ))
                        && constructor)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_owner_row(row: &[self::ml_string::Text]) -> Vec<f64> {
    if (row.len() as f64) < 5f64 {
        vec![-1f64]
    } else {
        {
            let fields = self::ml_string_number::convert_java_script_string_number_units(
                &((self::ml_array::optional_at(row, self::ml_array::number_index(3f64)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                .0
                .iter()
                .copied()
                .map(f64::from)
                .collect::<Vec<f64>>(),
            );
            {
                let methods = self::ml_string_number::convert_java_script_string_number_units(
                    &((self::ml_array::optional_at(row, self::ml_array::number_index(4f64)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                    .0
                    .iter()
                    .copied()
                    .map(f64::from)
                    .collect::<Vec<f64>>(),
                );
                if (((((((!(self::ml_math::is_safe_integer(fields)) || (fields < 0f64))
                    || (fields > 8192f64))
                    || (({
                        let source_ml_source1_string_argument56 = fields;
                        self::ml_string::Text::from_text(&self::ml_number::js_number(
                            source_ml_source1_string_argument56,
                        ))
                    }) != ((self::ml_array::optional_at(
                        row,
                        self::ml_array::number_index(3f64),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))))
                    || !(self::ml_math::is_safe_integer(methods)))
                    || (methods < 0f64))
                    || (methods > 8192f64))
                    || (({
                        let source_ml_source1_string_argument58 = methods;
                        self::ml_string::Text::from_text(&self::ml_number::js_number(
                            source_ml_source1_string_argument58,
                        ))
                    }) != ((self::ml_array::optional_at(
                        row,
                        self::ml_array::number_index(4f64),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))))
                    || ((row.len() as f64) != ((5f64 + fields) + (2f64 * methods)))
                {
                    vec![-1f64]
                } else {
                    vec![0f64, 5f64, fields, (5f64 + fields), methods]
                }
            }
        }
    }
}
