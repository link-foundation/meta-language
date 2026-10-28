// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryCompareUtf16TextLoop72Result;
use super::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63;
use super::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64;
use super::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result;
use super::MlSource1TemporaryIndexUtf16TextLoop70Result;
use super::MlSource1TemporaryLastIndexUtf16TextLoop71Result;
use super::MlSource1TemporaryMatchUtf16TextLoop69Result;
use super::apply_big_integer_bitwise_operation;
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
use super::read_java_script_call_integer_arity;
use super::read_java_script_call_integer_bit_code;
use super::read_java_script_call_integer_width;
use super::source_ml_source1_temporary_character_units_utf16_text_loop78;
use super::source_ml_source1_temporary_compare_utf16_text_loop72;
use super::source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop90;
use super::source_ml_source1_temporary_index_utf16_text_loop70;
use super::source_ml_source1_temporary_last_index_utf16_text_loop71;
use super::source_ml_source1_temporary_match_utf16_text_loop69;
use super::source_ml_source1_temporary_slice_utf16_text_loop65;
use super::source_ml_source1_temporary_trim_utf16_text_loop76;
use super::source_ml_source1_temporary_trim_utf16_text_loop77;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_integer_primitive(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    if operation.contains(&self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])) {
        self::execute_java_script_call_number_bits(operation, values)
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f158) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f159) => {
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
                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f164) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source2_f165) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Integer(value) => {
                                {
                                    let left = first_value.clone();
                                    {
                                        let right = value.clone();
                                        if ((((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114])))) && (right == (self::ml::Big::from_i128(0)))) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111]))
                                        } else {
                                            if (((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]))) && (right < (self::ml::Big::from_i128(0)))) {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![69, 120, 112, 111, 110, 101, 110, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101]))
                                            } else {
                                                {
                                                    let left_width = self::read_java_script_call_integer_width(left.clone());
                                                    {
                                                        let right_width = self::read_java_script_call_integer_width(right.clone());
                                                        if ((left_width < (0f64)) || (right_width < (0f64))) {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 105, 110, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                        } else {
                                                            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (left < right) {
                                                                    (-1f64)
                                                                } else {
                                                                    if (left > right) {
                                                                        1f64
                                                                    } else {
                                                                        0f64
                                                                    }
                                                                })))
                                                            } else {
                                                                if (((*operation) == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) && (((left_width + right_width)) > (4096f64))) {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 101, 115, 116, 105, 109, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                } else {
                                                                    if ((((((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]))) && (left != (self::ml::Big::from_i128(0)))) && (left != (self::ml::Big::from_i128(1)))) && (left != (self::ml::Big::from_i128(-1)))) && ((right.mul(&{
                                                                        let source_ml_source2_integer_operand172 = left_width;
                                                                        {
                                                                            let source_ml_source2_integer_result173 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(source_ml_source2_integer_operand172);
                                                                            if (((source_ml_source2_integer_result173.len() as f64)) == (1f64)) {
                                                                                (self::ml_array::optional_at(&source_ml_source2_integer_result173, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_integer_operand172)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                                                                            }
                                                                        }
                                                                    })) > (self::ml::Big::from_i128(4096)))) {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 101, 120, 112, 111, 110, 101, 110, 116, 105, 97, 116, 105, 111, 110, 32, 101, 115, 116, 105, 109, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                    } else {
                                                                        {
                                                                            let source_ml_source2_s178 = if (((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116])))) {
                                                                                {
                                                                                    let growing = if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]))) {
                                                                                        (right >= (self::ml::Big::from_i128(0)))
                                                                                    } else {
                                                                                        (right < (self::ml::Big::from_i128(0)))
                                                                                    };
                                                                                    {
                                                                                        let count = if (right < (self::ml::Big::from_i128(0))) {
                                                                                            right.neg()
                                                                                        } else {
                                                                                            right.clone()
                                                                                        };
                                                                                        if (((left != (self::ml::Big::from_i128(0))) && growing) && (count > ({
                                                                                            let source_ml_source2_integer_operand181 = (4096f64 - left_width);
                                                                                            {
                                                                                                let source_ml_source2_integer_result182 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(source_ml_source2_integer_operand181);
                                                                                                if (((source_ml_source2_integer_result182.len() as f64)) == (1f64)) {
                                                                                                    (self::ml_array::optional_at(&source_ml_source2_integer_result182, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                                                } else {
                                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_integer_operand181)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                                                                                                }
                                                                                            }
                                                                                        }))) {
                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 115, 104, 105, 102, 116, 32, 101, 115, 116, 105, 109, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))))
                                                                                        } else {
                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63Next
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63Next
                                                                            };
                                                                            match source_ml_source2_s178.clone() {
                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63Next => {
                                                                                    {
                                                                                        let code = self::read_java_script_call_integer_bit_code(operation);
                                                                                        {
                                                                                            let result = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source2_s183 = if ((*operation) == (self::ml_string::Text::from_units(vec![97, 100, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                    {
                                                                                                        let result_2 = left.add(&right);
                                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_2.clone())
                                                                                                    }
                                                                                                } else {
                                                                                                    if ((*operation) == (self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                        {
                                                                                                            let result_3 = left.sub(&right);
                                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_3.clone())
                                                                                                        }
                                                                                                    } else {
                                                                                                        if ((*operation) == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                            {
                                                                                                                let result_4 = left.mul(&right);
                                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_4.clone())
                                                                                                            }
                                                                                                        } else {
                                                                                                            if (((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114])))) {
                                                                                                                {
                                                                                                                    let result_5 = (self::ml_array::optional_at(&self::divide_java_script_big_integer(left.clone(), right.clone()), self::ml_array::number_index(if ((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                                        0f64
                                                                                                                    } else {
                                                                                                                        1f64
                                                                                                                    }))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_5.clone())
                                                                                                                }
                                                                                                            } else {
                                                                                                                if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]))) {
                                                                                                                    {
                                                                                                                        let result_6 = (self::ml_array::optional_at(&self::power_java_script_big_integer(left.clone(), right.clone()), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_6.clone())
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    if ((*operation) == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 105, 110, 116, 101, 103, 101, 114]))) {
                                                                                                                        {
                                                                                                                            let result_7 = left.neg();
                                                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_7.clone())
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        if (code >= (0f64)) {
                                                                                                                            {
                                                                                                                                let result_8 = self::apply_big_integer_bitwise_operation(code, left.clone(), right.clone());
                                                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_8.clone())
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))))
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                match source_ml_source2_s183.clone() {
                                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(result_9) => {
                                                                                                        if ((self::read_java_script_call_integer_width(result_9.clone())) < (0f64)) {
                                                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 117, 116, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                                                        } else {
                                                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(result_9.clone())))
                                                                                                        }
                                                                                                    }
                                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Return(source_ml_source1_temporary_result64) => {
                                                                                                        let source_ml_source1_temporary_result64 = *source_ml_source1_temporary_result64;
                                                                                                        source_ml_source1_temporary_result64.clone()
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63Return(source_ml_source1_temporary_result64_2) => {
                                                                                    let source_ml_source1_temporary_result64_2 = *source_ml_source1_temporary_result64_2;
                                                                                    source_ml_source1_temporary_result64_2.clone()
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
                            self::JavaScriptSemanticValue::Text(source_ml_source2_f166) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f167) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Object(source_ml_source2_f168, source_ml_source2_f169) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f160) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f161) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f162, source_ml_source2_f163) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
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
                        let result_2 = self::source_ml_source1_temporary_slice_utf16_text_loop65(text, last, result, at);
                        result_2
                    }
                }
            }
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
pub fn match_utf16_text(text: &[f64], search: &[f64], start: f64) -> bool {
    if ((start < (0f64)) || (((start + (search.len() as f64))) > ((text.len() as f64)))) {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source2_s185 = self::source_ml_source1_temporary_match_utf16_text_loop69(text, search, start, at);
                match source_ml_source2_s185.clone() {
                    self::MlSource1TemporaryMatchUtf16TextLoop69Result::MlSource1TemporaryMatchUtf16TextLoop69Done => {
                        true
                    }
                    self::MlSource1TemporaryMatchUtf16TextLoop69Result::MlSource1TemporaryMatchUtf16TextLoop69Return(source_ml_source1_temporary_result69) => {
                        source_ml_source1_temporary_result69
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
                let source_ml_source2_s186 = self::source_ml_source1_temporary_index_utf16_text_loop70(text, search, at);
                match source_ml_source2_s186.clone() {
                    self::MlSource1TemporaryIndexUtf16TextLoop70Result::MlSource1TemporaryIndexUtf16TextLoop70Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryIndexUtf16TextLoop70Result::MlSource1TemporaryIndexUtf16TextLoop70Return(source_ml_source1_temporary_result70) => {
                        source_ml_source1_temporary_result70
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
                    let source_ml_source2_s187 = self::source_ml_source1_temporary_last_index_utf16_text_loop71(text, search, at);
                    match source_ml_source2_s187.clone() {
                        self::MlSource1TemporaryLastIndexUtf16TextLoop71Result::MlSource1TemporaryLastIndexUtf16TextLoop71Done => {
                            (-1f64)
                        }
                        self::MlSource1TemporaryLastIndexUtf16TextLoop71Result::MlSource1TemporaryLastIndexUtf16TextLoop71Return(source_ml_source1_temporary_result71) => {
                            source_ml_source1_temporary_result71
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
                let source_ml_source2_s188 = self::source_ml_source1_temporary_compare_utf16_text_loop72(left, right, count, at);
                match source_ml_source2_s188.clone() {
                    self::MlSource1TemporaryCompareUtf16TextLoop72Result::MlSource1TemporaryCompareUtf16TextLoop72Done => {
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
                    self::MlSource1TemporaryCompareUtf16TextLoop72Result::MlSource1TemporaryCompareUtf16TextLoop72Return(source_ml_source1_temporary_result72) => {
                        source_ml_source1_temporary_result72
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
pub fn trim_utf16_text(text: &[f64], start: bool, end: bool) -> Vec<f64> {
    {
        let first = 0f64;
        {
            let last = (text.len() as f64);
            {
                let first_3 = if start {
                    {
                        let first_2 = self::source_ml_source1_temporary_trim_utf16_text_loop76(text, first, last);
                        first_2
                    }
                } else {
                    first
                };
                {
                    let last_3 = if end {
                        {
                            let last_2 = self::source_ml_source1_temporary_trim_utf16_text_loop77(text, first_3, last);
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
                let result_2 = self::source_ml_source1_temporary_character_units_utf16_text_loop78(values, result, at);
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
                        let unary = vec![self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116]), self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 101, 110, 100])];
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_numeric_primitive(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let numbers = Vec::<f64>::new();
        {
            let source_ml_source1_temporary_values89 = values;
            {
                let source_ml_source1_temporary_index89 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s192 = self::source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop90(numbers, source_ml_source1_temporary_values89, source_ml_source1_temporary_index89.clone());
                    match source_ml_source2_s192.clone() {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Done(numbers_2) => {
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
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::character_units_utf16_text(&(numbers_2)))))
                                        } else {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                        }
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(source_ml_source1_temporary_result90) => {
                            let source_ml_source1_temporary_result90 = *source_ml_source1_temporary_result90;
                            source_ml_source1_temporary_result90.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_text_primitive(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let receiver = (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        match receiver.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
            self::JavaScriptSemanticValue::Null => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f193) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f194) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f195) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
            self::JavaScriptSemanticValue::Text(text) => {
                if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 108, 101, 110, 103, 116, 104]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number((text.len() as f64))))
                } else {
                    if ((((*operation) == (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116])))) || ((*operation) == (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 101, 110, 100])))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::trim_utf16_text(&(text), ((*operation) != (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 101, 110, 100]))), ((*operation) != (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116])))))))
                    } else {
                        if (((((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 111, 100, 101, 32, 112, 111, 105, 110, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114])))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114])))) || ((*operation) == (self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 116, 101, 120, 116])))) {
                            self::execute_java_script_call_text_position(operation, &(text), values)
                        } else {
                            self::execute_java_script_call_text_pair(operation, &(text), values)
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f196) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f197, source_ml_source2_f198) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_text_position(operation: &self::ml_string::Text, text: &[f64], values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let second = (self::ml_array::optional_at(&values, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let third = (self::ml_array::optional_at(&values, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            if ((((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 111, 100, 101, 32, 112, 111, 105, 110, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114])))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114])))) {
                match second.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f199) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                    }
                    self::JavaScriptSemanticValue::Number(second_value) => {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114]))) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::character_utf16_text(text, second_value))))
                        } else {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114]))) {
                                {
                                    let result = self::relative_character_utf16_text(text, second_value);
                                    {
                                        let value = self::ml_array::optional_at(&result, self::ml_array::number_index(0f64));
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(if match (value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::JavaScriptSemanticValue::Undefined
                                        } else {
                                            self::JavaScriptSemanticValue::Text(value.clone().expect("a proven present value"))
                                        }))
                                    }
                                }
                            } else {
                                {
                                    let result_2 = self::code_point_utf16_text(text, second_value);
                                    {
                                        let value_2 = self::ml_array::optional_at(&result_2, self::ml_array::number_index(0f64));
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(if {
                                            let source_ml_source2_comparison_operand205 = value_2.clone();
                                            {
                                                let source_ml_source2_comparison_operand206 = None::<()>;
                                                match (source_ml_source2_comparison_operand205.clone(), source_ml_source2_comparison_operand206.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                            }
                                        } {
                                            self::JavaScriptSemanticValue::Undefined
                                        } else {
                                            self::JavaScriptSemanticValue::Number(value_2.clone().expect("a proven present value"))
                                        }))
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f200) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f201) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f202) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f203, source_ml_source2_f204) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                    }
                }
            } else {
                if ((*operation) == (self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 116, 101, 120, 116]))) {
                    match second.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f207) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Number(second_value_2) => {
                            match third.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f213) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Number(value_3) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::slice_utf16_text(text, second_value_2, value_3))))
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f214) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source2_f215) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f216) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source2_f217, source_ml_source2_f218) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f208) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f209) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f210) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f211, source_ml_source2_f212) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                    }
                } else {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_text_pair(operation: &self::ml_string::Text, text: &[f64], values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let second = (self::ml_array::optional_at(&values, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let third = (self::ml_array::optional_at(&values, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            match second.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Null => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f219) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f220) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f221) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Text(units) => {
                    if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]))) {
                        if ((((text.len() as f64) + (units.len() as f64))) > (1048576f64)) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 111, 117, 116, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text({
                                let source_ml_source2_concat_value225 = text.to_vec();
                                {
                                    let source_ml_source2_concat_value226 = units;
                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value225), source_ml_source2_concat_value226)
                                }
                            })))
                        }
                    } else {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116]))) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::compare_utf16_text(text, &(units)))))
                        } else {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 112, 114, 101, 102, 105, 120]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::starts_utf16_text(text, &(units)))))
                            } else {
                                if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 115, 117, 102, 102, 105, 120]))) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::ends_utf16_text(text, &(units)))))
                                } else {
                                    if ((((self::ml_math::max(0f64, ((text.len() as f64) - (units.len() as f64))) + 1f64) * (units.len() as f64))) > (67108864f64)) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 119, 111, 114, 107, 32, 113, 117, 111, 116, 97]))
                                    } else {
                                        if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 105, 110, 99, 108, 117, 115, 105, 111, 110]))) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::includes_utf16_text(text, &(units)))))
                                        } else {
                                            if (((*operation) == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 116, 101, 120, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 108, 97, 115, 116, 32, 116, 101, 120, 116])))) {
                                                match third.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f227) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Number(value) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if ((*operation) == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 116, 101, 120, 116]))) {
                                                            self::index_utf16_text(text, &(units), value)
                                                        } else {
                                                            self::last_index_utf16_text(text, &(units), value)
                                                        })))
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f228) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f229) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f230) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f231, source_ml_source2_f232) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                }
                                            } else {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f222) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f223, source_ml_source2_f224) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
            }
        }
    }
}
