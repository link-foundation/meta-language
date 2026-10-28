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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1226) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f1227) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f1228) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f1229) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1230) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f1231, source_ml_source2_f1232) => {
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
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1233, _, _) => {
                let source_ml_source2_data_property_value1233 = *source_ml_source2_data_property_value1233;
                source_ml_source2_data_property_value1233.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1234, _, _) => {
                let source_ml_source2_data_property_value1234 = *source_ml_source2_data_property_value1234;
                source_ml_source2_data_property_value1234.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source2_s1237 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1245, _, _) => {
                        let source_ml_source2_data_property_value1245 = *source_ml_source2_data_property_value1245;
                        source_ml_source2_data_property_value1245.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1246, _, _) => {
                        let source_ml_source2_data_property_value1246 = *source_ml_source2_data_property_value1246;
                        source_ml_source2_data_property_value1246.clone()
                    }
                };
                match source_ml_source2_s1237.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1238) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f1239) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f1240) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f1241) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1242) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f1243, source_ml_source2_f1244) => {
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
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1247, _, _) => {
                let source_ml_source2_data_property_value1247 = *source_ml_source2_data_property_value1247;
                source_ml_source2_data_property_value1247.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1248, _, _) => {
                let source_ml_source2_data_property_value1248 = *source_ml_source2_data_property_value1248;
                source_ml_source2_data_property_value1248.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source2_s1251 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1259, _, _) => {
                        let source_ml_source2_data_property_value1259 = *source_ml_source2_data_property_value1259;
                        source_ml_source2_data_property_value1259.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1260, _, _) => {
                        let source_ml_source2_data_property_value1260 = *source_ml_source2_data_property_value1260;
                        source_ml_source2_data_property_value1260.clone()
                    }
                };
                match source_ml_source2_s1251.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1252) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f1253) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f1254) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f1255) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1256) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f1257, source_ml_source2_f1258) => {
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
            if (((((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1262, _, _) => source_ml_source2_data_property_value1262.as_slice() }).len() as f64)) > (4096f64)) || ((((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source2_data_property_value1264, _) => source_ml_source2_data_property_value1264.as_slice() }).len() as f64)) > (4096f64))) {
                self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let previous = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_temporary_values123 = match program.clone() {
                            self::JavaScriptCallProgram::CallProgram(_, source_ml_source2_data_property_value1265, _) => {
                                source_ml_source2_data_property_value1265
                            }
                        };
                        {
                            let source_ml_source1_temporary_index123 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s1266 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop128(previous.clone(), source_ml_source1_temporary_values123, source_ml_source1_temporary_index123.clone());
                                match source_ml_source2_s1266.clone() {
                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Done(previous_2) => {
                                        if {
                                            let source_ml_source2_s1268 = match program.clone() {
                                                self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source2_data_property_value1276) => {
                                                    let source_ml_source2_data_property_value1276 = *source_ml_source2_data_property_value1276;
                                                    source_ml_source2_data_property_value1276.clone()
                                                }
                                            };
                                            match source_ml_source2_s1268.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1269) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source2_f1270) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f1271) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source2_f1272) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1273) => {
                                                    false
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source2_f1274, source_ml_source2_f1275) => {
                                                    true
                                                }
                                            }
                                        } {
                                            self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 117, 116, 104, 111, 114, 105, 116, 97, 116, 105, 118, 101, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 83, 121, 109, 98, 111, 108])
                                        } else {
                                            {
                                                let key_error = self::read_java_script_ordinary_value_error(heap.clone(), match program.clone() {
                                                    self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source2_data_property_value1277) => {
                                                        let source_ml_source2_data_property_value1277 = *source_ml_source2_data_property_value1277;
                                                        source_ml_source2_data_property_value1277.clone()
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
                                                                    self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1278, _, _) => {
                                                                        source_ml_source2_data_property_value1278
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_temporary_index124 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let source_ml_source2_s1279 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop129(program.clone(), heap.clone(), previous_3.clone(), count, source_ml_source1_temporary_values124, source_ml_source1_temporary_index124.clone());
                                                                        match source_ml_source2_s1279.clone() {
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
    self::JavaScriptCallMachine::CallMachine(Box::new(heap.clone()), { let ml_values3 = frames.clone(); let ml_callback3 = { let frames = frames.clone(); let frame = frame.clone(); std::rc::Rc::new(move |entry: self::JavaScriptCallFrame, index: f64| -> self::JavaScriptCallFrame { if (index == (((frames.len() as f64) - 1f64))) {
        frame.clone()
    } else {
        entry.clone()
    } }) }; ml_values3.iter().cloned().enumerate().map(|(ml_index3, ml_value3)| ml_callback3(ml_value3, (ml_index3 as f64))).collect::<Vec<_>>() })
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn write_java_script_call_register(frame: self::JavaScriptCallFrame, destination: f64, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallFrame {
    {
        let source_ml_source2_nominal_spread1281 = frame.clone();
        {
            let source_ml_source2_nominal_field1282 = match source_ml_source2_nominal_spread1281.clone() {
                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1283, _, _, _, _, _) => {
                    source_ml_source2_data_property_value1283.clone()
                }
            };
            {
                let source_ml_source2_nominal_field1284 = match source_ml_source2_nominal_spread1281.clone() {
                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1285, _, _, _, _) => {
                        source_ml_source2_data_property_value1285
                    }
                };
                {
                    let source_ml_source2_nominal_field1286 = match source_ml_source2_nominal_spread1281.clone() {
                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1287, _, _, _) => {
                            source_ml_source2_data_property_value1287
                        }
                    };
                    {
                        let source_ml_source2_nominal_field1288 = match source_ml_source2_nominal_spread1281.clone() {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1289, _, _) => {
                                source_ml_source2_data_property_value1289
                            }
                        };
                        {
                            let source_ml_source2_nominal_field1290 = match source_ml_source2_nominal_spread1281.clone() {
                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1291, _) => {
                                    source_ml_source2_data_property_value1291
                                }
                            };
                            {
                                let source_ml_source2_nominal_field1292 = match source_ml_source2_nominal_spread1281.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1293) => {
                                        let source_ml_source2_data_property_value1293 = *source_ml_source2_data_property_value1293;
                                        source_ml_source2_data_property_value1293.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_override1294 = { let ml_values4 = match frame.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1280, _, _, _) => {
                                            source_ml_source2_data_property_value1280
                                        }
                                    }; let ml_callback4 = { let destination = destination; let value = value.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, index: f64| -> self::JavaScriptSemanticValue { if (index == destination) {
                                        value.clone()
                                    } else {
                                        entry.clone()
                                    } }) }; ml_values4.iter().cloned().enumerate().map(|(ml_index4, ml_value4)| ml_callback4(ml_value4, (ml_index4 as f64))).collect::<Vec<_>>() };
                                    {
                                        let source_ml_source2_nominal_override1295 = self::JavaScriptCallPending::CallIdle;
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1282.clone(), source_ml_source2_nominal_field1284, source_ml_source2_nominal_override1294, source_ml_source2_nominal_field1288, source_ml_source2_nominal_field1290, Box::new(source_ml_source2_nominal_override1295.clone()))
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
                        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source2_data_property_value1297, _, _, _, _, _) => {
                            source_ml_source2_data_property_value1297.clone()
                        }
                    }) == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108]))) {
                        match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(_, _, _, source_ml_source2_data_property_value1298, _, _, _, _) => {
                                let source_ml_source2_data_property_value1298 = *source_ml_source2_data_property_value1298;
                                source_ml_source2_data_property_value1298.clone()
                            }
                        }
                    } else {
                        receiver.clone()
                    };
                    {
                        let registers_3 = { let ml_values5 = registers_2; let ml_callback5 = { let bound = bound.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, index_2: f64| -> self::JavaScriptSemanticValue { if (index_2 == (0f64)) {
                            bound.clone()
                        } else {
                            entry.clone()
                        } }) }; ml_values5.iter().cloned().enumerate().map(|(ml_index5, ml_value5)| ml_callback5(ml_value5, (ml_index5 as f64))).collect::<Vec<_>>() };
                        {
                            let index_2 = 0f64;
                            {
                                let registers_4 = self::source_ml_source1_temporary_create_java_script_call_frame_loop134(fn_.clone(), arguments_.clone(), registers_3, index_2);
                                {
                                    let index_3 = 0f64;
                                    {
                                        let registers_5 = self::source_ml_source1_temporary_create_java_script_call_frame_loop135(fn_.clone(), registers_4, index_3);
                                        self::JavaScriptCallFrame::CallFrame(match fn_.clone() {
                                            self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value1299, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value1299.clone()
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
            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1300, _) => {
                let source_ml_source2_data_property_value1300 = *source_ml_source2_data_property_value1300;
                source_ml_source2_data_property_value1300.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            if (((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1302) => source_ml_source2_data_property_value1302.as_slice() }).len() as f64)) == (0f64)) || ((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1304) => source_ml_source2_data_property_value1304.as_slice() }).len() as f64)) > (1024f64))) {
                self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 109, 97, 99, 104, 105, 110, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 97, 99, 116, 105, 118, 101, 32, 115, 116, 97, 99, 107])
            } else {
                {
                    let slots = 0f64;
                    {
                        let argument_slots = 0f64;
                        {
                            let index = 0f64;
                            {
                                let source_ml_source2_s1305 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop139(program.clone(), machine.clone(), slots, argument_slots, index);
                                match source_ml_source2_s1305.clone() {
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
            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1306) => {
                source_ml_source2_data_property_value1306
            }
        };
        {
            let source_ml_source2_s1307 = self::source_ml_source1_temporary_throw_java_script_call_value_loop145(machine.clone(), value.clone(), frames);
            match source_ml_source2_s1307.clone() {
                self::MlSource1TemporaryThrowJavaScriptCallValueLoop145Result::MlSource1TemporaryThrowJavaScriptCallValueLoop145Done => {
                    self::JavaScriptCallMachineTransition::MachineThrow(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1308, _) => {
                            let source_ml_source2_data_property_value1308 = *source_ml_source2_data_property_value1308;
                            source_ml_source2_data_property_value1308.clone()
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
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1309) => source_ml_source2_data_property_value1309.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1310) => source_ml_source2_data_property_value1310.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1311, _) => {
                    let source_ml_source2_data_property_value1311 = *source_ml_source2_data_property_value1311;
                    source_ml_source2_data_property_value1311.clone()
                }
            }), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
        } else {
            {
                let next = self::replace_java_script_call_frame(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1312, _) => {
                        let source_ml_source2_data_property_value1312 = *source_ml_source2_data_property_value1312;
                        source_ml_source2_data_property_value1312.clone()
                    }
                }, match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1313) => {
                        source_ml_source2_data_property_value1313
                    }
                }, {
                    let source_ml_source2_nominal_spread1314 = frame.clone().expect("a proven present value");
                    {
                        let source_ml_source2_nominal_field1315 = match source_ml_source2_nominal_spread1314.clone() {
                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1316, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1316.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field1317 = match source_ml_source2_nominal_spread1314.clone() {
                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1318, _, _, _, _) => {
                                    source_ml_source2_data_property_value1318
                                }
                            };
                            {
                                let source_ml_source2_nominal_field1319 = match source_ml_source2_nominal_spread1314.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1320, _, _, _) => {
                                        source_ml_source2_data_property_value1320
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1321 = match source_ml_source2_nominal_spread1314.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1322, _, _) => {
                                            source_ml_source2_data_property_value1322
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1323 = match source_ml_source2_nominal_spread1314.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1324, _) => {
                                                source_ml_source2_data_property_value1324
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1325 = match source_ml_source2_nominal_spread1314.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1326) => {
                                                    let source_ml_source2_data_property_value1326 = *source_ml_source2_data_property_value1326;
                                                    source_ml_source2_data_property_value1326.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override1327 = self::JavaScriptCallPending::WaitError(realm.clone(), error_class.clone(), reason.clone());
                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1315.clone(), source_ml_source2_nominal_field1317, source_ml_source2_nominal_field1319, source_ml_source2_nominal_field1321, source_ml_source2_nominal_field1323, Box::new(source_ml_source2_nominal_override1327.clone()))
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
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1328) => source_ml_source2_data_property_value1328.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1329) => source_ml_source2_data_property_value1329.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1330, _) => {
                    let source_ml_source2_data_property_value1330 = *source_ml_source2_data_property_value1330;
                    source_ml_source2_data_property_value1330.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 110, 101, 115, 116, 101, 100, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116]))
        } else {
            {
                let owner = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1331, _, _) => source_ml_source2_data_property_value1331.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1332, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1332.clone()
                    }
                })));
                if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1333, _) => {
                            let source_ml_source2_data_property_value1333 = *source_ml_source2_data_property_value1333;
                            source_ml_source2_data_property_value1333.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                } else {
                    match callee.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1339, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1339.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1340, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1340.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1334) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1341, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1341.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f1335) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1342, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1342.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f1336) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1343, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1343.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f1337) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1344, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1344.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1338) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1345, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1345.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Object(identity, callable) => {
                            if !callable {
                                self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1346, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1346.clone()
                                    }
                                }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                            } else {
                                {
                                    let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1347, _, _) => source_ml_source2_data_property_value1347.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), identity.clone())));
                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1348, _) => {
                                                let source_ml_source2_data_property_value1348 = *source_ml_source2_data_property_value1348;
                                                source_ml_source2_data_property_value1348.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))
                                    } else {
                                        {
                                            let slots = match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value1349, _, _) => {
                                                    source_ml_source2_data_property_value1349
                                                }
                                            };
                                            {
                                                let argument_slots = (arguments_.len() as f64);
                                                {
                                                    let source_ml_source1_temporary_values146 = match machine.clone() {
                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1350) => {
                                                            source_ml_source2_data_property_value1350
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_temporary_index146 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source2_s1351 = self::source_ml_source1_temporary_push_java_script_call_loop147(slots, argument_slots, source_ml_source1_temporary_values146, source_ml_source1_temporary_index146.clone());
                                                            match source_ml_source2_s1351.clone() {
                                                                self::MlSource1TemporaryPushJavaScriptCallLoop147Result::MlSource1TemporaryPushJavaScriptCallLoop147Done(slots_2, argument_slots_2) => {
                                                                    if ((((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1353) => source_ml_source2_data_property_value1353.as_slice() }).len() as f64)) >= (1024f64)) || (slots_2 > (1048576f64))) || (argument_slots_2 > (1048576f64))) {
                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1354, _) => {
                                                                                let source_ml_source2_data_property_value1354 = *source_ml_source2_data_property_value1354;
                                                                                source_ml_source2_data_property_value1354.clone()
                                                                            }
                                                                        }), {
                                                                            let source_ml_source2_concat_value1356 = match machine.clone() {
                                                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1355) => {
                                                                                    source_ml_source2_data_property_value1355
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_concat_value1357 = vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_)];
                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), source_ml_source2_concat_value1356), source_ml_source2_concat_value1357)
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
