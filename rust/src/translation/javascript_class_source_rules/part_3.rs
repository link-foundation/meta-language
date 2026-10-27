// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryLowerJavaScriptClosedClassSourceJoin32;
use super::MlTemporaryLowerJavaScriptClosedClassSourceJoin33;
use super::MlTemporaryLowerJavaScriptClosedClassSourceLoop31Result;
use super::MlTemporaryLowerJavaScriptClosedClassSourceLoop34Result;
use super::MlTemporaryLowerJavaScriptClosedClassSourceLoop36Result;
use super::MlTemporaryProjectJavaScriptClassSourceIntervalLoop38Result;
use super::MlTemporaryRenderJavaScriptClosedClassLoop28Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_string;
use super::read_java_script_closed_class_observation_error;
use super::read_java_script_closed_class_refusal;
use super::read_java_script_closed_class_static_calls;
use super::render_java_script_closed_class_range;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop30;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop31;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop34;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop35;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop36;
use super::source_ml_temporary_project_java_script_class_source_interval_loop38;
use super::source_ml_temporary_render_java_script_closed_class_loop27;
use super::source_ml_temporary_render_java_script_closed_class_loop28;

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
                let source_ml_source1_string_argument17 = ordinal;
                self::ml_string::Text::from_text(&self::ml_number::js_number(
                    source_ml_source1_string_argument17,
                ))
            });
            {
                let brand = (prefix.concat(&self::ml_string::Text::from_units(vec![
                    99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95,
                ])))
                .concat(&{
                    let source_ml_source1_string_argument18 = ordinal;
                    self::ml_string::Text::from_text(&self::ml_number::js_number(
                        source_ml_source1_string_argument18,
                    ))
                });
                {
                    let field_count =
                        (self::ml_array::optional_at(row, self::ml_array::number_index(7f64)))
                            .unwrap_or_else(|| 0f64);
                    {
                        let methods = 8f64 + (field_count * 3f64);
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
                                            let fields = brand.concat(
                                                &self::ml_string::Text::from_units(vec![
                                                    58, 32, 40, 118, 111, 105, 100, 32, 48, 41,
                                                ]),
                                            );
                                            {
                                                let at = 8f64;
                                                {
                                                    let fields_2 = self::source_ml_temporary_render_java_script_closed_class_loop27(source, values, starts, ends, row, class_names, static_calls, methods, fields, at);
                                                    {
                                                        let assignments =
                                                            self::ml_string::Text::from_units(
                                                                vec![],
                                                            );
                                                        {
                                                            let at_2 = methods;
                                                            {
                                                                let source_ml_source1_s19 = self::source_ml_temporary_render_java_script_closed_class_loop28(source, values, starts, ends, row, prefix, ordinal, class_names, static_calls, &(receiver), stubs, fields_2, assignments, at_2);
                                                                match source_ml_source1_s19 {
                                                                    self::MlTemporaryRenderJavaScriptClosedClassLoop28Result::MlTemporaryRenderJavaScriptClosedClassLoop28Done(stubs_2, fields_3, assignments_2) => {
                                                                        {
                                                                            let constructor_body = (self::ml_array::optional_at(row, self::ml_array::number_index(5f64))).unwrap_or_else(|| -1f64);
                                                                            {
                                                                                let constructor_end = (self::ml_array::optional_at(row, self::ml_array::number_index(6f64))).unwrap_or_else(|| -1f64);
                                                                                {
                                                                                    let remainder = if constructor_body < 0f64 {
                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                    } else {
                                                                                        self::render_java_script_closed_class_range(source, values, starts, ends, if field_count == 0f64 {
                                                                                            constructor_body + 1f64
                                                                                        } else {
                                                                                            (self::ml_array::optional_at(row, self::ml_array::number_index(methods - 1f64))).unwrap_or_else(|| 0f64) + 1f64
                                                                                        }, constructor_end, &(receiver), class_names, static_calls)
                                                                                    };
                                                                                    {
                                                                                        let constructor_name = (self::ml_array::optional_at(row, self::ml_array::number_index(2f64))).unwrap_or_else(|| -1f64);
                                                                                        {
                                                                                            let documentation = if constructor_name < 0f64 {
                                                                                                self::ml_string::Text::from_units(vec![])
                                                                                            } else {
                                                                                                {
                                                                                                    let source_ml_source1_string_range_operand20 = source.clone();
                                                                                                    {
                                                                                                        let source_ml_source1_string_range_operand21 = (self::ml_array::optional_at(ends, self::ml_array::number_index(constructor_name - 1f64))).unwrap_or_else(|| 0f64);
                                                                                                        {
                                                                                                            let source_ml_source1_string_range_operand22 = (self::ml_array::optional_at(starts, self::ml_array::number_index(constructor_name))).unwrap_or_else(|| 0f64);
                                                                                                            {
                                                                                                                let source_ml_source1_string_range_position23 = source_ml_source1_string_range_operand21;
                                                                                                                {
                                                                                                                    let source_ml_source1_string_range_position24 = source_ml_source1_string_range_operand22;
                                                                                                                    source_ml_source1_string_range_operand20.slice(source_ml_source1_string_range_position23, source_ml_source1_string_range_position24)
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            };
                                                                                            (((((((((((((((stubs_2.concat(&self::ml_string::Text::from_units(vec![10]))).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(&name)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&parameters)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 10, 99, 111, 110, 115, 116, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 123, 32]))).concat(&fields_3)).concat(&self::ml_string::Text::from_units(vec![32, 125, 59, 10]))).concat(&assignments_2)).concat(&remainder)).concat(&self::ml_string::Text::from_units(vec![10, 114, 101, 116, 117, 114, 110, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![59, 10, 125, 10]))
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
            let source_ml_source1_field25 = source.clone();
            {
                let source_ml_source1_field26 = Vec::<f64>::new();
                {
                    let source_ml_source1_field27 = Vec::<self::ml_string::Text>::new();
                    {
                        let source_ml_source1_field28 =
                            self::read_java_script_closed_class_refusal(-8f64);
                        {
                            let source_ml_source1_field29 = 0f64;
                            {
                                let source_ml_source1_field30 = false;
                                (
                                    source_ml_source1_field29,
                                    source_ml_source1_field27,
                                    source_ml_source1_field30,
                                    source_ml_source1_field28,
                                    source_ml_source1_field26,
                                    source_ml_source1_field25,
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
                        self::source_ml_temporary_lower_java_script_closed_class_source_loop30(
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
                                                            let source_ml_source1_s31 = self::source_ml_temporary_lower_java_script_closed_class_source_loop31(&(values_2), kinds, directory, names, depth, declarations_finished, error_3, error_at, at_2);
                                                            match source_ml_source1_s31 {
                                                                self::MlTemporaryLowerJavaScriptClosedClassSourceLoop31Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop31Done(directory_2, names_2, error_4, error_at_2) => {
                                                                    {
                                                                        let source_ml_source1_s33 = if (error_4 == 0f64) && ((directory_2.len() as f64) > 0f64) {
                                                                            {
                                                                                let at_3 = 0f64;
                                                                                {
                                                                                    let source_ml_source1_s34 = self::source_ml_temporary_lower_java_script_closed_class_source_loop36(source, &(values_2), kinds, starts, ends, &(names_2), error_4, error_at_2, at_3);
                                                                                    match source_ml_source1_s34 {
                                                                                        self::MlTemporaryLowerJavaScriptClosedClassSourceLoop36Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop36Done(error_5, error_at_3) => {
                                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin32::MlTemporaryLowerJavaScriptClosedClassSourceJoin32Next(error_5, error_at_3)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin32::MlTemporaryLowerJavaScriptClosedClassSourceJoin32Next(error_4, error_at_2)
                                                                        };
                                                                        match source_ml_source1_s33 {
                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin32::MlTemporaryLowerJavaScriptClosedClassSourceJoin32Next(error_6, error_at_4) => {
                                                                                {
                                                                                    let static_calls = self::read_java_script_closed_class_static_calls(&(values_2), &(directory_2), prefix);
                                                                                    {
                                                                                        let source_ml_source1_s35 = if (error_6 == 0f64) && ((directory_2.len() as f64) > 0f64) {
                                                                                            {
                                                                                                let observation = self::read_java_script_closed_class_observation_error(&(values_2), kinds, &(directory_2), &(static_calls));
                                                                                                if observation >= 0f64 {
                                                                                                    {
                                                                                                        let error_7 = -7f64;
                                                                                                        {
                                                                                                            let error_at_5 = observation;
                                                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin33::MlTemporaryLowerJavaScriptClosedClassSourceJoin33Next(error_7, error_at_5)
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    self::MlTemporaryLowerJavaScriptClosedClassSourceJoin33::MlTemporaryLowerJavaScriptClosedClassSourceJoin33Next(error_6, error_at_4)
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin33::MlTemporaryLowerJavaScriptClosedClassSourceJoin33Next(error_6, error_at_4)
                                                                                        };
                                                                                        match source_ml_source1_s35 {
                                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin33::MlTemporaryLowerJavaScriptClosedClassSourceJoin33Next(error_8, error_at_6) => {
                                                                                                {
let ml_library_inequality_1 = error_8 != 0f64 ;
if ml_library_inequality_1 {
                                                                                                    {
                                                                                                        let source_ml_source1_field36 = source.clone();
                                                                                                        {
                                                                                                            let source_ml_source1_field37 = Vec::<f64>::new();
                                                                                                            {
                                                                                                                let source_ml_source1_field38 = Vec::<self::ml_string::Text>::new();
                                                                                                                {
                                                                                                                    let source_ml_source1_field39 = self::read_java_script_closed_class_refusal(error_8);
                                                                                                                    {
                                                                                                                        let source_ml_source1_field40 = error_at_6;
                                                                                                                        {
                                                                                                                            let source_ml_source1_field41 = false;
                                                                                                                            (source_ml_source1_field40, source_ml_source1_field38, source_ml_source1_field41, source_ml_source1_field39, source_ml_source1_field37, source_ml_source1_field36)
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                } else if (directory_2.len() as f64) == 0f64 {
                                                                                                        {
                                                                                                            let source_ml_source1_field42 = source.clone();
                                                                                                            {
                                                                                                                let source_ml_source1_field43 = Vec::<f64>::new();
                                                                                                                {
                                                                                                                    let source_ml_source1_field44 = Vec::<self::ml_string::Text>::new();
                                                                                                                    {
                                                                                                                        let source_ml_source1_field45 = self::ml_string::Text::from_units(vec![]);
                                                                                                                        {
                                                                                                                            let source_ml_source1_field46 = 0f64;
                                                                                                                            {
                                                                                                                                let source_ml_source1_field47 = false;
                                                                                                                                (source_ml_source1_field46, source_ml_source1_field44, source_ml_source1_field47, source_ml_source1_field45, source_ml_source1_field43, source_ml_source1_field42)
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
                                                                                                                                let source_ml_temporary_values29 = directory_2;
                                                                                                                                {
                                                                                                                                    let source_ml_temporary_index29 = self::ml::Big::from_i128(0);
                                                                                                                                    {
                                                                                                                                        let source_ml_source1_s48 = self::source_ml_temporary_lower_java_script_closed_class_source_loop34(source, &(values_2), starts, ends, prefix, &(names_2), &(static_calls), text, intervals, previous, ordinal, brands, &(source_ml_temporary_values29), source_ml_temporary_index29);
                                                                                                                                        match source_ml_source1_s48 {
                                                                                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceLoop34Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop34Done(text_2, intervals_2, previous_2, brands_2) => {
                                                                                                                                                {
                                                                                                                                                    let first = 0f64;
                                                                                                                                                    {
                                                                                                                                                        let first_2 = self::source_ml_temporary_lower_java_script_closed_class_source_loop35(&(values_2), starts, previous_2, first);
                                                                                                                                                        {
                                                                                                                                                            let rest = if first_2 >= (values_2.len() as f64) {
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_string_range_operand49 = source.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_string_range_operand50 = previous_2;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_string_range_position51 = source_ml_source1_string_range_operand50;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_string_range_position52 = f64::INFINITY;
                                                                                                                                                                                source_ml_source1_string_range_operand49.slice(source_ml_source1_string_range_position51, source_ml_source1_string_range_position52)
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            } else {
                                                                                                                                                                (({
                                                                                                                                                                    let source_ml_source1_string_range_operand53 = source.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_string_range_operand54 = previous_2;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_string_range_operand55 = (self::ml_array::optional_at(starts, self::ml_array::number_index(first_2))).unwrap_or_else(|| previous_2);
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_string_range_position56 = source_ml_source1_string_range_operand54;
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source1_string_range_position57 = source_ml_source1_string_range_operand55;
                                                                                                                                                                                    source_ml_source1_string_range_operand53.slice(source_ml_source1_string_range_position56, source_ml_source1_string_range_position57)
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }).concat(&self::render_java_script_closed_class_range(source, &(values_2), starts, ends, first_2, (values_2.len() as f64), &(self::ml_string::Text::from_units(vec![])), &(names_2), &(static_calls)))).concat(&{
                                                                                                                                                                    let source_ml_source1_string_range_operand58 = source.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_string_range_operand59 = (self::ml_array::optional_at(ends, self::ml_array::number_index((values_2.len() as f64) - 1f64))).unwrap_or_else(|| source.length());
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_string_range_position60 = source_ml_source1_string_range_operand59;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_string_range_position61 = f64::INFINITY;
                                                                                                                                                                                source_ml_source1_string_range_operand58.slice(source_ml_source1_string_range_position60, source_ml_source1_string_range_position61)
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                })
                                                                                                                                                            };
                                                                                                                                                            {
                                                                                                                                                                let intervals_3 = {
                                                                                                                                                                    let source_ml_source1_concat_value62 = intervals_2;
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_concat_value63 = vec![text_2.length(), (text_2.length() + rest.length()), previous_2, source.length()];
                                                                                                                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value62), source_ml_source1_concat_value63)
                                                                                                                                                                    }
                                                                                                                                                                };
                                                                                                                                                                {
                                                                                                                                                                    let text_3 = text_2.concat(&rest);
                                                                                                                                                                    if (text_3.length()) > 1_048_576f64 {
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_field64 = source.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_field65 = Vec::<f64>::new();
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source1_field66 = Vec::<self::ml_string::Text>::new();
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_source1_field67 = self::read_java_script_closed_class_refusal(-8f64);
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source1_field68 = 0f64;
                                                                                                                                                                                            {
                                                                                                                                                                                                let source_ml_source1_field69 = false;
                                                                                                                                                                                                (source_ml_source1_field68, source_ml_source1_field66, source_ml_source1_field69, source_ml_source1_field67, source_ml_source1_field65, source_ml_source1_field64)
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    } else {
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_field70 = text_3.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_field71 = intervals_3;
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source1_field72 = brands_2;
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_source1_field73 = self::ml_string::Text::from_units(vec![]);
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source1_field74 = 0f64;
                                                                                                                                                                                            {
                                                                                                                                                                                                let source_ml_source1_field75 = true;
                                                                                                                                                                                                (source_ml_source1_field74, source_ml_source1_field72, source_ml_source1_field75, source_ml_source1_field73, source_ml_source1_field71, source_ml_source1_field70)
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
                    let source_ml_source1_s76 =
                        self::source_ml_temporary_project_java_script_class_source_interval_loop38(
                            intervals, first, last, low, high, at,
                        );
                    match source_ml_source1_s76 {
                        self::MlTemporaryProjectJavaScriptClassSourceIntervalLoop38Result::MlTemporaryProjectJavaScriptClassSourceIntervalLoop38Done(low_2, high_2) => {
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
                                        let source_ml_source1_concat_value77 = expected;
                                        {
                                            let source_ml_source1_concat_value78 =
                                                vec![closing.clone()];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::ml_string::Text>::new(),
                                                    source_ml_source1_concat_value77,
                                                ),
                                                source_ml_source1_concat_value78,
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Result {
    MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Done(Vec<f64>),
    MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3 {
    MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3Next(f64),
    MlTemporaryReadJavaScriptClosedClassParameterPositionsJoin3Return(Vec<f64>),
}
