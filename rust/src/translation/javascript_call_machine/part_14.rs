// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlTemporaryPushJavaScriptCallLoop89Result;
use super::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result;
use super::MlTemporaryReadJavaScriptCallProgramErrorLoop70Result;
use super::MlTemporaryReadJavaScriptCallProgramErrorLoop71Result;
use super::MlTemporaryThrowJavaScriptCallValueLoop87Result;
use super::begin_java_script_descriptor_conversion;
use super::begin_java_script_property_key_conversion;
use super::complete_java_script_property_definition;
use super::find_java_script_call_code_function;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::project_java_script_property_key_completion;
use super::read_java_script_call_code_registers;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::resume_java_script_descriptor_conversion;
use super::resume_java_script_property_key_conversion;
use super::source_ml_temporary_create_java_script_call_frame_loop75;
use super::source_ml_temporary_create_java_script_call_frame_loop76;
use super::source_ml_temporary_create_java_script_call_frame_loop77;
use super::source_ml_temporary_push_java_script_call_loop89;
use super::source_ml_temporary_read_java_script_call_machine_error_loop81;
use super::source_ml_temporary_read_java_script_call_program_error_loop70;
use super::source_ml_temporary_read_java_script_call_program_error_loop71;
use super::source_ml_temporary_throw_java_script_call_value_loop87;
use super::step_java_script_descriptor_conversion;
use super::step_java_script_property_key_conversion;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_property_descriptor_definition(heap: self::JavaScriptOrdinaryHeap, target: self::JavaScriptSemanticValue, key: self::JavaScriptSemanticValue, transition: self::JavaScriptDescriptorConversionTransition) -> self::JavaScriptPropertyDefinitionTransition {
    match transition.clone() {
        self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(state) => {
            let state = *state;
            self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(Box::new(self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(Box::new(target.clone()), Box::new(key.clone()), Box::new(state.clone()))))
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorCall(state_2, callee, receiver, arguments) => {
            let state_2 = *state_2;
            let callee = *callee;
            let receiver = *receiver;
            self::JavaScriptPropertyDefinitionTransition::DefinitionCall(Box::new(self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(Box::new(target.clone()), Box::new(key.clone()), Box::new(state_2.clone()))), Box::new(callee.clone()), Box::new(receiver.clone()), arguments)
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(descriptor) => {
            self::complete_java_script_property_definition(heap.clone(), target.clone(), key.clone(), descriptor.clone())
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorThrow(value) => {
            let value = *value;
            self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(Box::new(value.clone()))
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(error_class, reason) => {
            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(error_class.clone(), reason.clone())
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(reason_2) => {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(reason_2.clone())
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_property_key_definition(heap: self::JavaScriptOrdinaryHeap, target: self::JavaScriptSemanticValue, attributes: self::JavaScriptSemanticValue, transition: self::JavaScriptPropertyKeyConversionTransition) -> self::JavaScriptPropertyDefinitionTransition {
    match transition.clone() {
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(state) => {
            let state = *state;
            self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(Box::new(self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(Box::new(target.clone()), Box::new(attributes.clone()), Box::new(state.clone()))))
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCall(state_2, callee, receiver, arguments) => {
            let state_2 = *state_2;
            let callee = *callee;
            let receiver = *receiver;
            self::JavaScriptPropertyDefinitionTransition::DefinitionCall(Box::new(self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(Box::new(target.clone()), Box::new(attributes.clone()), Box::new(state_2.clone()))), Box::new(callee.clone()), Box::new(receiver.clone()), arguments)
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(key) => {
            let key = *key;
            self::continue_java_script_property_descriptor_definition(heap.clone(), target.clone(), key.clone(), self::begin_java_script_descriptor_conversion(heap.clone(), attributes.clone()))
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyThrow(value) => {
            let value = *value;
            self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(Box::new(value.clone()))
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyIntrinsicError(error_class, reason) => {
            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(error_class.clone(), reason.clone())
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(reason_2) => {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(reason_2.clone())
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_property_definition(heap: self::JavaScriptOrdinaryHeap, target: self::JavaScriptSemanticValue, key_input: self::JavaScriptSemanticValue, attributes: self::JavaScriptSemanticValue, exotic_key: self::JavaScriptSemanticValue) -> self::JavaScriptPropertyDefinitionTransition {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            {
                let value_error = self::read_java_script_ordinary_value_error(heap.clone(), target.clone());
                if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(value_error.clone())
                } else {
                    match target.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1147) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1148) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1149) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1150) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1151) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1152, source_ml_source1_f1153) => {
                            self::continue_java_script_property_key_definition(heap.clone(), target.clone(), attributes.clone(), self::begin_java_script_property_key_conversion(heap.clone(), key_input.clone(), exotic_key.clone()))
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_property_definition(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPropertyDefinitionState) -> self::JavaScriptPropertyDefinitionTransition {
    {
        let error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source1_data_property_value1154, _, _) => {
                let source_ml_source1_data_property_value1154 = *source_ml_source1_data_property_value1154;
                source_ml_source1_data_property_value1154.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source1_data_property_value1155, _, _) => {
                let source_ml_source1_data_property_value1155 = *source_ml_source1_data_property_value1155;
                source_ml_source1_data_property_value1155.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source1_s1158 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source1_data_property_value1166, _, _) => {
                        let source_ml_source1_data_property_value1166 = *source_ml_source1_data_property_value1166;
                        source_ml_source1_data_property_value1166.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source1_data_property_value1167, _, _) => {
                        let source_ml_source1_data_property_value1167 = *source_ml_source1_data_property_value1167;
                        source_ml_source1_data_property_value1167.clone()
                    }
                };
                match source_ml_source1_s1158.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1159) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f1160) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f1161) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f1162) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1163) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f1164, source_ml_source1_f1165) => {
                        false
                    }
                }
            } {
                self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
            } else {
                match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(target, attributes, conversion) => {
                        let target = *target;
                        let attributes = *attributes;
                        let conversion = *conversion;
                        self::continue_java_script_property_key_definition(heap.clone(), target.clone(), attributes.clone(), self::step_java_script_property_key_conversion(heap.clone(), conversion.clone()))
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(target_2, key, conversion_2) => {
                        let target_2 = *target_2;
                        let key = *key;
                        let conversion_2 = *conversion_2;
                        self::continue_java_script_property_descriptor_definition(heap.clone(), target_2.clone(), key.clone(), self::step_java_script_descriptor_conversion(heap.clone(), conversion_2.clone()))
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_property_definition(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPropertyDefinitionState, completion: self::JavaScriptSemanticCompletion) -> self::JavaScriptPropertyDefinitionTransition {
    {
        let error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source1_data_property_value1168, _, _) => {
                let source_ml_source1_data_property_value1168 = *source_ml_source1_data_property_value1168;
                source_ml_source1_data_property_value1168.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source1_data_property_value1169, _, _) => {
                let source_ml_source1_data_property_value1169 = *source_ml_source1_data_property_value1169;
                source_ml_source1_data_property_value1169.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source1_s1172 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source1_data_property_value1180, _, _) => {
                        let source_ml_source1_data_property_value1180 = *source_ml_source1_data_property_value1180;
                        source_ml_source1_data_property_value1180.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source1_data_property_value1181, _, _) => {
                        let source_ml_source1_data_property_value1181 = *source_ml_source1_data_property_value1181;
                        source_ml_source1_data_property_value1181.clone()
                    }
                };
                match source_ml_source1_s1172.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1173) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f1174) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f1175) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f1176) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1177) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f1178, source_ml_source1_f1179) => {
                        false
                    }
                }
            } {
                self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
            } else {
                match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(target, attributes, conversion) => {
                        let target = *target;
                        let attributes = *attributes;
                        let conversion = *conversion;
                        self::continue_java_script_property_key_definition(heap.clone(), target.clone(), attributes.clone(), self::resume_java_script_property_key_conversion(heap.clone(), conversion.clone(), self::project_java_script_property_key_completion(completion.clone())))
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(target_2, key, conversion_2) => {
                        let target_2 = *target_2;
                        let key = *key;
                        let conversion_2 = *conversion_2;
                        self::continue_java_script_property_descriptor_definition(heap.clone(), target_2.clone(), key.clone(), self::resume_java_script_descriptor_conversion(heap.clone(), conversion_2.clone(), completion.clone()))
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_coordinate(value: f64, limit: f64) -> bool {
    self::test_java_script_call_code_coordinate(value, limit)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_call_function(program: self::JavaScriptCallProgram, identity: self::ml::Big) -> f64 {
    self::find_java_script_call_code_function(program.clone(), identity.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_registers(instruction: self::JavaScriptCallInstruction) -> Vec<f64> {
    self::read_java_script_call_code_registers(instruction.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_program_error(program: self::JavaScriptCallProgram, heap: self::JavaScriptOrdinaryHeap) -> self::ml_string::Text {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            if (((((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1183, _, _) => source_ml_source1_data_property_value1183.as_slice() }).len() as f64)) > (4096f64)) || ((((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value1185, _) => source_ml_source1_data_property_value1185.as_slice() }).len() as f64)) > (4096f64))) {
                self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let previous = self::ml::Big::from_i128(0);
                    {
                        let source_ml_temporary_values65 = match program.clone() {
                            self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value1186, _) => {
                                source_ml_source1_data_property_value1186
                            }
                        };
                        {
                            let source_ml_temporary_index65 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source1_s1187 = self::source_ml_temporary_read_java_script_call_program_error_loop70(previous.clone(), source_ml_temporary_values65, source_ml_temporary_index65.clone());
                                match source_ml_source1_s1187.clone() {
                                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop70Result::MlTemporaryReadJavaScriptCallProgramErrorLoop70Done(previous_2) => {
                                        if {
                                            let source_ml_source1_s1189 = match program.clone() {
                                                self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source1_data_property_value1197) => {
                                                    let source_ml_source1_data_property_value1197 = *source_ml_source1_data_property_value1197;
                                                    source_ml_source1_data_property_value1197.clone()
                                                }
                                            };
                                            match source_ml_source1_s1189.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1190) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f1191) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f1192) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f1193) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1194) => {
                                                    false
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f1195, source_ml_source1_f1196) => {
                                                    true
                                                }
                                            }
                                        } {
                                            self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 117, 116, 104, 111, 114, 105, 116, 97, 116, 105, 118, 101, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 83, 121, 109, 98, 111, 108])
                                        } else {
                                            {
                                                let key_error = self::read_java_script_ordinary_value_error(heap.clone(), match program.clone() {
                                                    self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source1_data_property_value1198) => {
                                                        let source_ml_source1_data_property_value1198 = *source_ml_source1_data_property_value1198;
                                                        source_ml_source1_data_property_value1198.clone()
                                                    }
                                                });
                                                if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                                    key_error.clone()
                                                } else {
                                                    {
                                                        let previous_3 = self::ml::Big::from_i128(0);
                                                        {
                                                            let count = 0f64;
                                                            {
                                                                let source_ml_temporary_values66 = match program.clone() {
                                                                    self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1199, _, _) => {
                                                                        source_ml_source1_data_property_value1199
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_temporary_index66 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let source_ml_source1_s1200 = self::source_ml_temporary_read_java_script_call_program_error_loop71(program.clone(), heap.clone(), previous_3.clone(), count, source_ml_temporary_values66, source_ml_temporary_index66.clone());
                                                                        match source_ml_source1_s1200.clone() {
                                                                            self::MlTemporaryReadJavaScriptCallProgramErrorLoop71Result::MlTemporaryReadJavaScriptCallProgramErrorLoop71Done => {
                                                                                self::ml_string::Text::from_units(vec![])
                                                                            }
                                                                            self::MlTemporaryReadJavaScriptCallProgramErrorLoop71Result::MlTemporaryReadJavaScriptCallProgramErrorLoop71Return(source_ml_temporary_result71) => {
                                                                                source_ml_temporary_result71.clone()
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
                                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop70Result::MlTemporaryReadJavaScriptCallProgramErrorLoop70Return(source_ml_temporary_result74) => {
                                        source_ml_temporary_result74.clone()
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
pub fn replace_java_script_call_frame(heap: self::JavaScriptOrdinaryHeap, frames: Vec<self::JavaScriptCallFrame>, frame: self::JavaScriptCallFrame) -> self::JavaScriptCallMachine {
    self::JavaScriptCallMachine::CallMachine(Box::new(heap.clone()), { let ml_values4 = frames.clone(); let ml_callback4 = { let frames = frames.clone(); let frame = frame.clone(); std::rc::Rc::new(move |entry: self::JavaScriptCallFrame, index: f64| -> self::JavaScriptCallFrame { if (index == (((frames.len() as f64) - 1f64))) {
        frame.clone()
    } else {
        entry.clone()
    } }) }; ml_values4.iter().cloned().enumerate().map(|(ml_index4, ml_value4)| ml_callback4(ml_value4, (ml_index4 as f64))).collect::<Vec<_>>() })
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn write_java_script_call_register(frame: self::JavaScriptCallFrame, destination: f64, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallFrame {
    {
        let source_ml_source1_nominal_spread1202 = frame.clone();
        {
            let source_ml_source1_nominal_field1203 = match source_ml_source1_nominal_spread1202.clone() {
                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1204, _, _, _, _, _) => {
                    source_ml_source1_data_property_value1204.clone()
                }
            };
            {
                let source_ml_source1_nominal_field1205 = match source_ml_source1_nominal_spread1202.clone() {
                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1206, _, _, _, _) => {
                        source_ml_source1_data_property_value1206
                    }
                };
                {
                    let source_ml_source1_nominal_field1207 = match source_ml_source1_nominal_spread1202.clone() {
                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1208, _, _, _) => {
                            source_ml_source1_data_property_value1208
                        }
                    };
                    {
                        let source_ml_source1_nominal_field1209 = match source_ml_source1_nominal_spread1202.clone() {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1210, _, _) => {
                                source_ml_source1_data_property_value1210
                            }
                        };
                        {
                            let source_ml_source1_nominal_field1211 = match source_ml_source1_nominal_spread1202.clone() {
                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1212, _) => {
                                    source_ml_source1_data_property_value1212
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1213 = match source_ml_source1_nominal_spread1202.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1214) => {
                                        let source_ml_source1_data_property_value1214 = *source_ml_source1_data_property_value1214;
                                        source_ml_source1_data_property_value1214.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_override1215 = { let ml_values5 = match frame.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1201, _, _, _) => {
                                            source_ml_source1_data_property_value1201
                                        }
                                    }; let ml_callback5 = { let destination = destination; let value = value.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, index: f64| -> self::JavaScriptSemanticValue { if (index == destination) {
                                        value.clone()
                                    } else {
                                        entry.clone()
                                    } }) }; ml_values5.iter().cloned().enumerate().map(|(ml_index5, ml_value5)| ml_callback5(ml_value5, (ml_index5 as f64))).collect::<Vec<_>>() };
                                    {
                                        let source_ml_source1_nominal_override1216 = self::JavaScriptCallPending::CallIdle;
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1203.clone(), source_ml_source1_nominal_field1205, source_ml_source1_nominal_override1215, source_ml_source1_nominal_field1209, source_ml_source1_nominal_field1211, Box::new(source_ml_source1_nominal_override1216.clone()))
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
pub fn create_java_script_call_frame(fn_: self::JavaScriptCallFunction, receiver: self::JavaScriptSemanticValue, arguments_: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallFrame {
    {
        let registers = Vec::<self::JavaScriptSemanticValue>::new();
        {
            let index = 0f64;
            {
                let registers_2 = self::source_ml_temporary_create_java_script_call_frame_loop75(fn_.clone(), registers, index);
                {
                    let bound = if ((match fn_.clone() {
                        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source1_data_property_value1218, _, _, _, _, _) => {
                            source_ml_source1_data_property_value1218.clone()
                        }
                    }) == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108]))) {
                        match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(_, _, _, source_ml_source1_data_property_value1219, _, _, _, _) => {
                                let source_ml_source1_data_property_value1219 = *source_ml_source1_data_property_value1219;
                                source_ml_source1_data_property_value1219.clone()
                            }
                        }
                    } else {
                        receiver.clone()
                    };
                    {
                        let registers_3 = { let ml_values6 = registers_2; let ml_callback6 = { let bound = bound.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, index_2: f64| -> self::JavaScriptSemanticValue { if (index_2 == (0f64)) {
                            bound.clone()
                        } else {
                            entry.clone()
                        } }) }; ml_values6.iter().cloned().enumerate().map(|(ml_index6, ml_value6)| ml_callback6(ml_value6, (ml_index6 as f64))).collect::<Vec<_>>() };
                        {
                            let index_2 = 0f64;
                            {
                                let registers_4 = self::source_ml_temporary_create_java_script_call_frame_loop76(fn_.clone(), arguments_.clone(), registers_3, index_2);
                                {
                                    let index_3 = 0f64;
                                    {
                                        let registers_5 = self::source_ml_temporary_create_java_script_call_frame_loop77(fn_.clone(), registers_4, index_3);
                                        self::JavaScriptCallFrame::CallFrame(match fn_.clone() {
                                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value1220, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value1220.clone()
                                            }
                                        }, 0f64, registers_5, arguments_, Vec::<self::JavaScriptCallHandler>::new(), Box::new(self::JavaScriptCallPending::CallIdle))
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
pub fn read_java_script_call_machine_error(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine) -> self::ml_string::Text {
    {
        let error = self::read_java_script_call_program_error(program.clone(), match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1221, _) => {
                let source_ml_source1_data_property_value1221 = *source_ml_source1_data_property_value1221;
                source_ml_source1_data_property_value1221.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            if (((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1223) => source_ml_source1_data_property_value1223.as_slice() }).len() as f64)) == (0f64)) || ((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1225) => source_ml_source1_data_property_value1225.as_slice() }).len() as f64)) > (1024f64))) {
                self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 109, 97, 99, 104, 105, 110, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 97, 99, 116, 105, 118, 101, 32, 115, 116, 97, 99, 107])
            } else {
                {
                    let slots = 0f64;
                    {
                        let argument_slots = 0f64;
                        {
                            let index = 0f64;
                            {
                                let source_ml_source1_s1226 = self::source_ml_temporary_read_java_script_call_machine_error_loop81(program.clone(), machine.clone(), slots, argument_slots, index);
                                match source_ml_source1_s1226.clone() {
                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result::MlTemporaryReadJavaScriptCallMachineErrorLoop81Done => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result::MlTemporaryReadJavaScriptCallMachineErrorLoop81Return(source_ml_temporary_result81) => {
                                        source_ml_temporary_result81.clone()
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
pub fn throw_java_script_call_value(machine: self::JavaScriptCallMachine, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    {
        let frames = match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1227) => {
                source_ml_source1_data_property_value1227
            }
        };
        {
            let source_ml_source1_s1228 = self::source_ml_temporary_throw_java_script_call_value_loop87(machine.clone(), value.clone(), frames);
            match source_ml_source1_s1228.clone() {
                self::MlTemporaryThrowJavaScriptCallValueLoop87Result::MlTemporaryThrowJavaScriptCallValueLoop87Done => {
                    self::JavaScriptCallMachineTransition::MachineThrow(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1229, _) => {
                            let source_ml_source1_data_property_value1229 = *source_ml_source1_data_property_value1229;
                            source_ml_source1_data_property_value1229.clone()
                        }
                    }), Box::new(value.clone()))
                }
                self::MlTemporaryThrowJavaScriptCallValueLoop87Result::MlTemporaryThrowJavaScriptCallValueLoop87Return(source_ml_temporary_result87) => {
                    let source_ml_temporary_result87 = *source_ml_temporary_result87;
                    source_ml_temporary_result87.clone()
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn request_java_script_call_error(machine: self::JavaScriptCallMachine, realm: self::ml::Big, error_class: &self::ml_string::Text, reason: &self::ml_string::Text) -> self::JavaScriptCallMachineTransition {
    {
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1230) => source_ml_source1_data_property_value1230.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1231) => source_ml_source1_data_property_value1231.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1232, _) => {
                    let source_ml_source1_data_property_value1232 = *source_ml_source1_data_property_value1232;
                    source_ml_source1_data_property_value1232.clone()
                }
            }), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
        } else {
            {
                let next = self::replace_java_script_call_frame(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1233, _) => {
                        let source_ml_source1_data_property_value1233 = *source_ml_source1_data_property_value1233;
                        source_ml_source1_data_property_value1233.clone()
                    }
                }, match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1234) => {
                        source_ml_source1_data_property_value1234
                    }
                }, {
                    let source_ml_source1_nominal_spread1235 = frame.clone().expect("a proven present value");
                    {
                        let source_ml_source1_nominal_field1236 = match source_ml_source1_nominal_spread1235.clone() {
                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1237, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1237.clone()
                            }
                        };
                        {
                            let source_ml_source1_nominal_field1238 = match source_ml_source1_nominal_spread1235.clone() {
                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1239, _, _, _, _) => {
                                    source_ml_source1_data_property_value1239
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1240 = match source_ml_source1_nominal_spread1235.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1241, _, _, _) => {
                                        source_ml_source1_data_property_value1241
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1242 = match source_ml_source1_nominal_spread1235.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1243, _, _) => {
                                            source_ml_source1_data_property_value1243
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1244 = match source_ml_source1_nominal_spread1235.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1245, _) => {
                                                source_ml_source1_data_property_value1245
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1246 = match source_ml_source1_nominal_spread1235.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1247) => {
                                                    let source_ml_source1_data_property_value1247 = *source_ml_source1_data_property_value1247;
                                                    source_ml_source1_data_property_value1247.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_override1248 = self::JavaScriptCallPending::WaitError(realm.clone(), error_class.clone(), reason.clone());
                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1236.clone(), source_ml_source1_nominal_field1238, source_ml_source1_nominal_field1240, source_ml_source1_nominal_field1242, source_ml_source1_nominal_field1244, Box::new(source_ml_source1_nominal_override1248.clone()))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                });
                self::JavaScriptCallMachineTransition::MachineAllocateError(Box::new(next.clone()), realm.clone(), error_class.clone(), reason.clone())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn push_java_script_call(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, callee: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue, arguments_: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallMachineTransition {
    {
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1249) => source_ml_source1_data_property_value1249.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1250) => source_ml_source1_data_property_value1250.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1251, _) => {
                    let source_ml_source1_data_property_value1251 = *source_ml_source1_data_property_value1251;
                    source_ml_source1_data_property_value1251.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 110, 101, 115, 116, 101, 100, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116]))
        } else {
            {
                let owner = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1252, _, _) => source_ml_source1_data_property_value1252.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1253, _, _, _, _, _) => {
                        source_ml_source1_data_property_value1253.clone()
                    }
                })));
                if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1254, _) => {
                            let source_ml_source1_data_property_value1254 = *source_ml_source1_data_property_value1254;
                            source_ml_source1_data_property_value1254.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                } else {
                    match callee.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1260, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1260.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1261, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1261.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1255) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1262, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1262.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1256) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1263, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1263.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1257) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1264, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1264.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1258) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1265, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1265.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1259) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1266, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1266.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Object(identity, callable) => {
                            if !callable {
                                self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1267, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value1267.clone()
                                    }
                                }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                            } else {
                                {
                                    let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1268, _, _) => source_ml_source1_data_property_value1268.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), identity.clone())));
                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1269, _) => {
                                                let source_ml_source1_data_property_value1269 = *source_ml_source1_data_property_value1269;
                                                source_ml_source1_data_property_value1269.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))
                                    } else {
                                        {
                                            let slots = match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value1270, _, _) => {
                                                    source_ml_source1_data_property_value1270
                                                }
                                            };
                                            {
                                                let argument_slots = (arguments_.len() as f64);
                                                {
                                                    let source_ml_temporary_values88 = match machine.clone() {
                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1271) => {
                                                            source_ml_source1_data_property_value1271
                                                        }
                                                    };
                                                    {
                                                        let source_ml_temporary_index88 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source1_s1272 = self::source_ml_temporary_push_java_script_call_loop89(slots, argument_slots, source_ml_temporary_values88, source_ml_temporary_index88.clone());
                                                            match source_ml_source1_s1272.clone() {
                                                                self::MlTemporaryPushJavaScriptCallLoop89Result::MlTemporaryPushJavaScriptCallLoop89Done(slots_2, argument_slots_2) => {
                                                                    if ((((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1274) => source_ml_source1_data_property_value1274.as_slice() }).len() as f64)) >= (1024f64)) || (slots_2 > (1048576f64))) || (argument_slots_2 > (1048576f64))) {
                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1275, _) => {
                                                                                let source_ml_source1_data_property_value1275 = *source_ml_source1_data_property_value1275;
                                                                                source_ml_source1_data_property_value1275.clone()
                                                                            }
                                                                        }), {
                                                                            let source_ml_source1_concat_value1277 = match machine.clone() {
                                                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1276) => {
                                                                                    source_ml_source1_data_property_value1276
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_concat_value1278 = vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_)];
                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), source_ml_source1_concat_value1277), source_ml_source1_concat_value1278)
                                                                            }
                                                                        })))
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
