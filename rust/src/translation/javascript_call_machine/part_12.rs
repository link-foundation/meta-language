// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryCompareUtf16TextLoop99Result;
use super::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin89;
use super::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90;
use super::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Result;
use super::MlSource1TemporaryIndexUtf16TextLoop97Result;
use super::MlSource1TemporaryLastIndexUtf16TextLoop98Result;
use super::MlSource1TemporaryMatchUtf16TextLoop96Result;
use super::apply_big_integer_bitwise_operation;
use super::compare_java_script_number_big_integer;
use super::compare_java_script_string_big_integer_units;
use super::convert_java_script_big_integer_number;
use super::convert_java_script_boolean_big_integer;
use super::convert_java_script_number_big_integer;
use super::convert_java_script_string_big_integer_units;
use super::convert_java_script_string_number_units;
use super::divide_java_script_big_integer;
use super::execute_java_script_call_number_bits;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::power_java_script_big_integer;
use super::read_java_script_big_integer_conversion_error_class;
use super::read_java_script_big_integer_conversion_message_pieces;
use super::read_java_script_call_integer_arity;
use super::read_java_script_call_integer_bit_code;
use super::read_java_script_call_integer_width;
use super::source_ml_source1_temporary_character_units_utf16_text_loop105;
use super::source_ml_source1_temporary_compare_utf16_text_loop99;
use super::source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop117;
use super::source_ml_source1_temporary_index_utf16_text_loop97;
use super::source_ml_source1_temporary_last_index_utf16_text_loop98;
use super::source_ml_source1_temporary_match_utf16_text_loop96;
use super::source_ml_source1_temporary_read_java_script_call_integer_text_loop91;
use super::source_ml_source1_temporary_slice_utf16_text_loop92;
use super::source_ml_source1_temporary_trim_utf16_text_loop103;
use super::source_ml_source1_temporary_trim_utf16_text_loop104;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_integer_primitive(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    if operation.contains(&self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])) {
        self::execute_java_script_call_number_bits(operation, values)
    } else {
        if (((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 116, 101, 103, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116, 32, 105, 110, 116, 101, 103, 101, 114])))) {
            self::execute_java_script_call_mixed_integer_comparison(operation, values)
        } else {
            {
                let first = (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                {
                    let second = (self::ml_array::optional_at(&values, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(0)));
                    match first.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Number(_) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Integer(first_value) => {
                            match second.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Integer(value) => {
                                    if ((((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114])))) && (value == (self::ml::Big::from_i128(0)))) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111]))
                                    } else {
                                        if (((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]))) && (value < (self::ml::Big::from_i128(0)))) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![69, 120, 112, 111, 110, 101, 110, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101]))
                                        } else {
                                            {
                                                let left_width = self::read_java_script_call_integer_width(first_value.clone());
                                                {
                                                    let right_width = self::read_java_script_call_integer_width(value.clone());
                                                    if ((left_width < (0f64)) || (right_width < (0f64))) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 105, 110, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (first_value < value) {
                                                                (-1f64)
                                                            } else {
                                                                if (first_value > value) {
                                                                    1f64
                                                                } else {
                                                                    0f64
                                                                }
                                                            })))
                                                        } else {
                                                            if (((*operation) == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) && (((left_width + right_width)) > (4096f64))) {
                                                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 101, 115, 116, 105, 109, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                            } else {
                                                                if ((((((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]))) && (first_value != (self::ml::Big::from_i128(0)))) && (first_value != (self::ml::Big::from_i128(1)))) && (first_value != (self::ml::Big::from_i128(-1)))) && ((value.mul(&{
                                                                    let source_ml_source2_integer_result903 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(left_width);
                                                                    if (((source_ml_source2_integer_result903.len() as f64)) == (1f64)) {
                                                                        (self::ml_array::optional_at(&source_ml_source2_integer_result903, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                    } else {
                                                                        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(left_width)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                                                                    }
                                                                })) > (self::ml::Big::from_i128(4096)))) {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 101, 120, 112, 111, 110, 101, 110, 116, 105, 97, 116, 105, 111, 110, 32, 101, 115, 116, 105, 109, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                } else {
                                                                    {
                                                                        let source_ml_source2_s908 = if (((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116])))) {
                                                                            {
                                                                                let growing = if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]))) {
                                                                                    (value >= (self::ml::Big::from_i128(0)))
                                                                                } else {
                                                                                    (value < (self::ml::Big::from_i128(0)))
                                                                                };
                                                                                {
                                                                                    let count = if (value < (self::ml::Big::from_i128(0))) {
                                                                                        value.neg()
                                                                                    } else {
                                                                                        value.clone()
                                                                                    };
                                                                                    if (((first_value != (self::ml::Big::from_i128(0))) && growing) && (count > ({
                                                                                        let source_ml_source2_integer_operand911 = (4096f64 - left_width);
                                                                                        {
                                                                                            let source_ml_source2_integer_result912 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(source_ml_source2_integer_operand911);
                                                                                            if (((source_ml_source2_integer_result912.len() as f64)) == (1f64)) {
                                                                                                (self::ml_array::optional_at(&source_ml_source2_integer_result912, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                                            } else {
                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_integer_operand911)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                                                                                            }
                                                                                        }
                                                                                    }))) {
                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin89::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin89Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 115, 104, 105, 102, 116, 32, 101, 115, 116, 105, 109, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))))
                                                                                    } else {
                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin89::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin89Next
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin89::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin89Next
                                                                        };
                                                                        match source_ml_source2_s908.clone() {
                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin89::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin89Next => {
                                                                                {
                                                                                    let code = self::read_java_script_call_integer_bit_code(operation);
                                                                                    {
                                                                                        let result = self::ml::Big::from_i128(0);
                                                                                        {
                                                                                            let source_ml_source2_s913 = if ((*operation) == (self::ml_string::Text::from_units(vec![97, 100, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                {
                                                                                                    let result_2 = first_value.add(&value);
                                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90Next(result_2.clone())
                                                                                                }
                                                                                            } else {
                                                                                                if ((*operation) == (self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                    {
                                                                                                        let result_3 = first_value.sub(&value);
                                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90Next(result_3.clone())
                                                                                                    }
                                                                                                } else {
                                                                                                    if ((*operation) == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                        {
                                                                                                            let result_4 = first_value.mul(&value);
                                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90Next(result_4.clone())
                                                                                                        }
                                                                                                    } else {
                                                                                                        if (((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114])))) {
                                                                                                            {
                                                                                                                let result_5 = (self::ml_array::optional_at(&self::divide_java_script_big_integer(first_value.clone(), value.clone()), self::ml_array::number_index(if ((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                                    0f64
                                                                                                                } else {
                                                                                                                    1f64
                                                                                                                }))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90Next(result_5.clone())
                                                                                                            }
                                                                                                        } else {
                                                                                                            if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]))) {
                                                                                                                {
                                                                                                                    let result_6 = (self::ml_array::optional_at(&self::power_java_script_big_integer(first_value.clone(), value.clone()), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90Next(result_6.clone())
                                                                                                                }
                                                                                                            } else {
                                                                                                                if ((*operation) == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 105, 110, 116, 101, 103, 101, 114]))) {
                                                                                                                    {
                                                                                                                        let result_7 = first_value.neg();
                                                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90Next(result_7.clone())
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    if (code >= (0f64)) {
                                                                                                                        {
                                                                                                                            let result_8 = self::apply_big_integer_bitwise_operation(code, first_value.clone(), value.clone());
                                                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90Next(result_8.clone())
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))))
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            };
                                                                                            match source_ml_source2_s913.clone() {
                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90Next(result_9) => {
                                                                                                    if ((self::read_java_script_call_integer_width(result_9.clone())) < (0f64)) {
                                                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 117, 116, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                                                    } else {
                                                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(result_9.clone())))
                                                                                                    }
                                                                                                }
                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin90Return(source_ml_source1_temporary_result90) => {
                                                                                                    let source_ml_source1_temporary_result90 = *source_ml_source1_temporary_result90;
                                                                                                    source_ml_source1_temporary_result90.clone()
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin89::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin89Return(source_ml_source1_temporary_result90_2) => {
                                                                                let source_ml_source1_temporary_result90_2 = *source_ml_source1_temporary_result90_2;
                                                                                source_ml_source1_temporary_result90_2.clone()
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
                                self::JavaScriptSemanticValue::Text(_) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_mixed_integer_comparison(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let first = (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let second = (self::ml_array::optional_at(&values, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            match second.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Null => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Boolean(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Number(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Integer(second_value) => {
                    if ((self::read_java_script_call_integer_width(second_value.clone())) < (0f64)) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                    } else {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 116, 101, 103, 101, 114]))) {
                            match first.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                }
                                self::JavaScriptSemanticValue::Number(value) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::compare_java_script_number_big_integer(value, second_value.clone()))))
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                }
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                }
                            }
                        } else {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116, 32, 105, 110, 116, 101, 103, 101, 114]))) {
                                match first.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 116, 101, 120, 116, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 116, 101, 120, 116, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 116, 101, 120, 116, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                                    }
                                    self::JavaScriptSemanticValue::Number(_) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 116, 101, 120, 116, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                                    }
                                    self::JavaScriptSemanticValue::Integer(_) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 116, 101, 120, 116, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                                    }
                                    self::JavaScriptSemanticValue::Text(units) => {
                                        if (((units.len() as f64)) > (4096f64)) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                        } else {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::compare_java_script_string_big_integer_units(units, second_value.clone()))))
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 116, 101, 120, 116, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                                    }
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 116, 101, 120, 116, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                                    }
                                }
                            } else {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_number_conversion(operation: &self::ml_string::Text, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    if (((*operation) != (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) && ((*operation) != (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(f64::NAN)))
            }
            self::JavaScriptSemanticValue::Null => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(0f64)))
            }
            self::JavaScriptSemanticValue::Boolean(value_value) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if value_value {
                    1f64
                } else {
                    0f64
                })))
            }
            self::JavaScriptSemanticValue::Number(_) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
            }
            self::JavaScriptSemanticValue::Integer(value_value_2) => {
                if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114]))
                } else {
                    if ((self::read_java_script_call_integer_width(value_value_2.clone())) < (0f64)) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![101, 120, 112, 108, 105, 99, 105, 116, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                    } else {
                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::convert_java_script_big_integer_number(value_value_2.clone()))))
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(units) => {
                if (((units.len() as f64)) > (4096f64)) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                } else {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::convert_java_script_string_number_units(units))))
                }
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 83, 121, 109, 98, 111, 108, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114]))
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108]))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_call_integer_conversion(form: &self::ml_string::Text, value: &self::ml_string::Text) -> self::JavaScriptCallPrimitiveResult {
    {
        let pieces = self::read_java_script_big_integer_conversion_message_pieces(form.clone());
        self::JavaScriptCallPrimitiveResult::PrimitiveError(self::read_java_script_big_integer_conversion_error_class(form.clone()), (((self::ml_array::optional_at(&pieces, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))).concat(value)).concat(&(self::ml_array::optional_at(&pieces, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_text(units: &[f64]) -> self::ml_string::Text {
    {
        let text = self::ml_string::Text::from_units(vec![]);
        {
            let index = 0f64;
            {
                let text_2 = self::source_ml_source1_temporary_read_java_script_call_integer_text_loop91(units, text.clone(), index);
                text_2.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_integer_conversion(operation: &self::ml_string::Text, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    if (((*operation) != (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114]))) && ((*operation) != (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 101, 103, 101, 114])))) {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::refuse_java_script_call_integer_conversion(&(self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])), &(self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])))
            }
            self::JavaScriptSemanticValue::Null => {
                self::refuse_java_script_call_integer_conversion(&(self::ml_string::Text::from_units(vec![110, 117, 108, 108])), &(self::ml_string::Text::from_units(vec![110, 117, 108, 108])))
            }
            self::JavaScriptSemanticValue::Boolean(value_value) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(self::convert_java_script_boolean_big_integer(value_value))))
            }
            self::JavaScriptSemanticValue::Number(value_value_2) => {
                if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 101, 103, 101, 114]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 78, 117, 109, 98, 101, 114, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116]))
                } else {
                    {
                        let converted = self::convert_java_script_number_big_integer(value_value_2);
                        if (((converted.len() as f64)) != (1f64)) {
                            self::refuse_java_script_call_integer_conversion(&(self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])), &(self::ml_string::Text::from_text(&self::ml_number::js_number(value_value_2))))
                        } else {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer((self::ml_array::optional_at(&converted, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)))))
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
            }
            self::JavaScriptSemanticValue::Text(units) => {
                if (((units.len() as f64)) > (4096f64)) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                } else {
                    {
                        let converted_2 = self::convert_java_script_string_big_integer_units(units.clone());
                        if (((converted_2.len() as f64)) != (1f64)) {
                            self::refuse_java_script_call_integer_conversion(&(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])), &(self::read_java_script_call_integer_text(&(units))))
                        } else {
                            {
                                let integer = (self::ml_array::optional_at(&converted_2, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                if ((self::read_java_script_call_integer_width(integer.clone())) < (0f64)) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 111, 117, 116, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                } else {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(integer.clone())))
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 83, 121, 109, 98, 111, 108, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116]))
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108]))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_exception_class(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 114, 114, 111, 114]))) {
        self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114])
    } else {
        if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112, 101, 69, 114, 114, 111, 114]))) {
            self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
        } else {
            if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 97, 110, 103, 101, 69, 114, 114, 111, 114]))) {
                self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
            } else {
                if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114]))) {
                    self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
                } else {
                    if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114]))) {
                        self::ml_string::Text::from_units(vec![82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114])
                    } else {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 85, 82, 73, 69, 114, 114, 111, 114]))) {
                            self::ml_string::Text::from_units(vec![85, 82, 73, 69, 114, 114, 111, 114])
                        } else {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 118, 97, 108, 69, 114, 114, 111, 114]))) {
                                self::ml_string::Text::from_units(vec![69, 118, 97, 108, 69, 114, 114, 111, 114])
                            } else {
                                self::ml_string::Text::from_units(vec![])
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

pub fn slice_utf16_text(text: Vec<f64>, start: f64, end: f64) -> Vec<f64> {
    {
        let first = self::read_utf16_text_slice_position(start, (text.len() as f64));
        {
            let last = self::read_utf16_text_slice_position(end, (text.len() as f64));
            {
                let result = Vec::<f64>::new();
                {
                    let result_2 = self::source_ml_source1_temporary_slice_utf16_text_loop92(text, last, result, first);
                    result_2
                }
            }
        }
    }
}

pub fn character_utf16_text(text: Vec<f64>, position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            vec![(self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]
        }
    }
}

