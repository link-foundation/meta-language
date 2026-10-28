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
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryPushJavaScriptCallLoop149Result;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result;
use super::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin136;
use super::MlSource1TemporaryThrowJavaScriptCallValueLoop147Result;
use super::find_java_script_call_function;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_call_program_error;
use super::read_java_script_number_protocol_operation_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_state_error;
use super::replace_java_script_call_frame;
use super::source_ml_source1_temporary_create_java_script_call_frame_loop133;
use super::source_ml_source1_temporary_create_java_script_call_frame_loop134;
use super::source_ml_source1_temporary_create_java_script_call_frame_loop135;
use super::source_ml_source1_temporary_push_java_script_call_loop149;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop140;
use super::source_ml_source1_temporary_throw_java_script_call_value_loop147;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn write_java_script_call_register(frame: self::JavaScriptCallFrame, destination: f64, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallFrame {
    {
        let source_ml_source2_nominal_spread1285 = frame.clone();
        {
            let source_ml_source2_nominal_field1286 = match source_ml_source2_nominal_spread1285.clone() {
                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1287, _, _, _, _, _) => {
                    source_ml_source2_data_property_value1287.clone()
                }
            };
            {
                let source_ml_source2_nominal_field1288 = match source_ml_source2_nominal_spread1285.clone() {
                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1289, _, _, _, _) => {
                        source_ml_source2_data_property_value1289
                    }
                };
                {
                    let source_ml_source2_nominal_field1290 = match source_ml_source2_nominal_spread1285.clone() {
                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1291, _, _, _) => {
                            source_ml_source2_data_property_value1291
                        }
                    };
                    {
                        let source_ml_source2_nominal_field1292 = match source_ml_source2_nominal_spread1285.clone() {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1293, _, _) => {
                                source_ml_source2_data_property_value1293
                            }
                        };
                        {
                            let source_ml_source2_nominal_field1294 = match source_ml_source2_nominal_spread1285.clone() {
                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1295, _) => {
                                    source_ml_source2_data_property_value1295
                                }
                            };
                            {
                                let source_ml_source2_nominal_field1296 = match source_ml_source2_nominal_spread1285.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1297) => {
                                        let source_ml_source2_data_property_value1297 = *source_ml_source2_data_property_value1297;
                                        source_ml_source2_data_property_value1297.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_override1298 = { let ml_values4 = match frame.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1284, _, _, _) => {
                                            source_ml_source2_data_property_value1284
                                        }
                                    }; let ml_callback4 = { let destination = destination; let value = value.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, index: f64| -> self::JavaScriptSemanticValue { if (index == destination) {
                                        value.clone()
                                    } else {
                                        entry.clone()
                                    } }) }; ml_values4.iter().cloned().enumerate().map(|(ml_index4, ml_value4)| ml_callback4(ml_value4, (ml_index4 as f64))).collect::<Vec<_>>() };
                                    {
                                        let source_ml_source2_nominal_override1299 = self::JavaScriptCallPending::CallIdle;
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1286.clone(), source_ml_source2_nominal_field1288, source_ml_source2_nominal_override1298, source_ml_source2_nominal_field1292, source_ml_source2_nominal_field1294, Box::new(source_ml_source2_nominal_override1299.clone()))
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
                        let registers_2 = self::source_ml_source1_temporary_create_java_script_call_frame_loop133(fn_.clone(), registers, index);
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
                                        let registers_4 = self::source_ml_source1_temporary_create_java_script_call_frame_loop134(fn_.clone(), arguments_.clone(), registers_3, index_2);
                                        {
                                            let index_3 = 0f64;
                                            {
                                                let registers_5 = self::source_ml_source1_temporary_create_java_script_call_frame_loop135(fn_.clone(), registers_4, index_3);
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
                                    self::JavaScriptCallInstruction::Constant(_, _) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::Copy(_, _) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::Argument(_, _) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::Jump(_) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::Branch(_, _, _) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::Return(_) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::Throw(_) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::PushHandler(_, _) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::PopHandler => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::NumberBinary(_, _, _, _) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::RequireWrite(_) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
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
                                                        self::JavaScriptSemanticValue::Undefined => {
                                                            self::ml_string::Text::from_units(vec![])
                                                        }
                                                        self::JavaScriptSemanticValue::Null => {
                                                            self::ml_string::Text::from_units(vec![])
                                                        }
                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                            self::ml_string::Text::from_units(vec![])
                                                        }
                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                            self::ml_string::Text::from_units(vec![])
                                                        }
                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                            self::ml_string::Text::from_units(vec![])
                                                        }
                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                            self::ml_string::Text::from_units(vec![])
                                                        }
                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                            self::ml_string::Text::from_units(vec![])
                                                        }
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            {
                                                                let source_ml_source2_s1367 = match state.clone() {
                                                                    self::JavaScriptNumberCallState::NumberProtocolState(state_state) => {
                                                                        let state_state = *state_state;
                                                                        if (!(self::test_java_script_semantic_same_value(match state_state.clone() {
                                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value1369, _, _, _) => {
                                                                                let source_ml_source2_data_property_value1369 = *source_ml_source2_data_property_value1369;
                                                                                source_ml_source2_data_property_value1369.clone()
                                                                            }
                                                                        }, input.clone())) || !(self::test_java_script_semantic_same_value(match state_state.clone() {
                                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value1370, _, _) => {
                                                                                let source_ml_source2_data_property_value1370 = *source_ml_source2_data_property_value1370;
                                                                                source_ml_source2_data_property_value1370.clone()
                                                                            }
                                                                        }, exotic_key.clone()))) {
                                                                            self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin136::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin136Return(self::ml_string::Text::from_units(vec![]))
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin136::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin136Next
                                                                        }
                                                                    }
                                                                    self::JavaScriptNumberCallState::NumberPrimitiveValue(_) => {
                                                                        self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin136::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin136Next
                                                                    }
                                                                };
                                                                match source_ml_source2_s1367.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin136::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin136Next => {
                                                                        operation.clone()
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin136::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin136Return(source_ml_source1_temporary_result136) => {
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
                                    self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::PropertyKey(_, _, _) => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _) => {
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
                                        let source_ml_source2_s1376 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop140(program.clone(), machine.clone(), slots, argument_slots, index);
                                        match source_ml_source2_s1376.clone() {
                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Done => {
                                                self::ml_string::Text::from_units(vec![])
                                            }
                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Return(source_ml_source1_temporary_result140) => {
                                                source_ml_source1_temporary_result140.clone()
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
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1378, _) => {
                    source_ml_source2_data_property_value1378.clone()
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
pub fn throw_java_script_call_value(machine: self::JavaScriptCallMachine, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frames = shared_constructor_field2;
                {
                    let source_ml_source2_s1387 = self::source_ml_source1_temporary_throw_java_script_call_value_loop147(machine.clone(), value.clone(), frames);
                    match source_ml_source2_s1387.clone() {
                        self::MlSource1TemporaryThrowJavaScriptCallValueLoop147Result::MlSource1TemporaryThrowJavaScriptCallValueLoop147Done => {
                            self::JavaScriptCallMachineTransition::MachineThrow(Box::new(shared_constructor_field1.clone()), Box::new(value.clone()))
                        }
                        self::MlSource1TemporaryThrowJavaScriptCallValueLoop147Result::MlSource1TemporaryThrowJavaScriptCallValueLoop147Return(source_ml_source1_temporary_result147) => {
                            let source_ml_source1_temporary_result147 = *source_ml_source1_temporary_result147;
                            source_ml_source1_temporary_result147.clone()
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
                            let source_ml_source2_nominal_spread1394 = frame.clone().expect("a proven present value");
                            {
                                let source_ml_source2_nominal_field1395 = match source_ml_source2_nominal_spread1394.clone() {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1396, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1396.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1397 = match source_ml_source2_nominal_spread1394.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1398, _, _, _, _) => {
                                            source_ml_source2_data_property_value1398
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1399 = match source_ml_source2_nominal_spread1394.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1400, _, _, _) => {
                                                source_ml_source2_data_property_value1400
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1401 = match source_ml_source2_nominal_spread1394.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1402, _, _) => {
                                                    source_ml_source2_data_property_value1402
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1403 = match source_ml_source2_nominal_spread1394.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1404, _) => {
                                                        source_ml_source2_data_property_value1404
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1405 = match source_ml_source2_nominal_spread1394.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1406) => {
                                                            let source_ml_source2_data_property_value1406 = *source_ml_source2_data_property_value1406;
                                                            source_ml_source2_data_property_value1406.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_override1407 = self::JavaScriptCallPending::WaitError(realm.clone(), error_class.clone(), reason.clone());
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1395.clone(), source_ml_source2_nominal_field1397, source_ml_source2_nominal_field1399, source_ml_source2_nominal_field1401, source_ml_source2_nominal_field1403, Box::new(source_ml_source2_nominal_override1407.clone()))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn push_java_script_call(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, callee: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue, arguments_: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field2, shared_constructor_field3) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            match program.clone() {
                self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, _, _) => {
                    {
                        let frame = self::ml_array::optional_at(&shared_constructor_field3, self::ml_array::number_index(((shared_constructor_field3.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field2.clone()), self::ml_string::Text::from_units(vec![97, 32, 110, 101, 115, 116, 101, 100, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116]))
                        } else {
                            {
                                let owner = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1412, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1412.clone()
                                    }
                                })));
                                if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field2.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                } else {
                                    match callee.clone() {
                                        self::JavaScriptSemanticValue::Undefined => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1419, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1419.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Null => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1420, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1420.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1421, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1421.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Number(_) => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1422, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1422.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Integer(_) => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1423, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1423.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Text(_) => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1424, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1424.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1425, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1425.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Object(identity, callable) => {
                                            if !callable {
                                                self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1426, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value1426.clone()
                                                    }
                                                }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                            } else {
                                                {
                                                    let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), identity.clone())));
                                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field2.clone()), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))
                                                    } else {
                                                        {
                                                            let slots = match fn_.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value1429, _, _) => {
                                                                    source_ml_source2_data_property_value1429
                                                                }
                                                            };
                                                            {
                                                                let argument_slots = (arguments_.len() as f64);
                                                                {
                                                                    let source_ml_source1_temporary_values148 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source1_temporary_index148 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let source_ml_source2_s1431 = self::source_ml_source1_temporary_push_java_script_call_loop149(slots, argument_slots, source_ml_source1_temporary_values148, source_ml_source1_temporary_index148.clone());
                                                                            match source_ml_source2_s1431.clone() {
                                                                                self::MlSource1TemporaryPushJavaScriptCallLoop149Result::MlSource1TemporaryPushJavaScriptCallLoop149Done(slots_2, argument_slots_2) => {
                                                                                    if (((((shared_constructor_field3.len() as f64)) >= (1024f64)) || (slots_2 > (1048576f64))) || (argument_slots_2 > (1048576f64))) {
                                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                                    } else {
                                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(shared_constructor_field2.clone()), {
                                                                                            let source_ml_source2_concat_value1436 = shared_constructor_field3;
                                                                                            {
                                                                                                let source_ml_source2_concat_value1437 = vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_)];
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), source_ml_source2_concat_value1436), source_ml_source2_concat_value1437)
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
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_key(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptPropertyKeyConversionTransition) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(((shared_constructor_field2.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(state) => {
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, {
                                let source_ml_source2_nominal_spread1443 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1444 = match source_ml_source2_nominal_spread1443.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1445, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1445.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1446 = match source_ml_source2_nominal_spread1443.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1447, _, _, _, _) => {
                                                source_ml_source2_data_property_value1447
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1448 = match source_ml_source2_nominal_spread1443.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1449, _, _, _) => {
                                                    source_ml_source2_data_property_value1449
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1450 = match source_ml_source2_nominal_spread1443.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1451, _, _) => {
                                                        source_ml_source2_data_property_value1451
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1452 = match source_ml_source2_nominal_spread1443.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1453, _) => {
                                                            source_ml_source2_data_property_value1453
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1454 = match source_ml_source2_nominal_spread1443.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1455) => {
                                                                let source_ml_source2_data_property_value1455 = *source_ml_source2_data_property_value1455;
                                                                source_ml_source2_data_property_value1455.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1456 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state.clone()));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1444.clone(), source_ml_source2_nominal_field1446, source_ml_source2_nominal_field1448, source_ml_source2_nominal_field1450, source_ml_source2_nominal_field1452, Box::new(source_ml_source2_nominal_override1456.clone()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            })))
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCall(state_2, callee, receiver, arguments) => {
                            let state_2 = *state_2;
                            let callee = *callee;
                            let receiver = *receiver;
                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, {
                                let source_ml_source2_nominal_spread1459 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1460 = match source_ml_source2_nominal_spread1459.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1461, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1461.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1462 = match source_ml_source2_nominal_spread1459.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1463, _, _, _, _) => {
                                                source_ml_source2_data_property_value1463
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1464 = match source_ml_source2_nominal_spread1459.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1465, _, _, _) => {
                                                    source_ml_source2_data_property_value1465
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1466 = match source_ml_source2_nominal_spread1459.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1467, _, _) => {
                                                        source_ml_source2_data_property_value1467
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1468 = match source_ml_source2_nominal_spread1459.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1469, _) => {
                                                            source_ml_source2_data_property_value1469
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1470 = match source_ml_source2_nominal_spread1459.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1471) => {
                                                                let source_ml_source2_data_property_value1471 = *source_ml_source2_data_property_value1471;
                                                                source_ml_source2_data_property_value1471.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1472 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state_2.clone()));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1460.clone(), source_ml_source2_nominal_field1462, source_ml_source2_nominal_field1464, source_ml_source2_nominal_field1466, source_ml_source2_nominal_field1468, Box::new(source_ml_source2_nominal_override1472.clone()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }), callee.clone(), receiver.clone(), arguments)
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(key) => {
                            let key = *key;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, key.clone()))))
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyThrow(value) => {
                            let value = *value;
                            self::throw_java_script_call_value(machine.clone(), value.clone())
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyIntrinsicError(error_class, reason) => {
                            self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(reason_2) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), reason_2.clone())
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_definition(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptPropertyDefinitionTransition) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(((shared_constructor_field2.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                        self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(state) => {
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, {
                                let source_ml_source2_nominal_spread1482 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1483 = match source_ml_source2_nominal_spread1482.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1484, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1484.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1485 = match source_ml_source2_nominal_spread1482.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1486, _, _, _, _) => {
                                                source_ml_source2_data_property_value1486
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1487 = match source_ml_source2_nominal_spread1482.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1488, _, _, _) => {
                                                    source_ml_source2_data_property_value1488
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1489 = match source_ml_source2_nominal_spread1482.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1490, _, _) => {
                                                        source_ml_source2_data_property_value1490
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1491 = match source_ml_source2_nominal_spread1482.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1492, _) => {
                                                            source_ml_source2_data_property_value1492
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1493 = match source_ml_source2_nominal_spread1482.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1494) => {
                                                                let source_ml_source2_data_property_value1494 = *source_ml_source2_data_property_value1494;
                                                                source_ml_source2_data_property_value1494.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1495 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state.clone()));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1483.clone(), source_ml_source2_nominal_field1485, source_ml_source2_nominal_field1487, source_ml_source2_nominal_field1489, source_ml_source2_nominal_field1491, Box::new(source_ml_source2_nominal_override1495.clone()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            })))
                        }
                        self::JavaScriptPropertyDefinitionTransition::DefinitionCall(state_2, callee, receiver, arguments) => {
                            let state_2 = *state_2;
                            let callee = *callee;
                            let receiver = *receiver;
                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, {
                                let source_ml_source2_nominal_spread1498 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1499 = match source_ml_source2_nominal_spread1498.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1500, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1500.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1501 = match source_ml_source2_nominal_spread1498.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1502, _, _, _, _) => {
                                                source_ml_source2_data_property_value1502
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1503 = match source_ml_source2_nominal_spread1498.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1504, _, _, _) => {
                                                    source_ml_source2_data_property_value1504
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1505 = match source_ml_source2_nominal_spread1498.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1506, _, _) => {
                                                        source_ml_source2_data_property_value1506
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1507 = match source_ml_source2_nominal_spread1498.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1508, _) => {
                                                            source_ml_source2_data_property_value1508
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1509 = match source_ml_source2_nominal_spread1498.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1510) => {
                                                                let source_ml_source2_data_property_value1510 = *source_ml_source2_data_property_value1510;
                                                                source_ml_source2_data_property_value1510.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1511 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state_2.clone()));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1499.clone(), source_ml_source2_nominal_field1501, source_ml_source2_nominal_field1503, source_ml_source2_nominal_field1505, source_ml_source2_nominal_field1507, Box::new(source_ml_source2_nominal_override1511.clone()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }), callee.clone(), receiver.clone(), arguments)
                        }
                        self::JavaScriptPropertyDefinitionTransition::DefinitionCompleted(heap, value, _) => {
                            let heap = *heap;
                            let value = *value;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), shared_constructor_field2, self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                        }
                        self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(value_2) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(machine.clone(), value_2.clone())
                        }
                        self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(error_class, reason) => {
                            self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                        }
                        self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(reason_2) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), reason_2.clone())
                        }
                    }
                }
            }
        }
    }
}
