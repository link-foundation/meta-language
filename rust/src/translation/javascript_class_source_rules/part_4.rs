// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryAcceptJavaScriptClosedClassOwnFieldLoop71Result;
use super::MlTemporaryLowerJavaScriptClosedClassSourceJoin60;
use super::MlTemporaryLowerJavaScriptClosedClassSourceJoin61;
use super::MlTemporaryLowerJavaScriptClosedClassSourceJoin62;
use super::MlTemporaryLowerJavaScriptClosedClassSourceLoop59Result;
use super::MlTemporaryLowerJavaScriptClosedClassSourceLoop63Result;
use super::MlTemporaryLowerJavaScriptClosedClassSourceLoop69Result;
use super::MlTemporaryProjectJavaScriptClassSourceIntervalLoop72Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_closed_class_collection_default_class_owner_error;
use super::read_java_script_closed_class_observation_error;
use super::read_java_script_closed_class_refusal;
use super::read_java_script_closed_class_static_calls;
use super::render_java_script_closed_class_range;
use super::require_java_script_closed_class_prototype_methods;
use super::source_ml_temporary_accept_java_script_closed_class_own_field_loop71;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop58;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop59;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop63;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop64;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop69;
use super::source_ml_temporary_project_java_script_class_source_interval_loop72;

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
    Vec<Vec<self::ml_string::Text>>,
    Vec<self::ml_string::Text>,
    Vec<self::ml_string::Text>,
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
            let source_ml_source1_field70 = source.clone();
            {
                let source_ml_source1_field71 = Vec::<f64>::new();
                {
                    let source_ml_source1_field72 = Vec::<self::ml_string::Text>::new();
                    {
                        let source_ml_source1_field73 = Vec::<self::ml_string::Text>::new();
                        {
                            let source_ml_source1_field74 = Vec::<self::ml_string::Text>::new();
                            {
                                let source_ml_source1_field75 =
                                    Vec::<Vec<self::ml_string::Text>>::new();
                                {
                                    let source_ml_source1_field76 =
                                        self::read_java_script_closed_class_refusal(-8f64);
                                    {
                                        let source_ml_source1_field77 = 0f64;
                                        {
                                            let source_ml_source1_field78 = false;
                                            (
                                                source_ml_source1_field77,
                                                source_ml_source1_field72,
                                                source_ml_source1_field78,
                                                source_ml_source1_field76,
                                                source_ml_source1_field71,
                                                source_ml_source1_field75,
                                                source_ml_source1_field74,
                                                source_ml_source1_field73,
                                                source_ml_source1_field70,
                                            )
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
        {
            let syntax = Vec::<self::ml_string::Text>::new();
            {
                let at = 0f64;
                {
                    let syntax_2 =
                        self::source_ml_temporary_lower_java_script_closed_class_source_loop58(
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
                                                            let source_ml_source1_s79 = self::source_ml_temporary_lower_java_script_closed_class_source_loop59(&(values_2), kinds, directory, names, depth, declarations_finished, error_3, error_at, at_2);
                                                            match source_ml_source1_s79 {
                                                                self::MlTemporaryLowerJavaScriptClosedClassSourceLoop59Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop59Done(directory_2, names_2, error_4, error_at_2) => {
                                                                    {
                                                                        let source_ml_source1_s81 = if (error_4 == 0f64) && ((directory_2.len() as f64) > 0f64) {
                                                                            {
                                                                                let at_3 = 0f64;
                                                                                {
                                                                                    let source_ml_source1_s82 = self::source_ml_temporary_lower_java_script_closed_class_source_loop69(source, &(values_2), kinds, starts, ends, &(names_2), error_4, error_at_2, at_3);
                                                                                    match source_ml_source1_s82 {
                                                                                        self::MlTemporaryLowerJavaScriptClosedClassSourceLoop69Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop69Done(error_5, error_at_3) => {
                                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin60::MlTemporaryLowerJavaScriptClosedClassSourceJoin60Next(error_5, error_at_3)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin60::MlTemporaryLowerJavaScriptClosedClassSourceJoin60Next(error_4, error_at_2)
                                                                        };
                                                                        match source_ml_source1_s81 {
                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin60::MlTemporaryLowerJavaScriptClosedClassSourceJoin60Next(error_6, error_at_4) => {
                                                                                {
                                                                                    let static_calls = self::read_java_script_closed_class_static_calls(&(values_2), &(directory_2), prefix);
                                                                                    {
                                                                                        let source_ml_source1_s83 = if (error_6 == 0f64) && ((directory_2.len() as f64) > 0f64) {
                                                                                            {
                                                                                                let owner = self::read_java_script_closed_class_collection_default_class_owner_error(&(values_2), &(directory_2));
                                                                                                if owner >= 0f64 {
                                                                                                    {
                                                                                                        let error_7 = -11f64;
                                                                                                        {
                                                                                                            let error_at_5 = owner;
                                                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin61::MlTemporaryLowerJavaScriptClosedClassSourceJoin61Next(error_7, error_at_5)
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    self::MlTemporaryLowerJavaScriptClosedClassSourceJoin61::MlTemporaryLowerJavaScriptClosedClassSourceJoin61Next(error_6, error_at_4)
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin61::MlTemporaryLowerJavaScriptClosedClassSourceJoin61Next(error_6, error_at_4)
                                                                                        };
                                                                                        match source_ml_source1_s83 {
                                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin61::MlTemporaryLowerJavaScriptClosedClassSourceJoin61Next(error_8, error_at_6) => {
                                                                                                {
                                                                                                    let source_ml_source1_s84 = if (error_8 == 0f64) && ((directory_2.len() as f64) > 0f64) {
                                                                                                        {
                                                                                                            let observation = self::read_java_script_closed_class_observation_error(&(values_2), kinds, &(directory_2), &(static_calls));
                                                                                                            if observation >= 0f64 {
                                                                                                                {
                                                                                                                    let error_9 = -7f64;
                                                                                                                    {
                                                                                                                        let error_at_7 = observation;
                                                                                                                        self::MlTemporaryLowerJavaScriptClosedClassSourceJoin62::MlTemporaryLowerJavaScriptClosedClassSourceJoin62Next(error_9, error_at_7)
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                self::MlTemporaryLowerJavaScriptClosedClassSourceJoin62::MlTemporaryLowerJavaScriptClosedClassSourceJoin62Next(error_8, error_at_6)
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        self::MlTemporaryLowerJavaScriptClosedClassSourceJoin62::MlTemporaryLowerJavaScriptClosedClassSourceJoin62Next(error_8, error_at_6)
                                                                                                    };
                                                                                                    match source_ml_source1_s84 {
                                                                                                        self::MlTemporaryLowerJavaScriptClosedClassSourceJoin62::MlTemporaryLowerJavaScriptClosedClassSourceJoin62Next(error_10, error_at_8) => {
                                                                                                            {
let ml_library_inequality_1 = error_10 != 0f64 ;
if ml_library_inequality_1 {
                                                                                                                {
                                                                                                                    let source_ml_source1_field85 = source.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source1_field86 = Vec::<f64>::new();
                                                                                                                        {
                                                                                                                            let source_ml_source1_field87 = Vec::<self::ml_string::Text>::new();
                                                                                                                            {
                                                                                                                                let source_ml_source1_field88 = Vec::<self::ml_string::Text>::new();
                                                                                                                                {
                                                                                                                                    let source_ml_source1_field89 = Vec::<self::ml_string::Text>::new();
                                                                                                                                    {
                                                                                                                                        let source_ml_source1_field90 = Vec::<Vec<self::ml_string::Text>>::new();
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_field91 = self::read_java_script_closed_class_refusal(error_10);
                                                                                                                                            {
                                                                                                                                                let source_ml_source1_field92 = error_at_8;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source1_field93 = false;
                                                                                                                                                    (source_ml_source1_field92, source_ml_source1_field87, source_ml_source1_field93, source_ml_source1_field91, source_ml_source1_field86, source_ml_source1_field90, source_ml_source1_field89, source_ml_source1_field88, source_ml_source1_field85)
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } else if (directory_2.len() as f64) == 0f64 {
                                                                                                                    {
                                                                                                                        let source_ml_source1_field94 = source.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source1_field95 = Vec::<f64>::new();
                                                                                                                            {
                                                                                                                                let source_ml_source1_field96 = Vec::<self::ml_string::Text>::new();
                                                                                                                                {
                                                                                                                                    let source_ml_source1_field97 = Vec::<self::ml_string::Text>::new();
                                                                                                                                    {
                                                                                                                                        let source_ml_source1_field98 = Vec::<self::ml_string::Text>::new();
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_field99 = Vec::<Vec<self::ml_string::Text>>::new();
                                                                                                                                            {
                                                                                                                                                let source_ml_source1_field100 = self::ml_string::Text::from_units(vec![]);
                                                                                                                                                {
                                                                                                                                                    let source_ml_source1_field101 = 0f64;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source1_field102 = false;
                                                                                                                                                        (source_ml_source1_field101, source_ml_source1_field96, source_ml_source1_field102, source_ml_source1_field100, source_ml_source1_field95, source_ml_source1_field99, source_ml_source1_field98, source_ml_source1_field97, source_ml_source1_field94)
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
                                                                                                                                            let public_fields = Vec::<self::ml_string::Text>::new();
                                                                                                                                            {
                                                                                                                                                let prototype_methods = Vec::<self::ml_string::Text>::new();
                                                                                                                                                {
                                                                                                                                                    let owner_rows = Vec::<Vec<self::ml_string::Text>>::new();
                                                                                                                                                    {
                                                                                                                                                        let prototype = self::require_java_script_closed_class_prototype_methods(&(values_2), &(directory_2));
                                                                                                                                                        {
                                                                                                                                                            let source_ml_temporary_values57 = directory_2;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_temporary_index57 = self::ml::Big::from_i128(0);
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_s103 = self::source_ml_temporary_lower_java_script_closed_class_source_loop63(source, &(values_2), starts, ends, prefix, &(names_2), &(static_calls), text, intervals, previous, ordinal, brands, public_fields, prototype_methods, owner_rows, prototype, &(source_ml_temporary_values57), source_ml_temporary_index57);
                                                                                                                                                                    match source_ml_source1_s103 {
                                                                                                                                                                        self::MlTemporaryLowerJavaScriptClosedClassSourceLoop63Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop63Done(text_2, intervals_2, previous_2, brands_2, public_fields_2, prototype_methods_2, owner_rows_2) => {
                                                                                                                                                                            {
                                                                                                                                                                                let first = 0f64;
                                                                                                                                                                                {
                                                                                                                                                                                    let first_2 = self::source_ml_temporary_lower_java_script_closed_class_source_loop64(&(values_2), starts, previous_2, first);
                                                                                                                                                                                    {
                                                                                                                                                                                        let rest = if first_2 >= (values_2.len() as f64) {
                                                                                                                                                                                            {
                                                                                                                                                                                                let source_ml_source1_string_range_operand104 = source.clone();
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source1_string_range_operand105 = previous_2;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source1_string_range_position106 = source_ml_source1_string_range_operand105;
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source1_string_range_position107 = f64::INFINITY;
                                                                                                                                                                                                            source_ml_source1_string_range_operand104.slice(source_ml_source1_string_range_position106, source_ml_source1_string_range_position107)
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        } else {
                                                                                                                                                                                            (({
                                                                                                                                                                                                let source_ml_source1_string_range_operand108 = source.clone();
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source1_string_range_operand109 = previous_2;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source1_string_range_operand110 = (self::ml_array::optional_at(starts, self::ml_array::number_index(first_2))).unwrap_or_else(|| previous_2);
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source1_string_range_position111 = source_ml_source1_string_range_operand109;
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let source_ml_source1_string_range_position112 = source_ml_source1_string_range_operand110;
                                                                                                                                                                                                                source_ml_source1_string_range_operand108.slice(source_ml_source1_string_range_position111, source_ml_source1_string_range_position112)
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }).concat(&self::render_java_script_closed_class_range(source, &(values_2), starts, ends, first_2, (values_2.len() as f64), &(self::ml_string::Text::from_units(vec![])), &(names_2), &(static_calls)))).concat(&{
                                                                                                                                                                                                let source_ml_source1_string_range_operand113 = source.clone();
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source1_string_range_operand114 = (self::ml_array::optional_at(ends, self::ml_array::number_index((values_2.len() as f64) - 1f64))).unwrap_or_else(|| source.length());
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source1_string_range_position115 = source_ml_source1_string_range_operand114;
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source1_string_range_position116 = f64::INFINITY;
                                                                                                                                                                                                            source_ml_source1_string_range_operand113.slice(source_ml_source1_string_range_position115, source_ml_source1_string_range_position116)
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            })
                                                                                                                                                                                        };
                                                                                                                                                                                        {
                                                                                                                                                                                            let intervals_3 = {
                                                                                                                                                                                                let source_ml_source1_concat_value117 = intervals_2;
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source1_concat_value118 = vec![text_2.length(), (text_2.length() + rest.length()), previous_2, source.length()];
                                                                                                                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value117), source_ml_source1_concat_value118)
                                                                                                                                                                                                }
                                                                                                                                                                                            };
                                                                                                                                                                                            {
                                                                                                                                                                                                let text_3 = text_2.concat(&rest);
                                                                                                                                                                                                if (text_3.length()) > 1_048_576f64 {
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source1_field119 = source.clone();
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source1_field120 = Vec::<f64>::new();
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let source_ml_source1_field121 = Vec::<self::ml_string::Text>::new();
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let source_ml_source1_field122 = Vec::<self::ml_string::Text>::new();
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let source_ml_source1_field123 = Vec::<self::ml_string::Text>::new();
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            let source_ml_source1_field124 = Vec::<Vec<self::ml_string::Text>>::new();
                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                let source_ml_source1_field125 = self::read_java_script_closed_class_refusal(-8f64);
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    let source_ml_source1_field126 = 0f64;
                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                        let source_ml_source1_field127 = false;
                                                                                                                                                                                                                                        (source_ml_source1_field126, source_ml_source1_field121, source_ml_source1_field127, source_ml_source1_field125, source_ml_source1_field120, source_ml_source1_field124, source_ml_source1_field123, source_ml_source1_field122, source_ml_source1_field119)
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
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source1_field128 = text_3.clone();
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source1_field129 = intervals_3;
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let source_ml_source1_field130 = brands_2;
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let source_ml_source1_field131 = public_fields_2;
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let source_ml_source1_field132 = prototype_methods_2;
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            let source_ml_source1_field133 = owner_rows_2;
                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                let source_ml_source1_field134 = self::ml_string::Text::from_units(vec![]);
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    let source_ml_source1_field135 = 0f64;
                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                        let source_ml_source1_field136 = true;
                                                                                                                                                                                                                                        (source_ml_source1_field135, source_ml_source1_field130, source_ml_source1_field136, source_ml_source1_field134, source_ml_source1_field129, source_ml_source1_field133, source_ml_source1_field132, source_ml_source1_field131, source_ml_source1_field128)
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
pub fn accept_java_script_closed_class_own_field(
    receiver_fields: &[self::ml_string::Text],
    public_fields: &[self::ml_string::Text],
    field: &self::ml_string::Text,
) -> bool {
    {
        let ml_library_inequality_1 = ((public_fields.len() as f64) % 2f64) != 0f64;
        if ml_library_inequality_1 {
            false
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source1_s137 =
                        self::source_ml_temporary_accept_java_script_closed_class_own_field_loop71(
                            receiver_fields,
                            public_fields,
                            field,
                            at,
                        );
                    match source_ml_source1_s137 {
                    self::MlTemporaryAcceptJavaScriptClosedClassOwnFieldLoop71Result::MlTemporaryAcceptJavaScriptClosedClassOwnFieldLoop71Done => {
                        false
                    }
                    self::MlTemporaryAcceptJavaScriptClosedClassOwnFieldLoop71Result::MlTemporaryAcceptJavaScriptClosedClassOwnFieldLoop71Return(source_ml_temporary_result71) => {
                        source_ml_temporary_result71
                    }
                }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_own_field_use_error(
    kind: &self::ml_string::Text,
    operation: &self::ml_string::Text,
    instance: bool,
    private_field: bool,
    callable: bool,
    direct: bool,
    own_field: bool,
) -> self::ml_string::Text {
    if ((instance
        && ((*kind)
            == (self::ml_string::Text::from_units(vec![
                114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100,
            ]))))
        && !private_field)
        && own_field
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::read_java_script_closed_class_use_error(
            kind,
            operation,
            instance,
            private_field,
            callable,
            direct,
        )
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
                    let source_ml_source1_s138 =
                        self::source_ml_temporary_project_java_script_class_source_interval_loop72(
                            intervals, first, last, low, high, at,
                        );
                    match source_ml_source1_s138 {
                        self::MlTemporaryProjectJavaScriptClassSourceIntervalLoop72Result::MlTemporaryProjectJavaScriptClassSourceIntervalLoop72Done(low_2, high_2) => {
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
                                        let source_ml_source1_concat_value139 = expected;
                                        {
                                            let source_ml_source1_concat_value140 =
                                                vec![closing.clone()];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::ml_string::Text>::new(),
                                                    source_ml_source1_concat_value139,
                                                ),
                                                source_ml_source1_concat_value140,
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
pub enum MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result {
    MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Done(Vec<f64>),
    MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3 {
    MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(f64),
    MlTemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Return(Vec<f64>),
}
