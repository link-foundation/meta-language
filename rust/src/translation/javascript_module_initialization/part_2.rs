// Generated from complete js/src/translation/javascript-module-initialization.js. Do not edit by hand.
use super::JavaScriptModuleBindingSlot;
use super::JavaScriptModuleInitializationPlan;
use super::JavaScriptModuleInitializationState;
use super::JavaScriptModuleInitializationTransition;
use super::JavaScriptModuleInitializer;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop7Result;
use super::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Result;
use super::MlTemporaryReadJavaScriptModuleInitializationStateErrorJoin10;
use super::MlTemporaryReadJavaScriptModuleInitializationStateErrorLoop9Result;
use super::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_read_java_script_module_initialization_plan_error_loop7;
use super::source_ml_temporary_read_java_script_module_initialization_plan_error_loop8;
use super::source_ml_temporary_read_java_script_module_initialization_state_error_loop9;
use super::source_ml_temporary_test_java_script_semantic_text_equal_loop2;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_same_value(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone())) != (self::ml_string::Text::from_units(vec![]))) || ((self::read_java_script_semantic_value_error(right.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![83, 97, 109, 101, 86, 97, 108, 117, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 119, 111, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 117, 101, 115, 32, 105, 110, 32, 116, 104, 101, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 109, 111, 100, 101, 108])).0, message_present: true }).raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Null => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Boolean(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Boolean(value) => {
                        (left_value == value)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Number(left_value_2) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Number(value_2) => {
                        if (left_value_2 == value_2) {
                            ((left_value_2 != (0f64)) || (((1f64 / left_value_2)) == ((1f64 / value_2))))
                        } else {
                            (left_value_2.is_nan() && value_2.is_nan())
                        }
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(left_value_3) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Integer(value_3) => {
                        (left_value_3 == value_3)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(_) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(left_identity) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        (left_identity == identity)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Object(left_identity_2, _) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Object(identity_2, _) => {
                        (left_identity_2 == identity_2)
                    }
                    _ => {
                        false
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_text_length(value: self::JavaScriptSemanticValue) -> f64 {
    if match value.clone() {
        self::JavaScriptSemanticValue::SourceText(_, _) => {
            true
        }
        _ => {
            false
        }
    } {
        {
            let source_ml_source1_nullish_field_receiver78 = (match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    None::<self::ml_string::Text>
                }
                self::JavaScriptSemanticValue::Null => {
                    None::<self::ml_string::Text>
                }
                self::JavaScriptSemanticValue::Boolean(_) => {
                    None::<self::ml_string::Text>
                }
                self::JavaScriptSemanticValue::Number(_) => {
                    None::<self::ml_string::Text>
                }
                self::JavaScriptSemanticValue::Integer(_) => {
                    None::<self::ml_string::Text>
                }
                self::JavaScriptSemanticValue::Text(_) => {
                    None::<self::ml_string::Text>
                }
                self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value77) => {
                    Some(source_ml_source1_data_property_value77.clone())
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    None::<self::ml_string::Text>
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    None::<self::ml_string::Text>
                }
            },);
            if ((match (source_ml_source1_nullish_field_receiver78.clone()).0 { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                {
                    let source_ml_source1_nullish_field_present79 = ((source_ml_source1_nullish_field_receiver78.clone()).0).clone().expect("a proven present value");
                    source_ml_source1_nullish_field_present79.length()
                }
            } else {
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
            }
        }
    } else {
        if match value.clone() {
            self::JavaScriptSemanticValue::Text(_) => {
                true
            }
            _ => {
                false
            }
        } {
            {
                let source_ml_source1_nullish_field_receiver90 = (match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        None::<Vec<f64>>
                    }
                    self::JavaScriptSemanticValue::Null => {
                        None::<Vec<f64>>
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        None::<Vec<f64>>
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        None::<Vec<f64>>
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        None::<Vec<f64>>
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_data_property_value89) => {
                        Some(source_ml_source1_data_property_value89)
                    }
                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                        None::<Vec<f64>>
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        None::<Vec<f64>>
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        None::<Vec<f64>>
                    }
                },);
                if match (source_ml_source1_nullish_field_receiver90.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                    {
                        let source_ml_source1_nullish_field_present91 = ((source_ml_source1_nullish_field_receiver90.clone()).0).clone().expect("a proven present value");
                        (source_ml_source1_nullish_field_present91.len() as f64)
                    }
                } else {
                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                }
            }
        } else {
            (-1f64)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_text_equal(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    {
        let length = self::read_java_script_semantic_text_length(left.clone());
        if ((length < (0f64)) || (length != (self::read_java_script_semantic_text_length(right.clone())))) {
            false
        } else {
            if (match left.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => {
                    true
                }
                _ => {
                    false
                }
            } && match right.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => {
                    true
                }
                _ => {
                    false
                }
            }) {
                {
                    let source_ml_source1_comparison_operand146 = match left.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Null => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Number(_) => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value144) => {
                            Some(source_ml_source1_data_property_value144.clone())
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            None::<self::ml_string::Text>
                        }
                    };
                    {
                        let source_ml_source1_comparison_operand147 = match right.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Null => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Text(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value145) => {
                                Some(source_ml_source1_data_property_value145.clone())
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                None::<self::ml_string::Text>
                            }
                        };
                        match (source_ml_source1_comparison_operand146.clone(), source_ml_source1_comparison_operand147.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false }
                    }
                }
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s148 = self::source_ml_temporary_test_java_script_semantic_text_equal_loop2(left.clone(), right.clone(), length, at);
                        match source_ml_source1_s148.clone() {
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done => {
                                true
                            }
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(source_ml_temporary_result2) => {
                                source_ml_temporary_result2
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
pub fn read_java_script_module_initialization_profile_error(profile: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*profile) == (self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 114, 117, 110, 116, 105, 109, 101, 32, 76, 105, 110, 107, 115, 32, 112, 111, 108, 105, 99, 121, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 115, 32, 49]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![114, 117, 110, 116, 105, 109, 101, 32, 112, 111, 108, 105, 99, 121, 32, 109, 111, 100, 117, 108, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 45, 98, 111, 117, 110, 100, 32, 73, 79, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 112, 114, 111, 102, 105, 108, 101])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_module_initialization_phase(phase: &self::ml_string::Text) -> bool {
    self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![117, 110, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100]), self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 105, 110, 103]), self::ml_string::Text::from_units(vec![108, 111, 97, 100, 105, 110, 103]), self::ml_string::Text::from_units(vec![116, 101, 109, 112, 111, 114, 97, 108, 32, 100, 101, 97, 100, 32, 122, 111, 110, 101]), self::ml_string::Text::from_units(vec![101, 118, 97, 108, 117, 97, 116, 101, 100]), self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114, 101, 100])], phase, 0f64, false, |a, b| a == b)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_module_binding_read(phase: &self::ml_string::Text, initialized: bool) -> f64 {
    if ((*phase) == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114, 101, 100]))) {
        (-2f64)
    } else {
        if initialized {
            1f64
        } else {
            0f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_source_coordinate(value: f64, limit: f64) -> bool {
    (((self::ml_math::is_safe_integer(value) && (value >= (0f64))) && (value < limit)) && ((value != (0f64)) || (((1f64 / value)) > (0f64))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_module_initialization_plan_error(plan: self::JavaScriptModuleInitializationPlan) -> self::ml_string::Text {
    match plan.clone() {
        self::JavaScriptModuleInitializationPlan::ModuleInitializationPlan(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
            {
                let profile = self::read_java_script_module_initialization_profile_error(&(shared_constructor_field1));
                if (profile != (self::ml_string::Text::from_units(vec![]))) {
                    profile.clone()
                } else {
                    if (((((shared_constructor_field2 == (self::ml_string::Text::from_units(vec![]))) || (shared_constructor_field3 == (self::ml_string::Text::from_units(vec![])))) || (((shared_constructor_field4.len() as f64)) > (4096f64))) || (((shared_constructor_field4.len() as f64)) != ((shared_constructor_field5.len() as f64)))) || (((shared_constructor_field6.len() as f64)) != ((shared_constructor_field4.len() as f64)))) {
                        self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101, 32, 112, 108, 97, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 32, 111, 119, 110, 101, 114, 115])
                    } else {
                        {
                            let at = 0f64;
                            {
                                let source_ml_source1_s317 = self::source_ml_temporary_read_java_script_module_initialization_plan_error_loop7(plan.clone(), at);
                                match source_ml_source1_s317.clone() {
                                    self::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop7Result::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop7Done => {
                                        {
                                            let assigned = Vec::<f64>::new();
                                            {
                                                let source_ml_temporary_values6 = shared_constructor_field6;
                                                {
                                                    let source_ml_temporary_index6 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s319 = self::source_ml_temporary_read_java_script_module_initialization_plan_error_loop8(plan.clone(), assigned, source_ml_temporary_values6, source_ml_temporary_index6.clone());
                                                        match source_ml_source1_s319.clone() {
                                                            self::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Result::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Done => {
                                                                self::ml_string::Text::from_units(vec![])
                                                            }
                                                            self::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Result::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Return(source_ml_temporary_result8) => {
                                                                source_ml_temporary_result8.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop7Result::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop7Return(source_ml_temporary_result8_2) => {
                                        source_ml_temporary_result8_2.clone()
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
pub fn read_java_script_module_initialization_state_error(plan: self::JavaScriptModuleInitializationPlan, state: self::JavaScriptModuleInitializationState) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptModuleInitializationState::ModuleInitializationState(shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10, shared_constructor_field11, shared_constructor_field12) => {
            let shared_constructor_field12 = *shared_constructor_field12;
            match plan.clone() {
                self::JavaScriptModuleInitializationPlan::ModuleInitializationPlan(_, shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, _, shared_constructor_field4) => {
                    {
                        let error = self::read_java_script_module_initialization_plan_error(plan.clone());
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            error.clone()
                        } else {
                            if ((((((((shared_constructor_field5 != shared_constructor_field1) || (shared_constructor_field6 != shared_constructor_field2)) || (shared_constructor_field7 == (self::ml_string::Text::from_units(vec![])))) || (shared_constructor_field8 <= (self::ml::Big::from_i128(0)))) || (shared_constructor_field8 > (self::ml::Big::from_i128(18446744073709551615)))) || !(self::test_java_script_module_initialization_phase(&(shared_constructor_field9)))) || !(self::test_java_script_source_coordinate(shared_constructor_field10, ((shared_constructor_field4.len() as f64) + 1f64)))) || (((shared_constructor_field11.len() as f64)) != ((shared_constructor_field3.len() as f64)))) {
                                self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101, 32, 115, 116, 97, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 44, 32, 109, 111, 100, 117, 108, 101, 32, 85, 82, 76, 44, 32, 114, 101, 97, 108, 109, 32, 97, 110, 100, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115])
                            } else {
                                if ((((shared_constructor_field9 == (self::ml_string::Text::from_units(vec![117, 110, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100]))) && (shared_constructor_field10 != (0f64))) || ((shared_constructor_field9 == (self::ml_string::Text::from_units(vec![101, 118, 97, 108, 117, 97, 116, 101, 100]))) && (shared_constructor_field10 != ((shared_constructor_field4.len() as f64))))) || (self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![108, 111, 97, 100, 105, 110, 103]), self::ml_string::Text::from_units(vec![116, 101, 109, 112, 111, 114, 97, 108, 32, 100, 101, 97, 100, 32, 122, 111, 110, 101])], &shared_constructor_field9.clone(), 0f64, false, |a, b| a == b) && (shared_constructor_field10 == ((shared_constructor_field4.len() as f64))))) {
                                    self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101, 32, 112, 104, 97, 115, 101, 115, 32, 109, 117, 115, 116, 32, 97, 103, 114, 101, 101, 32, 119, 105, 116, 104, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])
                                } else {
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source1_s356 = self::source_ml_temporary_read_java_script_module_initialization_state_error_loop9(plan.clone(), state.clone(), at);
                                            match source_ml_source1_s356.clone() {
                                                self::MlTemporaryReadJavaScriptModuleInitializationStateErrorLoop9Result::MlTemporaryReadJavaScriptModuleInitializationStateErrorLoop9Done => {
                                                    if (((self::read_java_script_semantic_value_error(shared_constructor_field12.clone())) != (self::ml_string::Text::from_units(vec![]))) || ((shared_constructor_field9 != (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114, 101, 100]))) && {
                                                        let source_ml_source1_s362 = shared_constructor_field12.clone();
                                                        match source_ml_source1_s362.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    })) {
                                                        self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 32, 102, 97, 105, 108, 101, 100, 32, 109, 111, 100, 117, 108, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 99, 104, 101, 100, 32, 97, 98, 114, 117, 112, 116, 32, 118, 97, 108, 117, 101])
                                                    } else {
                                                        {
                                                            let source_ml_source1_s393 = if ((shared_constructor_field9 == (self::ml_string::Text::from_units(vec![108, 111, 97, 100, 105, 110, 103]))) || (shared_constructor_field9 == (self::ml_string::Text::from_units(vec![116, 101, 109, 112, 111, 114, 97, 108, 32, 100, 101, 97, 100, 32, 122, 111, 110, 101])))) {
                                                                {
                                                                    let initializer = self::ml_array::slice(&shared_constructor_field4, shared_constructor_field10, (shared_constructor_field10 + 1f64));
                                                                    {
                                                                        let valid = { let ml_values1 = initializer; let ml_callback1 = { let state = state.clone(); std::rc::Rc::new(move |entry: self::JavaScriptModuleInitializer| -> bool { match entry.clone() {
                                                                            self::JavaScriptModuleInitializer::ModuleDataLoad(_, _, _) => {
                                                                                ((match state.clone() {
                                                                                    self::JavaScriptModuleInitializationState::ModuleInitializationState(_, _, _, _, source_ml_source1_data_property_value407, _, _, _) => {
                                                                                        source_ml_source1_data_property_value407.clone()
                                                                                    }
                                                                                }) == (self::ml_string::Text::from_units(vec![108, 111, 97, 100, 105, 110, 103])))
                                                                            }
                                                                            self::JavaScriptModuleInitializer::ModuleConstAlias(_, target) => {
                                                                                (((match state.clone() {
                                                                                    self::JavaScriptModuleInitializationState::ModuleInitializationState(_, _, _, _, source_ml_source1_data_property_value409, _, _, _) => {
                                                                                        source_ml_source1_data_property_value409.clone()
                                                                                    }
                                                                                }) == (self::ml_string::Text::from_units(vec![116, 101, 109, 112, 111, 114, 97, 108, 32, 100, 101, 97, 100, 32, 122, 111, 110, 101]))) && !({ let ml_values2 = match state.clone() {
                                                                                    self::JavaScriptModuleInitializationState::ModuleInitializationState(_, _, _, _, _, _, source_ml_source1_data_property_value410, _) => {
                                                                                        source_ml_source1_data_property_value410.clone()
                                                                                    }
                                                                                }; let ml_callback2 = { let target = target; std::rc::Rc::new(move |slot: self::JavaScriptModuleBindingSlot| -> bool { (((match slot.clone() {
                                                                                    self::JavaScriptModuleBindingSlot::ModuleBindingSlot(source_ml_source1_data_property_value412, _, _) => {
                                                                                        source_ml_source1_data_property_value412
                                                                                    }
                                                                                }) == target) && match slot.clone() {
                                                                                    self::JavaScriptModuleBindingSlot::ModuleBindingSlot(_, source_ml_source1_data_property_value413, _) => {
                                                                                        source_ml_source1_data_property_value413
                                                                                    }
                                                                                }) }) }; ml_values2.iter().cloned().enumerate().any(|(ml_index2, ml_value2)| ml_callback2(ml_value2)) }))
                                                                            }
                                                                        } }) }; ml_values1.iter().cloned().enumerate().any(|(ml_index1, ml_value1)| ml_callback1(ml_value1)) };
                                                                        if !valid {
                                                                            self::MlTemporaryReadJavaScriptModuleInitializationStateErrorJoin10::MlTemporaryReadJavaScriptModuleInitializationStateErrorJoin10Return(self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101, 32, 115, 117, 115, 112, 101, 110, 115, 105, 111, 110, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 108, 111, 97, 100, 32, 111, 114, 32, 117, 110, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 97, 108, 105, 97, 115]))
                                                                        } else {
                                                                            self::MlTemporaryReadJavaScriptModuleInitializationStateErrorJoin10::MlTemporaryReadJavaScriptModuleInitializationStateErrorJoin10Next
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                self::MlTemporaryReadJavaScriptModuleInitializationStateErrorJoin10::MlTemporaryReadJavaScriptModuleInitializationStateErrorJoin10Next
                                                            };
                                                            match source_ml_source1_s393.clone() {
                                                                self::MlTemporaryReadJavaScriptModuleInitializationStateErrorJoin10::MlTemporaryReadJavaScriptModuleInitializationStateErrorJoin10Next => {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                }
                                                                self::MlTemporaryReadJavaScriptModuleInitializationStateErrorJoin10::MlTemporaryReadJavaScriptModuleInitializationStateErrorJoin10Return(source_ml_temporary_result10) => {
                                                                    source_ml_temporary_result10.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptModuleInitializationStateErrorLoop9Result::MlTemporaryReadJavaScriptModuleInitializationStateErrorLoop9Return(source_ml_temporary_result10_2) => {
                                                    source_ml_temporary_result10_2.clone()
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
pub fn create_java_script_module_initialization(plan: self::JavaScriptModuleInitializationPlan, module_url: &self::ml_string::Text, realm: self::ml::Big) -> self::JavaScriptModuleInitializationState {
    match plan.clone() {
        self::JavaScriptModuleInitializationPlan::ModuleInitializationPlan(_, shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, _, _) => {
            self::JavaScriptModuleInitializationState::ModuleInitializationState(shared_constructor_field1.clone(), shared_constructor_field2.clone(), module_url.clone(), realm.clone(), self::ml_string::Text::from_units(vec![117, 110, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100]), 0f64, { let ml_values3 = shared_constructor_field3; let ml_callback3 = std::rc::Rc::new(move |binding: f64| -> self::JavaScriptModuleBindingSlot { self::JavaScriptModuleBindingSlot::ModuleBindingSlot(binding, false, Box::new(self::JavaScriptSemanticValue::Undefined)) }); ml_values3.iter().cloned().enumerate().map(|(ml_index3, ml_value3)| ml_callback3(ml_value3)).collect::<Vec<_>>() }, Box::new(self::JavaScriptSemanticValue::Undefined))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_module_initialization(plan: self::JavaScriptModuleInitializationPlan, state: self::JavaScriptModuleInitializationState) -> self::JavaScriptModuleInitializationTransition {
    match state.clone() {
        self::JavaScriptModuleInitializationState::ModuleInitializationState(shared_constructor_field12, shared_constructor_field13, shared_constructor_field14, shared_constructor_field15, shared_constructor_field16, shared_constructor_field17, shared_constructor_field18, shared_constructor_field19) => {
            let shared_constructor_field19 = *shared_constructor_field19;
            match plan.clone() {
                self::JavaScriptModuleInitializationPlan::ModuleInitializationPlan(_, _, _, shared_constructor_field9, shared_constructor_field10, shared_constructor_field11) => {
                    {
                        let error = self::read_java_script_module_initialization_state_error(plan.clone(), state.clone());
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            self::JavaScriptModuleInitializationTransition::ModuleInvalid(Box::new(state.clone()), error.clone())
                        } else {
                            if (shared_constructor_field16 == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114, 101, 100]))) {
                                self::JavaScriptModuleInitializationTransition::ModuleAbrupt(Box::new(state.clone()), Box::new(shared_constructor_field19.clone()))
                            } else {
                                if (shared_constructor_field16 == (self::ml_string::Text::from_units(vec![101, 118, 97, 108, 117, 97, 116, 101, 100]))) {
                                    self::JavaScriptModuleInitializationTransition::ModuleEvaluated(Box::new(state.clone()))
                                } else {
                                    if ((shared_constructor_field16 == (self::ml_string::Text::from_units(vec![108, 111, 97, 100, 105, 110, 103]))) || (shared_constructor_field16 == (self::ml_string::Text::from_units(vec![116, 101, 109, 112, 111, 114, 97, 108, 32, 100, 101, 97, 100, 32, 122, 111, 110, 101])))) {
                                        self::JavaScriptModuleInitializationTransition::ModuleWaiting(Box::new(state.clone()))
                                    } else {
                                        if (shared_constructor_field16 == (self::ml_string::Text::from_units(vec![117, 110, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100]))) {
                                            self::JavaScriptModuleInitializationTransition::ModuleContinue(Box::new({
                                                let source_ml_source1_nominal_field429 = shared_constructor_field12.clone();
                                                {
                                                    let source_ml_source1_nominal_field431 = shared_constructor_field13.clone();
                                                    {
                                                        let source_ml_source1_nominal_field433 = shared_constructor_field14.clone();
                                                        {
                                                            let source_ml_source1_nominal_field435 = shared_constructor_field15.clone();
                                                            {
                                                                let source_ml_source1_nominal_field437 = shared_constructor_field16.clone();
                                                                {
                                                                    let source_ml_source1_nominal_field439 = shared_constructor_field17;
                                                                    {
                                                                        let source_ml_source1_nominal_field441 = shared_constructor_field18;
                                                                        {
                                                                            let source_ml_source1_nominal_field443 = shared_constructor_field19.clone();
                                                                            self::JavaScriptModuleInitializationState::ModuleInitializationState(source_ml_source1_nominal_field429.clone(), source_ml_source1_nominal_field431.clone(), source_ml_source1_nominal_field433.clone(), source_ml_source1_nominal_field435.clone(), self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 105, 110, 103]), source_ml_source1_nominal_field439, source_ml_source1_nominal_field441, Box::new(source_ml_source1_nominal_field443.clone()))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }))
                                        } else {
                                            if (shared_constructor_field17 == ((shared_constructor_field11.len() as f64))) {
                                                self::JavaScriptModuleInitializationTransition::ModuleEvaluated(Box::new({
                                                    let source_ml_source1_nominal_field451 = shared_constructor_field12.clone();
                                                    {
                                                        let source_ml_source1_nominal_field453 = shared_constructor_field13.clone();
                                                        {
                                                            let source_ml_source1_nominal_field455 = shared_constructor_field14.clone();
                                                            {
                                                                let source_ml_source1_nominal_field457 = shared_constructor_field15.clone();
                                                                {
                                                                    let source_ml_source1_nominal_field459 = shared_constructor_field16.clone();
                                                                    {
                                                                        let source_ml_source1_nominal_field461 = shared_constructor_field17;
                                                                        {
                                                                            let source_ml_source1_nominal_field463 = shared_constructor_field18;
                                                                            {
                                                                                let source_ml_source1_nominal_field465 = shared_constructor_field19.clone();
                                                                                self::JavaScriptModuleInitializationState::ModuleInitializationState(source_ml_source1_nominal_field451.clone(), source_ml_source1_nominal_field453.clone(), source_ml_source1_nominal_field455.clone(), source_ml_source1_nominal_field457.clone(), self::ml_string::Text::from_units(vec![101, 118, 97, 108, 117, 97, 116, 101, 100]), source_ml_source1_nominal_field461, source_ml_source1_nominal_field463, Box::new(source_ml_source1_nominal_field465.clone()))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }))
                                            } else {
                                                {
                                                    let initializer = self::ml_array::optional_at(&shared_constructor_field11, self::ml_array::number_index(shared_constructor_field17));
                                                    if match (initializer.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::JavaScriptModuleInitializationTransition::ModuleInvalid(Box::new(state.clone()), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 109, 111, 100, 117, 108, 101, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 109, 117, 115, 116, 32, 101, 120, 105, 115, 116]))
                                                    } else {
                                                        match initializer.clone().expect("a proven present value") {
                                                            self::JavaScriptModuleInitializer::ModuleDataLoad(binding, specifier, reader_owner) => {
                                                                self::JavaScriptModuleInitializationTransition::ModuleReadData(Box::new({
                                                                    let source_ml_source1_nominal_field471 = shared_constructor_field12.clone();
                                                                    {
                                                                        let source_ml_source1_nominal_field473 = shared_constructor_field13.clone();
                                                                        {
                                                                            let source_ml_source1_nominal_field475 = shared_constructor_field14.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field477 = shared_constructor_field15.clone();
                                                                                {
                                                                                    let source_ml_source1_nominal_field479 = shared_constructor_field16.clone();
                                                                                    {
                                                                                        let source_ml_source1_nominal_field481 = shared_constructor_field17;
                                                                                        {
                                                                                            let source_ml_source1_nominal_field483 = shared_constructor_field18;
                                                                                            {
                                                                                                let source_ml_source1_nominal_field485 = shared_constructor_field19.clone();
                                                                                                self::JavaScriptModuleInitializationState::ModuleInitializationState(source_ml_source1_nominal_field471.clone(), source_ml_source1_nominal_field473.clone(), source_ml_source1_nominal_field475.clone(), source_ml_source1_nominal_field477.clone(), self::ml_string::Text::from_units(vec![108, 111, 97, 100, 105, 110, 103]), source_ml_source1_nominal_field481, source_ml_source1_nominal_field483, Box::new(source_ml_source1_nominal_field485.clone()))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }), binding, specifier.clone(), reader_owner.clone())
                                                            }
                                                            self::JavaScriptModuleInitializer::ModuleConstAlias(binding_2, initializer_target) => {
                                                                {
                                                                    let target = { let ml_values4 = shared_constructor_field18.clone(); let ml_callback4 = { let initializer_target = initializer_target; std::rc::Rc::new(move |slot: self::JavaScriptModuleBindingSlot| -> bool { ((match slot.clone() {
                                                                        self::JavaScriptModuleBindingSlot::ModuleBindingSlot(source_ml_source1_data_property_value490, _, _) => {
                                                                            source_ml_source1_data_property_value490
                                                                        }
                                                                    }) == initializer_target) }) }; ml_values4.iter().cloned().enumerate().find_map(|(ml_index4, ml_value4)| if ml_callback4(ml_value4.clone()) { Some(ml_value4) } else { None }) };
                                                                    if match (target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                        self::JavaScriptModuleInitializationTransition::ModuleInvalid(Box::new(state.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 111, 110, 115, 116, 32, 97, 108, 105, 97, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 116, 97, 114, 103, 101, 116]))
                                                                    } else {
                                                                        if !(match target.clone().expect("a proven present value") {
                                                                            self::JavaScriptModuleBindingSlot::ModuleBindingSlot(_, source_ml_source1_data_property_value491, _) => {
                                                                                source_ml_source1_data_property_value491
                                                                            }
                                                                        }) {
                                                                            self::JavaScriptModuleInitializationTransition::ModuleTemporalDeadZone(Box::new({
                                                                                let source_ml_source1_nominal_field493 = shared_constructor_field12.clone();
                                                                                {
                                                                                    let source_ml_source1_nominal_field495 = shared_constructor_field13.clone();
                                                                                    {
                                                                                        let source_ml_source1_nominal_field497 = shared_constructor_field14.clone();
                                                                                        {
                                                                                            let source_ml_source1_nominal_field499 = shared_constructor_field15.clone();
                                                                                            {
                                                                                                let source_ml_source1_nominal_field501 = shared_constructor_field16.clone();
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field503 = shared_constructor_field17;
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_field505 = shared_constructor_field18;
                                                                                                        {
                                                                                                            let source_ml_source1_nominal_field507 = shared_constructor_field19.clone();
                                                                                                            self::JavaScriptModuleInitializationState::ModuleInitializationState(source_ml_source1_nominal_field493.clone(), source_ml_source1_nominal_field495.clone(), source_ml_source1_nominal_field497.clone(), source_ml_source1_nominal_field499.clone(), self::ml_string::Text::from_units(vec![116, 101, 109, 112, 111, 114, 97, 108, 32, 100, 101, 97, 100, 32, 122, 111, 110, 101]), source_ml_source1_nominal_field503, source_ml_source1_nominal_field505, Box::new(source_ml_source1_nominal_field507.clone()))
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }), initializer_target, (self::ml_array::optional_at(&shared_constructor_field10, self::ml_array::number_index(self::ml_array::index_of(&shared_constructor_field9, &initializer_target, 0f64, false, |a, b| a == b)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                                                                        } else {
                                                                            {
                                                                                let next = {
                                                                                    let source_ml_source1_nominal_spread527 = state.clone();
                                                                                    match source_ml_source1_nominal_spread527.clone() {
                                                                                        self::JavaScriptModuleInitializationState::ModuleInitializationState(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8) => {
                                                                                            let shared_constructor_field8 = *shared_constructor_field8;
                                                                                            {
                                                                                                let source_ml_source1_nominal_field528 = shared_constructor_field1.clone();
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field530 = shared_constructor_field2.clone();
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_field532 = shared_constructor_field3.clone();
                                                                                                        {
                                                                                                            let source_ml_source1_nominal_field534 = shared_constructor_field4.clone();
                                                                                                            {
                                                                                                                let source_ml_source1_nominal_field536 = shared_constructor_field5.clone();
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_field538 = shared_constructor_field6;
                                                                                                                    {
                                                                                                                        let source_ml_source1_nominal_field540 = shared_constructor_field7;
                                                                                                                        {
                                                                                                                            let source_ml_source1_nominal_field542 = shared_constructor_field8.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source1_nominal_override544 = (shared_constructor_field17 + 1f64);
                                                                                                                                {
                                                                                                                                    let source_ml_source1_nominal_override545 = { let ml_values5 = shared_constructor_field18; let ml_callback5 = { let binding_2 = binding_2; let target = target.clone(); std::rc::Rc::new(move |slot: self::JavaScriptModuleBindingSlot| -> self::JavaScriptModuleBindingSlot { if ((match slot.clone() {
                                                                                                                                        self::JavaScriptModuleBindingSlot::ModuleBindingSlot(source_ml_source1_data_property_value516, _, _) => {
                                                                                                                                            source_ml_source1_data_property_value516
                                                                                                                                        }
                                                                                                                                    }) == binding_2) {
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_nominal_spread518 = slot.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source1_nominal_field519 = match source_ml_source1_nominal_spread518.clone() {
                                                                                                                                                    self::JavaScriptModuleBindingSlot::ModuleBindingSlot(source_ml_source1_data_property_value520, _, _) => {
                                                                                                                                                        source_ml_source1_data_property_value520
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_source1_nominal_field521 = match source_ml_source1_nominal_spread518.clone() {
                                                                                                                                                        self::JavaScriptModuleBindingSlot::ModuleBindingSlot(_, source_ml_source1_data_property_value522, _) => {
                                                                                                                                                            source_ml_source1_data_property_value522
                                                                                                                                                        }
                                                                                                                                                    };
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source1_nominal_field523 = match source_ml_source1_nominal_spread518.clone() {
                                                                                                                                                            self::JavaScriptModuleBindingSlot::ModuleBindingSlot(_, _, source_ml_source1_data_property_value524) => {
                                                                                                                                                                let source_ml_source1_data_property_value524 = *source_ml_source1_data_property_value524;
                                                                                                                                                                source_ml_source1_data_property_value524.clone()
                                                                                                                                                            }
                                                                                                                                                        };
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source1_nominal_override525 = true;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source1_nominal_override526 = match target.clone().expect("a proven present value") {
                                                                                                                                                                    self::JavaScriptModuleBindingSlot::ModuleBindingSlot(_, _, source_ml_source1_data_property_value517) => {
                                                                                                                                                                        let source_ml_source1_data_property_value517 = *source_ml_source1_data_property_value517;
                                                                                                                                                                        source_ml_source1_data_property_value517.clone()
                                                                                                                                                                    }
                                                                                                                                                                };
                                                                                                                                                                self::JavaScriptModuleBindingSlot::ModuleBindingSlot(source_ml_source1_nominal_field519, source_ml_source1_nominal_override525, Box::new(source_ml_source1_nominal_override526.clone()))
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    } else {
                                                                                                                                        slot.clone()
                                                                                                                                    } }) }; ml_values5.iter().cloned().enumerate().map(|(ml_index5, ml_value5)| ml_callback5(ml_value5)).collect::<Vec<_>>() };
                                                                                                                                    self::JavaScriptModuleInitializationState::ModuleInitializationState(source_ml_source1_nominal_field528.clone(), source_ml_source1_nominal_field530.clone(), source_ml_source1_nominal_field532.clone(), source_ml_source1_nominal_field534.clone(), source_ml_source1_nominal_field536.clone(), source_ml_source1_nominal_override544, source_ml_source1_nominal_override545, Box::new(source_ml_source1_nominal_field542.clone()))
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
                                                                                };
                                                                                self::JavaScriptModuleInitializationTransition::ModuleContinue(Box::new(next.clone()))
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
