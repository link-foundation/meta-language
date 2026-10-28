// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
use super::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Result;
use super::MlTemporaryExecuteJavaScriptCallPrimitiveLoop34Result;
use super::MlTemporaryFindJavaScriptCallCodeFunctionLoop35Result;
use super::MlTemporaryPlanUtf16TextSubstitutionLoop25Result;
use super::MlTemporaryReplaceUtf16TextLoop26Result;
use super::character_units_utf16_text;
use super::character_utf16_text;
use super::code_point_utf16_text;
use super::compare_utf16_text;
use super::ends_utf16_text;
use super::includes_utf16_text;
use super::index_utf16_text;
use super::last_index_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_semantic_value_error;
use super::relative_character_utf16_text;
use super::slice_utf16_text;
use super::source_ml_temporary_display_points_utf16_text_loop22;
use super::source_ml_temporary_display_utf8_from_utf16_text_loop24;
use super::source_ml_temporary_execute_java_script_call_numeric_primitive_loop32;
use super::source_ml_temporary_execute_java_script_call_primitive_loop34;
use super::source_ml_temporary_find_java_script_call_code_function_loop35;
use super::source_ml_temporary_plan_utf16_text_substitution_loop25;
use super::source_ml_temporary_replace_utf16_text_loop26;
use super::source_ml_temporary_replace_utf16_text_loop27;
use super::starts_utf16_text;
use super::test_java_script_semantic_accessor;
use super::test_java_script_semantic_same_value;
use super::trim_utf16_text;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn display_points_utf16_text(text: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_temporary_display_points_utf16_text_loop22(text, result, at);
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
                    let bytes_2 = self::source_ml_temporary_display_utf8_from_utf16_text_loop24(bytes, &(points), at);
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
                    let source_ml_source1_s115 = self::source_ml_temporary_plan_utf16_text_substitution_loop25(replacement, result, start, at);
                    match source_ml_source1_s115.clone() {
                        self::MlTemporaryPlanUtf16TextSubstitutionLoop25Result::MlTemporaryPlanUtf16TextSubstitutionLoop25Done(result_2, start_2) => {
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
                        let source_ml_source1_s116 = self::source_ml_temporary_replace_utf16_text_loop26(text, search, replacement, all, result, &(plan), start, from);
                        match source_ml_source1_s116.clone() {
                            self::MlTemporaryReplaceUtf16TextLoop26Result::MlTemporaryReplaceUtf16TextLoop26Done(result_2, start_2) => {
                                {
                                    let at = start_2;
                                    {
                                        let result_3 = self::source_ml_temporary_replace_utf16_text_loop27(text, result_2, at);
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
pub fn read_java_script_call_primitive_arity_error(operation: &self::ml_string::Text, count: f64) -> self::ml_string::Text {
    if ((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || (count > (4096f64))) {
        self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 97, 108, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 98, 117, 100, 103, 101, 116])
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_numeric_primitive(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let numbers = Vec::<f64>::new();
        {
            let source_ml_temporary_values31 = values;
            {
                let source_ml_temporary_index31 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s117 = self::source_ml_temporary_execute_java_script_call_numeric_primitive_loop32(numbers, source_ml_temporary_values31, source_ml_temporary_index31.clone());
                    match source_ml_source1_s117.clone() {
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Done(numbers_2) => {
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
                        self::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Result::MlTemporaryExecuteJavaScriptCallNumericPrimitiveLoop32Return(source_ml_temporary_result32) => {
                            let source_ml_temporary_result32 = *source_ml_temporary_result32;
                            source_ml_temporary_result32.clone()
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f118) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f119) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f120) => {
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
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f121) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f122, source_ml_source1_f123) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f124) => {
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
                                            let source_ml_source1_comparison_operand130 = value_2.clone();
                                            {
                                                let source_ml_source1_comparison_operand131 = None::<()>;
                                                match (source_ml_source1_comparison_operand130.clone(), source_ml_source1_comparison_operand131.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
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
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f125) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f126) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f127) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f128, source_ml_source1_f129) => {
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f132) => {
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
                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f138) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Number(value_3) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::slice_utf16_text(text, second_value_2, value_3))))
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f139) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f140) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f141) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source1_f142, source_ml_source1_f143) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f133) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f134) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f135) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f136, source_ml_source1_f137) => {
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f144) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f145) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f146) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Text(units) => {
                    if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]))) {
                        if ((((text.len() as f64) + (units.len() as f64))) > (1048576f64)) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 111, 117, 116, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text({
                                let source_ml_source1_concat_value150 = text.to_vec();
                                {
                                    let source_ml_source1_concat_value151 = units;
                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value150), source_ml_source1_concat_value151)
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
                                                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f152) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Number(value) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if ((*operation) == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 116, 101, 120, 116]))) {
                                                            self::index_utf16_text(text, &(units), value)
                                                        } else {
                                                            self::last_index_utf16_text(text, &(units), value)
                                                        })))
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f153) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Text(source_ml_source1_f154) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f155) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Object(source_ml_source1_f156, source_ml_source1_f157) => {
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
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f147) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f148, source_ml_source1_f149) => {
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
                let source_ml_temporary_values33 = values.clone();
                {
                    let source_ml_temporary_index33 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s158 = self::source_ml_temporary_execute_java_script_call_primitive_loop34(source_ml_temporary_values33, source_ml_temporary_index33.clone());
                        match source_ml_source1_s158.clone() {
                            self::MlTemporaryExecuteJavaScriptCallPrimitiveLoop34Result::MlTemporaryExecuteJavaScriptCallPrimitiveLoop34Done => {
                                if (((((*operation) == (self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 105, 110, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114])))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 97, 120, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114])))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 116, 101, 120, 116, 32, 117, 110, 105, 116, 115])))) {
                                    self::execute_java_script_call_numeric_primitive(operation, values)
                                } else {
                                    self::execute_java_script_call_text_primitive(operation, values)
                                }
                            }
                            self::MlTemporaryExecuteJavaScriptCallPrimitiveLoop34Result::MlTemporaryExecuteJavaScriptCallPrimitiveLoop34Return(source_ml_temporary_result34) => {
                                let source_ml_temporary_result34 = *source_ml_temporary_result34;
                                source_ml_temporary_result34.clone()
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
pub fn test_java_script_call_code_coordinate(value: f64, limit: f64) -> bool {
    (((self::ml_math::is_safe_integer(value) && (value >= (0f64))) && (value < limit)) && ((value != (0f64)) || (((1f64 / value)) > (0f64))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_call_code_function(program: self::JavaScriptCallProgram, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = ((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value159, _, _) => source_ml_source1_data_property_value159.as_slice() }).len() as f64);
            {
                let source_ml_source1_s160 = self::source_ml_temporary_find_java_script_call_code_function_loop35(program.clone(), identity.clone(), low, high);
                match source_ml_source1_s160.clone() {
                    self::MlTemporaryFindJavaScriptCallCodeFunctionLoop35Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop35Done => {
                        (-1f64)
                    }
                    self::MlTemporaryFindJavaScriptCallCodeFunctionLoop35Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop35Return(source_ml_temporary_result35) => {
                        source_ml_temporary_result35
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
        self::JavaScriptCallInstruction::Constant(destination, source_ml_source1_f161) => {
            let source_ml_source1_f161 = *source_ml_source1_f161;
            vec![destination]
        }
        self::JavaScriptCallInstruction::Copy(destination_2, source) => {
            vec![destination_2, source]
        }
        self::JavaScriptCallInstruction::Argument(destination_3, source_ml_source1_f162) => {
            vec![destination_3]
        }
        self::JavaScriptCallInstruction::Jump(source_ml_source1_f163) => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::Branch(condition, source_ml_source1_f164, source_ml_source1_f165) => {
            vec![condition]
        }
        self::JavaScriptCallInstruction::Call(destination_4, callee, receiver, arguments) => {
            {
                let source_ml_source1_concat_value171 = vec![destination_4, callee, receiver];
                {
                    let source_ml_source1_concat_value172 = arguments;
                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value171), source_ml_source1_concat_value172)
                }
            }
        }
        self::JavaScriptCallInstruction::Return(source_2) => {
            vec![source_2]
        }
        self::JavaScriptCallInstruction::Throw(source_3) => {
            vec![source_3]
        }
        self::JavaScriptCallInstruction::PushHandler(destination_5, source_ml_source1_f166) => {
            vec![destination_5]
        }
        self::JavaScriptCallInstruction::PopHandler => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::NumberBinary(destination_6, left, right, source_ml_source1_f167) => {
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
        self::JavaScriptCallInstruction::Primitive(destination_9, source_ml_source1_f168, arguments_2) => {
            {
                let source_ml_source1_concat_value173 = vec![destination_9];
                {
                    let source_ml_source1_concat_value174 = arguments_2;
                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value173), source_ml_source1_concat_value174)
                }
            }
        }
        self::JavaScriptCallInstruction::OrdinaryGet(destination_10, object, key, receiver_2) => {
            vec![destination_10, object, key, receiver_2]
        }
        self::JavaScriptCallInstruction::OrdinarySet(destination_11, object_2, key_2, value, receiver_3) => {
            vec![destination_11, object_2, key_2, value, receiver_3]
        }
        self::JavaScriptCallInstruction::PropertyKey(destination_12, input, source_ml_source1_f169) => {
            vec![destination_12, input]
        }
        self::JavaScriptCallInstruction::PropertyDefinition(destination_13, target, key_3, attributes, source_ml_source1_f170) => {
            vec![destination_13, target, key_3, attributes]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_constant_error(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    self::read_java_script_semantic_value_error(value.clone())
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f175) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f181) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(value) => {
                        (left_value == value)
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f182) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f183) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f184) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f185, source_ml_source1_f186) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f176) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f177) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f178) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f179, source_ml_source1_f180) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_descriptor_kind(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> self::ml_string::Text {
    {
        let data = ((descriptor.clone()).9 || (descriptor.clone()).11);
        {
            let accessor = ((descriptor.clone()).5 || (descriptor.clone()).7);
            if (data && accessor) {
                self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])
            } else {
                if data {
                    self::ml_string::Text::from_units(vec![100, 97, 116, 97])
                } else {
                    if accessor {
                        self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])
                    } else {
                        self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99])
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_descriptor_error(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> self::ml_string::Text {
    if ((self::read_java_script_property_descriptor_kind(descriptor.clone())) == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]))) {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else {
        if ((descriptor.clone()).9 && ((self::read_java_script_semantic_value_error((descriptor.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
            self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 83, 101, 109, 97, 110, 116, 105, 99, 86, 97, 108, 117, 101])
        } else {
            if ((descriptor.clone()).5 && (((self::read_java_script_semantic_value_error((descriptor.clone()).4)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).4)))) {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                if ((descriptor.clone()).7 && (((self::read_java_script_semantic_value_error((descriptor.clone()).6)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).6)))) {
                    self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_complete_java_script_property_descriptor(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> bool {
    if ((!((descriptor.clone()).1) || !((descriptor.clone()).3)) || ((self::read_java_script_property_descriptor_error(descriptor.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        false
    } else {
        {
            let kind = self::read_java_script_property_descriptor_kind(descriptor.clone());
            if (kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97]))) {
                ((descriptor.clone()).9 && (descriptor.clone()).11)
            } else {
                (((kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) && (descriptor.clone()).5) && (descriptor.clone()).7)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_property_descriptor(kind: &self::ml_string::Text, configurable: bool, enumerable: bool) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source1_field187 = true;
        {
            let source_ml_source1_field188 = configurable;
            {
                let source_ml_source1_field189 = true;
                {
                    let source_ml_source1_field190 = enumerable;
                    {
                        let source_ml_source1_field191 = ((*kind) != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                        {
                            let source_ml_source1_field192 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field193 = ((*kind) != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                {
                                    let source_ml_source1_field194 = false;
                                    {
                                        let source_ml_source1_field195 = ((*kind) == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                        {
                                            let source_ml_source1_field196 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field197 = ((*kind) == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                                {
                                                    let source_ml_source1_field198 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source1_field188, source_ml_source1_field187, source_ml_source1_field190, source_ml_source1_field189, source_ml_source1_field196.clone(), source_ml_source1_field195, source_ml_source1_field198.clone(), source_ml_source1_field197, source_ml_source1_field192.clone(), source_ml_source1_field191, source_ml_source1_field194, source_ml_source1_field193)
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
pub fn apply_java_script_property_descriptor_fields(base: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), proposed: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source1_field199 = true;
        {
            let source_ml_source1_field200 = if (proposed.clone()).1 {
                (proposed.clone()).0
            } else {
                (base.clone()).0
            };
            {
                let source_ml_source1_field201 = true;
                {
                    let source_ml_source1_field202 = if (proposed.clone()).3 {
                        (proposed.clone()).2
                    } else {
                        (base.clone()).2
                    };
                    {
                        let source_ml_source1_field203 = (base.clone()).9;
                        {
                            let source_ml_source1_field204 = if ((base.clone()).9 && (proposed.clone()).9) {
                                (proposed.clone()).8
                            } else {
                                (base.clone()).8
                            };
                            {
                                let source_ml_source1_field205 = (base.clone()).11;
                                {
                                    let source_ml_source1_field206 = if ((base.clone()).11 && (proposed.clone()).11) {
                                        (proposed.clone()).10
                                    } else {
                                        (base.clone()).10
                                    };
                                    {
                                        let source_ml_source1_field207 = (base.clone()).5;
                                        {
                                            let source_ml_source1_field208 = if ((base.clone()).5 && (proposed.clone()).5) {
                                                (proposed.clone()).4
                                            } else {
                                                (base.clone()).4
                                            };
                                            {
                                                let source_ml_source1_field209 = (base.clone()).7;
                                                {
                                                    let source_ml_source1_field210 = if ((base.clone()).7 && (proposed.clone()).7) {
                                                        (proposed.clone()).6
                                                    } else {
                                                        (base.clone()).6
                                                    };
                                                    (source_ml_source1_field200, source_ml_source1_field199, source_ml_source1_field202, source_ml_source1_field201, source_ml_source1_field208.clone(), source_ml_source1_field207, source_ml_source1_field210.clone(), source_ml_source1_field209, source_ml_source1_field204.clone(), source_ml_source1_field203, source_ml_source1_field206, source_ml_source1_field205)
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
