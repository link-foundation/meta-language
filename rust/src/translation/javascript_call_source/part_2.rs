// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
use super::MlTemporaryCodePointUnitsUtf16TextLoop22Result;
use super::MlTemporaryCompareUtf16TextLoop15Result;
use super::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Result;
use super::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result;
use super::MlTemporaryIndexUtf16TextLoop13Result;
use super::MlTemporaryLastIndexUtf16TextLoop14Result;
use super::MlTemporaryMatchUtf16TextLoop12Result;
use super::MlTemporaryPlanUtf16TextSubstitutionLoop26Result;
use super::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result;
use super::MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Result;
use super::MlTemporaryReplaceUtf16TextLoop27Result;
use super::MlTemporarySplitUtf16TextLoop11Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_character_units_utf16_text_loop21;
use super::source_ml_temporary_code_point_units_utf16_text_loop22;
use super::source_ml_temporary_compare_utf16_text_loop15;
use super::source_ml_temporary_display_points_utf16_text_loop23;
use super::source_ml_temporary_display_utf8_from_utf16_text_loop25;
use super::source_ml_temporary_execute_java_script_call_numeric_primitive_loop33;
use super::source_ml_temporary_find_java_script_call_code_function_loop3;
use super::source_ml_temporary_index_utf16_text_loop13;
use super::source_ml_temporary_join_utf16_text_loop16;
use super::source_ml_temporary_last_index_utf16_text_loop14;
use super::source_ml_temporary_match_utf16_text_loop12;
use super::source_ml_temporary_plan_utf16_text_substitution_loop26;
use super::source_ml_temporary_points_utf16_text_loop9;
use super::source_ml_temporary_read_java_script_utf8_string_units_loop4;
use super::source_ml_temporary_read_java_script_utf8_string_units_loop5;
use super::source_ml_temporary_replace_utf16_text_loop27;
use super::source_ml_temporary_replace_utf16_text_loop28;
use super::source_ml_temporary_slice_utf16_text_loop8;
use super::source_ml_temporary_split_utf16_text_loop10;
use super::source_ml_temporary_split_utf16_text_loop11;
use super::source_ml_temporary_trim_utf16_text_loop19;
use super::source_ml_temporary_trim_utf16_text_loop20;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_accessor(value: self::JavaScriptSemanticValue) -> bool {
    if ((self::read_java_script_semantic_value_error(value.clone())) != "") {
        false
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                true
            }
            self::JavaScriptSemanticValue::Null => {
                false
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f58) => {
                {
                    let source_ml_source1_f71 = source_ml_source1_f58;
                    false
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f59) => {
                {
                    let source_ml_source1_f78 = source_ml_source1_f59;
                    false
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f60) => {
                {
                    let source_ml_source1_f85 = source_ml_source1_f60.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f61) => {
                {
                    let source_ml_source1_f92 = source_ml_source1_f61;
                    false
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f62) => {
                {
                    let source_ml_source1_f99 = source_ml_source1_f62.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f63, source_ml_source1_f64) => {
                {
                    let source_ml_source1_f106 = source_ml_source1_f63.clone();
                    {
                        let callable = source_ml_source1_f64;
                        callable
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_code_coordinate(value: f64, limit: f64) -> bool {
    (((self::ml_math::is_safe_integer(value) && (value >= (0f64))) && (value < limit)) && ((value != (0f64)) || (((1f64 / value)) > (0f64))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_call_code_function(program: self::JavaScriptCallProgram, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = ((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value107, _, _) => source_ml_source1_data_property_value107.as_slice() }).len() as f64);
            {
                let source_ml_source1_s108 = self::source_ml_temporary_find_java_script_call_code_function_loop3(program.clone(), identity.clone(), low, high);
                match source_ml_source1_s108.clone() {
                    self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Done => {
                        (-1f64)
                    }
                    self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Return(source_ml_temporary_result3) => {
                        source_ml_temporary_result3
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_code_registers(instruction: self::JavaScriptCallInstruction) -> Vec<f64> {
    match instruction.clone() {
        self::JavaScriptCallInstruction::Constant(destination, source_ml_source1_f109) => {
            let source_ml_source1_f109 = *source_ml_source1_f109;
            vec![destination]
        }
        self::JavaScriptCallInstruction::Copy(destination_2, source) => {
            vec![destination_2, source]
        }
        self::JavaScriptCallInstruction::Argument(destination_3, source_ml_source1_f110) => {
            vec![destination_3]
        }
        self::JavaScriptCallInstruction::Jump(source_ml_source1_f111) => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::Branch(condition, source_ml_source1_f112, source_ml_source1_f113) => {
            vec![condition]
        }
        self::JavaScriptCallInstruction::Call(destination_4, callee, receiver, arguments) => {
            {
                let source_ml_source1_concat_value119 = vec![destination_4, callee, receiver];
                {
                    let source_ml_source1_concat_value120 = arguments;
                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value119), source_ml_source1_concat_value120)
                }
            }
        }
        self::JavaScriptCallInstruction::Return(source_2) => {
            vec![source_2]
        }
        self::JavaScriptCallInstruction::Throw(source_3) => {
            vec![source_3]
        }
        self::JavaScriptCallInstruction::PushHandler(destination_5, source_ml_source1_f114) => {
            vec![destination_5]
        }
        self::JavaScriptCallInstruction::PopHandler => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::NumberBinary(destination_6, left, right, source_ml_source1_f115) => {
            vec![destination_6, left, right]
        }
        self::JavaScriptCallInstruction::SameValue(destination_7, left_2, right_2) => {
            vec![destination_7, left_2, right_2]
        }
        self::JavaScriptCallInstruction::StrictEqual(destination_8, left_3, right_3) => {
            vec![destination_8, left_3, right_3]
        }
        self::JavaScriptCallInstruction::RequireWrite(success) => {
            vec![success]
        }
        self::JavaScriptCallInstruction::Primitive(destination_9, source_ml_source1_f116, arguments_2) => {
            {
                let source_ml_source1_concat_value121 = vec![destination_9];
                {
                    let source_ml_source1_concat_value122 = arguments_2;
                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value121), source_ml_source1_concat_value122)
                }
            }
        }
        self::JavaScriptCallInstruction::OrdinaryGet(destination_10, object, key, receiver_2) => {
            vec![destination_10, object, key, receiver_2]
        }
        self::JavaScriptCallInstruction::OrdinarySet(destination_11, object_2, key_2, value, receiver_3) => {
            vec![destination_11, object_2, key_2, value, receiver_3]
        }
        self::JavaScriptCallInstruction::PropertyKey(destination_12, input, source_ml_source1_f117) => {
            vec![destination_12, input]
        }
        self::JavaScriptCallInstruction::PropertyDefinition(destination_13, target, key_3, attributes, source_ml_source1_f118) => {
            vec![destination_13, target, key_3, attributes]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_constant_error(value: self::JavaScriptSemanticValue) -> String {
    self::read_java_script_semantic_value_error(value.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_strict_equal(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone())) != "") || ((self::read_java_script_semantic_value_error(right.clone())) != "")) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("strict equality requires complete semantic operands")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Null => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f123) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Number(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f129) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(value) => {
                        (left_value == value)
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f130) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f131) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f132) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f133, source_ml_source1_f134) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f124) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f125) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f126) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f127, source_ml_source1_f128) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_whitespace_unit(unit: f64) -> bool {
    ((((((((((((unit >= (9f64)) && (unit <= (13f64))) || (unit == (32f64))) || (unit == (160f64))) || (unit == (5760f64))) || ((unit >= (8192f64)) && (unit <= (8202f64)))) || (unit == (8232f64))) || (unit == (8233f64))) || (unit == (8239f64))) || (unit == (8287f64))) || (unit == (12288f64))) || (unit == (65279f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_utf8_string_units(bytes: &[f64]) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let source_ml_source1_s135 = self::source_ml_temporary_read_java_script_utf8_string_units_loop4(bytes, index);
            match source_ml_source1_s135.clone() {
                self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Done => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source1_s136 = self::source_ml_temporary_read_java_script_utf8_string_units_loop5(bytes, units, index_2);
                                match source_ml_source1_s136.clone() {
                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Done(units_2) => {
                                        units_2
                                    }
                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Return(source_ml_temporary_result5) => {
                                        source_ml_temporary_result5
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Return(source_ml_temporary_result7) => {
                    source_ml_temporary_result7
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_java_script_unsigned_integer(value: f64) -> f64 {
    if (!(value.is_finite()) || (value == (0f64))) {
        0f64
    } else {
        {
            let integral = if (value < (0f64)) {
                (-(((-value)).floor()))
            } else {
                value.floor()
            };
            {
                let remainder = (integral % 4294967296f64);
                if (remainder < (0f64)) {
                    (remainder + 4294967296f64)
                } else {
                    if (remainder == (0f64)) {
                        0f64
                    } else {
                        remainder
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf16_text_integer(value: f64) -> f64 {
    if value.is_nan() {
        0f64
    } else {
        value.trunc()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf16_text_slice_position(value: f64, length: f64) -> f64 {
    {
        let integer = self::read_utf16_text_integer(value);
        if (integer < (0f64)) {
            self::ml_math::max((length + integer), 0f64)
        } else {
            self::ml_math::min(integer, length)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn slice_utf16_text(text: &[f64], start: f64, end: f64) -> Vec<f64> {
    {
        let first = self::read_utf16_text_slice_position(start, (text.len() as f64));
        {
            let last = self::read_utf16_text_slice_position(end, (text.len() as f64));
            {
                let result = Vec::<f64>::new();
                {
                    let at = first;
                    {
                        let result_2 = self::source_ml_temporary_slice_utf16_text_loop8(text, last, result, at);
                        result_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn substring_utf16_text(text: &[f64], start: f64, end: f64) -> Vec<f64> {
    {
        let first = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(start), 0f64), (text.len() as f64));
        {
            let last = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(end), 0f64), (text.len() as f64));
            self::slice_utf16_text(text, self::ml_math::min(first, last), self::ml_math::max(first, last))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn character_utf16_text(text: &[f64], position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn relative_character_utf16_text(text: &[f64], position: f64) -> Vec<Vec<f64>> {
    {
        let integer = self::read_utf16_text_integer(position);
        {
            let at = if (integer < (0f64)) {
                ((text.len() as f64) + integer)
            } else {
                integer
            };
            if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
                Vec::<Vec<f64>>::new()
            } else {
                vec![vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn code_point_utf16_text(text: &[f64], position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            {
                let first = (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                    vec![if ((((first >= (55296f64)) && (first <= (56319f64))) && (second >= (56320f64))) && (second <= (57343f64))) {
                        (((65536f64 + ((first - 55296f64) * 1024f64)) + second) - 56320f64)
                    } else {
                        first
                    }]
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn points_utf16_text(text: &[f64]) -> Vec<Vec<f64>> {
    {
        let result = Vec::<Vec<f64>>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_temporary_points_utf16_text_loop9(text, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn split_utf16_text(text: &[f64], separator: &[f64], limit: f64, missing: bool) -> Vec<Vec<f64>> {
    {
        let count = self::normalize_java_script_unsigned_integer(limit);
        {
            let result = Vec::<Vec<f64>>::new();
            if (count == (0f64)) {
                result
            } else {
                if missing {
                    vec![self::slice_utf16_text(text, 0f64, (text.len() as f64))]
                } else {
                    if (((separator.len() as f64)) == (0f64)) {
                        {
                            let at = 0f64;
                            {
                                let result_2 = self::source_ml_temporary_split_utf16_text_loop10(text, count, result, at);
                                result_2
                            }
                        }
                    } else {
                        {
                            let start = 0f64;
                            {
                                let source_ml_source1_s137 = self::source_ml_temporary_split_utf16_text_loop11(text, separator, count, result, start);
                                match source_ml_source1_s137.clone() {
                                    self::MlTemporarySplitUtf16TextLoop11Result::MlTemporarySplitUtf16TextLoop11Done(result_3, start_2) => {
                                        {
                                            let result_4 = { let (ml_append_right, ml_append_left) = (vec![self::slice_utf16_text(text, start_2, (text.len() as f64))], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                            result_4
                                        }
                                    }
                                    self::MlTemporarySplitUtf16TextLoop11Result::MlTemporarySplitUtf16TextLoop11Return(source_ml_temporary_result11) => {
                                        source_ml_temporary_result11
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
pub fn match_utf16_text(text: &[f64], search: &[f64], start: f64) -> bool {
    if ((start < (0f64)) || (((start + (search.len() as f64))) > ((text.len() as f64)))) {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source1_s138 = self::source_ml_temporary_match_utf16_text_loop12(text, search, start, at);
                match source_ml_source1_s138.clone() {
                    self::MlTemporaryMatchUtf16TextLoop12Result::MlTemporaryMatchUtf16TextLoop12Done => {
                        true
                    }
                    self::MlTemporaryMatchUtf16TextLoop12Result::MlTemporaryMatchUtf16TextLoop12Return(source_ml_temporary_result12) => {
                        source_ml_temporary_result12
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn index_utf16_text(text: &[f64], search: &[f64], position: f64) -> f64 {
    {
        let start = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(position), 0f64), (text.len() as f64));
        {
            let at = start;
            {
                let source_ml_source1_s139 = self::source_ml_temporary_index_utf16_text_loop13(text, search, at);
                match source_ml_source1_s139.clone() {
                    self::MlTemporaryIndexUtf16TextLoop13Result::MlTemporaryIndexUtf16TextLoop13Done => {
                        (-1f64)
                    }
                    self::MlTemporaryIndexUtf16TextLoop13Result::MlTemporaryIndexUtf16TextLoop13Return(source_ml_temporary_result13) => {
                        source_ml_temporary_result13
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn last_index_utf16_text(text: &[f64], search: &[f64], position: f64) -> f64 {
    {
        let integer = if position.is_nan() {
            f64::INFINITY
        } else {
            self::read_utf16_text_integer(position)
        };
        {
            let start = self::ml_math::min(self::ml_math::max(integer, 0f64), ((text.len() as f64) - (search.len() as f64)));
            {
                let at = start;
                {
                    let source_ml_source1_s140 = self::source_ml_temporary_last_index_utf16_text_loop14(text, search, at);
                    match source_ml_source1_s140.clone() {
                        self::MlTemporaryLastIndexUtf16TextLoop14Result::MlTemporaryLastIndexUtf16TextLoop14Done => {
                            (-1f64)
                        }
                        self::MlTemporaryLastIndexUtf16TextLoop14Result::MlTemporaryLastIndexUtf16TextLoop14Return(source_ml_temporary_result14) => {
                            source_ml_temporary_result14
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_utf16_text(left: &[f64], right: &[f64]) -> f64 {
    {
        let count = self::ml_math::min((left.len() as f64), (right.len() as f64));
        {
            let at = 0f64;
            {
                let source_ml_source1_s141 = self::source_ml_temporary_compare_utf16_text_loop15(left, right, count, at);
                match source_ml_source1_s141.clone() {
                    self::MlTemporaryCompareUtf16TextLoop15Result::MlTemporaryCompareUtf16TextLoop15Done => {
                        if (((left.len() as f64)) == ((right.len() as f64))) {
                            0f64
                        } else {
                            if (((left.len() as f64)) < ((right.len() as f64))) {
                                (-1f64)
                            } else {
                                1f64
                            }
                        }
                    }
                    self::MlTemporaryCompareUtf16TextLoop15Result::MlTemporaryCompareUtf16TextLoop15Return(source_ml_temporary_result15) => {
                        source_ml_temporary_result15
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn starts_utf16_text(text: &[f64], search: &[f64]) -> bool {
    self::match_utf16_text(text, search, 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ends_utf16_text(text: &[f64], search: &[f64]) -> bool {
    self::match_utf16_text(text, search, ((text.len() as f64) - (search.len() as f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn includes_utf16_text(text: &[f64], search: &[f64]) -> bool {
    ((self::index_utf16_text(text, search, 0f64)) >= (0f64))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn join_utf16_text(values: &[Vec<f64>], separator: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_temporary_join_utf16_text_loop16(values, separator, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn trim_utf16_text(text: &[f64], start: bool, end: bool) -> Vec<f64> {
    {
        let first = 0f64;
        {
            let last = (text.len() as f64);
            {
                let first_3 = if start {
                    {
                        let first_2 = self::source_ml_temporary_trim_utf16_text_loop19(text, first, last);
                        first_2
                    }
                } else {
                    first
                };
                {
                    let last_3 = if end {
                        {
                            let last_2 = self::source_ml_temporary_trim_utf16_text_loop20(text, first_3, last);
                            last_2
                        }
                    } else {
                        last
                    };
                    self::slice_utf16_text(text, first_3, last_3)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn character_units_utf16_text(values: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_temporary_character_units_utf16_text_loop21(values, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn code_point_units_utf16_text(values: &[f64]) -> Vec<f64> {
    {
        let result = vec![(-1f64)];
        {
            let at = 0f64;
            {
                let source_ml_source1_s142 = self::source_ml_temporary_code_point_units_utf16_text_loop22(values, result, at);
                match source_ml_source1_s142.clone() {
                    self::MlTemporaryCodePointUnitsUtf16TextLoop22Result::MlTemporaryCodePointUnitsUtf16TextLoop22Done(result_2) => {
                        result_2
                    }
                    self::MlTemporaryCodePointUnitsUtf16TextLoop22Result::MlTemporaryCodePointUnitsUtf16TextLoop22Return(source_ml_temporary_result22) => {
                        source_ml_temporary_result22
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf8_text_units(bytes: &[f64]) -> Vec<f64> {
    self::read_java_script_utf8_string_units(bytes)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn display_points_utf16_text(text: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_temporary_display_points_utf16_text_loop23(text, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn display_utf8_from_utf16_text(text: &[f64]) -> Vec<f64> {
    {
        let bytes = Vec::<f64>::new();
        {
            let points = self::display_points_utf16_text(text);
            {
                let at = 0f64;
                {
                    let bytes_2 = self::source_ml_temporary_display_utf8_from_utf16_text_loop25(bytes, &(points), at);
                    bytes_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_utf16_text_substitution(replacement: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let start = 0f64;
            {
                let at = 0f64;
                {
                    let source_ml_source1_s143 = self::source_ml_temporary_plan_utf16_text_substitution_loop26(replacement, result, start, at);
                    match source_ml_source1_s143.clone() {
                        self::MlTemporaryPlanUtf16TextSubstitutionLoop26Result::MlTemporaryPlanUtf16TextSubstitutionLoop26Done(result_2, start_2) => {
                            {
                                let result_4 = if (start_2 < ((replacement.len() as f64))) {
                                    {
                                        let result_3 = { let (ml_append_right, ml_append_left) = (vec![0f64, start_2, (replacement.len() as f64)], result_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                        result_3
                                    }
                                } else {
                                    result_2
                                };
                                result_4
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
pub fn replace_utf16_text(text: &[f64], search: &[f64], replacement: &[f64], all: bool) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let plan = self::plan_utf16_text_substitution(replacement);
            {
                let start = 0f64;
                {
                    let from = 0f64;
                    {
                        let source_ml_source1_s144 = self::source_ml_temporary_replace_utf16_text_loop27(text, search, replacement, all, result, &(plan), start, from);
                        match source_ml_source1_s144.clone() {
                            self::MlTemporaryReplaceUtf16TextLoop27Result::MlTemporaryReplaceUtf16TextLoop27Done(result_2, start_2) => {
                                {
                                    let at = start_2;
                                    {
                                        let result_3 = self::source_ml_temporary_replace_utf16_text_loop28(text, result_2, at);
                                        result_3
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
pub fn read_java_script_call_primitive_arity_error(operation: &str, count: f64) -> String {
    if ((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || (count > (4096f64))) {
        String::from("primitive operands exceed their explicit call register budget")
    } else {
        if (((operation == "read minimum number") || (operation == "read maximum number")) || (operation == "construct text units")) {
            String::from("")
        } else {
            {
                let unary = vec![String::from("truncate number"), String::from("read text length"), String::from("trim text"), String::from("trim text start"), String::from("trim text end")];
                {
                    let binary = vec![String::from("read text code point"), String::from("read text character"), String::from("read relative text character"), String::from("concatenate text"), String::from("compare text"), String::from("test text prefix"), String::from("test text suffix"), String::from("test text inclusion")];
                    {
                        let ternary = vec![String::from("slice text"), String::from("find text"), String::from("find last text")];
                        {
                            let arity = if self::ml_array::includes(&unary, &operation.to_owned(), 0f64, false, |a, b| a == b) {
                                1f64
                            } else {
                                if self::ml_array::includes(&binary, &operation.to_owned(), 0f64, false, |a, b| a == b) {
                                    2f64
                                } else {
                                    if self::ml_array::includes(&ternary, &operation.to_owned(), 0f64, false, |a, b| a == b) {
                                        3f64
                                    } else {
                                        (-1f64)
                                    }
                                }
                            };
                            if (arity < (0f64)) {
                                String::from("primitive instructions require their actual registered operation")
                            } else {
                                if (count != arity) {
                                    String::from("primitive instructions require every actual operand coordinate")
                                } else {
                                    String::from("")
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
pub fn execute_java_script_call_numeric_primitive(operation: &str, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let numbers = Vec::<f64>::new();
        {
            let source_ml_temporary_values32 = values;
            {
                let source_ml_temporary_index32 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s145 = self::source_ml_temporary_execute_java_script_call_numeric_primitive_loop33(numbers, source_ml_temporary_values32, source_ml_temporary_index32.clone());
                    match source_ml_source1_s145.clone() {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Done(numbers_2) => {
                            if (operation == "truncate number") {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(((self::ml_array::optional_at(&numbers_2, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| f64::NAN)).trunc())))
                            } else {
                                if (operation == "read minimum number") {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::ml_math::min_of(&self::ml_array::append(Vec::<f64>::new(), numbers_2)))))
                                } else {
                                    if (operation == "read maximum number") {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::ml_math::max_of(&self::ml_array::append(Vec::<f64>::new(), numbers_2)))))
                                    } else {
                                        if (operation == "construct text units") {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::character_units_utf16_text(&(numbers_2)))))
                                        } else {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("the numeric primitive executor needs its actual registered operation"))
                                        }
                                    }
                                }
                            }
                        }
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop33Return(source_ml_temporary_result33) => {
                            let source_ml_temporary_result33 = *source_ml_temporary_result33;
                            source_ml_temporary_result33.clone()
                        }
                    }
                }
            }
        }
    }
}