pub fn relative_character_utf16_text(text: Vec<f64>, position: f64) -> Vec<Vec<f64>> {
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
                vec![vec![(self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]]
            }
        }
    }
}

pub fn code_point_utf16_text(text: Vec<f64>, position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            {
                let first = (self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(&text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
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

pub fn match_utf16_text(text: Vec<f64>, search: Vec<f64>, start: f64) -> bool {
    if ((start < (0f64)) || (((start + (search.len() as f64))) > ((text.len() as f64)))) {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source2_s958 = self::source_ml_source1_temporary_match_utf16_text_loop96(text, search, start, at);
                match source_ml_source2_s958.clone() {
                    self::MlSource1TemporaryMatchUtf16TextLoop96Result::MlSource1TemporaryMatchUtf16TextLoop96Done => {
                        true
                    }
                    self::MlSource1TemporaryMatchUtf16TextLoop96Result::MlSource1TemporaryMatchUtf16TextLoop96Return(source_ml_source1_temporary_result96) => {
                        source_ml_source1_temporary_result96
                    }
                }
            }
        }
    }
}

pub fn index_utf16_text(text: Vec<f64>, search: Vec<f64>, position: f64) -> f64 {
    {
        let start = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(position), 0f64), (text.len() as f64));
        {
            let source_ml_source2_s959 = self::source_ml_source1_temporary_index_utf16_text_loop97(text, search, start);
            match source_ml_source2_s959.clone() {
                self::MlSource1TemporaryIndexUtf16TextLoop97Result::MlSource1TemporaryIndexUtf16TextLoop97Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryIndexUtf16TextLoop97Result::MlSource1TemporaryIndexUtf16TextLoop97Return(source_ml_source1_temporary_result97) => {
                    source_ml_source1_temporary_result97
                }
            }
        }
    }
}

