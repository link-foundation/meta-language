// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptNumberCallState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::JavaScriptTextCallState;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result;
use super::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin140;
use super::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin141;
use super::begin_java_script_property_key_conversion;
use super::continue_java_script_property_descriptor_definition;
use super::continue_java_script_property_key_definition;
use super::find_java_script_call_code_function;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::project_java_script_property_key_completion;
use super::read_java_script_call_code_registers;
use super::read_java_script_integer_protocol_operation_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_text_protocol_operation_error;
use super::resume_java_script_descriptor_conversion;
use super::resume_java_script_property_key_conversion;
use super::source_ml_source1_temporary_create_java_script_call_frame_loop137;
use super::source_ml_source1_temporary_create_java_script_call_frame_loop138;
use super::source_ml_source1_temporary_create_java_script_call_frame_loop139;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop142;
use super::source_ml_source1_temporary_read_java_script_call_program_error_loop132;
use super::source_ml_source1_temporary_read_java_script_call_program_error_loop133;
use super::step_java_script_descriptor_conversion;
use super::step_java_script_property_key_conversion;
use super::test_java_script_call_code_coordinate;
use super::test_java_script_semantic_same_value;

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
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1248, _, _) => {
                let source_ml_source2_data_property_value1248 = *source_ml_source2_data_property_value1248;
                source_ml_source2_data_property_value1248.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1249, _, _) => {
                let source_ml_source2_data_property_value1249 = *source_ml_source2_data_property_value1249;
                source_ml_source2_data_property_value1249.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source2_s1252 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1260, _, _) => {
                        let source_ml_source2_data_property_value1260 = *source_ml_source2_data_property_value1260;
                        source_ml_source2_data_property_value1260.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1261, _, _) => {
                        let source_ml_source2_data_property_value1261 = *source_ml_source2_data_property_value1261;
                        source_ml_source2_data_property_value1261.clone()
                    }
                };
                match source_ml_source2_s1252.clone() {
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
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1262, _, _) => {
                let source_ml_source2_data_property_value1262 = *source_ml_source2_data_property_value1262;
                source_ml_source2_data_property_value1262.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1263, _, _) => {
                let source_ml_source2_data_property_value1263 = *source_ml_source2_data_property_value1263;
                source_ml_source2_data_property_value1263.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source2_s1266 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1274, _, _) => {
                        let source_ml_source2_data_property_value1274 = *source_ml_source2_data_property_value1274;
                        source_ml_source2_data_property_value1274.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1275, _, _) => {
                        let source_ml_source2_data_property_value1275 = *source_ml_source2_data_property_value1275;
                        source_ml_source2_data_property_value1275.clone()
                    }
                };
                match source_ml_source2_s1266.clone() {
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
    if ((((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) || ((self::read_java_script_integer_protocol_operation_error(operation)) == (self::ml_string::Text::from_units(vec![])))) {
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
                                let source_ml_source1_temporary_values127 = shared_constructor_field2;
                                {
                                    let source_ml_source1_temporary_index127 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s1281 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop132(previous.clone(), source_ml_source1_temporary_values127, source_ml_source1_temporary_index127.clone());
                                        match source_ml_source2_s1281.clone() {
                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Done(previous_2) => {
                                                if {
                                                    let source_ml_source2_s1283 = shared_constructor_field3.clone();
                                                    match source_ml_source2_s1283.clone() {
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
                                                                        let source_ml_source1_temporary_values128 = shared_constructor_field1;
                                                                        {
                                                                            let source_ml_source1_temporary_index128 = self::ml::Big::from_i128(0);
                                                                            {
                                                                                let source_ml_source2_s1294 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop133(program.clone(), heap.clone(), previous_3.clone(), count, source_ml_source1_temporary_values128, source_ml_source1_temporary_index128.clone());
                                                                                match source_ml_source2_s1294.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Done => {
                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(source_ml_source1_temporary_result133) => {
                                                                                        source_ml_source1_temporary_result133.clone()
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
                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Return(source_ml_source1_temporary_result136) => {
                                                source_ml_source1_temporary_result136.clone()
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
        let source_ml_source2_nominal_spread1296 = frame.clone();
        match source_ml_source2_nominal_spread1296.clone() {
            self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                let shared_constructor_field6 = *shared_constructor_field6;
                {
                    let source_ml_source2_nominal_field1297 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_nominal_field1299 = shared_constructor_field2;
                        {
                            let source_ml_source2_nominal_field1301 = shared_constructor_field3;
                            {
                                let source_ml_source2_nominal_field1303 = shared_constructor_field4;
                                {
                                    let source_ml_source2_nominal_field1305 = shared_constructor_field5;
                                    {
                                        let source_ml_source2_nominal_field1307 = shared_constructor_field6.clone();
                                        {
                                            let source_ml_source2_nominal_override1309 = { let ml_values4 = match frame.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1295, _, _, _) => {
                                                    source_ml_source2_data_property_value1295
                                                }
                                            }; let ml_callback4 = { let destination = destination; let value = value.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, index: f64| -> self::JavaScriptSemanticValue { if (index == destination) {
                                                value.clone()
                                            } else {
                                                entry.clone()
                                            } }) }; ml_values4.iter().cloned().enumerate().map(|(ml_index4, ml_value4)| ml_callback4(ml_value4, (ml_index4 as f64))).collect::<Vec<_>>() };
                                            {
                                                let source_ml_source2_nominal_override1310 = self::JavaScriptCallPending::CallIdle;
                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1297.clone(), source_ml_source2_nominal_field1299, source_ml_source2_nominal_override1309, source_ml_source2_nominal_field1303, source_ml_source2_nominal_field1305, Box::new(source_ml_source2_nominal_override1310.clone()))
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
pub fn replace_java_script_call_pending(frame: self::JavaScriptCallFrame, pending: self::JavaScriptCallPending) -> self::JavaScriptCallFrame {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
            let shared_constructor_field6 = *shared_constructor_field6;
            {
                let source_ml_source2_nominal_field1312 = shared_constructor_field1.clone();
                {
                    let source_ml_source2_nominal_field1314 = shared_constructor_field2;
                    {
                        let source_ml_source2_nominal_field1316 = shared_constructor_field3;
                        {
                            let source_ml_source2_nominal_field1318 = shared_constructor_field4;
                            {
                                let source_ml_source2_nominal_field1320 = shared_constructor_field5;
                                {
                                    let source_ml_source2_nominal_field1322 = shared_constructor_field6.clone();
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1312.clone(), source_ml_source2_nominal_field1314, source_ml_source2_nominal_field1316, source_ml_source2_nominal_field1318, source_ml_source2_nominal_field1320, Box::new(pending.clone()))
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
pub fn advance_java_script_call_frame(frame: self::JavaScriptCallFrame) -> self::JavaScriptCallFrame {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
            let shared_constructor_field6 = *shared_constructor_field6;
            {
                let source_ml_source2_nominal_field1327 = shared_constructor_field1.clone();
                {
                    let source_ml_source2_nominal_field1329 = shared_constructor_field2;
                    {
                        let source_ml_source2_nominal_field1331 = shared_constructor_field3;
                        {
                            let source_ml_source2_nominal_field1333 = shared_constructor_field4;
                            {
                                let source_ml_source2_nominal_field1335 = shared_constructor_field5;
                                {
                                    let source_ml_source2_nominal_field1337 = shared_constructor_field6.clone();
                                    {
                                        let source_ml_source2_nominal_override1339 = (shared_constructor_field2 + 1f64);
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1327.clone(), source_ml_source2_nominal_override1339, source_ml_source2_nominal_field1331, source_ml_source2_nominal_field1333, source_ml_source2_nominal_field1335, Box::new(source_ml_source2_nominal_field1337.clone()))
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
                        let registers_2 = self::source_ml_source1_temporary_create_java_script_call_frame_loop137(fn_.clone(), registers, index);
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
                                        let registers_4 = self::source_ml_source1_temporary_create_java_script_call_frame_loop138(fn_.clone(), arguments_.clone(), registers_3, index_2);
                                        {
                                            let index_3 = 0f64;
                                            {
                                                let registers_5 = self::source_ml_source1_temporary_create_java_script_call_frame_loop139(fn_.clone(), registers_4, index_3);
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
        self::JavaScriptCallFrame::CallFrame(_, shared_constructor_field5, shared_constructor_field6, _, _, _) => {
            match fn_.clone() {
                self::JavaScriptCallFunction::CallFunction(_, shared_constructor_field3, _, _, _, _, _, shared_constructor_field4) => {
                    if ((shared_constructor_field5 == (0f64)) || (shared_constructor_field3 != realm)) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        {
                            let instruction = self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index((shared_constructor_field5 - 1f64)));
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
                                                    let input = (self::ml_array::optional_at(&shared_constructor_field6, self::ml_array::number_index((self::ml_array::optional_at(&arguments, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| (-1f64))))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                    match input.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            {
                                                                let source_ml_source2_s1407 = match state.clone() {
                                                                    self::JavaScriptNumberCallState::NumberProtocolState(state_state) => {
                                                                        let state_state = *state_state;
                                                                        match state_state.clone() {
                                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, _, _) => {
                                                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                                                let shared_constructor_field2 = *shared_constructor_field2;
                                                                                if (!(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), input.clone())) || !(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), exotic_key.clone()))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin140::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin140Return(self::ml_string::Text::from_units(vec![]))
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin140::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin140Next
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::JavaScriptNumberCallState::NumberPrimitiveValue(_) => {
                                                                        self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin140::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin140Next
                                                                    }
                                                                };
                                                                match source_ml_source2_s1407.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin140::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin140Next => {
                                                                        operation.clone()
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin140::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin140Return(source_ml_source1_temporary_result140) => {
                                                                        source_ml_source1_temporary_result140.clone()
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
        self::JavaScriptCallFrame::CallFrame(_, shared_constructor_field5, shared_constructor_field6, _, _, _) => {
            match fn_.clone() {
                self::JavaScriptCallFunction::CallFunction(_, shared_constructor_field3, _, _, _, _, _, shared_constructor_field4) => {
                    if ((shared_constructor_field5 == (0f64)) || (shared_constructor_field3 != realm)) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        {
                            let instruction = self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index((shared_constructor_field5 - 1f64)));
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
                                                    let input = (self::ml_array::optional_at(&shared_constructor_field6, self::ml_array::number_index((self::ml_array::optional_at(&arguments, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| (-1f64))))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                    match input.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            {
                                                                let source_ml_source2_s1474 = match state.clone() {
                                                                    self::JavaScriptTextCallState::TextProtocolState(state_state) => {
                                                                        let state_state = *state_state;
                                                                        match state_state.clone() {
                                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, _, _) => {
                                                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                                                let shared_constructor_field2 = *shared_constructor_field2;
                                                                                if (!(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), input.clone())) || !(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), exotic_key.clone()))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin141::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin141Return(self::ml_string::Text::from_units(vec![]))
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin141::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin141Next
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::JavaScriptTextCallState::TextPrimitiveValue(_) => {
                                                                        self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin141::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin141Next
                                                                    }
                                                                };
                                                                match source_ml_source2_s1474.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin141::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin141Next => {
                                                                        operation.clone()
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin141::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin141Return(source_ml_source1_temporary_result141) => {
                                                                        source_ml_source1_temporary_result141.clone()
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
                                        let source_ml_source2_s1483 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop142(program.clone(), machine.clone(), slots, argument_slots, index);
                                        match source_ml_source2_s1483.clone() {
                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Done => {
                                                self::ml_string::Text::from_units(vec![])
                                            }
                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(source_ml_source1_temporary_result142) => {
                                                source_ml_source1_temporary_result142.clone()
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
pub fn test_java_script_call_pending_destination(pending: self::JavaScriptCallPending, registers: f64) -> bool {
    match pending.clone() {
        self::JavaScriptCallPending::WaitCall(destination) => {
            self::test_java_script_call_coordinate(destination, registers)
        }
        self::JavaScriptCallPending::WaitGet(destination_2) => {
            self::test_java_script_call_coordinate(destination_2, registers)
        }
        self::JavaScriptCallPending::WaitSet(destination_3) => {
            self::test_java_script_call_coordinate(destination_3, registers)
        }
        self::JavaScriptCallPending::WaitKey(destination_4, _, _) => {
            self::test_java_script_call_coordinate(destination_4, registers)
        }
        self::JavaScriptCallPending::WaitDefinition(destination_5, _, _) => {
            self::test_java_script_call_coordinate(destination_5, registers)
        }
        self::JavaScriptCallPending::WaitNumber(destination_6, _, _) => {
            self::test_java_script_call_coordinate(destination_6, registers)
        }
        self::JavaScriptCallPending::WaitText(destination_7, _, _) => {
            self::test_java_script_call_coordinate(destination_7, registers)
        }
        _ => {
            true
        }
    }
}
