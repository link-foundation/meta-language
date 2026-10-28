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
use super::MlSource1TemporaryPushJavaScriptCallLoop147Result;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Result;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result;
use super::MlSource1TemporaryThrowJavaScriptCallValueLoop145Result;
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
use super::source_ml_source1_temporary_create_java_script_call_frame_loop133;
use super::source_ml_source1_temporary_create_java_script_call_frame_loop134;
use super::source_ml_source1_temporary_create_java_script_call_frame_loop135;
use super::source_ml_source1_temporary_push_java_script_call_loop147;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop139;
use super::source_ml_source1_temporary_read_java_script_call_program_error_loop128;
use super::source_ml_source1_temporary_read_java_script_call_program_error_loop129;
use super::source_ml_source1_temporary_throw_java_script_call_value_loop145;
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1222) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f1223) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f1224) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f1225) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1226) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f1227, source_ml_source2_f1228) => {
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
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1229, _, _) => {
                let source_ml_source2_data_property_value1229 = *source_ml_source2_data_property_value1229;
                source_ml_source2_data_property_value1229.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1230, _, _) => {
                let source_ml_source2_data_property_value1230 = *source_ml_source2_data_property_value1230;
                source_ml_source2_data_property_value1230.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source2_s1233 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1241, _, _) => {
                        let source_ml_source2_data_property_value1241 = *source_ml_source2_data_property_value1241;
                        source_ml_source2_data_property_value1241.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1242, _, _) => {
                        let source_ml_source2_data_property_value1242 = *source_ml_source2_data_property_value1242;
                        source_ml_source2_data_property_value1242.clone()
                    }
                };
                match source_ml_source2_s1233.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1234) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f1235) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f1236) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f1237) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1238) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f1239, source_ml_source2_f1240) => {
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
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1243, _, _) => {
                let source_ml_source2_data_property_value1243 = *source_ml_source2_data_property_value1243;
                source_ml_source2_data_property_value1243.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1244, _, _) => {
                let source_ml_source2_data_property_value1244 = *source_ml_source2_data_property_value1244;
                source_ml_source2_data_property_value1244.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source2_s1247 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1255, _, _) => {
                        let source_ml_source2_data_property_value1255 = *source_ml_source2_data_property_value1255;
                        source_ml_source2_data_property_value1255.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1256, _, _) => {
                        let source_ml_source2_data_property_value1256 = *source_ml_source2_data_property_value1256;
                        source_ml_source2_data_property_value1256.clone()
                    }
                };
                match source_ml_source2_s1247.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1248) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f1249) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f1250) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f1251) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1252) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f1253, source_ml_source2_f1254) => {
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
            if (((((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1258, _, _) => source_ml_source2_data_property_value1258.as_slice() }).len() as f64)) > (4096f64)) || ((((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source2_data_property_value1260, _) => source_ml_source2_data_property_value1260.as_slice() }).len() as f64)) > (4096f64))) {
                self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let previous = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_temporary_values123 = match program.clone() {
                            self::JavaScriptCallProgram::CallProgram(_, source_ml_source2_data_property_value1261, _) => {
                                source_ml_source2_data_property_value1261
                            }
                        };
                        {
                            let source_ml_source1_temporary_index123 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s1262 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop128(previous.clone(), source_ml_source1_temporary_values123, source_ml_source1_temporary_index123.clone());
                                match source_ml_source2_s1262.clone() {
                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Done(previous_2) => {
                                        if {
                                            let source_ml_source2_s1264 = match program.clone() {
                                                self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source2_data_property_value1272) => {
                                                    let source_ml_source2_data_property_value1272 = *source_ml_source2_data_property_value1272;
                                                    source_ml_source2_data_property_value1272.clone()
                                                }
                                            };
                                            match source_ml_source2_s1264.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1265) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source2_f1266) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f1267) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source2_f1268) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1269) => {
                                                    false
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source2_f1270, source_ml_source2_f1271) => {
                                                    true
                                                }
                                            }
                                        } {
                                            self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 117, 116, 104, 111, 114, 105, 116, 97, 116, 105, 118, 101, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 83, 121, 109, 98, 111, 108])
                                        } else {
                                            {
                                                let key_error = self::read_java_script_ordinary_value_error(heap.clone(), match program.clone() {
                                                    self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source2_data_property_value1273) => {
                                                        let source_ml_source2_data_property_value1273 = *source_ml_source2_data_property_value1273;
                                                        source_ml_source2_data_property_value1273.clone()
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
                                                                let source_ml_source1_temporary_values124 = match program.clone() {
                                                                    self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1274, _, _) => {
                                                                        source_ml_source2_data_property_value1274
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_temporary_index124 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let source_ml_source2_s1275 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop129(program.clone(), heap.clone(), previous_3.clone(), count, source_ml_source1_temporary_values124, source_ml_source1_temporary_index124.clone());
                                                                        match source_ml_source2_s1275.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Done => {
                                                                                self::ml_string::Text::from_units(vec![])
                                                                            }
                                                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(source_ml_source1_temporary_result129) => {
                                                                                source_ml_source1_temporary_result129.clone()
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
                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Return(source_ml_source1_temporary_result132) => {
                                        source_ml_source1_temporary_result132.clone()
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
        let source_ml_source2_nominal_spread1277 = frame.clone();
        {
            let source_ml_source2_nominal_field1278 = match source_ml_source2_nominal_spread1277.clone() {
                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1279, _, _, _, _, _) => {
                    source_ml_source2_data_property_value1279.clone()
                }
            };
            {
                let source_ml_source2_nominal_field1280 = match source_ml_source2_nominal_spread1277.clone() {
                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1281, _, _, _, _) => {
                        source_ml_source2_data_property_value1281
                    }
                };
                {
                    let source_ml_source2_nominal_field1282 = match source_ml_source2_nominal_spread1277.clone() {
                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1283, _, _, _) => {
                            source_ml_source2_data_property_value1283
                        }
                    };
                    {
                        let source_ml_source2_nominal_field1284 = match source_ml_source2_nominal_spread1277.clone() {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1285, _, _) => {
                                source_ml_source2_data_property_value1285
                            }
                        };
                        {
                            let source_ml_source2_nominal_field1286 = match source_ml_source2_nominal_spread1277.clone() {
                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1287, _) => {
                                    source_ml_source2_data_property_value1287
                                }
                            };
                            {
                                let source_ml_source2_nominal_field1288 = match source_ml_source2_nominal_spread1277.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1289) => {
                                        let source_ml_source2_data_property_value1289 = *source_ml_source2_data_property_value1289;
                                        source_ml_source2_data_property_value1289.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_override1290 = { let ml_values5 = match frame.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1276, _, _, _) => {
                                            source_ml_source2_data_property_value1276
                                        }
                                    }; let ml_callback5 = { let destination = destination; let value = value.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, index: f64| -> self::JavaScriptSemanticValue { if (index == destination) {
                                        value.clone()
                                    } else {
                                        entry.clone()
                                    } }) }; ml_values5.iter().cloned().enumerate().map(|(ml_index5, ml_value5)| ml_callback5(ml_value5, (ml_index5 as f64))).collect::<Vec<_>>() };
                                    {
                                        let source_ml_source2_nominal_override1291 = self::JavaScriptCallPending::CallIdle;
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1278.clone(), source_ml_source2_nominal_field1280, source_ml_source2_nominal_override1290, source_ml_source2_nominal_field1284, source_ml_source2_nominal_field1286, Box::new(source_ml_source2_nominal_override1291.clone()))
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
                let registers_2 = self::source_ml_source1_temporary_create_java_script_call_frame_loop133(fn_.clone(), registers, index);
                {
                    let bound = if ((match fn_.clone() {
                        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source2_data_property_value1293, _, _, _, _, _) => {
                            source_ml_source2_data_property_value1293.clone()
                        }
                    }) == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108]))) {
                        match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(_, _, _, source_ml_source2_data_property_value1294, _, _, _, _) => {
                                let source_ml_source2_data_property_value1294 = *source_ml_source2_data_property_value1294;
                                source_ml_source2_data_property_value1294.clone()
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
                                let registers_4 = self::source_ml_source1_temporary_create_java_script_call_frame_loop134(fn_.clone(), arguments_.clone(), registers_3, index_2);
                                {
                                    let index_3 = 0f64;
                                    {
                                        let registers_5 = self::source_ml_source1_temporary_create_java_script_call_frame_loop135(fn_.clone(), registers_4, index_3);
                                        self::JavaScriptCallFrame::CallFrame(match fn_.clone() {
                                            self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value1295, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value1295.clone()
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
            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1296, _) => {
                let source_ml_source2_data_property_value1296 = *source_ml_source2_data_property_value1296;
                source_ml_source2_data_property_value1296.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            if (((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1298) => source_ml_source2_data_property_value1298.as_slice() }).len() as f64)) == (0f64)) || ((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1300) => source_ml_source2_data_property_value1300.as_slice() }).len() as f64)) > (1024f64))) {
                self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 109, 97, 99, 104, 105, 110, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 97, 99, 116, 105, 118, 101, 32, 115, 116, 97, 99, 107])
            } else {
                {
                    let slots = 0f64;
                    {
                        let argument_slots = 0f64;
                        {
                            let index = 0f64;
                            {
                                let source_ml_source2_s1301 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop139(program.clone(), machine.clone(), slots, argument_slots, index);
                                match source_ml_source2_s1301.clone() {
                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Done => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Return(source_ml_source1_temporary_result139) => {
                                        source_ml_source1_temporary_result139.clone()
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
            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1302) => {
                source_ml_source2_data_property_value1302
            }
        };
        {
            let source_ml_source2_s1303 = self::source_ml_source1_temporary_throw_java_script_call_value_loop145(machine.clone(), value.clone(), frames);
            match source_ml_source2_s1303.clone() {
                self::MlSource1TemporaryThrowJavaScriptCallValueLoop145Result::MlSource1TemporaryThrowJavaScriptCallValueLoop145Done => {
                    self::JavaScriptCallMachineTransition::MachineThrow(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1304, _) => {
                            let source_ml_source2_data_property_value1304 = *source_ml_source2_data_property_value1304;
                            source_ml_source2_data_property_value1304.clone()
                        }
                    }), Box::new(value.clone()))
                }
                self::MlSource1TemporaryThrowJavaScriptCallValueLoop145Result::MlSource1TemporaryThrowJavaScriptCallValueLoop145Return(source_ml_source1_temporary_result145) => {
                    let source_ml_source1_temporary_result145 = *source_ml_source1_temporary_result145;
                    source_ml_source1_temporary_result145.clone()
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn request_java_script_call_error(machine: self::JavaScriptCallMachine, realm: self::ml::Big, error_class: &self::ml_string::Text, reason: &self::ml_string::Text) -> self::JavaScriptCallMachineTransition {
    {
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1305) => source_ml_source2_data_property_value1305.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1306) => source_ml_source2_data_property_value1306.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1307, _) => {
                    let source_ml_source2_data_property_value1307 = *source_ml_source2_data_property_value1307;
                    source_ml_source2_data_property_value1307.clone()
                }
            }), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
        } else {
            {
                let next = self::replace_java_script_call_frame(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1308, _) => {
                        let source_ml_source2_data_property_value1308 = *source_ml_source2_data_property_value1308;
                        source_ml_source2_data_property_value1308.clone()
                    }
                }, match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1309) => {
                        source_ml_source2_data_property_value1309
                    }
                }, {
                    let source_ml_source2_nominal_spread1310 = frame.clone().expect("a proven present value");
                    {
                        let source_ml_source2_nominal_field1311 = match source_ml_source2_nominal_spread1310.clone() {
                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1312, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1312.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field1313 = match source_ml_source2_nominal_spread1310.clone() {
                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1314, _, _, _, _) => {
                                    source_ml_source2_data_property_value1314
                                }
                            };
                            {
                                let source_ml_source2_nominal_field1315 = match source_ml_source2_nominal_spread1310.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1316, _, _, _) => {
                                        source_ml_source2_data_property_value1316
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1317 = match source_ml_source2_nominal_spread1310.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1318, _, _) => {
                                            source_ml_source2_data_property_value1318
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1319 = match source_ml_source2_nominal_spread1310.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1320, _) => {
                                                source_ml_source2_data_property_value1320
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1321 = match source_ml_source2_nominal_spread1310.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1322) => {
                                                    let source_ml_source2_data_property_value1322 = *source_ml_source2_data_property_value1322;
                                                    source_ml_source2_data_property_value1322.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override1323 = self::JavaScriptCallPending::WaitError(realm.clone(), error_class.clone(), reason.clone());
                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1311.clone(), source_ml_source2_nominal_field1313, source_ml_source2_nominal_field1315, source_ml_source2_nominal_field1317, source_ml_source2_nominal_field1319, Box::new(source_ml_source2_nominal_override1323.clone()))
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
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1324) => source_ml_source2_data_property_value1324.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1325) => source_ml_source2_data_property_value1325.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1326, _) => {
                    let source_ml_source2_data_property_value1326 = *source_ml_source2_data_property_value1326;
                    source_ml_source2_data_property_value1326.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 110, 101, 115, 116, 101, 100, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116]))
        } else {
            {
                let owner = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1327, _, _) => source_ml_source2_data_property_value1327.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1328, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1328.clone()
                    }
                })));
                if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1329, _) => {
                            let source_ml_source2_data_property_value1329 = *source_ml_source2_data_property_value1329;
                            source_ml_source2_data_property_value1329.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                } else {
                    match callee.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1335, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1335.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1336, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1336.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1330) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1337, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1337.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f1331) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1338, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1338.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f1332) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1339, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1339.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f1333) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1340, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1340.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1334) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1341, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1341.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Object(identity, callable) => {
                            if !callable {
                                self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1342, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1342.clone()
                                    }
                                }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                            } else {
                                {
                                    let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1343, _, _) => source_ml_source2_data_property_value1343.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), identity.clone())));
                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1344, _) => {
                                                let source_ml_source2_data_property_value1344 = *source_ml_source2_data_property_value1344;
                                                source_ml_source2_data_property_value1344.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))
                                    } else {
                                        {
                                            let slots = match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value1345, _, _) => {
                                                    source_ml_source2_data_property_value1345
                                                }
                                            };
                                            {
                                                let argument_slots = (arguments_.len() as f64);
                                                {
                                                    let source_ml_source1_temporary_values146 = match machine.clone() {
                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1346) => {
                                                            source_ml_source2_data_property_value1346
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_temporary_index146 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source2_s1347 = self::source_ml_source1_temporary_push_java_script_call_loop147(slots, argument_slots, source_ml_source1_temporary_values146, source_ml_source1_temporary_index146.clone());
                                                            match source_ml_source2_s1347.clone() {
                                                                self::MlSource1TemporaryPushJavaScriptCallLoop147Result::MlSource1TemporaryPushJavaScriptCallLoop147Done(slots_2, argument_slots_2) => {
                                                                    if ((((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1349) => source_ml_source2_data_property_value1349.as_slice() }).len() as f64)) >= (1024f64)) || (slots_2 > (1048576f64))) || (argument_slots_2 > (1048576f64))) {
                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1350, _) => {
                                                                                let source_ml_source2_data_property_value1350 = *source_ml_source2_data_property_value1350;
                                                                                source_ml_source2_data_property_value1350.clone()
                                                                            }
                                                                        }), {
                                                                            let source_ml_source2_concat_value1352 = match machine.clone() {
                                                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1351) => {
                                                                                    source_ml_source2_data_property_value1351
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_concat_value1353 = vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_)];
                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), source_ml_source2_concat_value1352), source_ml_source2_concat_value1353)
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
