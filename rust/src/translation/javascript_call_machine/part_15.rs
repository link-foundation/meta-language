// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptNumberCallState;
use super::JavaScriptNumberConversionTransition;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::accept_java_script_call_definition;
use super::accept_java_script_call_key;
use super::find_java_script_call_function;
use super::finish_java_script_number_primitive;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::plan_java_script_ordinary_set;
use super::push_java_script_call;
use super::read_java_script_number_call_source_operation;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::resume_java_script_number_protocol;
use super::resume_java_script_property_definition;
use super::resume_java_script_property_key_conversion;
use super::step_java_script_number_protocol;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_number(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptNumberConversionTransition) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(((shared_constructor_field2.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                        self::JavaScriptNumberConversionTransition::NumberContinuation(state) => {
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, {
                                let source_ml_source2_nominal_spread1519 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1520 = match source_ml_source2_nominal_spread1519.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1521, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1521.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1522 = match source_ml_source2_nominal_spread1519.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1523, _, _, _, _) => {
                                                source_ml_source2_data_property_value1523
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1524 = match source_ml_source2_nominal_spread1519.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1525, _, _, _) => {
                                                    source_ml_source2_data_property_value1525
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1526 = match source_ml_source2_nominal_spread1519.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1527, _, _) => {
                                                        source_ml_source2_data_property_value1527
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1528 = match source_ml_source2_nominal_spread1519.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1529, _) => {
                                                            source_ml_source2_data_property_value1529
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1530 = match source_ml_source2_nominal_spread1519.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1531) => {
                                                                let source_ml_source2_data_property_value1531 = *source_ml_source2_data_property_value1531;
                                                                source_ml_source2_data_property_value1531.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1532 = self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberProtocolState(Box::new(state.clone()))));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1520.clone(), source_ml_source2_nominal_field1522, source_ml_source2_nominal_field1524, source_ml_source2_nominal_field1526, source_ml_source2_nominal_field1528, Box::new(source_ml_source2_nominal_override1532.clone()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            })))
                        }
                        self::JavaScriptNumberConversionTransition::NumberCall(state_2, callee, receiver, arguments) => {
                            let state_2 = *state_2;
                            let callee = *callee;
                            let receiver = *receiver;
                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, {
                                let source_ml_source2_nominal_spread1535 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1536 = match source_ml_source2_nominal_spread1535.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1537, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1537.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1538 = match source_ml_source2_nominal_spread1535.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1539, _, _, _, _) => {
                                                source_ml_source2_data_property_value1539
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1540 = match source_ml_source2_nominal_spread1535.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1541, _, _, _) => {
                                                    source_ml_source2_data_property_value1541
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1542 = match source_ml_source2_nominal_spread1535.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1543, _, _) => {
                                                        source_ml_source2_data_property_value1543
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1544 = match source_ml_source2_nominal_spread1535.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1545, _) => {
                                                            source_ml_source2_data_property_value1545
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1546 = match source_ml_source2_nominal_spread1535.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1547) => {
                                                                let source_ml_source2_data_property_value1547 = *source_ml_source2_data_property_value1547;
                                                                source_ml_source2_data_property_value1547.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1548 = self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberProtocolState(Box::new(state_2.clone()))));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1536.clone(), source_ml_source2_nominal_field1538, source_ml_source2_nominal_field1540, source_ml_source2_nominal_field1542, source_ml_source2_nominal_field1544, Box::new(source_ml_source2_nominal_override1548.clone()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }), callee.clone(), receiver.clone(), arguments)
                        }
                        self::JavaScriptNumberConversionTransition::NumberCompleted(value) => {
                            let value = *value;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                        }
                        self::JavaScriptNumberConversionTransition::NumberThrow(value_2) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(machine.clone(), value_2.clone())
                        }
                        self::JavaScriptNumberConversionTransition::NumberIntrinsicError(error_class, reason) => {
                            self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                        }
                        self::JavaScriptNumberConversionTransition::NumberInvalid(reason_2) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), reason_2.clone())
                        }
                        self::JavaScriptNumberConversionTransition::NumberResourceLimit(value_3, reason_3) => {
                            let value_3 = *value_3;
                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, {
                                let source_ml_source2_nominal_spread1554 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1555 = match source_ml_source2_nominal_spread1554.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1556, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1556.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1557 = match source_ml_source2_nominal_spread1554.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1558, _, _, _, _) => {
                                                source_ml_source2_data_property_value1558
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1559 = match source_ml_source2_nominal_spread1554.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1560, _, _, _) => {
                                                    source_ml_source2_data_property_value1560
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1561 = match source_ml_source2_nominal_spread1554.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1562, _, _) => {
                                                        source_ml_source2_data_property_value1562
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1563 = match source_ml_source2_nominal_spread1554.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1564, _) => {
                                                            source_ml_source2_data_property_value1564
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1565 = match source_ml_source2_nominal_spread1554.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1566) => {
                                                                let source_ml_source2_data_property_value1566 = *source_ml_source2_data_property_value1566;
                                                                source_ml_source2_data_property_value1566.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1567 = self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberPrimitiveValue(Box::new(value_3.clone()))));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1555.clone(), source_ml_source2_nominal_field1557, source_ml_source2_nominal_field1559, source_ml_source2_nominal_field1561, source_ml_source2_nominal_field1563, Box::new(source_ml_source2_nominal_override1567.clone()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            })), reason_3.clone())
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_call_number(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, state: self::JavaScriptNumberCallState) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            match program.clone() {
                self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, _, shared_constructor_field2) => {
                    let shared_constructor_field2 = *shared_constructor_field2;
                    {
                        let frame = self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index(((shared_constructor_field4.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                        } else {
                            {
                                let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1572, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1572.clone()
                                    }
                                })));
                                if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                } else {
                                    {
                                        let operation = self::read_java_script_number_call_source_operation(fn_.clone().expect("a proven present value"), frame.clone().expect("a proven present value"), destination, realm.clone(), shared_constructor_field2.clone(), state.clone());
                                        if (operation == (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                        } else {
                                            match state.clone() {
                                                self::JavaScriptNumberCallState::NumberProtocolState(state_state) => {
                                                    let state_state = *state_state;
                                                    self::accept_java_script_call_number(program.clone(), machine.clone(), destination, realm.clone(), self::step_java_script_number_protocol(shared_constructor_field3.clone(), state_state.clone(), &(operation)))
                                                }
                                                self::JavaScriptNumberCallState::NumberPrimitiveValue(value) => {
                                                    let value = *value;
                                                    self::accept_java_script_call_number(program.clone(), machine.clone(), destination, realm.clone(), self::finish_java_script_number_primitive(&(operation), value.clone()))
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
pub fn resume_java_script_call_number(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, state: self::JavaScriptNumberCallState, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            match program.clone() {
                self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, _, shared_constructor_field2) => {
                    let shared_constructor_field2 = *shared_constructor_field2;
                    {
                        let frame = self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index(((shared_constructor_field4.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                        } else {
                            {
                                let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1581, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1581.clone()
                                    }
                                })));
                                if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                } else {
                                    {
                                        let operation = self::read_java_script_number_call_source_operation(fn_.clone().expect("a proven present value"), frame.clone().expect("a proven present value"), destination, realm.clone(), shared_constructor_field2.clone(), state.clone());
                                        if (operation == (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                        } else {
                                            match state.clone() {
                                                self::JavaScriptNumberCallState::NumberProtocolState(state_state) => {
                                                    let state_state = *state_state;
                                                    self::accept_java_script_call_number(program.clone(), machine.clone(), destination, realm.clone(), self::resume_java_script_number_protocol(shared_constructor_field3.clone(), state_state.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone())), &(operation)))
                                                }
                                                self::JavaScriptNumberCallState::NumberPrimitiveValue(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 97, 110, 110, 111, 116, 32, 104, 97, 118, 101, 32, 97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108]))
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
pub fn return_java_script_call_value(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frames = self::ml_array::slice(&shared_constructor_field2, 0f64, (-1f64));
                {
                    let parent = self::ml_array::optional_at(&frames, self::ml_array::number_index(((frames.len() as f64) - 1f64)));
                    if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::JavaScriptCallMachineTransition::MachineCompleted(Box::new(shared_constructor_field1.clone()), Box::new(value.clone()))
                    } else {
                        {
                            let current = self::JavaScriptCallMachine::CallMachine(Box::new(shared_constructor_field1.clone()), frames.clone());
                            {
                                let pending = match parent.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1592) => {
                                        let source_ml_source2_data_property_value1592 = *source_ml_source2_data_property_value1592;
                                        source_ml_source2_data_property_value1592.clone()
                                    }
                                };
                                match pending.clone() {
                                    self::JavaScriptCallPending::CallIdle => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                    }
                                    self::JavaScriptCallPending::WaitCall(destination) => {
                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination, value.clone()))))
                                    }
                                    self::JavaScriptCallPending::WaitGet(destination_2) => {
                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination_2, value.clone()))))
                                    }
                                    self::JavaScriptCallPending::WaitSet(destination_3) => {
                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination_3, self::JavaScriptSemanticValue::Boolean(true)))))
                                    }
                                    self::JavaScriptCallPending::WaitKey(destination_4, realm, state) => {
                                        let state = *state;
                                        self::accept_java_script_call_key(program.clone(), current.clone(), destination_4, realm.clone(), self::resume_java_script_property_key_conversion(shared_constructor_field1.clone(), state.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone()))))
                                    }
                                    self::JavaScriptCallPending::WaitDefinition(destination_5, realm_2, state_2) => {
                                        let state_2 = *state_2;
                                        self::accept_java_script_call_definition(program.clone(), current.clone(), destination_5, realm_2.clone(), self::resume_java_script_property_definition(shared_constructor_field1.clone(), state_2.clone(), self::JavaScriptSemanticCompletion::NormalCompletion(Box::new(value.clone()))))
                                    }
                                    self::JavaScriptCallPending::WaitNumber(destination_6, realm_3, state_3) => {
                                        let state_3 = *state_3;
                                        self::resume_java_script_call_number(program.clone(), current.clone(), destination_6, realm_3.clone(), state_3.clone(), value.clone())
                                    }
                                    self::JavaScriptCallPending::WaitError(_, _, _) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
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
pub fn execute_java_script_ordinary_call_property(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, frame: self::JavaScriptCallFrame, instruction: self::JavaScriptCallInstruction) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, _, _, _) => {
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
                    let shared_constructor_field1 = *shared_constructor_field1;
                    {
                        let next = {
                            let source_ml_source2_nominal_spread1604 = frame.clone();
                            {
                                let source_ml_source2_nominal_field1605 = match source_ml_source2_nominal_spread1604.clone() {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1606, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1606.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1607 = match source_ml_source2_nominal_spread1604.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1608, _, _, _, _) => {
                                            source_ml_source2_data_property_value1608
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1609 = match source_ml_source2_nominal_spread1604.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1610, _, _, _) => {
                                                source_ml_source2_data_property_value1610
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1611 = match source_ml_source2_nominal_spread1604.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1612, _, _) => {
                                                    source_ml_source2_data_property_value1612
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1613 = match source_ml_source2_nominal_spread1604.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1614, _) => {
                                                        source_ml_source2_data_property_value1614
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1615 = match source_ml_source2_nominal_spread1604.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1616) => {
                                                            let source_ml_source2_data_property_value1616 = *source_ml_source2_data_property_value1616;
                                                            source_ml_source2_data_property_value1616.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_override1617 = (shared_constructor_field4 + 1f64);
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1605.clone(), source_ml_source2_nominal_override1617, source_ml_source2_nominal_field1609, source_ml_source2_nominal_field1611, source_ml_source2_nominal_field1613, Box::new(source_ml_source2_nominal_field1615.clone()))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        {
                            let registers = shared_constructor_field5;
                            match instruction.clone() {
                                self::JavaScriptCallInstruction::Constant(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Copy(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Argument(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Jump(_) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Branch(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Return(_) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Throw(_) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::PushHandler(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::PopHandler => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::NumberBinary(_, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::RequireWrite(instruction_success) => {
                                    {
                                        let success = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_success))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                        {
                                            let owner = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1671, _, _) => source_ml_source2_data_property_value1671.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), shared_constructor_field3.clone())));
                                            if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                            } else {
                                                match success.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(value) => {
                                                        {
                                                            let current = self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, next.clone());
                                                            if value {
                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(current.clone()))
                                                            } else {
                                                                self::request_java_script_call_error(current.clone(), match owner.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1684, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value1684.clone()
                                                                    }
                                                                }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100])))
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Number(_) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                    }
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                    }
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptCallInstruction::Primitive(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::OrdinaryGet(destination, instruction_object, key, receiver) => {
                                    {
                                        let object = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_object))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                        match object.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                            }
                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                            }
                                            self::JavaScriptSemanticValue::Number(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                            }
                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                            }
                                            self::JavaScriptSemanticValue::Text(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                            }
                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                            }
                                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                                {
                                                    let plan = self::plan_java_script_ordinary_get(shared_constructor_field1.clone(), identity.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                    if (((plan.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), (plan.clone()).3)
                                                    } else {
                                                        if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, self::write_java_script_call_register(next.clone(), destination, (plan.clone()).5))))
                                                        } else {
                                                            if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, {
                                                                    let source_ml_source2_nominal_spread1710 = next.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field1711 = match source_ml_source2_nominal_spread1710.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1712, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value1712.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field1713 = match source_ml_source2_nominal_spread1710.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1714, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1714
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1715 = match source_ml_source2_nominal_spread1710.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1716, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1716
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1717 = match source_ml_source2_nominal_spread1710.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1718, _, _) => {
                                                                                            source_ml_source2_data_property_value1718
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1719 = match source_ml_source2_nominal_spread1710.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1720, _) => {
                                                                                                source_ml_source2_data_property_value1720
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1721 = match source_ml_source2_nominal_spread1710.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1722) => {
                                                                                                    let source_ml_source2_data_property_value1722 = *source_ml_source2_data_property_value1722;
                                                                                                    source_ml_source2_data_property_value1722.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_override1723 = self::JavaScriptCallPending::WaitGet(destination);
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1711.clone(), source_ml_source2_nominal_field1713, source_ml_source2_nominal_field1715, source_ml_source2_nominal_field1717, source_ml_source2_nominal_field1719, Box::new(source_ml_source2_nominal_override1723.clone()))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }), (plan.clone()).2, (plan.clone()).4, (plan.clone()).1)
                                                            } else {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114]))
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptCallInstruction::OrdinarySet(destination_2, instruction_object_2, key_2, value_2, receiver_2) => {
                                    {
                                        let object_2 = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_object_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                        match object_2.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                            }
                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                            }
                                            self::JavaScriptSemanticValue::Number(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                            }
                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                            }
                                            self::JavaScriptSemanticValue::Text(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                            }
                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                            }
                                            self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                                {
                                                    let plan_2 = self::plan_java_script_ordinary_set(shared_constructor_field1.clone(), identity_2.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(value_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                    if (((plan_2.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), (plan_2.clone()).3)
                                                    } else {
                                                        if (((plan_2.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, {
                                                                let source_ml_source2_nominal_spread1742 = next.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field1743 = match source_ml_source2_nominal_spread1742.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1744, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value1744.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field1745 = match source_ml_source2_nominal_spread1742.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1746, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value1746
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field1747 = match source_ml_source2_nominal_spread1742.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1748, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1748
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1749 = match source_ml_source2_nominal_spread1742.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1750, _, _) => {
                                                                                        source_ml_source2_data_property_value1750
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1751 = match source_ml_source2_nominal_spread1742.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1752, _) => {
                                                                                            source_ml_source2_data_property_value1752
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1753 = match source_ml_source2_nominal_spread1742.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1754) => {
                                                                                                let source_ml_source2_data_property_value1754 = *source_ml_source2_data_property_value1754;
                                                                                                source_ml_source2_data_property_value1754.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override1755 = self::JavaScriptCallPending::WaitSet(destination_2);
                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1743.clone(), source_ml_source2_nominal_field1745, source_ml_source2_nominal_field1747, source_ml_source2_nominal_field1749, source_ml_source2_nominal_field1751, Box::new(source_ml_source2_nominal_override1755.clone()))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }), (plan_2.clone()).2, (plan_2.clone()).5, (plan_2.clone()).1)
                                                        } else {
                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame((plan_2.clone()).4, shared_constructor_field2, self::write_java_script_call_register(next.clone(), destination_2, self::JavaScriptSemanticValue::Boolean((plan_2.clone()).6)))))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptCallInstruction::PropertyKey(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
