// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptNumberCallState;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryPushJavaScriptCallLoop153Result;
use super::create_java_script_call_frame;
use super::find_java_script_call_function;
use super::finish_java_script_scalar_primitive;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_number_call_source_operation;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::resume_java_script_scalar_protocol;
use super::source_ml_source1_temporary_push_java_script_call_loop153;
use super::step_java_script_scalar_protocol;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

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
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1501, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1501.clone()
                                    }
                                })));
                                if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field2.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                } else {
                                    match callee.clone() {
                                        self::JavaScriptSemanticValue::Undefined => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1508, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1508.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Null => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1509, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1509.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1510, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1510.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Number(_) => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1511, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1511.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Integer(_) => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1512, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1512.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Text(_) => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1513, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1513.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1514, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1514.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Object(identity, callable) => {
                                            if !callable {
                                                self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1515, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value1515.clone()
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
                                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value1518, _, _) => {
                                                                    source_ml_source2_data_property_value1518
                                                                }
                                                            };
                                                            {
                                                                let argument_slots = (arguments_.len() as f64);
                                                                {
                                                                    let source_ml_source1_temporary_values152 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source1_temporary_index152 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let source_ml_source2_s1520 = self::source_ml_source1_temporary_push_java_script_call_loop153(slots, argument_slots, source_ml_source1_temporary_values152, source_ml_source1_temporary_index152.clone());
                                                                            match source_ml_source2_s1520.clone() {
                                                                                self::MlSource1TemporaryPushJavaScriptCallLoop153Result::MlSource1TemporaryPushJavaScriptCallLoop153Done(slots_2, argument_slots_2) => {
                                                                                    if (((((shared_constructor_field3.len() as f64)) >= (1024f64)) || (slots_2 > (1048576f64))) || (argument_slots_2 > (1048576f64))) {
                                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                                    } else {
                                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(shared_constructor_field2.clone()), {
                                                                                            let source_ml_source2_concat_value1525 = shared_constructor_field3;
                                                                                            {
                                                                                                let source_ml_source2_concat_value1526 = vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_)];
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), source_ml_source2_concat_value1525), source_ml_source2_concat_value1526)
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
                                                            let source_ml_source2_nominal_override1545 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state.clone()));
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
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCall(state_2, callee, receiver, arguments) => {
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
                                                            let source_ml_source2_nominal_override1561 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state_2.clone()));
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
                                let source_ml_source2_nominal_spread1571 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1572 = match source_ml_source2_nominal_spread1571.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1573, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1573.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1574 = match source_ml_source2_nominal_spread1571.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1575, _, _, _, _) => {
                                                source_ml_source2_data_property_value1575
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1576 = match source_ml_source2_nominal_spread1571.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1577, _, _, _) => {
                                                    source_ml_source2_data_property_value1577
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1578 = match source_ml_source2_nominal_spread1571.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1579, _, _) => {
                                                        source_ml_source2_data_property_value1579
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1580 = match source_ml_source2_nominal_spread1571.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1581, _) => {
                                                            source_ml_source2_data_property_value1581
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1582 = match source_ml_source2_nominal_spread1571.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1583) => {
                                                                let source_ml_source2_data_property_value1583 = *source_ml_source2_data_property_value1583;
                                                                source_ml_source2_data_property_value1583.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1584 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state.clone()));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1572.clone(), source_ml_source2_nominal_field1574, source_ml_source2_nominal_field1576, source_ml_source2_nominal_field1578, source_ml_source2_nominal_field1580, Box::new(source_ml_source2_nominal_override1584.clone()))
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
                                let source_ml_source2_nominal_spread1587 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1588 = match source_ml_source2_nominal_spread1587.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1589, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1589.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1590 = match source_ml_source2_nominal_spread1587.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1591, _, _, _, _) => {
                                                source_ml_source2_data_property_value1591
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1592 = match source_ml_source2_nominal_spread1587.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1593, _, _, _) => {
                                                    source_ml_source2_data_property_value1593
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1594 = match source_ml_source2_nominal_spread1587.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1595, _, _) => {
                                                        source_ml_source2_data_property_value1595
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1596 = match source_ml_source2_nominal_spread1587.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1597, _) => {
                                                            source_ml_source2_data_property_value1597
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1598 = match source_ml_source2_nominal_spread1587.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1599) => {
                                                                let source_ml_source2_data_property_value1599 = *source_ml_source2_data_property_value1599;
                                                                source_ml_source2_data_property_value1599.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1600 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state_2.clone()));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1588.clone(), source_ml_source2_nominal_field1590, source_ml_source2_nominal_field1592, source_ml_source2_nominal_field1594, source_ml_source2_nominal_field1596, Box::new(source_ml_source2_nominal_override1600.clone()))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_number(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptScalarConversionTransition) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(((shared_constructor_field2.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                        self::JavaScriptScalarConversionTransition::ScalarContinuation(state) => {
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, {
                                let source_ml_source2_nominal_spread1608 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1609 = match source_ml_source2_nominal_spread1608.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1610, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1610.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1611 = match source_ml_source2_nominal_spread1608.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1612, _, _, _, _) => {
                                                source_ml_source2_data_property_value1612
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1613 = match source_ml_source2_nominal_spread1608.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1614, _, _, _) => {
                                                    source_ml_source2_data_property_value1614
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1615 = match source_ml_source2_nominal_spread1608.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1616, _, _) => {
                                                        source_ml_source2_data_property_value1616
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1617 = match source_ml_source2_nominal_spread1608.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1618, _) => {
                                                            source_ml_source2_data_property_value1618
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1619 = match source_ml_source2_nominal_spread1608.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1620) => {
                                                                let source_ml_source2_data_property_value1620 = *source_ml_source2_data_property_value1620;
                                                                source_ml_source2_data_property_value1620.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1621 = self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberProtocolState(Box::new(state.clone()))));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1609.clone(), source_ml_source2_nominal_field1611, source_ml_source2_nominal_field1613, source_ml_source2_nominal_field1615, source_ml_source2_nominal_field1617, Box::new(source_ml_source2_nominal_override1621.clone()))
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
                                let source_ml_source2_nominal_spread1624 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1625 = match source_ml_source2_nominal_spread1624.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1626, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1626.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1627 = match source_ml_source2_nominal_spread1624.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1628, _, _, _, _) => {
                                                source_ml_source2_data_property_value1628
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1629 = match source_ml_source2_nominal_spread1624.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1630, _, _, _) => {
                                                    source_ml_source2_data_property_value1630
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1631 = match source_ml_source2_nominal_spread1624.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1632, _, _) => {
                                                        source_ml_source2_data_property_value1632
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1633 = match source_ml_source2_nominal_spread1624.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1634, _) => {
                                                            source_ml_source2_data_property_value1634
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1635 = match source_ml_source2_nominal_spread1624.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1636) => {
                                                                let source_ml_source2_data_property_value1636 = *source_ml_source2_data_property_value1636;
                                                                source_ml_source2_data_property_value1636.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1637 = self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberProtocolState(Box::new(state_2.clone()))));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1625.clone(), source_ml_source2_nominal_field1627, source_ml_source2_nominal_field1629, source_ml_source2_nominal_field1631, source_ml_source2_nominal_field1633, Box::new(source_ml_source2_nominal_override1637.clone()))
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
                                let source_ml_source2_nominal_spread1643 = frame.clone().expect("a proven present value");
                                {
                                    let source_ml_source2_nominal_field1644 = match source_ml_source2_nominal_spread1643.clone() {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1645, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1645.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1646 = match source_ml_source2_nominal_spread1643.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1647, _, _, _, _) => {
                                                source_ml_source2_data_property_value1647
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1648 = match source_ml_source2_nominal_spread1643.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1649, _, _, _) => {
                                                    source_ml_source2_data_property_value1649
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1650 = match source_ml_source2_nominal_spread1643.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1651, _, _) => {
                                                        source_ml_source2_data_property_value1651
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1652 = match source_ml_source2_nominal_spread1643.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1653, _) => {
                                                            source_ml_source2_data_property_value1653
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1654 = match source_ml_source2_nominal_spread1643.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1655) => {
                                                                let source_ml_source2_data_property_value1655 = *source_ml_source2_data_property_value1655;
                                                                source_ml_source2_data_property_value1655.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override1656 = self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberPrimitiveValue(Box::new(value_3.clone()))));
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1644.clone(), source_ml_source2_nominal_field1646, source_ml_source2_nominal_field1648, source_ml_source2_nominal_field1650, source_ml_source2_nominal_field1652, Box::new(source_ml_source2_nominal_override1656.clone()))
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
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1661, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1661.clone()
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
                                                    self::accept_java_script_call_number(program.clone(), machine.clone(), destination, realm.clone(), self::step_java_script_scalar_protocol(shared_constructor_field3.clone(), state_state.clone(), &(operation)))
                                                }
                                                self::JavaScriptNumberCallState::NumberPrimitiveValue(value) => {
                                                    let value = *value;
                                                    self::accept_java_script_call_number(program.clone(), machine.clone(), destination, realm.clone(), self::finish_java_script_scalar_primitive(&(operation), value.clone()))
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
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1670, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1670.clone()
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
                                                    self::accept_java_script_call_number(program.clone(), machine.clone(), destination, realm.clone(), self::resume_java_script_scalar_protocol(shared_constructor_field3.clone(), state_state.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone())), &(operation)))
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