pub fn last_index_utf16_text(text: Vec<f64>, search: Vec<f64>, position: f64) -> f64 {
    {
        let integer = if position.is_nan() {
            f64::INFINITY
        } else {
            self::read_utf16_text_integer(position)
        };
        {
            let start = self::ml_math::min(self::ml_math::max(integer, 0f64), ((text.len() as f64) - (search.len() as f64)));
            {
                let source_ml_source2_s960 = self::source_ml_source1_temporary_last_index_utf16_text_loop98(text, search, start);
                match source_ml_source2_s960.clone() {
                    self::MlSource1TemporaryLastIndexUtf16TextLoop98Result::MlSource1TemporaryLastIndexUtf16TextLoop98Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryLastIndexUtf16TextLoop98Result::MlSource1TemporaryLastIndexUtf16TextLoop98Return(source_ml_source1_temporary_result98) => {
                        source_ml_source1_temporary_result98
                    }
                }
            }
        }
    }
}

pub fn compare_utf16_text(left: Vec<f64>, right: Vec<f64>) -> f64 {
    {
        let count = self::ml_math::min((left.len() as f64), (right.len() as f64));
        {
            let at = 0f64;
            {
                let source_ml_source2_s961 = self::source_ml_source1_temporary_compare_utf16_text_loop99(left.clone(), right.clone(), count, at);
                match source_ml_source2_s961.clone() {
                    self::MlSource1TemporaryCompareUtf16TextLoop99Result::MlSource1TemporaryCompareUtf16TextLoop99Done => {
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
                    self::MlSource1TemporaryCompareUtf16TextLoop99Result::MlSource1TemporaryCompareUtf16TextLoop99Return(source_ml_source1_temporary_result99) => {
                        source_ml_source1_temporary_result99
                    }
                }
            }
        }
    }
}

pub fn starts_utf16_text(text: Vec<f64>, search: Vec<f64>) -> bool {
    self::match_utf16_text(text, search, 0f64)
}

pub fn ends_utf16_text(text: Vec<f64>, search: Vec<f64>) -> bool {
    self::match_utf16_text(text.clone(), search.clone(), ((text.len() as f64) - (search.len() as f64)))
}

pub fn includes_utf16_text(text: Vec<f64>, search: Vec<f64>) -> bool {
    ((self::index_utf16_text(text, search, 0f64)) >= (0f64))
}

pub fn trim_utf16_text(text: Vec<f64>, start: bool, end: bool) -> Vec<f64> {
    {
        let first = 0f64;
        {
            let last = (text.len() as f64);
            {
                let first_3 = if start {
                    {
                        let first_2 = self::source_ml_source1_temporary_trim_utf16_text_loop103(text.clone(), first, last);
                        first_2
                    }
                } else {
                    first
                };
                {
                    let last_3 = if end {
                        {
                            let last_2 = self::source_ml_source1_temporary_trim_utf16_text_loop104(text.clone(), first_3, last);
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

pub fn character_units_utf16_text(values: Vec<f64>) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_character_units_utf16_text_loop105(values, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_primitive_arity_error(operation: &self::ml_string::Text, count: f64) -> self::ml_string::Text {
    if ((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || (count > (4096f64))) {
        self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 97, 108, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 98, 117, 100, 103, 101, 116])
    } else {
        if ((self::read_java_script_source_exception_class(operation)) != (self::ml_string::Text::from_units(vec![]))) {
            if (count == (2f64)) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 109, 101, 115, 115, 97, 103, 101, 32, 97, 110, 100, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103, 101, 32, 112, 114, 101, 115, 101, 110, 99, 101])
            }
        } else {
            {
                let integer_arity = self::read_java_script_call_integer_arity(operation);
                if (integer_arity != (0f64)) {
                    if (count == integer_arity) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])
                    }
                } else {
                    if ((((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 105, 110, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 97, 120, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114])))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 116, 101, 120, 116, 32, 117, 110, 105, 116, 115])))) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        {
                            let unary = vec![self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116]), self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 101, 110, 100]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![102, 108, 111, 111, 114, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 101, 105, 108, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 98, 101, 114, 32, 115, 105, 103, 110]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 113, 117, 97, 114, 101, 32, 114, 111, 111, 116]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 115, 97, 102, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114])];
                            {
                                let binary = vec![self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 111, 100, 101, 32, 112, 111, 105, 110, 116]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 112, 114, 101, 102, 105, 120]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 115, 117, 102, 102, 105, 120]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 105, 110, 99, 108, 117, 115, 105, 111, 110])];
                                {
                                    let ternary = vec![self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 108, 97, 115, 116, 32, 116, 101, 120, 116])];
                                    {
                                        let arity = if self::ml_array::includes(&unary, operation, 0f64, false, |a, b| a == b) {
                                            1f64
                                        } else {
                                            if self::ml_array::includes(&binary, operation, 0f64, false, |a, b| a == b) {
                                                2f64
                                            } else {
                                                if self::ml_array::includes(&ternary, operation, 0f64, false, |a, b| a == b) {
                                                    3f64
                                                } else {
                                                    (-1f64)
                                                }
                                            }
                                        };
                                        if (arity < (0f64)) {
                                            self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
                                        } else {
                                            if (count != arity) {
                                                self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])
                                            } else {
                                                self::ml_string::Text::from_units(vec![])
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
pub fn execute_java_script_call_numeric_primitive(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let numbers = Vec::<f64>::new();
        {
            let source_ml_source1_temporary_index116 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s965 = self::source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop117(numbers, values, source_ml_source1_temporary_index116.clone());
                match source_ml_source2_s965.clone() {
                    self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Done(numbers_2) => {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114]))) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(((self::ml_array::optional_at(&numbers_2, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| f64::NAN)).trunc())))
                        } else {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 105, 110, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::ml_math::min_of(&self::ml_array::append(Vec::<f64>::new(), numbers_2)))))
                            } else {
                                if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 97, 120, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]))) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::ml_math::max_of(&self::ml_array::append(Vec::<f64>::new(), numbers_2)))))
                                } else {
                                    if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 116, 101, 120, 116, 32, 117, 110, 105, 116, 115]))) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::character_units_utf16_text(numbers_2))))
                                    } else {
                                        if self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![102, 108, 111, 111, 114, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 101, 105, 108, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 98, 101, 114, 32, 115, 105, 103, 110]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 113, 117, 97, 114, 101, 32, 114, 111, 111, 116]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 115, 97, 102, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114])], operation, 0f64, false, |a, b| a == b) {
                                            self::execute_java_script_call_unary_number(operation, (self::ml_array::optional_at(&numbers_2, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| f64::NAN))
                                        } else {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Return(source_ml_source1_temporary_result117) => {
                        let source_ml_source1_temporary_result117 = *source_ml_source1_temporary_result117;
                        source_ml_source1_temporary_result117.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_unary_number(operation: &self::ml_string::Text, number: f64) -> self::JavaScriptCallPrimitiveResult {
    if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114]))) {
        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::ml_math::is_integer(number))))
    } else {
        if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 115, 97, 102, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114]))) {
            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::ml_math::is_safe_integer(number))))
        } else {
            if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![102, 108, 111, 111, 114, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 101, 105, 108, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 98, 101, 114, 32, 115, 105, 103, 110]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 113, 117, 97, 114, 101, 32, 114, 111, 111, 116])], operation, 0f64, false, |a, b| a == b)) {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![117, 110, 97, 114, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
            } else {
                {
                    let result = if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 110, 117, 109, 98, 101, 114]))) {
                        number.abs()
                    } else {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![102, 108, 111, 111, 114, 32, 110, 117, 109, 98, 101, 114]))) {
                            number.floor()
                        } else {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 101, 105, 108, 32, 110, 117, 109, 98, 101, 114]))) {
                                number.ceil()
                            } else {
                                if ((*operation) == (self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100, 32, 110, 117, 109, 98, 101, 114]))) {
                                    self::ml_math::round(number)
                                } else {
                                    if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 98, 101, 114, 32, 115, 105, 103, 110]))) {
                                        self::ml_math::sign(number)
                                    } else {
                                        number.sqrt()
                                    }
                                }
                            }
                        }
                    };
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(result)))
                }
            }
        }
    }
}
