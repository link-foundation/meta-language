// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryLowerJavaScriptClosedClassSourceJoin24;
use super::MlTemporaryLowerJavaScriptClosedClassSourceJoin25;
use super::MlTemporaryLowerJavaScriptClosedClassSourceLoop23Result;
use super::MlTemporaryLowerJavaScriptClosedClassSourceLoop26Result;
use super::MlTemporaryLowerJavaScriptClosedClassSourceLoop28Result;
use super::MlTemporaryProjectJavaScriptClassSourceIntervalLoop30Result;
use super::find_java_script_class_group_end;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_closed_class_observation_error;
use super::read_java_script_closed_class_refusal;
use super::render_java_script_closed_class_range;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop22;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop23;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop26;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop27;
use super::source_ml_temporary_lower_java_script_closed_class_source_loop28;
use super::source_ml_temporary_project_java_script_class_source_interval_loop30;

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
            let source_ml_source1_field23 = source.clone();
            {
                let source_ml_source1_field24 = Vec::<f64>::new();
                {
                    let source_ml_source1_field25 = Vec::<self::ml_string::Text>::new();
                    {
                        let source_ml_source1_field26 =
                            self::read_java_script_closed_class_refusal(-8f64);
                        {
                            let source_ml_source1_field27 = 0f64;
                            {
                                let source_ml_source1_field28 = false;
                                (
                                    source_ml_source1_field27,
                                    source_ml_source1_field25,
                                    source_ml_source1_field28,
                                    source_ml_source1_field26,
                                    source_ml_source1_field24,
                                    source_ml_source1_field23,
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
                        self::source_ml_temporary_lower_java_script_closed_class_source_loop22(
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
                                                            let source_ml_source1_s29 = self::source_ml_temporary_lower_java_script_closed_class_source_loop23(&(values_2), kinds, directory, names, depth, declarations_finished, error_3, error_at, at_2);
                                                            match source_ml_source1_s29 {
                                                                self::MlTemporaryLowerJavaScriptClosedClassSourceLoop23Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop23Done(directory_2, names_2, error_4, error_at_2) => {
                                                                    {
                                                                        let source_ml_source1_s31 = if (error_4 == 0f64) && ((directory_2.len() as f64) > 0f64) {
                                                                            {
                                                                                let at_3 = 0f64;
                                                                                {
                                                                                    let source_ml_source1_s32 = self::source_ml_temporary_lower_java_script_closed_class_source_loop28(source, &(values_2), starts, ends, &(names_2), error_4, error_at_2, at_3);
                                                                                    match source_ml_source1_s32 {
                                                                                        self::MlTemporaryLowerJavaScriptClosedClassSourceLoop28Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop28Done(error_5, error_at_3) => {
                                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin24::MlTemporaryLowerJavaScriptClosedClassSourceJoin24Next(error_5, error_at_3)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin24::MlTemporaryLowerJavaScriptClosedClassSourceJoin24Next(error_4, error_at_2)
                                                                        };
                                                                        match source_ml_source1_s31 {
                                                                            self::MlTemporaryLowerJavaScriptClosedClassSourceJoin24::MlTemporaryLowerJavaScriptClosedClassSourceJoin24Next(error_6, error_at_4) => {
                                                                                {
                                                                                    let source_ml_source1_s33 = if (error_6 == 0f64) && ((directory_2.len() as f64) > 0f64) {
                                                                                        {
                                                                                            let observation = self::read_java_script_closed_class_observation_error(&(values_2), kinds, &(directory_2));
                                                                                            if observation >= 0f64 {
                                                                                                {
                                                                                                    let error_7 = -7f64;
                                                                                                    {
                                                                                                        let error_at_5 = observation;
                                                                                                        self::MlTemporaryLowerJavaScriptClosedClassSourceJoin25::MlTemporaryLowerJavaScriptClosedClassSourceJoin25Next(error_7, error_at_5)
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlTemporaryLowerJavaScriptClosedClassSourceJoin25::MlTemporaryLowerJavaScriptClosedClassSourceJoin25Next(error_6, error_at_4)
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        self::MlTemporaryLowerJavaScriptClosedClassSourceJoin25::MlTemporaryLowerJavaScriptClosedClassSourceJoin25Next(error_6, error_at_4)
                                                                                    };
                                                                                    match source_ml_source1_s33 {
                                                                                        self::MlTemporaryLowerJavaScriptClosedClassSourceJoin25::MlTemporaryLowerJavaScriptClosedClassSourceJoin25Next(error_8, error_at_6) => {
                                                                                            {
let ml_library_inequality_1 = error_8 != 0f64 ;
if ml_library_inequality_1 {
                                                                                                {
                                                                                                    let source_ml_source1_field34 = source.clone();
                                                                                                    {
                                                                                                        let source_ml_source1_field35 = Vec::<f64>::new();
                                                                                                        {
                                                                                                            let source_ml_source1_field36 = Vec::<self::ml_string::Text>::new();
                                                                                                            {
                                                                                                                let source_ml_source1_field37 = self::read_java_script_closed_class_refusal(error_8);
                                                                                                                {
                                                                                                                    let source_ml_source1_field38 = error_at_6;
                                                                                                                    {
                                                                                                                        let source_ml_source1_field39 = false;
                                                                                                                        (source_ml_source1_field38, source_ml_source1_field36, source_ml_source1_field39, source_ml_source1_field37, source_ml_source1_field35, source_ml_source1_field34)
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else if (directory_2.len() as f64) == 0f64 {
                                                                                                    {
                                                                                                        let source_ml_source1_field40 = source.clone();
                                                                                                        {
                                                                                                            let source_ml_source1_field41 = Vec::<f64>::new();
                                                                                                            {
                                                                                                                let source_ml_source1_field42 = Vec::<self::ml_string::Text>::new();
                                                                                                                {
                                                                                                                    let source_ml_source1_field43 = self::ml_string::Text::from_units(vec![]);
                                                                                                                    {
                                                                                                                        let source_ml_source1_field44 = 0f64;
                                                                                                                        {
                                                                                                                            let source_ml_source1_field45 = false;
                                                                                                                            (source_ml_source1_field44, source_ml_source1_field42, source_ml_source1_field45, source_ml_source1_field43, source_ml_source1_field41, source_ml_source1_field40)
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
                                                                                                                            let source_ml_temporary_values21 = directory_2;
                                                                                                                            {
                                                                                                                                let source_ml_temporary_index21 = self::ml::Big::from_i128(0);
                                                                                                                                {
                                                                                                                                    let source_ml_source1_s46 = self::source_ml_temporary_lower_java_script_closed_class_source_loop26(source, &(values_2), starts, ends, prefix, &(names_2), text, intervals, previous, ordinal, brands, &(source_ml_temporary_values21), source_ml_temporary_index21);
                                                                                                                                    match source_ml_source1_s46 {
                                                                                                                                        self::MlTemporaryLowerJavaScriptClosedClassSourceLoop26Result::MlTemporaryLowerJavaScriptClosedClassSourceLoop26Done(text_2, intervals_2, previous_2, brands_2) => {
                                                                                                                                            {
                                                                                                                                                let first = 0f64;
                                                                                                                                                {
                                                                                                                                                    let first_2 = self::source_ml_temporary_lower_java_script_closed_class_source_loop27(&(values_2), starts, previous_2, first);
                                                                                                                                                    {
                                                                                                                                                        let rest = if first_2 >= (values_2.len() as f64) {
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source1_string_range_operand47 = source.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_string_range_operand48 = previous_2;
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_string_range_position49 = source_ml_source1_string_range_operand48;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_string_range_position50 = f64::INFINITY;
                                                                                                                                                                            source_ml_source1_string_range_operand47.slice(source_ml_source1_string_range_position49, source_ml_source1_string_range_position50)
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        } else {
                                                                                                                                                            (({
                                                                                                                                                                let source_ml_source1_string_range_operand51 = source.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_string_range_operand52 = previous_2;
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_string_range_operand53 = (self::ml_array::optional_at(starts, self::ml_array::number_index(first_2))).unwrap_or_else(|| previous_2);
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_string_range_position54 = source_ml_source1_string_range_operand52;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_string_range_position55 = source_ml_source1_string_range_operand53;
                                                                                                                                                                                source_ml_source1_string_range_operand51.slice(source_ml_source1_string_range_position54, source_ml_source1_string_range_position55)
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }).concat(&self::render_java_script_closed_class_range(source, &(values_2), starts, ends, first_2, (values_2.len() as f64), &(self::ml_string::Text::from_units(vec![])), &(names_2)))).concat(&{
                                                                                                                                                                let source_ml_source1_string_range_operand56 = source.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_string_range_operand57 = (self::ml_array::optional_at(ends, self::ml_array::number_index((values_2.len() as f64) - 1f64))).unwrap_or_else(|| source.length());
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_string_range_position58 = source_ml_source1_string_range_operand57;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_string_range_position59 = f64::INFINITY;
                                                                                                                                                                            source_ml_source1_string_range_operand56.slice(source_ml_source1_string_range_position58, source_ml_source1_string_range_position59)
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            })
                                                                                                                                                        };
                                                                                                                                                        {
                                                                                                                                                            let intervals_3 = {
                                                                                                                                                                let source_ml_source1_concat_value60 = intervals_2;
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_concat_value61 = vec![text_2.length(), (text_2.length() + rest.length()), previous_2, source.length()];
                                                                                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value60), source_ml_source1_concat_value61)
                                                                                                                                                                }
                                                                                                                                                            };
                                                                                                                                                            {
                                                                                                                                                                let text_3 = text_2.concat(&rest);
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_field62 = text_3;
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_field63 = intervals_3;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_field64 = brands_2;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_field65 = self::ml_string::Text::from_units(vec![]);
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source1_field66 = 0f64;
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_source1_field67 = true;
                                                                                                                                                                                        (source_ml_source1_field66, source_ml_source1_field64, source_ml_source1_field67, source_ml_source1_field65, source_ml_source1_field63, source_ml_source1_field62)
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
}
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
                    let source_ml_source1_s68 =
                        self::source_ml_temporary_project_java_script_class_source_interval_loop30(
                            intervals, first, last, low, high, at,
                        );
                    match source_ml_source1_s68 {
                        self::MlTemporaryProjectJavaScriptClassSourceIntervalLoop30Result::MlTemporaryProjectJavaScriptClassSourceIntervalLoop30Done(low_2, high_2) => {
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
                                        let source_ml_source1_concat_value69 = expected;
                                        {
                                            let source_ml_source1_concat_value70 =
                                                vec![closing.clone()];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::ml_string::Text>::new(),
                                                    source_ml_source1_concat_value69,
                                                ),
                                                source_ml_source1_concat_value70,
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
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryJoin4 {
    MlTemporaryReadJavaScriptClosedClassDirectoryJoin4Next,
    MlTemporaryReadJavaScriptClosedClassDirectoryJoin4Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Done,
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop5(
    values: &[self::ml_string::Text],
    mut body_end: f64,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Result {
    loop {
        return if at < body_end {
            if ((self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]))
            {
                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Return(vec![-6f64, at])
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
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop3Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop3Done(Vec<f64>),
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop3Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Done(f64),
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop8Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop8Done,
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop8Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop8(
    values: &[self::ml_string::Text],
    mut group: f64,
    mut inner: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop8Result {
    loop {
        return if inner < group {
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
                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop8Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop8Return(vec![-5f64, inner])
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
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop8Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop8Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop7(
    values: &[self::ml_string::Text],
    mut body_end: f64,
    mut finish: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result {
    loop {
        return if finish < body_end {
            {
                let value =
                    (self::ml_array::optional_at(values, self::ml_array::number_index(finish)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if value == (self::ml_string::Text::from_units(vec![59])) {
                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Done(finish)
                } else if value == (self::ml_string::Text::from_units(vec![44])) {
                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Return(vec![-5f64, finish])
                } else if ((value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115])))
                    && (((self::ml_array::optional_at(
                        values,
                        self::ml_array::number_index(finish - 1f64),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                        != (self::ml_string::Text::from_units(vec![46]))))
                    && (((self::ml_array::optional_at(
                        values,
                        self::ml_array::number_index(finish - 1f64),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                        != (self::ml_string::Text::from_units(vec![63, 46])))
                {
                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Return(vec![-5f64, finish])
                } else if ((value == (self::ml_string::Text::from_units(vec![40])))
                    || (value == (self::ml_string::Text::from_units(vec![91]))))
                    || (value == (self::ml_string::Text::from_units(vec![123])))
                {
                    {
                        let group = self::find_java_script_class_group_end(values, finish);
                        if (group < 0f64) || (group >= body_end) {
                            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Return(vec![-5f64, finish])
                        } else {
                            {
                                let inner = finish + 1f64;
                                {
                                    let source_ml_source1_s71 = self::source_ml_temporary_read_java_script_closed_class_directory_loop8(values, group, inner);
                                    match source_ml_source1_s71 {
                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop8Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop8Done => {
                                                        {
                                                            let finish_2 = group;
                                                            {
                                                                let finish_3 = finish_2 + 1f64;
                                                                {
                                                                    finish = finish_3;
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop8Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop8Return(source_ml_temporary_result8) => {
                                                        self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Return(source_ml_temporary_result8)
                                                    }
                                                }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let finish_4 = finish + 1f64;
                        {
                            finish = finish_4;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Done(finish)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Done,
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop6(
    values: &[self::ml_string::Text],
    fields: &[f64],
    mut cursor: f64,
    name: &self::ml_string::Text,
    mut previous: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Result {
    loop {
        return if previous < (fields.len() as f64) {
            if ((self::ml_array::optional_at(
                values,
                self::ml_array::number_index(
                    (self::ml_array::optional_at(fields, self::ml_array::number_index(previous)))
                        .unwrap_or_else(|| 0f64),
                ),
            ))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (*name)
            {
                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Return(vec![-4f64, (cursor + 2f64)])
            } else {
                {
                    let previous_2 = previous + 3f64;
                    {
                        previous = previous_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop3(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut fields: Vec<f64>,
    names: &[self::ml_string::Text],
    mut body_start: f64,
    mut body_end: f64,
    mut cursor: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop3Result {
    loop {
        return if (((((body_start >= 0f64) && (cursor < body_end))
            && (((self::ml_array::optional_at(values, self::ml_array::number_index(cursor)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))))
            && (((self::ml_array::optional_at(
                values,
                self::ml_array::number_index(cursor + 1f64),
            ))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![46]))))
            && (((self::ml_array::optional_at(
                kinds,
                self::ml_array::number_index(cursor + 2f64),
            ))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![
                    105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                ]))))
            && (((self::ml_array::optional_at(
                values,
                self::ml_array::number_index(cursor + 3f64),
            ))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![61])))
        {
            {
                let name = (self::ml_array::optional_at(
                    values,
                    self::ml_array::number_index(cursor + 2f64),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if (name == (self::ml_string::Text::from_units(vec![36])))
                    || self::ml_array::includes(names, &name.clone(), 0f64, false, |a, b| a == b)
                {
                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop3Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop3Return(vec![-4f64, (cursor + 2f64)])
                } else {
                    {
                        let previous = 0f64;
                        {
                            let source_ml_source1_s72 = self::source_ml_temporary_read_java_script_closed_class_directory_loop6(values, &(fields), cursor, &(name), previous);
                            match source_ml_source1_s72 {
                                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Done => {
                                    {
                                        let start = cursor + 4f64;
                                        {
                                            let finish = start;
                                            {
                                                let source_ml_source1_s73 = self::source_ml_temporary_read_java_script_closed_class_directory_loop7(values, body_end, finish);
                                                match source_ml_source1_s73 {
                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Done(finish_2) => {
                                                        if (finish_2 == start) || (finish_2 >= body_end) {
                                                            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop3Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop3Return(vec![-5f64, start])
                                                        } else {
                                                            {
                                                                let fields_2 = {
                                                                    let source_ml_source1_concat_value74 = fields;
                                                                    {
                                                                        let source_ml_source1_concat_value75 = vec![(cursor + 2f64), start, finish_2];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value74), source_ml_source1_concat_value75)
                                                                    }
                                                                };
                                                                {
                                                                    let cursor_2 = finish_2 + 1f64;
                                                                    {
                                                                        (fields, cursor) = (fields_2, cursor_2);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Return(source_ml_temporary_result7) => {
                                                        self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop3Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop3Return(source_ml_temporary_result7)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Return(source_ml_temporary_result8) => {
                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop3Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop3Return(source_ml_temporary_result8)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop3Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop3Done(fields)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop2Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop2Done(
        Vec<f64>,
        Vec<f64>,
        Vec<self::ml_string::Text>,
    ),
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop2Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryJoin10 {
    MlTemporaryReadJavaScriptClosedClassDirectoryJoin10Next(Vec<f64>, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Done,
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Return(Vec<f64>),
}
