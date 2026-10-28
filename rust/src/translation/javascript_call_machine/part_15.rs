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
                                let source_ml_source2_nominal_spread1532 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1533 = match source_ml_source2_nominal_spread1532.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1534, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1534.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1535 = match source_ml_source2_nominal_spread1532.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1536, _, _, _, _) => {
                                                source_ml_source2_data_property_value1536
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1537 = match source_ml_source2_nominal_spread1532.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1538, _, _, _) => {
                                                    source_ml_source2_data_property_value1538
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1539 = match source_ml_source2_nominal_spread1532.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1540, _, _) => {
                                                        source_ml_source2_data_property_value1540
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1541 = match source_ml_source2_nominal_spread1532.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1542, _) => {
                                                            source_ml_source2_data_property_value1542
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1543 = match source_ml_source2_nominal_spread1532.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1544) => {
                                                                let source_ml_source2_data_property_value1544 = *source_ml_source2_data_property_value1544;
                                                                source_ml_source2_data_property_value1544.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1545 = self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberProtocolState(Box::new(state.clone()))));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1533.clone(), source_ml_source2_nominal_field1535, source_ml_source2_nominal_field1537, source_ml_source2_nominal_field1539, source_ml_source2_nominal_field1541, Box::new(source_ml_source2_nominal_override1545.clone()))
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
                                let source_ml_source2_nominal_spread1548 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1549 = match source_ml_source2_nominal_spread1548.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1550, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1550.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1551 = match source_ml_source2_nominal_spread1548.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1552, _, _, _, _) => {
                                                source_ml_source2_data_property_value1552
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1553 = match source_ml_source2_nominal_spread1548.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1554, _, _, _) => {
                                                    source_ml_source2_data_property_value1554
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1555 = match source_ml_source2_nominal_spread1548.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1556, _, _) => {
                                                        source_ml_source2_data_property_value1556
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1557 = match source_ml_source2_nominal_spread1548.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1558, _) => {
                                                            source_ml_source2_data_property_value1558
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1559 = match source_ml_source2_nominal_spread1548.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1560) => {
                                                                let source_ml_source2_data_property_value1560 = *source_ml_source2_data_property_value1560;
                                                                source_ml_source2_data_property_value1560.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1561 = self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberProtocolState(Box::new(state_2.clone()))));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1549.clone(), source_ml_source2_nominal_field1551, source_ml_source2_nominal_field1553, source_ml_source2_nominal_field1555, source_ml_source2_nominal_field1557, Box::new(source_ml_source2_nominal_override1561.clone()))
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
                                let source_ml_source2_nominal_spread1567 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1568 = match source_ml_source2_nominal_spread1567.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1569, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1569.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1570 = match source_ml_source2_nominal_spread1567.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1571, _, _, _, _) => {
                                                source_ml_source2_data_property_value1571
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1572 = match source_ml_source2_nominal_spread1567.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1573, _, _, _) => {
                                                    source_ml_source2_data_property_value1573
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1574 = match source_ml_source2_nominal_spread1567.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1575, _, _) => {
                                                        source_ml_source2_data_property_value1575
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1576 = match source_ml_source2_nominal_spread1567.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1577, _) => {
                                                            source_ml_source2_data_property_value1577
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1578 = match source_ml_source2_nominal_spread1567.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1579) => {
                                                                let source_ml_source2_data_property_value1579 = *source_ml_source2_data_property_value1579;
                                                                source_ml_source2_data_property_value1579.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1580 = self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberPrimitiveValue(Box::new(value_3.clone()))));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1568.clone(), source_ml_source2_nominal_field1570, source_ml_source2_nominal_field1572, source_ml_source2_nominal_field1574, source_ml_source2_nominal_field1576, Box::new(source_ml_source2_nominal_override1580.clone()))
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
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1585, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1585.clone()
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
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1594, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1594.clone()
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
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1605) => {
                                        let source_ml_source2_data_property_value1605 = *source_ml_source2_data_property_value1605;
                                        source_ml_source2_data_property_value1605.clone()
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
                            let source_ml_source2_nominal_spread1617 = frame.clone();
                            {
                                let source_ml_source2_nominal_field1618 = match source_ml_source2_nominal_spread1617.clone() {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1619, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1619.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1620 = match source_ml_source2_nominal_spread1617.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1621, _, _, _, _) => {
                                            source_ml_source2_data_property_value1621
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1622 = match source_ml_source2_nominal_spread1617.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1623, _, _, _) => {
                                                source_ml_source2_data_property_value1623
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1624 = match source_ml_source2_nominal_spread1617.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1625, _, _) => {
                                                    source_ml_source2_data_property_value1625
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1626 = match source_ml_source2_nominal_spread1617.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1627, _) => {
                                                        source_ml_source2_data_property_value1627
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1628 = match source_ml_source2_nominal_spread1617.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1629) => {
                                                            let source_ml_source2_data_property_value1629 = *source_ml_source2_data_property_value1629;
                                                            source_ml_source2_data_property_value1629.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_override1630 = (shared_constructor_field4 + 1f64);
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1618.clone(), source_ml_source2_nominal_override1630, source_ml_source2_nominal_field1622, source_ml_source2_nominal_field1624, source_ml_source2_nominal_field1626, Box::new(source_ml_source2_nominal_field1628.clone()))
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
                                            let owner = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1684, _, _) => source_ml_source2_data_property_value1684.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), shared_constructor_field3.clone())));
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
                                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1697, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value1697.clone()
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
                                                                    let source_ml_source2_nominal_spread1723 = next.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field1724 = match source_ml_source2_nominal_spread1723.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1725, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value1725.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field1726 = match source_ml_source2_nominal_spread1723.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1727, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1727
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1728 = match source_ml_source2_nominal_spread1723.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1729, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1729
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1730 = match source_ml_source2_nominal_spread1723.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1731, _, _) => {
                                                                                            source_ml_source2_data_property_value1731
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1732 = match source_ml_source2_nominal_spread1723.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1733, _) => {
                                                                                                source_ml_source2_data_property_value1733
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1734 = match source_ml_source2_nominal_spread1723.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1735) => {
                                                                                                    let source_ml_source2_data_property_value1735 = *source_ml_source2_data_property_value1735;
                                                                                                    source_ml_source2_data_property_value1735.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_override1736 = self::JavaScriptCallPending::WaitGet(destination);
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1724.clone(), source_ml_source2_nominal_field1726, source_ml_source2_nominal_field1728, source_ml_source2_nominal_field1730, source_ml_source2_nominal_field1732, Box::new(source_ml_source2_nominal_override1736.clone()))
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
                                                                let source_ml_source2_nominal_spread1755 = next.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field1756 = match source_ml_source2_nominal_spread1755.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1757, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value1757.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field1758 = match source_ml_source2_nominal_spread1755.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1759, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value1759
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field1760 = match source_ml_source2_nominal_spread1755.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1761, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1761
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1762 = match source_ml_source2_nominal_spread1755.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1763, _, _) => {
                                                                                        source_ml_source2_data_property_value1763
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1764 = match source_ml_source2_nominal_spread1755.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1765, _) => {
                                                                                            source_ml_source2_data_property_value1765
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1766 = match source_ml_source2_nominal_spread1755.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1767) => {
                                                                                                let source_ml_source2_data_property_value1767 = *source_ml_source2_data_property_value1767;
                                                                                                source_ml_source2_data_property_value1767.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override1768 = self::JavaScriptCallPending::WaitSet(destination_2);
                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1756.clone(), source_ml_source2_nominal_field1758, source_ml_source2_nominal_field1760, source_ml_source2_nominal_field1762, source_ml_source2_nominal_field1764, Box::new(source_ml_source2_nominal_override1768.clone()))
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
