// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryLowerJavaScriptClosedClassSourceJoin42;
use super::MlTemporaryLowerJavaScriptClosedClassSourceJoin43;
use super::MlTemporaryLowerJavaScriptClosedClassSourceLoop41Result;
use super::MlTemporaryLowerJavaScriptClosedClassSourceLoop44Result;
use super::MlTemporaryLowerJavaScriptClosedClassSourceLoop46Result;
use super::MlTemporaryProjectJavaScriptClassSourceIntervalLoop48Result;
use super::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop29Result;
use super::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Result;
use super::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop26Result;
use super::MlTemporaryRenderJavaScriptClosedClassLoop35Result;
use super::MlTemporaryRenderJavaScriptClosedClassLoop36Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_string;
use super::read_java_script_closed_class_prefix_end;
use super::read_java_script_closed_class_refusal;
use super::render_java_script_closed_class_range;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop40;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop41;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop44;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop45;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop46;
use super::source_ml_temporary_project_java_script_class_source_interval_loop48;
use super::source_ml_temporary_read_java_script_closed_class_observation_error_loop29;
use super::source_ml_temporary_read_java_script_closed_class_observation_error_loop30;
use super::source_ml_temporary_read_java_script_closed_class_static_calls_loop24;
use super::source_ml_temporary_read_java_script_closed_class_static_target_loop26;
use super::source_ml_temporary_render_java_script_closed_class_loop35;
use super::source_ml_temporary_render_java_script_closed_class_loop36;

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
                let source_ml_temporary_values27 = directory.to_vec();
                {
                    let source_ml_temporary_index27 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s27 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop29(values, names, methods, &(source_ml_temporary_values27), source_ml_temporary_index27);
                        match source_ml_source1_s27 {
                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop29Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop29Done(names_2, methods_2) => {
                                {
                                    let at = 0f64;
                                    {
                                        let source_ml_source1_s28 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop30(values, kinds, directory, static_calls, &(names_2), &(methods_2), at);
                                        match source_ml_source1_s28 {
                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Done => {
                                                -1f64
                                            }
                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop30Return(source_ml_temporary_result30) => {
                                                source_ml_temporary_result30
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop29Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop29Return(source_ml_temporary_result32) => {
                                source_ml_temporary_result32
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
                let source_ml_source1_string_argument29 = ordinal;
                self::ml_string::Text::from_text(&self::ml_number::js_number(
                    source_ml_source1_string_argument29,
                ))
            });
            {
                let brand = (prefix.concat(&self::ml_string::Text::from_units(vec![
                    99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95,
                ])))
                .concat(&{
                    let source_ml_source1_string_argument30 = ordinal;
                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                        source_ml_source1_string_argument30,
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
                                        let stubs = self::ml_string::Text::from_units(vec![]);
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
                                                        let fields = brand.concat(
                                                            &self::ml_string::Text::from_units(
                                                                vec![
                                                                    58, 32, 40, 118, 111, 105, 100,
                                                                    32, 48, 41,
                                                                ],
                                                            ),
                                                        );
                                                        {
                                                            let at = 9f64;
                                                            {
                                                                let source_ml_source1_s31 = self::source_ml_temporary_render_java_script_closed_class_loop35(source, values, starts, ends, row, prefix, ordinal, class_names, static_calls, methods, stubs, initializations, constructor_fields, field_names, fields, at);
                                                                match source_ml_source1_s31 {
                                                                    self::MlTemporaryRenderJavaScriptClosedClassLoop35Result::MlTemporaryRenderJavaScriptClosedClassLoop35Done(stubs_2, initializations_2, constructor_fields_2, fields_2) => {
                                                                        {
                                                                            let assignments = self::ml_string::Text::from_units(vec![]);
                                                                            {
                                                                                let at_2 = methods;
                                                                                {
                                                                                    let source_ml_source1_s32 = self::source_ml_temporary_render_java_script_closed_class_loop36(source, values, starts, ends, row, prefix, ordinal, class_names, static_calls, &(receiver), stubs_2.clone(), fields_2.clone(), assignments, at_2);
                                                                                    match source_ml_source1_s32 {
                                                                                        self::MlTemporaryRenderJavaScriptClosedClassLoop36Result::MlTemporaryRenderJavaScriptClosedClassLoop36Done(stubs_3, fields_3, assignments_2) => {
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
                                                                                                                                let source_ml_source1_string_range_operand33 = source.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source1_string_range_operand34 = (self::ml_array::optional_at(ends, self::ml_array::number_index(constructor_name - 1f64))).unwrap_or_else(|| 0f64);
                                                                                                                                    {
                                                                                                                                        let source_ml_source1_string_range_operand35 = (self::ml_array::optional_at(starts, self::ml_array::number_index(constructor_name))).unwrap_or_else(|| 0f64);
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_string_range_position36 = source_ml_source1_string_range_operand34;
                                                                                                                                            {
                                                                                                                                                let source_ml_source1_string_range_position37 = source_ml_source1_string_range_operand35;
                                                                                                                                                source_ml_source1_string_range_operand33.slice(source_ml_source1_string_range_position36, source_ml_source1_string_range_position37)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_closed_class_source(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    prefix: &self::ml_string::Text,
) -> (
    f64,
    Vec<self::ml_string::Text>,
    bool,
    self::ml_string::Text,
    Vec<f64>,
    self::ml_string::Text,
) {
    if ((!(self::accept_java_script_closed_class_source_size(
        source.length(),
        (values.len() as f64),
    )) || ((kinds.len() as f64) != (values.len() as f64)))
        || ((starts.len() as f64) != (values.len() as f64)))
        || ((ends.len() as f64) != (values.len() as f64))
    {
        {
            let source_ml_source1_field38 = source.clone();
            {
                let source_ml_source1_field39 = Vec::<f64>::new();
                {
                    let source_ml_source1_field40 = Vec::<self::ml_string::Text>::new();
                    {
                        let source_ml_source1_field41 =
                            self::read_java_script_closed_class_refusal(-8f64);
                        {
                            let source_ml_source1_field42 = 0f64;
                            {
                                let source_ml_source1_field43 = false;
                                (
                                    source_ml_source1_field42,
                                    source_ml_source1_field40,
                                    source_ml_source1_field43,
                                    source_ml_source1_field41,
                                    source_ml_source1_field39,
                                    source_ml_source1_field38,
                                )
                            }
                        }
                    }
                }
            }
        }
    } else {
        {
            let syntax = Vec::<self::ml_string::Text>::new();
            {
                let at = 0f64;
                {
                    let syntax_2 =
                        self::source_ml_temporary_lower_java_script_closed_class_source_loop40(
                            values, kinds, syntax, at,
                        );
                    {
                        let values_2 = syntax_2;
                        {
                            let directory = Vec::<Vec<f64>>::new();
                            {
                                let names = Vec::<self::ml_string::Text>::new();
                                {
                                    let depth = 0f64;
                                    {
                                        let declarations_finished = false;
                                        {
                                            let error = 0f64;
                                            {
                                                let error_at = 0f64;
                                                {
                                                    let error_3 = if (((((source.length())
                                                        > 131_072f64)
                                                        || ((values_2.len() as f64) > 8192f64))
                                                        || ((kinds.len() as f64)
                                                            != (values_2.len() as f64)))
                                                        || ((starts.len() as f64)
                                                            != (values_2.len() as f64)))
                                                        || ((ends.len() as f64)
                                                            != (values_2.len() as f64))
                                                    {
                                                        { -8f64 }
                                                    } else {
                                                        error
                                                    };
                                                    {
                                                        let at_2 = 0f64;
                                                        {
                                                            let source_ml_source1_s44 = self::source_ml_temporary_lower_java_script_closed_class_source_loop41(&(values_2), kinds, directory, names, depth, declarations_finished, error_3, error_at, at_2);
                                                            match source_ml_source1_s44 {
                                                                self::MlTemporaryLowerJavaScriptClosedClassSourceLoop41Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop41Done(directory_2, names_2, error_4, error_at_2) => {
                                                                    {
                                                                        let source_ml_source1_s46 = if (error_4 == 0f64) && ((directory_2.len() as f64) > 0f64) {
                                                                            {
                                                                                let at_3 = 0f64;
                                                                                {
                                                                                    let source_ml_source1_s47 = self::source_ml_temporary_lower_java_script_closed_class_source_loop46(source, &(values_2), kinds, starts, ends, &(names_2), error_4, error_at_2, at_3);
                                                                                    match source_ml_source1_s47 {
                                                                                        self::MlTemporaryLowerJavaScriptClosedClassSourceLoop46Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop46Done(error_5, error_at_3) => {
                                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin42::MlTemporaryLowerJavaScriptClosedClassSourceJoin42Next(error_5, error_at_3)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin42::MlTemporaryLowerJavaScriptClosedClassSourceJoin42Next(error_4, error_at_2)
                                                                        };
                                                                        match source_ml_source1_s46 {
                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin42::MlTemporaryLowerJavaScriptClosedClassSourceJoin42Next(error_6, error_at_4) => {
                                                                                {
                                                                                    let static_calls = self::read_java_script_closed_class_static_calls(&(values_2), &(directory_2), prefix);
                                                                                    {
                                                                                        let source_ml_source1_s48 = if (error_6 == 0f64) && ((directory_2.len() as f64) > 0f64) {
                                                                                            {
                                                                                                let observation = self::read_java_script_closed_class_observation_error(&(values_2), kinds, &(directory_2), &(static_calls));
                                                                                                if observation >= 0f64 {
                                                                                                    {
                                                                                                        let error_7 = -7f64;
                                                                                                        {
                                                                                                            let error_at_5 = observation;
                                                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin43::MlTemporaryLowerJavaScriptClosedClassSourceJoin43Next(error_7, error_at_5)
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    self::MlTemporaryLowerJavaScriptClosedClassSourceJoin43::MlTemporaryLowerJavaScriptClosedClassSourceJoin43Next(error_6, error_at_4)
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin43::MlTemporaryLowerJavaScriptClosedClassSourceJoin43Next(error_6, error_at_4)
                                                                                        };
                                                                                        match source_ml_source1_s48 {
                                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin43::MlTemporaryLowerJavaScriptClosedClassSourceJoin43Next(error_8, error_at_6) => {
                                                                                                {
let ml_library_inequality_1 = error_8 != 0f64 ;
if ml_library_inequality_1 {
                                                                                                    {
                                                                                                        let source_ml_source1_field49 = source.clone();
                                                                                                        {
                                                                                                            let source_ml_source1_field50 = Vec::<f64>::new();
                                                                                                            {
                                                                                                                let source_ml_source1_field51 = Vec::<self::ml_string::Text>::new();
                                                                                                                {
                                                                                                                    let source_ml_source1_field52 = self::read_java_script_closed_class_refusal(error_8);
                                                                                                                    {
                                                                                                                        let source_ml_source1_field53 = error_at_6;
                                                                                                                        {
                                                                                                                            let source_ml_source1_field54 = false;
                                                                                                                            (source_ml_source1_field53, source_ml_source1_field51, source_ml_source1_field54, source_ml_source1_field52, source_ml_source1_field50, source_ml_source1_field49)
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                } else if (directory_2.len() as f64) == 0f64 {
                                                                                                        {
                                                                                                            let source_ml_source1_field55 = source.clone();
                                                                                                            {
                                                                                                                let source_ml_source1_field56 = Vec::<f64>::new();
                                                                                                                {
                                                                                                                    let source_ml_source1_field57 = Vec::<self::ml_string::Text>::new();
                                                                                                                    {
                                                                                                                        let source_ml_source1_field58 = self::ml_string::Text::from_units(vec![]);
                                                                                                                        {
                                                                                                                            let source_ml_source1_field59 = 0f64;
                                                                                                                            {
                                                                                                                                let source_ml_source1_field60 = false;
                                                                                                                                (source_ml_source1_field59, source_ml_source1_field57, source_ml_source1_field60, source_ml_source1_field58, source_ml_source1_field56, source_ml_source1_field55)
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        {
                                                                                                            let text = self::ml_string::Text::from_units(vec![]);
                                                                                                            {
                                                                                                                let intervals = Vec::<f64>::new();
                                                                                                                {
                                                                                                                    let previous = 0f64;
                                                                                                                    {
                                                                                                                        let ordinal = 0f64;
                                                                                                                        {
                                                                                                                            let brands = Vec::<self::ml_string::Text>::new();
                                                                                                                            {
                                                                                                                                let source_ml_temporary_values39 = directory_2;
                                                                                                                                {
                                                                                                                                    let source_ml_temporary_index39 = self::ml::Big::from_i128(0);
                                                                                                                                    {
                                                                                                                                        let source_ml_source1_s61 = self::source_ml_temporary_lower_java_script_closed_class_source_loop44(source, &(values_2), starts, ends, prefix, &(names_2), &(static_calls), text, intervals, previous, ordinal, brands, &(source_ml_temporary_values39), source_ml_temporary_index39);
                                                                                                                                        match source_ml_source1_s61 {
                                                                                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceLoop44Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop44Done(text_2, intervals_2, previous_2, brands_2) => {
                                                                                                                                                {
                                                                                                                                                    let first = 0f64;
                                                                                                                                                    {
                                                                                                                                                        let first_2 = self::source_ml_temporary_lower_java_script_closed_class_source_loop45(&(values_2), starts, previous_2, first);
                                                                                                                                                        {
                                                                                                                                                            let rest = if first_2 >= (values_2.len() as f64) {
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_string_range_operand62 = source.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_string_range_operand63 = previous_2;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_string_range_position64 = source_ml_source1_string_range_operand63;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_string_range_position65 = f64::INFINITY;
                                                                                                                                                                                source_ml_source1_string_range_operand62.slice(source_ml_source1_string_range_position64, source_ml_source1_string_range_position65)
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            } else {
                                                                                                                                                                (({
                                                                                                                                                                    let source_ml_source1_string_range_operand66 = source.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_string_range_operand67 = previous_2;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_string_range_operand68 = (self::ml_array::optional_at(starts, self::ml_array::number_index(first_2))).unwrap_or_else(|| previous_2);
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_string_range_position69 = source_ml_source1_string_range_operand67;
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source1_string_range_position70 = source_ml_source1_string_range_operand68;
                                                                                                                                                                                    source_ml_source1_string_range_operand66.slice(source_ml_source1_string_range_position69, source_ml_source1_string_range_position70)
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }).concat(&self::render_java_script_closed_class_range(source, &(values_2), starts, ends, first_2, (values_2.len() as f64), &(self::ml_string::Text::from_units(vec![])), &(names_2), &(static_calls)))).concat(&{
                                                                                                                                                                    let source_ml_source1_string_range_operand71 = source.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_string_range_operand72 = (self::ml_array::optional_at(ends, self::ml_array::number_index((values_2.len() as f64) - 1f64))).unwrap_or_else(|| source.length());
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_string_range_position73 = source_ml_source1_string_range_operand72;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_string_range_position74 = f64::INFINITY;
                                                                                                                                                                                source_ml_source1_string_range_operand71.slice(source_ml_source1_string_range_position73, source_ml_source1_string_range_position74)
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                })
                                                                                                                                                            };
                                                                                                                                                            {
                                                                                                                                                                let intervals_3 = {
                                                                                                                                                                    let source_ml_source1_concat_value75 = intervals_2;
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_concat_value76 = vec![text_2.length(), (text_2.length() + rest.length()), previous_2, source.length()];
                                                                                                                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value75), source_ml_source1_concat_value76)
                                                                                                                                                                    }
                                                                                                                                                                };
                                                                                                                                                                {
                                                                                                                                                                    let text_3 = text_2.concat(&rest);
                                                                                                                                                                    if (text_3.length()) > 1_048_576f64 {
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_field77 = source.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_field78 = Vec::<f64>::new();
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source1_field79 = Vec::<self::ml_string::Text>::new();
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_source1_field80 = self::read_java_script_closed_class_refusal(-8f64);
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source1_field81 = 0f64;
                                                                                                                                                                                            {
                                                                                                                                                                                                let source_ml_source1_field82 = false;
                                                                                                                                                                                                (source_ml_source1_field81, source_ml_source1_field79, source_ml_source1_field82, source_ml_source1_field80, source_ml_source1_field78, source_ml_source1_field77)
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    } else {
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_field83 = text_3.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_field84 = intervals_3;
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source1_field85 = brands_2;
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_source1_field86 = self::ml_string::Text::from_units(vec![]);
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source1_field87 = 0f64;
                                                                                                                                                                                            {
                                                                                                                                                                                                let source_ml_source1_field88 = true;
                                                                                                                                                                                                (source_ml_source1_field87, source_ml_source1_field85, source_ml_source1_field88, source_ml_source1_field86, source_ml_source1_field84, source_ml_source1_field83)
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
pub fn accept_java_script_closed_class_source_size(units: f64, tokens: f64) -> bool {
    ((((self::ml_math::is_safe_integer(units) && self::ml_math::is_safe_integer(tokens))
        && (units >= 0f64))
        && (units <= 131_072f64))
        && (tokens >= 0f64))
        && (tokens <= 8192f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_context_error(
    shared: bool,
    externals: f64,
    constants: f64,
    bindings: f64,
    known: bool,
    module: bool,
) -> self::ml_string::Text {
    if ((((shared && (externals == 0f64)) && (constants == 0f64)) && (bindings == 0f64)) && known)
        && !module
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            116, 104, 101, 32, 99, 108, 111, 115, 101, 100, 32, 112, 114, 111, 102, 105, 108, 101,
            32, 110, 101, 101, 100, 115, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 102, 101,
            114, 101, 110, 99, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115, 32, 97, 110,
            100, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 108, 111, 99,
            97, 108, 32, 109, 111, 100, 117, 108, 101, 32, 114, 97, 116, 104, 101, 114, 32, 116,
            104, 97, 110, 32, 117, 110, 112, 114, 111, 118, 101, 100, 32, 101, 120, 116, 101, 114,
            110, 97, 108, 32, 99, 111, 110, 115, 117, 109, 101, 114, 115,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_use_error(
    kind: &self::ml_string::Text,
    operation: &self::ml_string::Text,
    instance: bool,
    private_field: bool,
    callable: bool,
    direct: bool,
) -> self::ml_string::Text {
    if instance {
        if (((*kind)
            == (self::ml_string::Text::from_units(vec![
                114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100,
            ])))
            && !private_field)
            && (!callable || direct)
        {
            self::ml_string::Text::from_units(vec![])
        } else if ((*kind)
            == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])))
            && operation.starts_with(&self::ml_string::Text::from_units(vec![
                119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100,
            ]))
        {
            self::ml_string::Text::from_units(vec![])
        } else {
            self::ml_string::Text::from_units(vec![
                99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 105, 110, 115, 116, 97,
                110, 99, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 114, 101,
                99, 116, 32, 112, 117, 98, 108, 105, 99, 32, 102, 105, 101, 108, 100, 32, 111, 112,
                101, 114, 97, 116, 105, 111, 110, 115, 32, 97, 110, 100, 32, 114, 101, 99, 101,
                105, 118, 101, 114, 45, 114, 101, 116, 97, 105, 110, 105, 110, 103, 32, 109, 101,
                116, 104, 111, 100, 32, 99, 97, 108, 108, 115, 59, 32, 112, 114, 105, 118, 97, 116,
                101, 32, 115, 108, 111, 116, 115, 44, 32, 100, 101, 116, 97, 99, 104, 101, 100, 32,
                99, 97, 108, 108, 97, 98, 108, 101, 115, 32, 97, 110, 100, 32, 105, 110, 100, 105,
                114, 101, 99, 116, 32, 114, 101, 102, 108, 101, 99, 116, 105, 118, 101, 32, 108,
                111, 111, 107, 117, 112, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 32, 102,
                117, 108, 108, 32, 111, 98, 106, 101, 99, 116, 32, 109, 111, 100, 101, 108,
            ])
        }
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn project_java_script_class_source_interval(
    intervals: &[f64],
    first: f64,
    last: f64,
) -> Vec<f64> {
    {
        let low = -1f64;
        {
            let high = -1f64;
            {
                let at = 0f64;
                {
                    let source_ml_source1_s89 =
                        self::source_ml_temporary_project_java_script_class_source_interval_loop48(
                            intervals, first, last, low, high, at,
                        );
                    match source_ml_source1_s89 {
                        self::MlTemporaryProjectJavaScriptClassSourceIntervalLoop48Result::MlTemporaryProjectJavaScriptClassSourceIntervalLoop48Done(low_2, high_2) => {
                            if low_2 < 0f64 {
                                Vec::<f64>::new()
                            } else {
                                vec![low_2, high_2]
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptClassGroupEndLoop1Result {
    MlTemporaryFindJavaScriptClassGroupEndLoop1Done,
    MlTemporaryFindJavaScriptClassGroupEndLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_class_group_end_loop1(
    values: &[self::ml_string::Text],
    mut open: f64,
    mut expected: Vec<self::ml_string::Text>,
    mut at: f64,
) -> self::MlTemporaryFindJavaScriptClassGroupEndLoop1Result {
    loop {
        return if (at < (values.len() as f64)) && ((at - open) <= 8192f64) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let closing = if value == (self::ml_string::Text::from_units(vec![40])) {
                        self::ml_string::Text::from_units(vec![41])
                    } else if value == (self::ml_string::Text::from_units(vec![91])) {
                        self::ml_string::Text::from_units(vec![93])
                    } else if value == (self::ml_string::Text::from_units(vec![123])) {
                        self::ml_string::Text::from_units(vec![125])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    };
                    {
                        let ml_library_inequality_2 =
                            closing != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_2 {
                            if (expected.len() as f64) >= 128f64 {
                                self::MlTemporaryFindJavaScriptClassGroupEndLoop1Result::MlTemporaryFindJavaScriptClassGroupEndLoop1Return(-1f64)
                            } else {
                                {
                                    let expected_2 = {
                                        let source_ml_source1_concat_value90 = expected;
                                        {
                                            let source_ml_source1_concat_value91 =
                                                vec![closing.clone()];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::ml_string::Text>::new(),
                                                    source_ml_source1_concat_value90,
                                                ),
                                                source_ml_source1_concat_value91,
                                            )
                                        }
                                    };
                                    {
                                        let at_2 = at + 1f64;
                                        {
                                            (expected, at) = (expected_2, at_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else if ((value == (self::ml_string::Text::from_units(vec![41])))
                            || (value == (self::ml_string::Text::from_units(vec![93]))))
                            || (value == (self::ml_string::Text::from_units(vec![125])))
                        {
                            {
                                let ml_library_inequality_1 = value
                                    != ((self::ml_array::optional_at(
                                        &expected,
                                        self::ml_array::number_index(
                                            (expected.len() as f64) - 1f64,
                                        ),
                                    ))
                                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])));
                                if ml_library_inequality_1 {
                                    self::MlTemporaryFindJavaScriptClassGroupEndLoop1Result::MlTemporaryFindJavaScriptClassGroupEndLoop1Return(-1f64)
                                } else {
                                    {
                                        let expected_3 =
                                            self::ml_array::slice(&expected, 0f64, -1f64);
                                        if (expected_3.len() as f64) == 0f64 {
                                            self::MlTemporaryFindJavaScriptClassGroupEndLoop1Result::MlTemporaryFindJavaScriptClassGroupEndLoop1Return(at)
                                        } else {
                                            {
                                                let at_3 = at + 1f64;
                                                {
                                                    (expected, at) = (expected_3, at_3);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let at_4 = at + 1f64;
                                {
                                    at = at_4;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptClassGroupEndLoop1Result::MlTemporaryFindJavaScriptClassGroupEndLoop1Done
        };
    }
}
