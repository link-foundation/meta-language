// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::JavaScriptTextCallState;
use super::accept_java_script_call_definition;
use super::accept_java_script_call_key;
use super::find_java_script_call_function;
use super::finish_java_script_scalar_primitive;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::plan_java_script_ordinary_set;
use super::push_java_script_call;
use super::read_java_script_text_call_source_operation;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::resume_java_script_call_number;
use super::resume_java_script_property_definition;
use super::resume_java_script_property_key_conversion;
use super::resume_java_script_scalar_protocol;
use super::step_java_script_scalar_protocol;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_text(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptScalarConversionTransition) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(((shared_constructor_field2.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                        self::JavaScriptScalarConversionTransition::ScalarContinuation(state) => {
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, {
                                let source_ml_source2_nominal_spread1682 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1683 = match source_ml_source2_nominal_spread1682.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1684, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1684.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1685 = match source_ml_source2_nominal_spread1682.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1686, _, _, _, _) => {
                                                source_ml_source2_data_property_value1686
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1687 = match source_ml_source2_nominal_spread1682.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1688, _, _, _) => {
                                                    source_ml_source2_data_property_value1688
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1689 = match source_ml_source2_nominal_spread1682.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1690, _, _) => {
                                                        source_ml_source2_data_property_value1690
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1691 = match source_ml_source2_nominal_spread1682.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1692, _) => {
                                                            source_ml_source2_data_property_value1692
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1693 = match source_ml_source2_nominal_spread1682.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1694) => {
                                                                let source_ml_source2_data_property_value1694 = *source_ml_source2_data_property_value1694;
                                                                source_ml_source2_data_property_value1694.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1695 = self::JavaScriptCallPending::WaitText(destination, realm.clone(), Box::new(self::JavaScriptTextCallState::TextProtocolState(Box::new(state.clone()))));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1683.clone(), source_ml_source2_nominal_field1685, source_ml_source2_nominal_field1687, source_ml_source2_nominal_field1689, source_ml_source2_nominal_field1691, Box::new(source_ml_source2_nominal_override1695.clone()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            })))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarCall(state_2, callee, receiver, arguments) => {
                            let state_2 = *state_2;
                            let callee = *callee;
                            let receiver = *receiver;
                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, {
                                let source_ml_source2_nominal_spread1698 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1699 = match source_ml_source2_nominal_spread1698.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1700, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1700.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1701 = match source_ml_source2_nominal_spread1698.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1702, _, _, _, _) => {
                                                source_ml_source2_data_property_value1702
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1703 = match source_ml_source2_nominal_spread1698.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1704, _, _, _) => {
                                                    source_ml_source2_data_property_value1704
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1705 = match source_ml_source2_nominal_spread1698.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1706, _, _) => {
                                                        source_ml_source2_data_property_value1706
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1707 = match source_ml_source2_nominal_spread1698.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1708, _) => {
                                                            source_ml_source2_data_property_value1708
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1709 = match source_ml_source2_nominal_spread1698.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1710) => {
                                                                let source_ml_source2_data_property_value1710 = *source_ml_source2_data_property_value1710;
                                                                source_ml_source2_data_property_value1710.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1711 = self::JavaScriptCallPending::WaitText(destination, realm.clone(), Box::new(self::JavaScriptTextCallState::TextProtocolState(Box::new(state_2.clone()))));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1699.clone(), source_ml_source2_nominal_field1701, source_ml_source2_nominal_field1703, source_ml_source2_nominal_field1705, source_ml_source2_nominal_field1707, Box::new(source_ml_source2_nominal_override1711.clone()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }), callee.clone(), receiver.clone(), arguments)
                        }
                        self::JavaScriptScalarConversionTransition::ScalarCompleted(value) => {
                            let value = *value;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarThrow(value_2) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(machine.clone(), value_2.clone())
                        }
                        self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class, reason) => {
                            self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), reason_2.clone())
                        }
                        self::JavaScriptScalarConversionTransition::ScalarResourceLimit(value_3, reason_3) => {
                            let value_3 = *value_3;
                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, {
                                let source_ml_source2_nominal_spread1717 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1718 = match source_ml_source2_nominal_spread1717.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1719, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1719.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1720 = match source_ml_source2_nominal_spread1717.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1721, _, _, _, _) => {
                                                source_ml_source2_data_property_value1721
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1722 = match source_ml_source2_nominal_spread1717.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1723, _, _, _) => {
                                                    source_ml_source2_data_property_value1723
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1724 = match source_ml_source2_nominal_spread1717.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1725, _, _) => {
                                                        source_ml_source2_data_property_value1725
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1726 = match source_ml_source2_nominal_spread1717.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1727, _) => {
                                                            source_ml_source2_data_property_value1727
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1728 = match source_ml_source2_nominal_spread1717.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1729) => {
                                                                let source_ml_source2_data_property_value1729 = *source_ml_source2_data_property_value1729;
                                                                source_ml_source2_data_property_value1729.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1730 = self::JavaScriptCallPending::WaitText(destination, realm.clone(), Box::new(self::JavaScriptTextCallState::TextPrimitiveValue(Box::new(value_3.clone()))));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1718.clone(), source_ml_source2_nominal_field1720, source_ml_source2_nominal_field1722, source_ml_source2_nominal_field1724, source_ml_source2_nominal_field1726, Box::new(source_ml_source2_nominal_override1730.clone()))
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
pub fn step_java_script_call_text(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, state: self::JavaScriptTextCallState) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            match program.clone() {
                self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, _, shared_constructor_field2) => {
                    let shared_constructor_field2 = *shared_constructor_field2;
                    {
                        let frame = self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index(((shared_constructor_field4.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                        } else {
                            {
                                let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1735, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1735.clone()
                                    }
                                })));
                                if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                } else {
                                    {
                                        let operation = self::read_java_script_text_call_source_operation(fn_.clone().expect("a proven present value"), frame.clone().expect("a proven present value"), destination, realm.clone(), shared_constructor_field2.clone(), state.clone());
                                        if (operation == (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                        } else {
                                            match state.clone() {
                                                self::JavaScriptTextCallState::TextProtocolState(state_state) => {
                                                    let state_state = *state_state;
                                                    self::accept_java_script_call_text(program.clone(), machine.clone(), destination, realm.clone(), self::step_java_script_scalar_protocol(shared_constructor_field3.clone(), state_state.clone(), &(operation)))
                                                }
                                                self::JavaScriptTextCallState::TextPrimitiveValue(value) => {
                                                    let value = *value;
                                                    self::accept_java_script_call_text(program.clone(), machine.clone(), destination, realm.clone(), self::finish_java_script_scalar_primitive(&(operation), value.clone()))
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
pub fn resume_java_script_call_text(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, state: self::JavaScriptTextCallState, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            match program.clone() {
                self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, _, shared_constructor_field2) => {
                    let shared_constructor_field2 = *shared_constructor_field2;
                    {
                        let frame = self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index(((shared_constructor_field4.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                        } else {
                            {
                                let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1744, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1744.clone()
                                    }
                                })));
                                if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                } else {
                                    {
                                        let operation = self::read_java_script_text_call_source_operation(fn_.clone().expect("a proven present value"), frame.clone().expect("a proven present value"), destination, realm.clone(), shared_constructor_field2.clone(), state.clone());
                                        if (operation == (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                        } else {
                                            match state.clone() {
                                                self::JavaScriptTextCallState::TextProtocolState(state_state) => {
                                                    let state_state = *state_state;
                                                    self::accept_java_script_call_text(program.clone(), machine.clone(), destination, realm.clone(), self::resume_java_script_scalar_protocol(shared_constructor_field3.clone(), state_state.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone())), &(operation)))
                                                }
                                                self::JavaScriptTextCallState::TextPrimitiveValue(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 115, 116, 114, 105, 110, 103, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 97, 110, 110, 111, 116, 32, 104, 97, 118, 101, 32, 97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108]))
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
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1755) => {
                                        let source_ml_source2_data_property_value1755 = *source_ml_source2_data_property_value1755;
                                        source_ml_source2_data_property_value1755.clone()
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
                                    self::JavaScriptCallPending::WaitText(destination_7, realm_4, state_4) => {
                                        let state_4 = *state_4;
                                        self::resume_java_script_call_text(program.clone(), current.clone(), destination_7, realm_4.clone(), state_4.clone(), value.clone())
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
        self::JavaScriptCallFrame::CallFrame(shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8) => {
            let shared_constructor_field8 = *shared_constructor_field8;
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
                    let shared_constructor_field1 = *shared_constructor_field1;
                    {
                        let next = {
                            let source_ml_source2_nominal_field1768 = shared_constructor_field3.clone();
                            {
                                let source_ml_source2_nominal_field1770 = shared_constructor_field4;
                                {
                                    let source_ml_source2_nominal_field1772 = shared_constructor_field5.clone();
                                    {
                                        let source_ml_source2_nominal_field1774 = shared_constructor_field6;
                                        {
                                            let source_ml_source2_nominal_field1776 = shared_constructor_field7;
                                            {
                                                let source_ml_source2_nominal_field1778 = shared_constructor_field8.clone();
                                                {
                                                    let source_ml_source2_nominal_override1780 = (shared_constructor_field4 + 1f64);
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1768.clone(), source_ml_source2_nominal_override1780, source_ml_source2_nominal_field1772, source_ml_source2_nominal_field1774, source_ml_source2_nominal_field1776, Box::new(source_ml_source2_nominal_field1778.clone()))
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
                                            let owner = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1834, _, _) => source_ml_source2_data_property_value1834.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), shared_constructor_field3.clone())));
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
                                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1847, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value1847.clone()
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
                                                                    let source_ml_source2_nominal_field1874 = match next.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1875, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value1875.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field1876 = match next.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1877, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value1877
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field1878 = match next.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1879, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1879
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1880 = match next.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1881, _, _) => {
                                                                                        source_ml_source2_data_property_value1881
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1882 = match next.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1883, _) => {
                                                                                            source_ml_source2_data_property_value1883
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1884 = match next.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1885) => {
                                                                                                let source_ml_source2_data_property_value1885 = *source_ml_source2_data_property_value1885;
                                                                                                source_ml_source2_data_property_value1885.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override1886 = self::JavaScriptCallPending::WaitGet(destination);
                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1874.clone(), source_ml_source2_nominal_field1876, source_ml_source2_nominal_field1878, source_ml_source2_nominal_field1880, source_ml_source2_nominal_field1882, Box::new(source_ml_source2_nominal_override1886.clone()))
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
                                                                let source_ml_source2_nominal_field1906 = match next.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1907, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value1907.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field1908 = match next.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1909, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value1909
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field1910 = match next.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1911, _, _, _) => {
                                                                                source_ml_source2_data_property_value1911
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field1912 = match next.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1913, _, _) => {
                                                                                    source_ml_source2_data_property_value1913
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1914 = match next.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1915, _) => {
                                                                                        source_ml_source2_data_property_value1915
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1916 = match next.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1917) => {
                                                                                            let source_ml_source2_data_property_value1917 = *source_ml_source2_data_property_value1917;
                                                                                            source_ml_source2_data_property_value1917.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override1918 = self::JavaScriptCallPending::WaitSet(destination_2);
                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1906.clone(), source_ml_source2_nominal_field1908, source_ml_source2_nominal_field1910, source_ml_source2_nominal_field1912, source_ml_source2_nominal_field1914, Box::new(source_ml_source2_nominal_override1918.clone()))
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
