// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptNumberCallState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::JavaScriptTextCallState;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result;
use super::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin138;
use super::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin139;
use super::MlSource1TemporaryThrowJavaScriptCallValueLoop151Result;
use super::begin_java_script_descriptor_conversion;
use super::begin_java_script_property_key_conversion;
use super::continue_java_script_property_descriptor_definition;
use super::find_java_script_call_code_function;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::project_java_script_property_key_completion;
use super::read_java_script_call_code_registers;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_state_error;
use super::read_java_script_text_protocol_operation_error;
use super::resume_java_script_descriptor_conversion;
use super::resume_java_script_property_key_conversion;
use super::source_ml_source1_temporary_create_java_script_call_frame_loop135;
use super::source_ml_source1_temporary_create_java_script_call_frame_loop136;
use super::source_ml_source1_temporary_create_java_script_call_frame_loop137;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop143;
use super::source_ml_source1_temporary_read_java_script_call_program_error_loop130;
use super::source_ml_source1_temporary_read_java_script_call_program_error_loop131;
use super::source_ml_source1_temporary_throw_java_script_call_value_loop151;
use super::step_java_script_descriptor_conversion;
use super::step_java_script_property_key_conversion;
use super::test_java_script_call_code_coordinate;
use super::test_java_script_semantic_same_value;

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
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
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
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1250, _, _) => {
                let source_ml_source2_data_property_value1250 = *source_ml_source2_data_property_value1250;
                source_ml_source2_data_property_value1250.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1251, _, _) => {
                let source_ml_source2_data_property_value1251 = *source_ml_source2_data_property_value1251;
                source_ml_source2_data_property_value1251.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source2_s1254 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1262, _, _) => {
                        let source_ml_source2_data_property_value1262 = *source_ml_source2_data_property_value1262;
                        source_ml_source2_data_property_value1262.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1263, _, _) => {
                        let source_ml_source2_data_property_value1263 = *source_ml_source2_data_property_value1263;
                        source_ml_source2_data_property_value1263.clone()
                    }
                };
                match source_ml_source2_s1254.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        false
                    }
                    _ => {
                        true
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
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1264, _, _) => {
                let source_ml_source2_data_property_value1264 = *source_ml_source2_data_property_value1264;
                source_ml_source2_data_property_value1264.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1265, _, _) => {
                let source_ml_source2_data_property_value1265 = *source_ml_source2_data_property_value1265;
                source_ml_source2_data_property_value1265.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source2_s1268 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1276, _, _) => {
                        let source_ml_source2_data_property_value1276 = *source_ml_source2_data_property_value1276;
                        source_ml_source2_data_property_value1276.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1277, _, _) => {
                        let source_ml_source2_data_property_value1277 = *source_ml_source2_data_property_value1277;
                        source_ml_source2_data_property_value1277.clone()
                    }
                };
                match source_ml_source2_s1268.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        false
                    }
                    _ => {
                        true
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
pub fn read_java_script_number_protocol_operation_error(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if (((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
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
    match program.clone() {
        self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            {
                let error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    if ((((shared_constructor_field1.len() as f64)) > (4096f64)) || (((shared_constructor_field2.len() as f64)) > (4096f64))) {
                        self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
                    } else {
                        {
                            let previous = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source1_temporary_values125 = shared_constructor_field2;
                                {
                                    let source_ml_source1_temporary_index125 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s1283 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop130(previous.clone(), source_ml_source1_temporary_values125, source_ml_source1_temporary_index125.clone());
                                        match source_ml_source2_s1283.clone() {
                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Done(previous_2) => {
                                                if {
                                                    let source_ml_source2_s1285 = shared_constructor_field3.clone();
                                                    match source_ml_source2_s1285.clone() {
                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }
                                                } {
                                                    self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 117, 116, 104, 111, 114, 105, 116, 97, 116, 105, 118, 101, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 83, 121, 109, 98, 111, 108])
                                                } else {
                                                    {
                                                        let key_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field3.clone());
                                                        if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                                            key_error.clone()
                                                        } else {
                                                            {
                                                                let previous_3 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let count = 0f64;
                                                                    {
                                                                        let source_ml_source1_temporary_values126 = shared_constructor_field1;
                                                                        {
                                                                            let source_ml_source1_temporary_index126 = self::ml::Big::from_i128(0);
                                                                            {
                                                                                let source_ml_source2_s1296 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop131(program.clone(), heap.clone(), previous_3.clone(), count, source_ml_source1_temporary_values126, source_ml_source1_temporary_index126.clone());
                                                                                match source_ml_source2_s1296.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Done => {
                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(source_ml_source1_temporary_result131) => {
                                                                                        source_ml_source1_temporary_result131.clone()
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
                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Return(source_ml_source1_temporary_result134) => {
                                                source_ml_source1_temporary_result134.clone()
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
        let source_ml_source2_nominal_spread1298 = frame.clone();
        {
            let source_ml_source2_nominal_field1299 = match source_ml_source2_nominal_spread1298.clone() {
                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1300, _, _, _, _, _) => {
                    source_ml_source2_data_property_value1300.clone()
                }
            };
            {
                let source_ml_source2_nominal_field1301 = match source_ml_source2_nominal_spread1298.clone() {
                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1302, _, _, _, _) => {
                        source_ml_source2_data_property_value1302
                    }
                };
                {
                    let source_ml_source2_nominal_field1303 = match source_ml_source2_nominal_spread1298.clone() {
                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1304, _, _, _) => {
                            source_ml_source2_data_property_value1304
                        }
                    };
                    {
                        let source_ml_source2_nominal_field1305 = match source_ml_source2_nominal_spread1298.clone() {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1306, _, _) => {
                                source_ml_source2_data_property_value1306
                            }
                        };
                        {
                            let source_ml_source2_nominal_field1307 = match source_ml_source2_nominal_spread1298.clone() {
                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1308, _) => {
                                    source_ml_source2_data_property_value1308
                                }
                            };
                            {
                                let source_ml_source2_nominal_field1309 = match source_ml_source2_nominal_spread1298.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1310) => {
                                        let source_ml_source2_data_property_value1310 = *source_ml_source2_data_property_value1310;
                                        source_ml_source2_data_property_value1310.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_override1311 = { let ml_values4 = match frame.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1297, _, _, _) => {
                                            source_ml_source2_data_property_value1297
                                        }
                                    }; let ml_callback4 = { let destination = destination; let value = value.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, index: f64| -> self::JavaScriptSemanticValue { if (index == destination) {
                                        value.clone()
                                    } else {
                                        entry.clone()
                                    } }) }; ml_values4.iter().cloned().enumerate().map(|(ml_index4, ml_value4)| ml_callback4(ml_value4, (ml_index4 as f64))).collect::<Vec<_>>() };
                                    {
                                        let source_ml_source2_nominal_override1312 = self::JavaScriptCallPending::CallIdle;
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1299.clone(), source_ml_source2_nominal_field1301, source_ml_source2_nominal_override1311, source_ml_source2_nominal_field1305, source_ml_source2_nominal_field1307, Box::new(source_ml_source2_nominal_override1312.clone()))
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
    match fn_.clone() {
        self::JavaScriptCallFunction::CallFunction(shared_constructor_field1, _, shared_constructor_field2, shared_constructor_field3, _, _, _, _) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            {
                let registers = Vec::<self::JavaScriptSemanticValue>::new();
                {
                    let index = 0f64;
                    {
                        let registers_2 = self::source_ml_source1_temporary_create_java_script_call_frame_loop135(fn_.clone(), registers, index);
                        {
                            let bound = if (shared_constructor_field2 == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108]))) {
                                shared_constructor_field3.clone()
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
                                        let registers_4 = self::source_ml_source1_temporary_create_java_script_call_frame_loop136(fn_.clone(), arguments_.clone(), registers_3, index_2);
                                        {
                                            let index_3 = 0f64;
                                            {
                                                let registers_5 = self::source_ml_source1_temporary_create_java_script_call_frame_loop137(fn_.clone(), registers_4, index_3);
                                                self::JavaScriptCallFrame::CallFrame(shared_constructor_field1.clone(), 0f64, registers_5, arguments_, Vec::<self::JavaScriptCallHandler>::new(), Box::new(self::JavaScriptCallPending::CallIdle))
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
pub fn read_java_script_number_call_source_operation(fn_: self::JavaScriptCallFunction, frame: self::JavaScriptCallFrame, destination: f64, realm: self::ml::Big, exotic_key: self::JavaScriptSemanticValue, state: self::JavaScriptNumberCallState) -> self::ml_string::Text {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(_, shared_constructor_field3, shared_constructor_field4, _, _, _) => {
            match fn_.clone() {
                self::JavaScriptCallFunction::CallFunction(_, shared_constructor_field1, _, _, _, _, _, shared_constructor_field2) => {
                    if ((shared_constructor_field3 == (0f64)) || (shared_constructor_field1 != realm)) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        {
                            let instruction = self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index((shared_constructor_field3 - 1f64)));
                            if match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                match instruction.clone().expect("a proven present value") {
                                    self::JavaScriptCallInstruction::Primitive(instruction_destination, operation, arguments) => {
                                        if ((instruction_destination != destination) || (((arguments.len() as f64)) != (1f64))) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            if ((self::read_java_script_number_protocol_operation_error(&(operation))) != (self::ml_string::Text::from_units(vec![]))) {
                                                self::ml_string::Text::from_units(vec![])
                                            } else {
                                                {
                                                    let input = (self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index((self::ml_array::optional_at(&arguments, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| (-1f64))))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                    match input.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            {
                                                                let source_ml_source2_s1380 = match state.clone() {
                                                                    self::JavaScriptNumberCallState::NumberProtocolState(state_state) => {
                                                                        let state_state = *state_state;
                                                                        if (!(self::test_java_script_semantic_same_value(match state_state.clone() {
                                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value1382, _, _, _) => {
                                                                                let source_ml_source2_data_property_value1382 = *source_ml_source2_data_property_value1382;
                                                                                source_ml_source2_data_property_value1382.clone()
                                                                            }
                                                                        }, input.clone())) || !(self::test_java_script_semantic_same_value(match state_state.clone() {
                                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value1383, _, _) => {
                                                                                let source_ml_source2_data_property_value1383 = *source_ml_source2_data_property_value1383;
                                                                                source_ml_source2_data_property_value1383.clone()
                                                                            }
                                                                        }, exotic_key.clone()))) {
                                                                            self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin138::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin138Return(self::ml_string::Text::from_units(vec![]))
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin138::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin138Next
                                                                        }
                                                                    }
                                                                    self::JavaScriptNumberCallState::NumberPrimitiveValue(_) => {
                                                                        self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin138::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin138Next
                                                                    }
                                                                };
                                                                match source_ml_source2_s1380.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin138::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin138Next => {
                                                                        operation.clone()
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin138::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin138Return(source_ml_source1_temporary_result138) => {
                                                                        source_ml_source1_temporary_result138.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            self::ml_string::Text::from_units(vec![])
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_text_call_source_operation(fn_: self::JavaScriptCallFunction, frame: self::JavaScriptCallFrame, destination: f64, realm: self::ml::Big, exotic_key: self::JavaScriptSemanticValue, state: self::JavaScriptTextCallState) -> self::ml_string::Text {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(_, shared_constructor_field3, shared_constructor_field4, _, _, _) => {
            match fn_.clone() {
                self::JavaScriptCallFunction::CallFunction(_, shared_constructor_field1, _, _, _, _, _, shared_constructor_field2) => {
                    if ((shared_constructor_field3 == (0f64)) || (shared_constructor_field1 != realm)) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        {
                            let instruction = self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index((shared_constructor_field3 - 1f64)));
                            if match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                match instruction.clone().expect("a proven present value") {
                                    self::JavaScriptCallInstruction::Primitive(instruction_destination, operation, arguments) => {
                                        if ((instruction_destination != destination) || (((arguments.len() as f64)) != (1f64))) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            if ((self::read_java_script_text_protocol_operation_error(&(operation))) != (self::ml_string::Text::from_units(vec![]))) {
                                                self::ml_string::Text::from_units(vec![])
                                            } else {
                                                {
                                                    let input = (self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index((self::ml_array::optional_at(&arguments, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| (-1f64))))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                    match input.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            {
                                                                let source_ml_source2_s1447 = match state.clone() {
                                                                    self::JavaScriptTextCallState::TextProtocolState(state_state) => {
                                                                        let state_state = *state_state;
                                                                        if (!(self::test_java_script_semantic_same_value(match state_state.clone() {
                                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value1449, _, _, _) => {
                                                                                let source_ml_source2_data_property_value1449 = *source_ml_source2_data_property_value1449;
                                                                                source_ml_source2_data_property_value1449.clone()
                                                                            }
                                                                        }, input.clone())) || !(self::test_java_script_semantic_same_value(match state_state.clone() {
                                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value1450, _, _) => {
                                                                                let source_ml_source2_data_property_value1450 = *source_ml_source2_data_property_value1450;
                                                                                source_ml_source2_data_property_value1450.clone()
                                                                            }
                                                                        }, exotic_key.clone()))) {
                                                                            self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin139::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin139Return(self::ml_string::Text::from_units(vec![]))
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin139::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin139Next
                                                                        }
                                                                    }
                                                                    self::JavaScriptTextCallState::TextPrimitiveValue(_) => {
                                                                        self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin139::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin139Next
                                                                    }
                                                                };
                                                                match source_ml_source2_s1447.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin139::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin139Next => {
                                                                        operation.clone()
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin139::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin139Return(source_ml_source1_temporary_result139) => {
                                                                        source_ml_source1_temporary_result139.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            self::ml_string::Text::from_units(vec![])
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_machine_error(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine) -> self::ml_string::Text {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let error = self::read_java_script_call_program_error(program.clone(), shared_constructor_field1.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    if ((((shared_constructor_field2.len() as f64)) == (0f64)) || (((shared_constructor_field2.len() as f64)) > (1024f64))) {
                        self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 109, 97, 99, 104, 105, 110, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 97, 99, 116, 105, 118, 101, 32, 115, 116, 97, 99, 107])
                    } else {
                        {
                            let slots = 0f64;
                            {
                                let argument_slots = 0f64;
                                {
                                    let index = 0f64;
                                    {
                                        let source_ml_source2_s1456 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop143(program.clone(), machine.clone(), slots, argument_slots, index);
                                        match source_ml_source2_s1456.clone() {
                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Done => {
                                                self::ml_string::Text::from_units(vec![])
                                            }
                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(source_ml_source1_temporary_result143) => {
                                                source_ml_source1_temporary_result143.clone()
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
pub fn read_java_script_number_call_state_error(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptNumberCallState) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptNumberCallState::NumberProtocolState(state_state) => {
            let state_state = *state_state;
            if ((match state_state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1458, _) => {
                    source_ml_source2_data_property_value1458.clone()
                }
            }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
                self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 104, 105, 110, 116])
            } else {
                self::read_java_script_primitive_conversion_state_error(heap.clone(), state_state.clone())
            }
        }
        self::JavaScriptNumberCallState::NumberPrimitiveValue(value) => {
            let value = *value;
            if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101])
            } else {
                if match value.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        true
                    }
                    _ => {
                        false
                    }
                } {
                    self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_text_call_state_error(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptTextCallState) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptTextCallState::TextProtocolState(state_state) => {
            let state_state = *state_state;
            if ((match state_state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1467, _) => {
                    source_ml_source2_data_property_value1467.clone()
                }
            }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                self::ml_string::Text::from_units(vec![84, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 110, 103, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 104, 105, 110, 116])
            } else {
                self::read_java_script_primitive_conversion_state_error(heap.clone(), state_state.clone())
            }
        }
        self::JavaScriptTextCallState::TextPrimitiveValue(value) => {
            let value = *value;
            if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101])
            } else {
                if match value.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        true
                    }
                    _ => {
                        false
                    }
                } {
                    self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn throw_java_script_call_value(machine: self::JavaScriptCallMachine, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frames = shared_constructor_field2;
                {
                    let source_ml_source2_s1476 = self::source_ml_source1_temporary_throw_java_script_call_value_loop151(machine.clone(), value.clone(), frames);
                    match source_ml_source2_s1476.clone() {
                        self::MlSource1TemporaryThrowJavaScriptCallValueLoop151Result::MlSource1TemporaryThrowJavaScriptCallValueLoop151Done => {
                            self::JavaScriptCallMachineTransition::MachineThrow(Box::new(shared_constructor_field1.clone()), Box::new(value.clone()))
                        }
                        self::MlSource1TemporaryThrowJavaScriptCallValueLoop151Result::MlSource1TemporaryThrowJavaScriptCallValueLoop151Return(source_ml_source1_temporary_result151) => {
                            let source_ml_source1_temporary_result151 = *source_ml_source1_temporary_result151;
                            source_ml_source1_temporary_result151.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn request_java_script_call_error(machine: self::JavaScriptCallMachine, realm: self::ml::Big, error_class: &self::ml_string::Text, reason: &self::ml_string::Text) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(((shared_constructor_field2.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    {
                        let next = self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, {
                            let source_ml_source2_nominal_spread1483 = frame.clone().expect("a proven present value");
                            {
                                let source_ml_source2_nominal_field1484 = match source_ml_source2_nominal_spread1483.clone() {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1485, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1485.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1486 = match source_ml_source2_nominal_spread1483.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1487, _, _, _, _) => {
                                            source_ml_source2_data_property_value1487
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1488 = match source_ml_source2_nominal_spread1483.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1489, _, _, _) => {
                                                source_ml_source2_data_property_value1489
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1490 = match source_ml_source2_nominal_spread1483.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1491, _, _) => {
                                                    source_ml_source2_data_property_value1491
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1492 = match source_ml_source2_nominal_spread1483.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1493, _) => {
                                                        source_ml_source2_data_property_value1493
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1494 = match source_ml_source2_nominal_spread1483.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1495) => {
                                                            let source_ml_source2_data_property_value1495 = *source_ml_source2_data_property_value1495;
                                                            source_ml_source2_data_property_value1495.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_override1496 = self::JavaScriptCallPending::WaitError(realm.clone(), error_class.clone(), reason.clone());
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1484.clone(), source_ml_source2_nominal_field1486, source_ml_source2_nominal_field1488, source_ml_source2_nominal_field1490, source_ml_source2_nominal_field1492, Box::new(source_ml_source2_nominal_override1496.clone()))
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
    }
}
