// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Result;
use super::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop122Result;
use super::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123;
use super::character_units_utf16_text;
use super::character_utf16_text;
use super::code_point_utf16_text;
use super::compare_utf16_text;
use super::construct_utf16_text_padding;
use super::construct_utf16_text_repetition;
use super::ends_utf16_text;
use super::execute_java_script_call_integer_conversion;
use super::execute_java_script_call_integer_primitive;
use super::execute_java_script_call_number_conversion;
use super::includes_utf16_text;
use super::index_utf16_text;
use super::last_index_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_string;
use super::plan_utf16_text_substitution;
use super::read_java_script_call_integer_arity;
use super::read_java_script_call_integer_width;
use super::read_java_script_integer_protocol_operation_error;
use super::read_java_script_source_exception_class;
use super::read_utf16_text_padding_length;
use super::read_utf16_text_repetition_length;
use super::relative_character_utf16_text;
use super::replace_utf16_text;
use super::slice_utf16_text;
use super::source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop119;
use super::source_ml_source1_temporary_execute_java_script_call_primitive_loop122;
use super::source_ml_source1_temporary_execute_java_script_call_text_conversion_loop124;
use super::source_ml_source1_temporary_read_java_script_call_text_replacement_work_error_loop120;
use super::starts_utf16_text;
use super::test_java_script_call_unary_number_operation;
use super::trim_utf16_text;

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
                        if self::test_java_script_call_unary_number_operation(operation) {
                            if (count == (1f64)) {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])
                            }
                        } else {
                            {
                                let unary = vec![self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116]), self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 101, 110, 100]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 101, 103, 101, 114])];
                                {
                                    let binary = vec![self::ml_string::Text::from_units(vec![114, 101, 112, 101, 97, 116, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 111, 100, 101, 32, 112, 111, 105, 110, 116]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 112, 114, 101, 102, 105, 120]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 115, 117, 102, 102, 105, 120]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 105, 110, 99, 108, 117, 115, 105, 111, 110])];
                                    {
                                        let ternary = vec![self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116]), self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 101, 110, 100]), self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 108, 97, 115, 116, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 32, 116, 101, 120, 116]), self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 32, 97, 108, 108, 32, 116, 101, 120, 116])];
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_numeric_primitive(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    if self::test_java_script_call_unary_number_operation(operation) {
        if (((values.len() as f64)) != (1f64)) {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101]))
        } else {
            {
                let first = (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                match first.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                    }
                    self::JavaScriptSemanticValue::Number(value) => {
                        self::execute_java_script_call_unary_number(operation, value)
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                    }
                }
            }
        }
    } else {
        {
            let numbers = Vec::<f64>::new();
            {
                let source_ml_source1_temporary_index118 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s971 = self::source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop119(numbers, values, source_ml_source1_temporary_index118.clone());
                    match source_ml_source2_s971.clone() {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Done(numbers_2) => {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 105, 110, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::ml_math::min_of(&self::ml_array::append(Vec::<f64>::new(), numbers_2)))))
                            } else {
                                if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 97, 120, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]))) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::ml_math::max_of(&self::ml_array::append(Vec::<f64>::new(), numbers_2)))))
                                } else {
                                    if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 116, 101, 120, 116, 32, 117, 110, 105, 116, 115]))) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::character_units_utf16_text(numbers_2))))
                                    } else {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Return(source_ml_source1_temporary_result119) => {
                            let source_ml_source1_temporary_result119 = *source_ml_source1_temporary_result119;
                            source_ml_source1_temporary_result119.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_unary_number(operation: &self::ml_string::Text, number: f64) -> self::JavaScriptCallPrimitiveResult {
    if ((*operation) == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114]))) {
        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number((-number))))
    } else {
        if ((*operation) == (self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114]))) {
            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(number.trunc())))
        } else {
            if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114]))) {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::ml_math::is_integer(number))))
            } else {
                if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 115, 97, 102, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::ml_math::is_safe_integer(number))))
                } else {
                    if !(self::test_java_script_call_unary_number_operation(operation)) {
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_text_primitive(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    if ((((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 112, 101, 97, 116, 32, 116, 101, 120, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116])))) || ((*operation) == (self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 101, 110, 100])))) {
        self::execute_java_script_call_text_expansion(operation, values)
    } else {
        if (((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 32, 116, 101, 120, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 32, 97, 108, 108, 32, 116, 101, 120, 116])))) {
            self::execute_java_script_call_text_replacement_operands(values, ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 32, 97, 108, 108, 32, 116, 101, 120, 116]))))
        } else {
            {
                let receiver = (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                match receiver.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                    }
                    self::JavaScriptSemanticValue::Text(text) => {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 108, 101, 110, 103, 116, 104]))) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number((text.len() as f64))))
                        } else {
                            if ((((*operation) == (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116])))) || ((*operation) == (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 101, 110, 100])))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::trim_utf16_text(text, ((*operation) != (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 101, 110, 100]))), ((*operation) != (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116])))))))
                            } else {
                                if (((((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 111, 100, 101, 32, 112, 111, 105, 110, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114])))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114])))) || ((*operation) == (self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 116, 101, 120, 116])))) {
                                    self::execute_java_script_call_text_position(operation, &(text), values)
                                } else {
                                    self::execute_java_script_call_text_pair(operation, &(text), values)
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_text_expansion(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let receiver = (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        match receiver.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
            self::JavaScriptSemanticValue::Null => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
            self::JavaScriptSemanticValue::Boolean(_) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
            self::JavaScriptSemanticValue::Number(_) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
            self::JavaScriptSemanticValue::Text(receiver_units) => {
                {
                    let count = (self::ml_array::optional_at(&values, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                    match count.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 98, 111, 117, 110, 100]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 98, 111, 117, 110, 100]))
                        }
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 98, 111, 117, 110, 100]))
                        }
                        self::JavaScriptSemanticValue::Number(value) => {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 112, 101, 97, 116, 32, 116, 101, 120, 116]))) {
                                {
                                    let length = self::read_utf16_text_repetition_length((receiver_units.len() as f64), value);
                                    if (length == ((-1f64))) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), (self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 32, 99, 111, 117, 110, 116, 32, 118, 97, 108, 117, 101, 58, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(value))))
                                    } else {
                                        if (length < (0f64)) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 111, 117, 116, 112, 117, 116, 32, 98, 117, 100, 103, 101, 116]))
                                        } else {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::construct_utf16_text_repetition(&(receiver_units), length))))
                                        }
                                    }
                                }
                            } else {
                                if (((*operation) != (self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116]))) && ((*operation) != (self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 101, 110, 100])))) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                } else {
                                    {
                                        let filler = (self::ml_array::optional_at(&values, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                        match filler.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 97, 100, 100, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 102, 105, 108, 108, 101, 114]))
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 97, 100, 100, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 102, 105, 108, 108, 101, 114]))
                                            }
                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 97, 100, 100, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 102, 105, 108, 108, 101, 114]))
                                            }
                                            self::JavaScriptSemanticValue::Number(_) => {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 97, 100, 100, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 102, 105, 108, 108, 101, 114]))
                                            }
                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 97, 100, 100, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 102, 105, 108, 108, 101, 114]))
                                            }
                                            self::JavaScriptSemanticValue::Text(units) => {
                                                {
                                                    let length_2 = self::read_utf16_text_padding_length((receiver_units.len() as f64), value, (units.len() as f64));
                                                    if (length_2 < (0f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 111, 117, 116, 112, 117, 116, 32, 98, 117, 100, 103, 101, 116]))
                                                    } else {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::construct_utf16_text_padding(&(receiver_units), &(units), length_2, ((*operation) == (self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116])))))))
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 97, 100, 100, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 102, 105, 108, 108, 101, 114]))
                                            }
                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 97, 100, 100, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 102, 105, 108, 108, 101, 114]))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 98, 111, 117, 110, 100]))
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 98, 111, 117, 110, 100]))
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 98, 111, 117, 110, 100]))
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 98, 111, 117, 110, 100]))
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
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
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                    }
                    self::JavaScriptSemanticValue::Number(second_value) => {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114]))) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::character_utf16_text(text.to_vec(), second_value))))
                        } else {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114]))) {
                                {
                                    let result = self::relative_character_utf16_text(text.to_vec(), second_value);
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
                                    let result_2 = self::code_point_utf16_text(text.to_vec(), second_value);
                                    {
                                        let value_2 = self::ml_array::optional_at(&result_2, self::ml_array::number_index(0f64));
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(if {
                                            let source_ml_source2_comparison_operand1004 = None::<()>;
                                            match (value_2.clone(), source_ml_source2_comparison_operand1004.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
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
                    self::JavaScriptSemanticValue::Integer(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
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
                        self::JavaScriptSemanticValue::Boolean(_) => {
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
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Number(value_3) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::slice_utf16_text(text.to_vec(), second_value_2, value_3))))
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
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
                self::JavaScriptSemanticValue::Boolean(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Number(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Integer(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Text(needle) => {
                    if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]))) {
                        if ((((text.len() as f64) + (needle.len() as f64))) > (1048576f64)) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 111, 117, 116, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), text.to_vec()), needle))))
                        }
                    } else {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116]))) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::compare_utf16_text(text.to_vec(), needle))))
                        } else {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 112, 114, 101, 102, 105, 120]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::starts_utf16_text(text.to_vec(), needle))))
                            } else {
                                if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 115, 117, 102, 102, 105, 120]))) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::ends_utf16_text(text.to_vec(), needle))))
                                } else {
                                    if ((((self::ml_math::max(0f64, ((text.len() as f64) - (needle.len() as f64))) + 1f64) * (needle.len() as f64))) > (67108864f64)) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 119, 111, 114, 107, 32, 113, 117, 111, 116, 97]))
                                    } else {
                                        if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 105, 110, 99, 108, 117, 115, 105, 111, 110]))) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::includes_utf16_text(text.to_vec(), needle))))
                                        } else {
                                            if (((*operation) == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 116, 101, 120, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 108, 97, 115, 116, 32, 116, 101, 120, 116])))) {
                                                match third.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Number(value) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if ((*operation) == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 116, 101, 120, 116]))) {
                                                            self::index_utf16_text(text.to_vec(), needle, value)
                                                        } else {
                                                            self::last_index_utf16_text(text.to_vec(), needle, value)
                                                        })))
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
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
                self::JavaScriptSemanticValue::Symbol(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_text_replacement_operands(values: Vec<self::JavaScriptSemanticValue>, all: bool) -> self::JavaScriptCallPrimitiveResult {
    {
        let text = (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let search = (self::ml_array::optional_at(&values, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            {
                let replacement = (self::ml_array::optional_at(&values, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                match text.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                    self::JavaScriptSemanticValue::Text(text_units) => {
                        match search.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Text(search_units) => {
                                match replacement.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                    }
                                    self::JavaScriptSemanticValue::Number(_) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                    }
                                    self::JavaScriptSemanticValue::Integer(_) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                    }
                                    self::JavaScriptSemanticValue::Text(units) => {
                                        self::execute_java_script_call_text_replacement(&(text_units), &(search_units), &(units), all)
                                    }
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                    }
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_text_replacement_work_error(length: f64, search: f64, replacement: &[f64], all: bool) -> self::ml_string::Text {
    if (((length > (1048576f64)) || (search > (1048576f64))) || (((replacement.len() as f64)) > (4096f64))) {
        self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 112, 117, 116, 32, 111, 114, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 105, 111, 110, 45, 112, 108, 97, 110, 32, 98, 117, 100, 103, 101, 116])
    } else {
        {
            let matches = if (search == (0f64)) {
                (length + 1f64)
            } else {
                ((length / search)).floor()
            };
            {
                let matches_3 = if !all {
                    {
                        let matches_2 = self::ml_math::min(1f64, matches);
                        matches_2
                    }
                } else {
                    matches
                };
                if (matches_3 == (0f64)) {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    if ((((length + 1f64) * search)) > (67108864f64)) {
                        self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 101, 97, 114, 99, 104, 32, 119, 111, 114, 107, 32, 113, 117, 111, 116, 97])
                    } else {
                        {
                            let plan = self::plan_utf16_text_substitution(replacement.to_vec());
                            if (((matches_3 * ((plan.len() as f64) / 3f64))) > (67108864f64)) {
                                self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 105, 111, 110, 32, 119, 111, 114, 107, 32, 113, 117, 111, 116, 97])
                            } else {
                                {
                                    let width = 0f64;
                                    {
                                        let index = 0f64;
                                        {
                                            let width_2 = self::source_ml_source1_temporary_read_java_script_call_text_replacement_work_error_loop120(length, search, &(plan), width, index);
                                            if (((length + (matches_3 * width_2))) > (1048576f64)) {
                                                self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 99, 111, 110, 115, 101, 114, 118, 97, 116, 105, 118, 101, 32, 85, 84, 70, 49, 54, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 98, 111, 117, 110, 100])
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
pub fn execute_java_script_call_text_replacement(text: &[f64], search: &[f64], replacement: &[f64], all: bool) -> self::JavaScriptCallPrimitiveResult {
    {
        let error = self::read_java_script_call_text_replacement_work_error((text.len() as f64), (search.len() as f64), replacement, all);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(error.clone())
        } else {
            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::replace_utf16_text(text.to_vec(), search.to_vec(), replacement.to_vec(), all))))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_primitive(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let arity = self::read_java_script_call_primitive_arity_error(operation, (values.len() as f64));
        if (arity != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(arity.clone())
        } else {
            {
                let source_ml_source1_temporary_index121 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s1049 = self::source_ml_source1_temporary_execute_java_script_call_primitive_loop122(values.clone(), source_ml_source1_temporary_index121.clone());
                    match source_ml_source2_s1049.clone() {
                        self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop122Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop122Done => {
                            if ((self::read_java_script_source_exception_class(operation)) != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 114, 101, 97, 108, 109]))
                            } else {
                                if (((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) {
                                    self::execute_java_script_call_number_conversion(operation, (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                } else {
                                    if (((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 101, 103, 101, 114])))) {
                                        self::execute_java_script_call_integer_conversion(operation, (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                    } else {
                                        if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]))) {
                                            self::execute_java_script_call_text_conversion((self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                        } else {
                                            if ((self::read_java_script_call_integer_arity(operation)) != (0f64)) {
                                                self::execute_java_script_call_integer_primitive(operation, values)
                                            } else {
                                                if (self::test_java_script_call_unary_number_operation(operation) || self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 105, 110, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 97, 120, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 116, 101, 120, 116, 32, 117, 110, 105, 116, 115])], operation, 0f64, false, |a, b| a == b)) {
                                                    self::execute_java_script_call_numeric_primitive(operation, values)
                                                } else {
                                                    self::execute_java_script_call_text_primitive(operation, values)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop122Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop122Return(source_ml_source1_temporary_result122) => {
                            let source_ml_source1_temporary_result122 = *source_ml_source1_temporary_result122;
                            source_ml_source1_temporary_result122.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_text_conversion(value: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    {
        let text = self::ml_string::Text::from_units(vec![]);
        {
            let source_ml_source2_s1056 = match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123Next(self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))
                }
                self::JavaScriptSemanticValue::Null => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123Next(self::ml_string::Text::from_units(vec![110, 117, 108, 108]))
                }
                self::JavaScriptSemanticValue::Boolean(value_value) => {
                    {
                        let text_2 = if value_value {
                            self::ml_string::Text::from_units(vec![116, 114, 117, 101])
                        } else {
                            self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
                        };
                        self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123Next(text_2.clone())
                    }
                }
                self::JavaScriptSemanticValue::Number(value_value_2) => {
                    {
                        let text_3 = self::ml_string::Text::from_text(&self::ml_number::js_number(value_value_2));
                        self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123Next(text_3.clone())
                    }
                }
                self::JavaScriptSemanticValue::Integer(value_value_3) => {
                    if ((self::read_java_script_call_integer_width(value_value_3.clone())) < (0f64)) {
                        self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))))
                    } else {
                        {
                            let text_4 = self::ml_string::Text::from_text(&value_value_3.to_string());
                            self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123Next(text_4.clone())
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(_) => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))))
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 83, 121, 109, 98, 111, 108, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 115, 116, 114, 105, 110, 103]))))
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108]))))
                }
            };
            match source_ml_source2_s1056.clone() {
                self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123Next(text_5) => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index = 0f64;
                            {
                                let units_2 = self::source_ml_source1_temporary_execute_java_script_call_text_conversion_loop124(&(text_5), units, index);
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(units_2)))
                            }
                        }
                    }
                }
                self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123Return(source_ml_source1_temporary_result124) => {
                    let source_ml_source1_temporary_result124 = *source_ml_source1_temporary_result124;
                    source_ml_source1_temporary_result124.clone()
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_scalar_protocol_operation_error(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if (((((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116])))) || ((self::read_java_script_integer_protocol_operation_error(operation)) == (self::ml_string::Text::from_units(vec![])))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 78, 117, 109, 98, 101, 114, 44, 32, 105, 110, 116, 101, 103, 101, 114, 32, 111, 114, 32, 116, 101, 120, 116, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
    }
}
