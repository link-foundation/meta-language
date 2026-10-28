// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::find_java_script_ordinary_object;
use super::index_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_semantic_value_error;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_character_units_utf16_text_loop20(values: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let integer = if value.is_finite() {
                        value.trunc()
                    } else {
                        0f64
                    };
                    {
                        let residue = (integer % 65536f64);
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![if (residue < (0f64)) {
                                (residue + 65536f64)
                            } else {
                                if (residue == (0f64)) {
                                    0f64
                                } else {
                                    residue
                                }
                            }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCodePointUnitsUtf16TextLoop21Result {
    MlTemporaryCodePointUnitsUtf16TextLoop21Done(Vec<f64>),
    MlTemporaryCodePointUnitsUtf16TextLoop21Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_code_point_units_utf16_text_loop21(values: &[f64], mut result: Vec<f64>, mut at: f64) -> self::MlTemporaryCodePointUnitsUtf16TextLoop21Result {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(value)) || (value < (0f64))) || (value > (1114111f64))) {
                    self::MlTemporaryCodePointUnitsUtf16TextLoop21Result::MlTemporaryCodePointUnitsUtf16TextLoop21Return(vec![at])
                } else {
                    if (value < (65536f64)) {
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![if (value == (0f64)) {
                                0f64
                            } else {
                                value
                            }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2, at_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let result_3 = { let (ml_append_right, ml_append_left) = (vec![(55296f64 + (((value - 65536f64) / 1024f64)).floor()), (56320f64 + ((value - 65536f64) % 1024f64))], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_3 = (at + 1f64);
                                {
                                    (result, at) = (result_3, at_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCodePointUnitsUtf16TextLoop21Result::MlTemporaryCodePointUnitsUtf16TextLoop21Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDisplayPointsUtf16TextJoin23 {
    MlTemporaryDisplayPointsUtf16TextJoin23Next(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_display_points_utf16_text_loop22(text: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let point = (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 65533f64);
                {
                    let source_ml_source1_s1906 = if ((point >= (55296f64)) && (point <= (56319f64))) {
                        {
                            let low = (self::ml_array::optional_at(text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                            if ((low >= (56320f64)) && (low <= (57343f64))) {
                                {
                                    let point_2 = (((65536f64 + ((point - 55296f64) * 1024f64)) + low) - 56320f64);
                                    {
                                        let at_2 = (at + 1f64);
                                        self::MlTemporaryDisplayPointsUtf16TextJoin23::MlTemporaryDisplayPointsUtf16TextJoin23Next(at_2, point_2)
                                    }
                                }
                            } else {
                                {
                                    let point_3 = 65533f64;
                                    self::MlTemporaryDisplayPointsUtf16TextJoin23::MlTemporaryDisplayPointsUtf16TextJoin23Next(at, point_3)
                                }
                            }
                        }
                    } else {
                        if ((point >= (56320f64)) && (point <= (57343f64))) {
                            {
                                let point_4 = 65533f64;
                                self::MlTemporaryDisplayPointsUtf16TextJoin23::MlTemporaryDisplayPointsUtf16TextJoin23Next(at, point_4)
                            }
                        } else {
                            self::MlTemporaryDisplayPointsUtf16TextJoin23::MlTemporaryDisplayPointsUtf16TextJoin23Next(at, point)
                        }
                    };
                    match source_ml_source1_s1906.clone() {
                        self::MlTemporaryDisplayPointsUtf16TextJoin23::MlTemporaryDisplayPointsUtf16TextJoin23Next(at_3, point_5) => {
                            {
                                let result_2 = { let (ml_append_right, ml_append_left) = (vec![point_5], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_4 = (at_3 + 1f64);
                                    {
                                        (result, at) = (result_2, at_4);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_display_utf8_from_utf16_text_loop24(mut bytes: Vec<f64>, points: &[f64], mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((points.len() as f64))) {
            {
                let point = (self::ml_array::optional_at(points, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 65533f64);
                if (point < (128f64)) {
                    {
                        let bytes_2 = { let (ml_append_right, ml_append_left) = (vec![point], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                        {
                            let at_2 = (at + 1f64);
                            {
                                (bytes, at) = (bytes_2, at_2);
                                continue;
                            }
                        }
                    }
                } else {
                    if (point < (2048f64)) {
                        {
                            let bytes_3 = { let (ml_append_right, ml_append_left) = (vec![(192f64 + ((point / 64f64)).floor()), (128f64 + (point % 64f64))], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_3 = (at + 1f64);
                                {
                                    (bytes, at) = (bytes_3, at_3);
                                    continue;
                                }
                            }
                        }
                    } else {
                        if (point < (65536f64)) {
                            {
                                let bytes_4 = { let (ml_append_right, ml_append_left) = (vec![(224f64 + ((point / 4096f64)).floor()), (128f64 + (((point / 64f64)).floor() % 64f64)), (128f64 + (point % 64f64))], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_4 = (at + 1f64);
                                    {
                                        (bytes, at) = (bytes_4, at_4);
                                        continue;
                                    }
                                }
                            }
                        } else {
                            {
                                let bytes_5 = { let (ml_append_right, ml_append_left) = (vec![(240f64 + ((point / 262144f64)).floor()), (128f64 + (((point / 4096f64)).floor() % 64f64)), (128f64 + (((point / 64f64)).floor() % 64f64)), (128f64 + (point % 64f64))], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_5 = (at + 1f64);
                                    {
                                        (bytes, at) = (bytes_5, at_5);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            bytes
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanUtf16TextSubstitutionLoop25Result {
    MlTemporaryPlanUtf16TextSubstitutionLoop25Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_utf16_text_substitution_loop25(replacement: &[f64], mut result: Vec<f64>, mut start: f64, mut at: f64) -> self::MlTemporaryPlanUtf16TextSubstitutionLoop25Result {
    loop {
        return if (at < ((replacement.len() as f64))) {
            {
                let next = (self::ml_array::optional_at(replacement, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                if ((((self::ml_array::optional_at(replacement, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)) == (36f64)) && ((((next == (36f64)) || (next == (38f64))) || (next == (96f64))) || (next == (39f64)))) {
                    {
                        let result_3 = if (start < at) {
                            {
                                let result_2 = { let (ml_append_right, ml_append_left) = (vec![0f64, start, at], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                result_2
                            }
                        } else {
                            result
                        };
                        {
                            let result_6 = if (next == (36f64)) {
                                {
                                    let result_4 = { let (ml_append_right, ml_append_left) = (vec![0f64, at, (at + 1f64)], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                    result_4
                                }
                            } else {
                                {
                                    let result_5 = { let (ml_append_right, ml_append_left) = (vec![if (next == (38f64)) {
                                        1f64
                                    } else {
                                        if (next == (96f64)) {
                                            2f64
                                        } else {
                                            3f64
                                        }
                                    }, 0f64, 0f64], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                    result_5
                                }
                            };
                            {
                                let at_2 = (at + 2f64);
                                {
                                    let start_2 = at_2;
                                    {
                                        (result, start, at) = (result_6, start_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let at_3 = (at + 1f64);
                        {
                            at = at_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanUtf16TextSubstitutionLoop25Result::MlTemporaryPlanUtf16TextSubstitutionLoop25Done(result, start)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop27(text: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReplaceUtf16TextLoop26Result {
    MlTemporaryReplaceUtf16TextLoop26Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop28(text: &[f64], mut result: Vec<f64>, mut found: f64, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < found) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop30(text: &[f64], replacement: &[f64], mut result: Vec<f64>, mut kind: f64, mut last: f64, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![if (kind == (0f64)) {
                    (self::ml_array::optional_at(replacement, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)
                } else {
                    (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)
                }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop29(text: &[f64], search: &[f64], replacement: &[f64], mut result: Vec<f64>, plan: &[f64], mut found: f64, mut instruction: f64) -> Vec<f64> {
    loop {
        return if (instruction < ((plan.len() as f64))) {
            {
                let kind = (self::ml_array::optional_at(plan, self::ml_array::number_index(instruction))).clone().unwrap_or_else(|| 0f64);
                {
                    let first = if (kind == (0f64)) {
                        (self::ml_array::optional_at(plan, self::ml_array::number_index((instruction + 1f64)))).clone().unwrap_or_else(|| 0f64)
                    } else {
                        if (kind == (2f64)) {
                            0f64
                        } else {
                            if (kind == (1f64)) {
                                found
                            } else {
                                (found + (search.len() as f64))
                            }
                        }
                    };
                    {
                        let last = if (kind == (0f64)) {
                            (self::ml_array::optional_at(plan, self::ml_array::number_index((instruction + 2f64)))).clone().unwrap_or_else(|| 0f64)
                        } else {
                            if (kind == (2f64)) {
                                found
                            } else {
                                if (kind == (1f64)) {
                                    (found + (search.len() as f64))
                                } else {
                                    (text.len() as f64)
                                }
                            }
                        };
                        {
                            let at = first;
                            {
                                let result_2 = self::source_ml_temporary_replace_utf16_text_loop30(text, replacement, result, kind, last, at);
                                {
                                    let instruction_2 = (instruction + 3f64);
                                    {
                                        (result, instruction) = (result_2, instruction_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop26(text: &[f64], search: &[f64], replacement: &[f64], mut all: bool, mut result: Vec<f64>, plan: &[f64], mut start: f64, mut from: f64) -> self::MlTemporaryReplaceUtf16TextLoop26Result {
    loop {
        return {
            let found = self::index_utf16_text(text, search, from);
            if (found < (0f64)) {
                self::MlTemporaryReplaceUtf16TextLoop26Result::MlTemporaryReplaceUtf16TextLoop26Done(result, start)
            } else {
                {
                    let at = start;
                    {
                        let result_2 = self::source_ml_temporary_replace_utf16_text_loop28(text, result, found, at);
                        {
                            let instruction = 0f64;
                            {
                                let result_3 = self::source_ml_temporary_replace_utf16_text_loop29(text, search, replacement, result_2, plan, found, instruction);
                                {
                                    let start_2 = (found + (search.len() as f64));
                                    if (!all || ((((search.len() as f64)) == (0f64)) && (found == ((text.len() as f64))))) {
                                        self::MlTemporaryReplaceUtf16TextLoop26Result::MlTemporaryReplaceUtf16TextLoop26Done(result_3, start_2)
                                    } else {
                                        {
                                            let from_2 = (start_2 + if (((search.len() as f64)) == (0f64)) {
                                                1f64
                                            } else {
                                                0f64
                                            });
                                            {
                                                (result, start, from) = (result_3, start_2, from_2);
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
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Result {
    MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Done(Vec<f64>),
    MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_execute_java_script_call_numeric_primitive_loop32(mut numbers: Vec<f64>, mut source_ml_temporary_values31: Vec<self::JavaScriptSemanticValue>, mut source_ml_temporary_index31: self::ml::Big) -> self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Result {
    loop {
        return if (source_ml_temporary_index31 < (self::ml::Big::from_u128(source_ml_temporary_values31.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_temporary_values31, source_ml_temporary_index31.to_index());
                match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1907) => {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        {
                            let numbers_2 = { let (ml_append_right, ml_append_left) = (vec![value_value], numbers); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_temporary_index31_2 = source_ml_temporary_index31.add(&self::ml::Big::from_i128(1));
                                {
                                    (numbers, source_ml_temporary_index31) = (numbers_2, source_ml_temporary_index31_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f1908) => {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f1909) => {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1910) => {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f1911, source_ml_source1_f1912) => {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                }
            }
        } else {
            self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Done(numbers)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryExecuteJavaScriptCallPrimitiveLoop34Result {
    MlTemporaryExecuteJavaScriptCallPrimitiveLoop34Done,
    MlTemporaryExecuteJavaScriptCallPrimitiveLoop34Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_execute_java_script_call_primitive_loop34(mut source_ml_temporary_values33: Vec<self::JavaScriptSemanticValue>, mut source_ml_temporary_index33: self::ml::Big) -> self::MlTemporaryExecuteJavaScriptCallPrimitiveLoop34Result {
    loop {
        return if (source_ml_temporary_index33 < (self::ml::Big::from_u128(source_ml_temporary_values33.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_temporary_values33, source_ml_temporary_index33.to_index());
                {
                    let error = self::read_java_script_semantic_value_error(value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlTemporaryExecuteJavaScriptCallPrimitiveLoop34Result::MlTemporaryExecuteJavaScriptCallPrimitiveLoop34Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(error.clone())))
                    } else {
                        {
                            let source_ml_temporary_index33_2 = source_ml_temporary_index33.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index33 = source_ml_temporary_index33_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryExecuteJavaScriptCallPrimitiveLoop34Result::MlTemporaryExecuteJavaScriptCallPrimitiveLoop34Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptCallCodeFunctionLoop35Result {
    MlTemporaryFindJavaScriptCallCodeFunctionLoop35Done,
    MlTemporaryFindJavaScriptCallCodeFunctionLoop35Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_call_code_function_loop35(mut program: self::JavaScriptCallProgram, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryFindJavaScriptCallCodeFunctionLoop35Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let entry = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1913, _, _) => source_ml_source1_data_property_value1913.as_slice() }), self::ml_array::number_index(middle));
                    if match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryFindJavaScriptCallCodeFunctionLoop35Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop35Return((-1f64))
                    } else {
                        if ((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value1915, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1915.clone()
                            }
                        }) == identity) {
                            self::MlTemporaryFindJavaScriptCallCodeFunctionLoop35Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop35Return(middle)
                        } else {
                            if ((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value1917, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1917.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptCallCodeFunctionLoop35Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop35Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin36 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin36Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin36Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>)),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop37Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop37Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop37Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_literal_property_index_loop37(key: &[f64], mut value: f64, mut at: f64) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop37Result {
    loop {
        return if (at < ((key.len() as f64))) {
            {
                let digit = (match self::ml_array::optional_at(key, self::ml_array::number_index(at)) { Some(ml_present) => ml_present, None => f64::NAN } - 48f64);
                if ((!(self::ml_math::is_integer(digit)) || (digit < (0f64))) || (digit > (9f64))) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop37Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop37Return((-1f64))
                } else {
                    {
                        let value_2 = ((value * 10f64) + digit);
                        {
                            let at_2 = (at + 1f64);
                            {
                                (value, at) = (value_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop37Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop37Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptOrdinaryObjectLoop38Result {
    MlTemporaryFindJavaScriptOrdinaryObjectLoop38Done,
    MlTemporaryFindJavaScriptOrdinaryObjectLoop38Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_ordinary_object_loop38(mut objects: Vec<self::JavaScriptOrdinaryObject>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryFindJavaScriptOrdinaryObjectLoop38Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = self::ml_array::optional_at(&objects, self::ml_array::number_index(middle));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryFindJavaScriptOrdinaryObjectLoop38Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop38Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1919, _, _, _, _) => {
                                source_ml_source1_data_property_value1919.clone()
                            }
                        }) == identity) {
                            self::MlTemporaryFindJavaScriptOrdinaryObjectLoop38Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop38Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1921, _, _, _, _) => {
                                    source_ml_source1_data_property_value1921.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptOrdinaryObjectLoop38Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop38Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptOrdinarySymbolLoop39Result {
    MlTemporaryTestJavaScriptOrdinarySymbolLoop39Done,
    MlTemporaryTestJavaScriptOrdinarySymbolLoop39Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_ordinary_symbol_loop39(mut symbols: Vec<self::ml::Big>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryTestJavaScriptOrdinarySymbolLoop39Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = self::ml_array::optional_at(&symbols, self::ml_array::number_index(middle));
                    if {
                        let source_ml_source1_comparison_operand1922 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand1923 = None::<()>;
                            match (source_ml_source1_comparison_operand1922.clone(), source_ml_source1_comparison_operand1923.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                        }
                    } {
                        self::MlTemporaryTestJavaScriptOrdinarySymbolLoop39Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop39Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlTemporaryTestJavaScriptOrdinarySymbolLoop39Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop39Return(true)
                        } else {
                            if ((symbol.clone().expect("a proven present value")) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptOrdinarySymbolLoop39Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop39Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptOrdinaryPropertyLoop40Result {
    MlTemporaryFindJavaScriptOrdinaryPropertyLoop40Done,
    MlTemporaryFindJavaScriptOrdinaryPropertyLoop40Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_ordinary_property_loop40(mut properties: Vec<self::JavaScriptOrdinaryProperty>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop40Result {
    loop {
        return if (at < ((properties.len() as f64))) {
            {
                let property = self::ml_array::optional_at(&properties, self::ml_array::number_index(at));
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value({
                    let source_ml_source1_nullish_field_receiver1924 = (property.clone(),);
                    if match (source_ml_source1_nullish_field_receiver1924.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present1925 = ((source_ml_source1_nullish_field_receiver1924.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present1925.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1926, _) => {
                                    let source_ml_source1_data_property_value1926 = *source_ml_source1_data_property_value1926;
                                    source_ml_source1_data_property_value1926.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                    }
                }, key.clone())) {
                    self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop40Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop40Return(at)
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop40Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop40Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop48(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut prototype: self::JavaScriptSemanticValue, mut depth: f64) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Result {
    loop {
        return match prototype.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Done
            }
            self::JavaScriptSemanticValue::Null => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Done
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1927) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Done
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f1928) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Done
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1929) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Done
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f1930) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Done
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1931) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Done
            }
            self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f1932) => {
                {
                    let depth_2 = (depth + 1f64);
                    if ((identity == (match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1934, _, _, _, _) => {
                            source_ml_source1_data_property_value1934.clone()
                        }
                    })) || (depth_2 > (((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1936, _) => source_ml_source1_data_property_value1936.as_slice() }).len() as f64)))) {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 104, 97, 105, 110, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 121, 99, 108, 105, 99]))
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1937, _) => {
                                    source_ml_source1_data_property_value1937
                                }
                            }, identity.clone());
                            {
                                let parent = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1938, _) => source_ml_source1_data_property_value1938.as_slice() }), self::ml_array::number_index(index));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                } else {
                                    {
                                        let prototype_2 = match parent.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1939, _, _, _) => {
                                                let source_ml_source1_data_property_value1939 = *source_ml_source1_data_property_value1939;
                                                source_ml_source1_data_property_value1939.clone()
                                            }
                                        };
                                        {
                                            (prototype, depth) = (prototype_2.clone(), depth_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin50 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin50Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin50Return(self::ml_string::Text),
}
