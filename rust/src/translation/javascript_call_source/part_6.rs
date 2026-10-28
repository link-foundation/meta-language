// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::accept_java_script_whitespace_unit;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::index_utf16_text;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_join_utf16_text_loop17;
use super::source_ml_temporary_join_utf16_text_loop18;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_join_utf16_text_loop16(values: &[Vec<f64>], separator: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let result_3 = if (at != (0f64)) {
                    {
                        let offset = 0f64;
                        {
                            let result_2 = self::source_ml_temporary_join_utf16_text_loop17(separator, result, offset);
                            result_2
                        }
                    }
                } else {
                    result
                };
                {
                    let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| Vec::<f64>::new());
                    {
                        let offset_2 = 0f64;
                        {
                            let result_4 = self::source_ml_temporary_join_utf16_text_loop18(result_3, &(value), offset_2);
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_4, at_2);
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_trim_utf16_text_loop19(text: &[f64], mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((first < last) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(text, self::ml_array::number_index(first))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let first_2 = (first + 1f64);
                {
                    first = first_2;
                    continue;
                }
            }
        } else {
            first
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_trim_utf16_text_loop20(text: &[f64], mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((last > first) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(text, self::ml_array::number_index((last - 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let last_2 = (last - 1f64);
                {
                    last = last_2;
                    continue;
                }
            }
        } else {
            last
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_character_units_utf16_text_loop21(values: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
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
pub enum MlTemporaryCodePointUnitsUtf16TextLoop22Result {
    MlTemporaryCodePointUnitsUtf16TextLoop22Done(Vec<f64>),
    MlTemporaryCodePointUnitsUtf16TextLoop22Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_code_point_units_utf16_text_loop22(values: &[f64], mut result: Vec<f64>, mut at: f64) -> self::MlTemporaryCodePointUnitsUtf16TextLoop22Result {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(value)) || (value < (0f64))) || (value > (1114111f64))) {
                    self::MlTemporaryCodePointUnitsUtf16TextLoop22Result::MlTemporaryCodePointUnitsUtf16TextLoop22Return(vec![at])
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
            self::MlTemporaryCodePointUnitsUtf16TextLoop22Result::MlTemporaryCodePointUnitsUtf16TextLoop22Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDisplayPointsUtf16TextJoin24 {
    MlTemporaryDisplayPointsUtf16TextJoin24Next(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_display_points_utf16_text_loop23(text: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let point = (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 65533f64);
                {
                    let source_ml_source1_s519 = if ((point >= (55296f64)) && (point <= (56319f64))) {
                        {
                            let low = (self::ml_array::optional_at(text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                            if ((low >= (56320f64)) && (low <= (57343f64))) {
                                {
                                    let point_2 = (((65536f64 + ((point - 55296f64) * 1024f64)) + low) - 56320f64);
                                    {
                                        let at_2 = (at + 1f64);
                                        self::MlTemporaryDisplayPointsUtf16TextJoin24::MlTemporaryDisplayPointsUtf16TextJoin24Next(at_2, point_2)
                                    }
                                }
                            } else {
                                {
                                    let point_3 = 65533f64;
                                    self::MlTemporaryDisplayPointsUtf16TextJoin24::MlTemporaryDisplayPointsUtf16TextJoin24Next(at, point_3)
                                }
                            }
                        }
                    } else {
                        if ((point >= (56320f64)) && (point <= (57343f64))) {
                            {
                                let point_4 = 65533f64;
                                self::MlTemporaryDisplayPointsUtf16TextJoin24::MlTemporaryDisplayPointsUtf16TextJoin24Next(at, point_4)
                            }
                        } else {
                            self::MlTemporaryDisplayPointsUtf16TextJoin24::MlTemporaryDisplayPointsUtf16TextJoin24Next(at, point)
                        }
                    };
                    match source_ml_source1_s519.clone() {
                        self::MlTemporaryDisplayPointsUtf16TextJoin24::MlTemporaryDisplayPointsUtf16TextJoin24Next(at_3, point_5) => {
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
pub fn source_ml_temporary_display_utf8_from_utf16_text_loop25(mut bytes: Vec<f64>, points: &[f64], mut at: f64) -> Vec<f64> {
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
pub enum MlTemporaryPlanUtf16TextSubstitutionLoop26Result {
    MlTemporaryPlanUtf16TextSubstitutionLoop26Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_utf16_text_substitution_loop26(replacement: &[f64], mut result: Vec<f64>, mut start: f64, mut at: f64) -> self::MlTemporaryPlanUtf16TextSubstitutionLoop26Result {
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
            self::MlTemporaryPlanUtf16TextSubstitutionLoop26Result::MlTemporaryPlanUtf16TextSubstitutionLoop26Done(result, start)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop28(text: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
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
pub enum MlTemporaryReplaceUtf16TextLoop27Result {
    MlTemporaryReplaceUtf16TextLoop27Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop29(text: &[f64], mut result: Vec<f64>, mut found: f64, mut at: f64) -> Vec<f64> {
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
pub fn source_ml_temporary_replace_utf16_text_loop31(text: &[f64], replacement: &[f64], mut result: Vec<f64>, mut kind: f64, mut last: f64, mut at: f64) -> Vec<f64> {
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
pub fn source_ml_temporary_replace_utf16_text_loop30(text: &[f64], search: &[f64], replacement: &[f64], mut result: Vec<f64>, plan: &[f64], mut found: f64, mut instruction: f64) -> Vec<f64> {
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
                                let result_2 = self::source_ml_temporary_replace_utf16_text_loop31(text, replacement, result, kind, last, at);
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
pub fn source_ml_temporary_replace_utf16_text_loop27(text: &[f64], search: &[f64], replacement: &[f64], mut all: bool, mut result: Vec<f64>, plan: &[f64], mut start: f64, mut from: f64) -> self::MlTemporaryReplaceUtf16TextLoop27Result {
    loop {
        return {
            let found = self::index_utf16_text(text, search, from);
            if (found < (0f64)) {
                self::MlTemporaryReplaceUtf16TextLoop27Result::MlTemporaryReplaceUtf16TextLoop27Done(result, start)
            } else {
                {
                    let at = start;
                    {
                        let result_2 = self::source_ml_temporary_replace_utf16_text_loop29(text, result, found, at);
                        {
                            let instruction = 0f64;
                            {
                                let result_3 = self::source_ml_temporary_replace_utf16_text_loop30(text, search, replacement, result_2, plan, found, instruction);
                                {
                                    let start_2 = (found + (search.len() as f64));
                                    if (!all || ((((search.len() as f64)) == (0f64)) && (found == ((text.len() as f64))))) {
                                        self::MlTemporaryReplaceUtf16TextLoop27Result::MlTemporaryReplaceUtf16TextLoop27Done(result_3, start_2)
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
pub enum MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Result {
    MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Done(Vec<f64>),
    MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_execute_java_script_call_numeric_primitive_loop33(mut numbers: Vec<f64>, mut source_ml_temporary_values32: Vec<self::JavaScriptSemanticValue>, mut source_ml_temporary_index32: self::ml::Big) -> self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Result {
    loop {
        return if (source_ml_temporary_index32 < (self::ml::Big::from_u128(source_ml_temporary_values32.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_temporary_values32, source_ml_temporary_index32.to_index());
                match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("numeric primitives require actual Number operands after source conversion"))))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("numeric primitives require actual Number operands after source conversion"))))
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f520) => {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("numeric primitives require actual Number operands after source conversion"))))
                    }
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        {
                            let numbers_2 = { let (ml_append_right, ml_append_left) = (vec![value_value], numbers); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_temporary_index32_2 = source_ml_temporary_index32.add(&self::ml::Big::from_i128(1));
                                {
                                    (numbers, source_ml_temporary_index32) = (numbers_2, source_ml_temporary_index32_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f521) => {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("numeric primitives require actual Number operands after source conversion"))))
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f522) => {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("numeric primitives require actual Number operands after source conversion"))))
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f523) => {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("numeric primitives require actual Number operands after source conversion"))))
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f524, source_ml_source1_f525) => {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("numeric primitives require actual Number operands after source conversion"))))
                    }
                }
            }
        } else {
            self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Done(numbers)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryExecuteJavaScriptCallPrimitiveLoop35Result {
    MlTemporaryExecuteJavaScriptCallPrimitiveLoop35Done,
    MlTemporaryExecuteJavaScriptCallPrimitiveLoop35Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_execute_java_script_call_primitive_loop35(mut source_ml_temporary_values34: Vec<self::JavaScriptSemanticValue>, mut source_ml_temporary_index34: self::ml::Big) -> self::MlTemporaryExecuteJavaScriptCallPrimitiveLoop35Result {
    loop {
        return if (source_ml_temporary_index34 < (self::ml::Big::from_u128(source_ml_temporary_values34.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_temporary_values34, source_ml_temporary_index34.to_index());
                {
                    let error = self::read_java_script_semantic_value_error(value.clone());
                    if (error != "") {
                        self::MlTemporaryExecuteJavaScriptCallPrimitiveLoop35Result::MlTemporaryExecuteJavaScriptCallPrimitiveLoop35Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(error.clone())))
                    } else {
                        {
                            let source_ml_temporary_index34_2 = source_ml_temporary_index34.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index34 = source_ml_temporary_index34_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryExecuteJavaScriptCallPrimitiveLoop35Result::MlTemporaryExecuteJavaScriptCallPrimitiveLoop35Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptCallSourceExpressionLoop39Result {
    MlTemporaryLowerJavaScriptCallSourceExpressionLoop39Done,
    MlTemporaryLowerJavaScriptCallSourceExpressionLoop39Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_call_source_expression_loop39(mut source: self::JavaScriptCallSource, mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut state: self::JavaScriptCallSourceBuilder, mut index: f64) -> self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop39Result {
    loop {
        return if (index >= (0f64)) {
            {
                let entry = self::ml_array::optional_at(&bindings, self::ml_array::number_index(index));
                if (!(match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && {
                    let source_ml_source1_comparison_operand531 = {
                        let source_ml_source1_nullish_field_receiver526 = (entry.clone(),);
                        if match (source_ml_source1_nullish_field_receiver526.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source1_nullish_field_present527 = ((source_ml_source1_nullish_field_receiver526.clone()).0).clone().expect("a proven present value");
                                match source_ml_source1_nullish_field_present527.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source1_data_property_value528, _) => {
                                        source_ml_source1_data_property_value528
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'identity')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    };
                    {
                        let source_ml_source1_comparison_operand532 = match source.clone() {
                            self::JavaScriptCallSource::SourceConstant(_) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceBinding(source_ml_source1_data_property_value529) => {
                                Some(source_ml_source1_data_property_value529)
                            }
                            self::JavaScriptCallSource::SourceLet(source_ml_source1_data_property_value530, _, _) => {
                                Some(source_ml_source1_data_property_value530)
                            }
                            self::JavaScriptCallSource::SourceIf(_, _, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceSameValue(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceRecur(_) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceSet(_, _, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceGet(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceCall(_, _, _) => {
                                None::<f64>
                            }
                        };
                        match (source_ml_source1_comparison_operand531, source_ml_source1_comparison_operand532.clone()) { (ml_left, Some(ml_right)) => ml_left == ml_right, _ => false }
                    }
                }) {
                    self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop39Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop39Return(Box::new(self::JavaScriptCallSourceResult::SourceResult(Box::new(state.clone()), {
                        let source_ml_source1_nullish_field_receiver533 = (entry.clone(),);
                        if match (source_ml_source1_nullish_field_receiver533.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source1_nullish_field_present534 = ((source_ml_source1_nullish_field_receiver533.clone()).0).clone().expect("a proven present value");
                                match source_ml_source1_nullish_field_present534.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source1_data_property_value535) => {
                                        source_ml_source1_data_property_value535
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'register')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    })))
                } else {
                    {
                        let index_2 = (index - 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop39Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop39Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptCallSourceExpressionLoop40Result {
    MlTemporaryLowerJavaScriptCallSourceExpressionLoop40Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_call_source_expression_loop40(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_temporary_values36: Vec<self::JavaScriptCallSource>, mut source_ml_temporary_index36: self::ml::Big) -> self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop40Result {
    loop {
        return if (source_ml_temporary_index36 < (self::ml::Big::from_u128(source_ml_temporary_values36.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_temporary_values36, source_ml_temporary_index36.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value536, _) => {
                                let source_ml_source1_data_property_value536 = *source_ml_source1_data_property_value536;
                                source_ml_source1_data_property_value536.clone()
                            }
                        };
                        {
                            let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value537) => {
                                    source_ml_source1_data_property_value537
                                }
                            }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_temporary_index36_2 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_temporary_index36) = (current_2.clone(), arguments__2, source_ml_temporary_index36_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop40Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop40Done(Box::new(current.clone()), arguments_)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_call_source_expression_loop42(mut current: self::JavaScriptCallSourceBuilder, arguments_: &[f64], mut index: f64) -> self::JavaScriptCallSourceBuilder {
    loop {
        return if (index < ((arguments_.len() as f64))) {
            {
                let current_2 = self::append_java_script_call_source_instruction(current.clone(), self::JavaScriptCallInstruction::Copy((index + 1f64), (self::ml_array::optional_at(arguments_, self::ml_array::number_index(index))).clone().unwrap_or_else(|| 0f64)));
                {
                    let index_2 = (index + 1f64);
                    {
                        (current, index) = (current_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptCallSourceExpressionLoop41Result {
    MlTemporaryLowerJavaScriptCallSourceExpressionLoop41Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_call_source_expression_loop41(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_temporary_values37: Vec<self::JavaScriptCallSource>, mut source_ml_temporary_index37: self::ml::Big) -> self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop41Result {
    loop {
        return if (source_ml_temporary_index37 < (self::ml::Big::from_u128(source_ml_temporary_values37.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_temporary_values37, source_ml_temporary_index37.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let captured = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value538, _) => {
                                let source_ml_source1_data_property_value538 = *source_ml_source1_data_property_value538;
                                source_ml_source1_data_property_value538.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value539, _) => {
                                    let source_ml_source1_data_property_value539 = *source_ml_source1_data_property_value539;
                                    source_ml_source1_data_property_value539.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value540) => {
                                    source_ml_source1_data_property_value540
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value541) => {
                                    source_ml_source1_data_property_value541
                                }
                            }));
                            {
                                let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match captured.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value542) => {
                                        source_ml_source1_data_property_value542
                                    }
                                }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let source_ml_temporary_index37_2 = source_ml_temporary_index37.add(&self::ml::Big::from_i128(1));
                                    {
                                        (current, arguments_, source_ml_temporary_index37) = (current_2.clone(), arguments__2, source_ml_temporary_index37_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop41Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop41Done(Box::new(current.clone()), arguments_)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptCallSourceExpressionLoop43Result {
    MlTemporaryLowerJavaScriptCallSourceExpressionLoop43Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_call_source_expression_loop43(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_temporary_values38: Vec<self::JavaScriptCallSource>, mut source_ml_temporary_index38: self::ml::Big) -> self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop43Result {
    loop {
        return if (source_ml_temporary_index38 < (self::ml::Big::from_u128(source_ml_temporary_values38.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_temporary_values38, source_ml_temporary_index38.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value543, _) => {
                                let source_ml_source1_data_property_value543 = *source_ml_source1_data_property_value543;
                                source_ml_source1_data_property_value543.clone()
                            }
                        };
                        {
                            let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value544) => {
                                    source_ml_source1_data_property_value544
                                }
                            }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_temporary_index38_2 = source_ml_temporary_index38.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_temporary_index38) = (current_2.clone(), arguments__2, source_ml_temporary_index38_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop43Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop43Done(Box::new(current.clone()), arguments_)
        };
    }
}
