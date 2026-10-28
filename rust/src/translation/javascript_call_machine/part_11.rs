// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop118Result;
use super::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119;
use super::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop121Result;
use super::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Result;
use super::begin_java_script_primitive_conversion;
use super::character_utf16_text;
use super::code_point_utf16_text;
use super::compare_utf16_text;
use super::ends_utf16_text;
use super::execute_java_script_call_integer_primitive;
use super::execute_java_script_call_number_conversion;
use super::execute_java_script_call_numeric_primitive;
use super::execute_java_script_call_text_primitive;
use super::includes_utf16_text;
use super::index_utf16_text;
use super::last_index_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_call_integer_arity;
use super::read_java_script_call_integer_width;
use super::read_java_script_call_primitive_arity_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_value_error;
use super::read_java_script_source_exception_class;
use super::relative_character_utf16_text;
use super::resume_java_script_primitive_conversion;
use super::slice_utf16_text;
use super::source_ml_source1_temporary_execute_java_script_call_primitive_loop118;
use super::source_ml_source1_temporary_execute_java_script_call_text_conversion_loop120;
use super::source_ml_source1_temporary_find_java_script_call_code_function_loop121;
use super::source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop122;
use super::starts_utf16_text;
use super::step_java_script_primitive_conversion;
use super::test_java_script_semantic_accessor;
use super::test_java_script_semantic_same_value;

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
                                            let source_ml_source2_comparison_operand974 = None::<()>;
                                            match (value_2.clone(), source_ml_source2_comparison_operand974.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
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
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::slice_utf16_text(text, second_value_2, value_3))))
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
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::compare_utf16_text(text, &(needle)))))
                        } else {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 112, 114, 101, 102, 105, 120]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::starts_utf16_text(text, &(needle)))))
                            } else {
                                if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 115, 117, 102, 102, 105, 120]))) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::ends_utf16_text(text, &(needle)))))
                                } else {
                                    if ((((self::ml_math::max(0f64, ((text.len() as f64) - (needle.len() as f64))) + 1f64) * (needle.len() as f64))) > (67108864f64)) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 119, 111, 114, 107, 32, 113, 117, 111, 116, 97]))
                                    } else {
                                        if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 105, 110, 99, 108, 117, 115, 105, 111, 110]))) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::includes_utf16_text(text, &(needle)))))
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
                                                            self::index_utf16_text(text, &(needle), value)
                                                        } else {
                                                            self::last_index_utf16_text(text, &(needle), value)
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
pub fn execute_java_script_call_primitive(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let arity = self::read_java_script_call_primitive_arity_error(operation, (values.len() as f64));
        if (arity != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(arity.clone())
        } else {
            {
                let source_ml_source1_temporary_index117 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s1001 = self::source_ml_source1_temporary_execute_java_script_call_primitive_loop118(values.clone(), source_ml_source1_temporary_index117.clone());
                    match source_ml_source2_s1001.clone() {
                        self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop118Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop118Done => {
                            if ((self::read_java_script_source_exception_class(operation)) != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 114, 101, 97, 108, 109]))
                            } else {
                                if (((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) {
                                    self::execute_java_script_call_number_conversion(operation, (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                } else {
                                    if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]))) {
                                        self::execute_java_script_call_text_conversion((self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                    } else {
                                        if ((self::read_java_script_call_integer_arity(operation)) != (0f64)) {
                                            self::execute_java_script_call_integer_primitive(operation, values)
                                        } else {
                                            if self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 105, 110, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 97, 120, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 116, 101, 120, 116, 32, 117, 110, 105, 116, 115]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![102, 108, 111, 111, 114, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 101, 105, 108, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 98, 101, 114, 32, 115, 105, 103, 110]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 113, 117, 97, 114, 101, 32, 114, 111, 111, 116]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 115, 97, 102, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114])], operation, 0f64, false, |a, b| a == b) {
                                                self::execute_java_script_call_numeric_primitive(operation, values)
                                            } else {
                                                self::execute_java_script_call_text_primitive(operation, values)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop118Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop118Return(source_ml_source1_temporary_result118) => {
                            let source_ml_source1_temporary_result118 = *source_ml_source1_temporary_result118;
                            source_ml_source1_temporary_result118.clone()
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
            let source_ml_source2_s1008 = match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119Next(self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))
                }
                self::JavaScriptSemanticValue::Null => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119Next(self::ml_string::Text::from_units(vec![110, 117, 108, 108]))
                }
                self::JavaScriptSemanticValue::Boolean(value_value) => {
                    {
                        let text_2 = if value_value {
                            self::ml_string::Text::from_units(vec![116, 114, 117, 101])
                        } else {
                            self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
                        };
                        self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119Next(text_2.clone())
                    }
                }
                self::JavaScriptSemanticValue::Number(value_value_2) => {
                    {
                        let text_3 = self::ml_string::Text::from_text(&self::ml_number::js_number(value_value_2));
                        self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119Next(text_3.clone())
                    }
                }
                self::JavaScriptSemanticValue::Integer(value_value_3) => {
                    if ((self::read_java_script_call_integer_width(value_value_3.clone())) < (0f64)) {
                        self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))))
                    } else {
                        {
                            let text_4 = self::ml_string::Text::from_text(&value_value_3.to_string());
                            self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119Next(text_4.clone())
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(_) => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))))
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 83, 121, 109, 98, 111, 108, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 115, 116, 114, 105, 110, 103]))))
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108]))))
                }
            };
            match source_ml_source2_s1008.clone() {
                self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119Next(text_5) => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index = 0f64;
                            {
                                let units_2 = self::source_ml_source1_temporary_execute_java_script_call_text_conversion_loop120(&(text_5), units, index);
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(units_2)))
                            }
                        }
                    }
                }
                self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119Return(source_ml_source1_temporary_result120) => {
                    let source_ml_source1_temporary_result120 = *source_ml_source1_temporary_result120;
                    source_ml_source1_temporary_result120.clone()
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_scalar_protocol_operation_error(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if ((((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116])))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 114, 32, 116, 101, 120, 116, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_scalar_protocol(operation: &self::ml_string::Text, transition: self::JavaScriptPrimitiveConversionTransition) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        } else {
            match transition.clone() {
                self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(state) => {
                    let state = *state;
                    self::JavaScriptScalarConversionTransition::ScalarContinuation(Box::new(state.clone()))
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(state_2, callee, receiver, arguments) => {
                    let state_2 = *state_2;
                    let callee = *callee;
                    let receiver = *receiver;
                    self::JavaScriptScalarConversionTransition::ScalarCall(Box::new(state_2.clone()), Box::new(callee.clone()), Box::new(receiver.clone()), arguments)
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(value) => {
                    let value = *value;
                    self::finish_java_script_scalar_primitive(operation, value.clone())
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveThrow(value_2) => {
                    let value_2 = *value_2;
                    self::JavaScriptScalarConversionTransition::ScalarThrow(Box::new(value_2.clone()))
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(error_class, reason) => {
                    self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class.clone(), reason.clone())
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(reason_2) => {
                    self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2.clone())
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_scalar_primitive(operation: &self::ml_string::Text, value: self::JavaScriptSemanticValue) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        } else {
            {
                let result = self::execute_java_script_call_primitive(operation, vec![value.clone()]);
                match result.clone() {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_2) => {
                        let value_2 = *value_2;
                        self::JavaScriptScalarConversionTransition::ScalarCompleted(Box::new(value_2.clone()))
                    }
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(reason) => {
                        self::JavaScriptScalarConversionTransition::ScalarInvalid(reason.clone())
                    }
                    self::JavaScriptCallPrimitiveResult::PrimitiveError(error_class, reason_2) => {
                        self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class.clone(), reason_2.clone())
                    }
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(reason_3) => {
                        self::JavaScriptScalarConversionTransition::ScalarResourceLimit(Box::new(value.clone()), reason_3.clone())
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_scalar_protocol(heap: self::JavaScriptOrdinaryHeap, input: self::JavaScriptSemanticValue, exotic_key: self::JavaScriptSemanticValue, operation: &self::ml_string::Text) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        } else {
            self::finish_java_script_scalar_protocol(operation, self::begin_java_script_primitive_conversion(heap.clone(), input.clone(), exotic_key.clone(), &(if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]))) {
                self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
            } else {
                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
            })))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_scalar_protocol(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState, operation: &self::ml_string::Text) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        } else {
            if ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1016, _) => {
                    source_ml_source2_data_property_value1016.clone()
                }
            }) != (if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]))) {
                self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
            } else {
                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
            })) {
                self::JavaScriptScalarConversionTransition::ScalarInvalid(self::ml_string::Text::from_units(vec![115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 104, 105, 110, 116, 32, 111, 102, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
            } else {
                self::finish_java_script_scalar_protocol(operation, self::step_java_script_primitive_conversion(heap.clone(), state.clone()))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_scalar_protocol(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState, completion: self::JavaScriptPrimitiveConversionCompletion, operation: &self::ml_string::Text) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        } else {
            if ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1018, _) => {
                    source_ml_source2_data_property_value1018.clone()
                }
            }) != (if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]))) {
                self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
            } else {
                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
            })) {
                self::JavaScriptScalarConversionTransition::ScalarInvalid(self::ml_string::Text::from_units(vec![115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 104, 105, 110, 116, 32, 111, 102, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
            } else {
                self::finish_java_script_scalar_protocol(operation, self::resume_java_script_primitive_conversion(heap.clone(), state.clone(), completion.clone()))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_text_protocol_operation_error(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
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
            let high = ((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1019, _, _) => source_ml_source2_data_property_value1019.as_slice() }).len() as f64);
            {
                let source_ml_source2_s1020 = self::source_ml_source1_temporary_find_java_script_call_code_function_loop121(program.clone(), identity.clone(), low, high);
                match source_ml_source2_s1020.clone() {
                    self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop121Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop121Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop121Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop121Return(source_ml_source1_temporary_result121) => {
                        source_ml_source1_temporary_result121
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
        self::JavaScriptCallInstruction::Constant(destination, _) => {
            vec![destination]
        }
        self::JavaScriptCallInstruction::Copy(destination_2, source) => {
            vec![destination_2, source]
        }
        self::JavaScriptCallInstruction::Argument(destination_3, _) => {
            vec![destination_3]
        }
        self::JavaScriptCallInstruction::Jump(_) => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::Branch(condition, _, _) => {
            vec![condition]
        }
        self::JavaScriptCallInstruction::Call(destination_4, callee, receiver, arguments) => {
            {
                let source_ml_source2_concat_value1031 = vec![destination_4, callee, receiver];
                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value1031), arguments)
            }
        }
        self::JavaScriptCallInstruction::Return(source_2) => {
            vec![source_2]
        }
        self::JavaScriptCallInstruction::Throw(source_3) => {
            vec![source_3]
        }
        self::JavaScriptCallInstruction::PushHandler(destination_5, _) => {
            vec![destination_5]
        }
        self::JavaScriptCallInstruction::PopHandler => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::NumberBinary(destination_6, left, right, _) => {
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
        self::JavaScriptCallInstruction::Primitive(destination_9, _, arguments_2) => {
            {
                let source_ml_source2_concat_value1033 = vec![destination_9];
                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value1033), arguments_2)
            }
        }
        self::JavaScriptCallInstruction::OrdinaryGet(destination_10, object, key, receiver_2) => {
            vec![destination_10, object, key, receiver_2]
        }
        self::JavaScriptCallInstruction::OrdinarySet(destination_11, object_2, key_2, value, receiver_3) => {
            vec![destination_11, object_2, key_2, value, receiver_3]
        }
        self::JavaScriptCallInstruction::PropertyKey(destination_12, input, _) => {
            vec![destination_12, input]
        }
        self::JavaScriptCallInstruction::PropertyDefinition(destination_13, target, key_3, attributes, _) => {
            vec![destination_13, target, key_3, attributes]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_strict_equal(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone())) != (self::ml_string::Text::from_units(vec![]))) || ((self::read_java_script_semantic_value_error(right.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 111, 112, 101, 114, 97, 110, 100, 115])).0, message_present: true }).raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Null => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Boolean(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Number(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Number(value) => {
                        (left_value == value)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Text(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_descriptor_field(index: f64) -> (self::ml_string::Text, self::ml_string::Text, Vec<f64>) {
    if (index == (0f64)) {
        {
            let source_ml_source2_field1049 = vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64];
            (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101]), source_ml_source2_field1049)
        }
    } else {
        if (index == (1f64)) {
            {
                let source_ml_source2_field1052 = vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101]), source_ml_source2_field1052)
            }
        } else {
            if (index == (2f64)) {
                {
                    let source_ml_source2_field1055 = vec![118f64, 97f64, 108f64, 117f64, 101f64];
                    (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), source_ml_source2_field1055)
                }
            } else {
                if (index == (3f64)) {
                    {
                        let source_ml_source2_field1058 = vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64];
                        (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]), source_ml_source2_field1058)
                    }
                } else {
                    if (index == (4f64)) {
                        {
                            let source_ml_source2_field1061 = vec![103f64, 101f64, 116f64];
                            (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 116]), source_ml_source2_field1061)
                        }
                    } else {
                        if (index == (5f64)) {
                            {
                                let source_ml_source2_field1064 = vec![115f64, 101f64, 116f64];
                                (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]), self::ml_string::Text::from_units(vec![115, 101, 116]), source_ml_source2_field1064)
                            }
                        } else {
                            {
                                let source_ml_source2_field1067 = Vec::<f64>::new();
                                (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]), self::ml_string::Text::from_units(vec![]), source_ml_source2_field1067)
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
pub fn convert_java_script_semantic_boolean(value: self::JavaScriptSemanticValue) -> bool {
    if ((self::read_java_script_semantic_value_error(value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![84, 111, 66, 111, 111, 108, 101, 97, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Boolean(value_value) => {
                value_value
            }
            self::JavaScriptSemanticValue::Number(value_value_2) => {
                ((value_value_2 != (0f64)) && !(value_value_2.is_nan()))
            }
            self::JavaScriptSemanticValue::Integer(value_value_3) => {
                (value_value_3 != (self::ml::Big::from_i128(0)))
            }
            self::JavaScriptSemanticValue::Text(units) => {
                (((units.len() as f64)) != (0f64))
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                true
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                true
            }
            _ => {
                false
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_empty_descriptor() -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field1076 = self::JavaScriptSemanticValue::Undefined;
        {
            let source_ml_source2_field1080 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source2_field1082 = self::JavaScriptSemanticValue::Undefined;
                (false, false, false, false, source_ml_source2_field1080.clone(), false, source_ml_source2_field1082.clone(), false, source_ml_source2_field1076.clone(), false, false, false)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_descriptor_field_presence(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), index: f64) -> bool {
    if (index == (0f64)) {
        (descriptor.clone()).3
    } else {
        if (index == (1f64)) {
            (descriptor.clone()).1
        } else {
            if (index == (2f64)) {
                (descriptor.clone()).9
            } else {
                if (index == (3f64)) {
                    (descriptor.clone()).11
                } else {
                    if (index == (4f64)) {
                        (descriptor.clone()).5
                    } else {
                        if (index == (5f64)) {
                            (descriptor.clone()).7
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_descriptor_conversion_state_error(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                    heap_error.clone()
                } else {
                    if {
                        let source_ml_source2_s1084 = shared_constructor_field1.clone();
                        match source_ml_source2_s1084.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                false
                            }
                            _ => {
                                true
                            }
                        }
                    } {
                        self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                    } else {
                        {
                            let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field1.clone());
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                receiver_error.clone()
                            } else {
                                if ((((!(self::ml_math::is_safe_integer(shared_constructor_field2)) || (shared_constructor_field2 < (0f64))) || (shared_constructor_field2 > (6f64))) || ((shared_constructor_field2 == (0f64)) && (((1f64 / shared_constructor_field2)) < (0f64)))) || (shared_constructor_field4 && (shared_constructor_field2 == (6f64)))) {
                                    self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 102, 105, 101, 108, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                                } else {
                                    {
                                        let index = shared_constructor_field2;
                                        {
                                            let source_ml_source2_s1107 = self::source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop122(state.clone(), index);
                                            match source_ml_source2_s1107.clone() {
                                                self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Done => {
                                                    {
                                                        let descriptor = shared_constructor_field3.clone();
                                                        if ((descriptor.clone()).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
                                                            self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 118, 97, 108, 117, 101, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116])
                                                        } else {
                                                            if ((descriptor.clone()).5 && (((self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).4)))) {
                                                                self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 103, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121])
                                                            } else {
                                                                if ((descriptor.clone()).7 && (((self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).6)))) {
                                                                    self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Return(source_ml_source1_temporary_result122) => {
                                                    source_ml_source1_temporary_result122.clone()
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
