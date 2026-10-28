// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
use super::accept_java_script_call_definition;
use super::accept_java_script_call_key;
use super::accept_java_script_call_number;
use super::begin_java_script_number_protocol;
use super::begin_java_script_property_definition;
use super::begin_java_script_property_key_conversion;
use super::convert_java_script_semantic_boolean;
use super::execute_java_script_call_primitive;
use super::execute_java_script_ordinary_call_property;
use super::find_java_script_call_function;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_string;
use super::push_java_script_call;
use super::read_java_script_call_machine_error;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::return_java_script_call_value;
use super::step_java_script_call_number;
use super::step_java_script_property_definition;
use super::step_java_script_property_key_conversion;
use super::test_java_script_call_strict_equal;
use super::test_java_script_semantic_same_value;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_primitive_instruction(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, frame: self::JavaScriptCallFrame, instruction: self::JavaScriptCallInstruction) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(shared_constructor_field5, shared_constructor_field6, _, _, _, _) => {
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(shared_constructor_field3, shared_constructor_field4) => {
                    let shared_constructor_field3 = *shared_constructor_field3;
                    match program.clone() {
                        self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, _, shared_constructor_field2) => {
                            let shared_constructor_field2 = *shared_constructor_field2;
                            match instruction.clone() {
                                self::JavaScriptCallInstruction::Constant(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Copy(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Argument(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Jump(_) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Branch(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Return(_) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Throw(_) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::PushHandler(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::PopHandler => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::NumberBinary(_, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::RequireWrite(_) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Primitive(destination, operation, arguments) => {
                                    {
                                        let arguments_ = { let ml_values6 = arguments; let ml_callback6 = { let frame = frame.clone(); std::rc::Rc::new(move |index: f64| -> self::JavaScriptSemanticValue { (self::ml_array::optional_at((match &frame { self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1832, _, _, _) => source_ml_source2_data_property_value1832.as_slice() }), self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values6.iter().cloned().enumerate().map(|(ml_index6, ml_value6)| ml_callback6(ml_value6)).collect::<Vec<_>>() };
                                        {
                                            let input = (self::ml_array::optional_at(&arguments_, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                            if (((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) && match input.clone() {
                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                    true
                                                }
                                                _ => {
                                                    false
                                                }
                                            }) {
                                                {
                                                    let owner = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), shared_constructor_field5.clone())));
                                                    if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                                    } else {
                                                        {
                                                            let current = self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                let source_ml_source2_nominal_spread1846 = frame.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field1847 = match source_ml_source2_nominal_spread1846.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1848, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value1848.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field1849 = match source_ml_source2_nominal_spread1846.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1850, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value1850
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field1851 = match source_ml_source2_nominal_spread1846.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1852, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1852
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1853 = match source_ml_source2_nominal_spread1846.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1854, _, _) => {
                                                                                        source_ml_source2_data_property_value1854
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1855 = match source_ml_source2_nominal_spread1846.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1856, _) => {
                                                                                            source_ml_source2_data_property_value1856
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1857 = match source_ml_source2_nominal_spread1846.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1858) => {
                                                                                                let source_ml_source2_data_property_value1858 = *source_ml_source2_data_property_value1858;
                                                                                                source_ml_source2_data_property_value1858.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override1859 = (shared_constructor_field6 + 1f64);
                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1847.clone(), source_ml_source2_nominal_override1859, source_ml_source2_nominal_field1851, source_ml_source2_nominal_field1853, source_ml_source2_nominal_field1855, Box::new(source_ml_source2_nominal_field1857.clone()))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            });
                                                            self::accept_java_script_call_number(program.clone(), current.clone(), destination, match owner.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1860, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value1860.clone()
                                                                }
                                                            }, self::begin_java_script_number_protocol(shared_constructor_field3.clone(), input.clone(), shared_constructor_field2.clone(), &(operation)))
                                                        }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let result = self::execute_java_script_call_primitive(&(operation), arguments_);
                                                    match result.clone() {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value) => {
                                                            let value = *value;
                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register({
                                                                let source_ml_source2_nominal_spread1866 = frame.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field1867 = match source_ml_source2_nominal_spread1866.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1868, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value1868.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field1869 = match source_ml_source2_nominal_spread1866.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1870, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value1870
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field1871 = match source_ml_source2_nominal_spread1866.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1872, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1872
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1873 = match source_ml_source2_nominal_spread1866.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1874, _, _) => {
                                                                                        source_ml_source2_data_property_value1874
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1875 = match source_ml_source2_nominal_spread1866.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1876, _) => {
                                                                                            source_ml_source2_data_property_value1876
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1877 = match source_ml_source2_nominal_spread1866.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1878) => {
                                                                                                let source_ml_source2_data_property_value1878 = *source_ml_source2_data_property_value1878;
                                                                                                source_ml_source2_data_property_value1878.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override1879 = (shared_constructor_field6 + 1f64);
                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1867.clone(), source_ml_source2_nominal_override1879, source_ml_source2_nominal_field1871, source_ml_source2_nominal_field1873, source_ml_source2_nominal_field1875, Box::new(source_ml_source2_nominal_field1877.clone()))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }, destination, value.clone()))))
                                                        }
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(reason) => {
                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), reason.clone())
                                                        }
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(error_class, reason_2) => {
                                                            {
                                                                let owner_2 = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), shared_constructor_field5.clone())));
                                                                if match (owner_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 102, 97, 105, 108, 117, 114, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                                                } else {
                                                                    self::request_java_script_call_error(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                        let source_ml_source2_nominal_spread1887 = frame.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field1888 = match source_ml_source2_nominal_spread1887.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1889, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1889.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1890 = match source_ml_source2_nominal_spread1887.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1891, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1891
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1892 = match source_ml_source2_nominal_spread1887.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1893, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1893
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1894 = match source_ml_source2_nominal_spread1887.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1895, _, _) => {
                                                                                                source_ml_source2_data_property_value1895
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1896 = match source_ml_source2_nominal_spread1887.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1897, _) => {
                                                                                                    source_ml_source2_data_property_value1897
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1898 = match source_ml_source2_nominal_spread1887.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1899) => {
                                                                                                        let source_ml_source2_data_property_value1899 = *source_ml_source2_data_property_value1899;
                                                                                                        source_ml_source2_data_property_value1899.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override1900 = (shared_constructor_field6 + 1f64);
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1888.clone(), source_ml_source2_nominal_override1900, source_ml_source2_nominal_field1892, source_ml_source2_nominal_field1894, source_ml_source2_nominal_field1896, Box::new(source_ml_source2_nominal_field1898.clone()))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }), match owner_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1901, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value1901.clone()
                                                                        }
                                                                    }, &(error_class), &(reason_2))
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(reason_3) => {
                                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_3.clone())
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::PropertyKey(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
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
pub fn step_java_script_call_machine(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            match program.clone() {
                self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, _, shared_constructor_field2) => {
                    let shared_constructor_field2 = *shared_constructor_field2;
                    {
                        let error = self::read_java_script_call_machine_error(program.clone(), machine.clone());
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), error.clone())
                        } else {
                            {
                                let frame = self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index(((shared_constructor_field4.len() as f64) - 1f64)));
                                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                                } else {
                                    {
                                        let pending = match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1910) => {
                                                let source_ml_source2_data_property_value1910 = *source_ml_source2_data_property_value1910;
                                                source_ml_source2_data_property_value1910.clone()
                                            }
                                        };
                                        match pending.clone() {
                                            self::JavaScriptCallPending::CallIdle => {
                                                {
                                                    let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1918, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value1918.clone()
                                                        }
                                                    })));
                                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                                    } else {
                                                        {
                                                            let instruction = self::ml_array::optional_at(&match fn_.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value1920) => {
                                                                    source_ml_source2_data_property_value1920
                                                                }
                                                            }, self::ml_array::number_index(match frame.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1921, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value1921
                                                                }
                                                            }));
                                                            if match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                self::return_java_script_call_value(program.clone(), machine.clone(), self::JavaScriptSemanticValue::Undefined)
                                                            } else {
                                                                {
                                                                    let next = {
                                                                        let source_ml_source2_nominal_spread1923 = frame.clone().expect("a proven present value");
                                                                        {
                                                                            let source_ml_source2_nominal_field1924 = match source_ml_source2_nominal_spread1923.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1925, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1925.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1926 = match source_ml_source2_nominal_spread1923.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1927, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1927
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1928 = match source_ml_source2_nominal_spread1923.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1929, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1929
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1930 = match source_ml_source2_nominal_spread1923.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1931, _, _) => {
                                                                                                source_ml_source2_data_property_value1931
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1932 = match source_ml_source2_nominal_spread1923.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1933, _) => {
                                                                                                    source_ml_source2_data_property_value1933
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1934 = match source_ml_source2_nominal_spread1923.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1935) => {
                                                                                                        let source_ml_source2_data_property_value1935 = *source_ml_source2_data_property_value1935;
                                                                                                        source_ml_source2_data_property_value1935.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override1936 = (match frame.clone().expect("a proven present value") {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1922, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value1922
                                                                                                        }
                                                                                                    } + 1f64);
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1924.clone(), source_ml_source2_nominal_override1936, source_ml_source2_nominal_field1928, source_ml_source2_nominal_field1930, source_ml_source2_nominal_field1932, Box::new(source_ml_source2_nominal_field1934.clone()))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    {
                                                                        let current = self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4.clone(), next.clone());
                                                                        {
                                                                            let registers = match frame.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1939, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1939
                                                                                }
                                                                            };
                                                                            match instruction.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallInstruction::Constant(destination, value) => {
                                                                                    let value = *value;
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register(next.clone(), destination, value.clone()))))
                                                                                }
                                                                                self::JavaScriptCallInstruction::Copy(destination_2, source) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register(next.clone(), destination_2, (self::ml_array::optional_at(&registers, self::ml_array::number_index(source))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                                }
                                                                                self::JavaScriptCallInstruction::Argument(destination_3, index) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register(next.clone(), destination_3, (self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1959, _, _) => {
                                                                                            source_ml_source2_data_property_value1959
                                                                                        }
                                                                                    }, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                                }
                                                                                self::JavaScriptCallInstruction::Jump(target) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                                        let source_ml_source2_nominal_spread1962 = next.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1963 = match source_ml_source2_nominal_spread1962.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1964, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value1964.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1965 = match source_ml_source2_nominal_spread1962.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1966, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value1966
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field1967 = match source_ml_source2_nominal_spread1962.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1968, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value1968
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field1969 = match source_ml_source2_nominal_spread1962.clone() {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1970, _, _) => {
                                                                                                                source_ml_source2_data_property_value1970
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field1971 = match source_ml_source2_nominal_spread1962.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1972, _) => {
                                                                                                                    source_ml_source2_data_property_value1972
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field1973 = match source_ml_source2_nominal_spread1962.clone() {
                                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1974) => {
                                                                                                                        let source_ml_source2_data_property_value1974 = *source_ml_source2_data_property_value1974;
                                                                                                                        source_ml_source2_data_property_value1974.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override1975 = target;
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1963.clone(), source_ml_source2_nominal_override1975, source_ml_source2_nominal_field1967, source_ml_source2_nominal_field1969, source_ml_source2_nominal_field1971, Box::new(source_ml_source2_nominal_field1973.clone()))
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    })))
                                                                                }
                                                                                self::JavaScriptCallInstruction::Branch(condition, yes, no) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                                        let source_ml_source2_nominal_spread1978 = next.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1979 = match source_ml_source2_nominal_spread1978.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1980, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value1980.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1981 = match source_ml_source2_nominal_spread1978.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1982, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value1982
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field1983 = match source_ml_source2_nominal_spread1978.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1984, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value1984
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field1985 = match source_ml_source2_nominal_spread1978.clone() {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1986, _, _) => {
                                                                                                                source_ml_source2_data_property_value1986
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field1987 = match source_ml_source2_nominal_spread1978.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1988, _) => {
                                                                                                                    source_ml_source2_data_property_value1988
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field1989 = match source_ml_source2_nominal_spread1978.clone() {
                                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1990) => {
                                                                                                                        let source_ml_source2_data_property_value1990 = *source_ml_source2_data_property_value1990;
                                                                                                                        source_ml_source2_data_property_value1990.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override1991 = if self::convert_java_script_semantic_boolean((self::ml_array::optional_at(&registers, self::ml_array::number_index(condition))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)) {
                                                                                                                        yes
                                                                                                                    } else {
                                                                                                                        no
                                                                                                                    };
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1979.clone(), source_ml_source2_nominal_override1991, source_ml_source2_nominal_field1983, source_ml_source2_nominal_field1985, source_ml_source2_nominal_field1987, Box::new(source_ml_source2_nominal_field1989.clone()))
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    })))
                                                                                }
                                                                                self::JavaScriptCallInstruction::Call(destination_4, callee, receiver, arguments) => {
                                                                                    {
                                                                                        let arguments_ = { let ml_values7 = arguments; let ml_callback7 = { let registers = registers.clone(); std::rc::Rc::new(move |index_2: f64| -> self::JavaScriptSemanticValue { (self::ml_array::optional_at(&registers, self::ml_array::number_index(index_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values7.iter().cloned().enumerate().map(|(ml_index7, ml_value7)| ml_callback7(ml_value7)).collect::<Vec<_>>() };
                                                                                        self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                                            let source_ml_source2_nominal_spread1994 = next.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1995 = match source_ml_source2_nominal_spread1994.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1996, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value1996.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field1997 = match source_ml_source2_nominal_spread1994.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1998, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value1998
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field1999 = match source_ml_source2_nominal_spread1994.clone() {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2000, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value2000
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2001 = match source_ml_source2_nominal_spread1994.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2002, _, _) => {
                                                                                                                    source_ml_source2_data_property_value2002
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field2003 = match source_ml_source2_nominal_spread1994.clone() {
                                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2004, _) => {
                                                                                                                        source_ml_source2_data_property_value2004
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field2005 = match source_ml_source2_nominal_spread1994.clone() {
                                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2006) => {
                                                                                                                            let source_ml_source2_data_property_value2006 = *source_ml_source2_data_property_value2006;
                                                                                                                            source_ml_source2_data_property_value2006.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override2007 = self::JavaScriptCallPending::WaitCall(destination_4);
                                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1995.clone(), source_ml_source2_nominal_field1997, source_ml_source2_nominal_field1999, source_ml_source2_nominal_field2001, source_ml_source2_nominal_field2003, Box::new(source_ml_source2_nominal_override2007.clone()))
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }), (self::ml_array::optional_at(&registers, self::ml_array::number_index(callee))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), arguments_)
                                                                                    }
                                                                                }
                                                                                self::JavaScriptCallInstruction::Return(source_2) => {
                                                                                    self::return_java_script_call_value(program.clone(), current.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(source_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                                                }
                                                                                self::JavaScriptCallInstruction::Throw(source_3) => {
                                                                                    self::throw_java_script_call_value(current.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(source_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                                                }
                                                                                self::JavaScriptCallInstruction::PushHandler(destination_5, target_2) => {
                                                                                    if ((((match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2009, _) => {
                                                                                            source_ml_source2_data_property_value2009
                                                                                        }
                                                                                    }).len() as f64)) >= (1024f64)) {
                                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![104, 97, 110, 100, 108, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                                    } else {
                                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                                            let source_ml_source2_nominal_spread2015 = next.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2016 = match source_ml_source2_nominal_spread2015.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2017, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value2017.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2018 = match source_ml_source2_nominal_spread2015.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2019, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value2019
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2020 = match source_ml_source2_nominal_spread2015.clone() {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2021, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value2021
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2022 = match source_ml_source2_nominal_spread2015.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2023, _, _) => {
                                                                                                                    source_ml_source2_data_property_value2023
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field2024 = match source_ml_source2_nominal_spread2015.clone() {
                                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2025, _) => {
                                                                                                                        source_ml_source2_data_property_value2025
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field2026 = match source_ml_source2_nominal_spread2015.clone() {
                                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2027) => {
                                                                                                                            let source_ml_source2_data_property_value2027 = *source_ml_source2_data_property_value2027;
                                                                                                                            source_ml_source2_data_property_value2027.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override2028 = {
                                                                                                                            let source_ml_source2_concat_value2013 = match frame.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2012, _) => {
                                                                                                                                    source_ml_source2_data_property_value2012
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_concat_value2014 = vec![self::JavaScriptCallHandler::CallHandler(target_2, destination_5)];
                                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallHandler>::new(), source_ml_source2_concat_value2013), source_ml_source2_concat_value2014)
                                                                                                                            }
                                                                                                                        };
                                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2016.clone(), source_ml_source2_nominal_field2018, source_ml_source2_nominal_field2020, source_ml_source2_nominal_field2022, source_ml_source2_nominal_override2028, Box::new(source_ml_source2_nominal_field2026.clone()))
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        })))
                                                                                    }
                                                                                }
                                                                                self::JavaScriptCallInstruction::PopHandler => {
                                                                                    if ((((match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2030, _) => {
                                                                                            source_ml_source2_data_property_value2030
                                                                                        }
                                                                                    }).len() as f64)) == (0f64)) {
                                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 104, 97, 110, 100, 108, 101, 114, 32, 112, 111, 112, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 104, 97, 110, 100, 108, 101, 114]))
                                                                                    } else {
                                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                                            let source_ml_source2_nominal_spread2036 = next.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2037 = match source_ml_source2_nominal_spread2036.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2038, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value2038.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2039 = match source_ml_source2_nominal_spread2036.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2040, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value2040
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2041 = match source_ml_source2_nominal_spread2036.clone() {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2042, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value2042
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2043 = match source_ml_source2_nominal_spread2036.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2044, _, _) => {
                                                                                                                    source_ml_source2_data_property_value2044
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field2045 = match source_ml_source2_nominal_spread2036.clone() {
                                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2046, _) => {
                                                                                                                        source_ml_source2_data_property_value2046
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field2047 = match source_ml_source2_nominal_spread2036.clone() {
                                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2048) => {
                                                                                                                            let source_ml_source2_data_property_value2048 = *source_ml_source2_data_property_value2048;
                                                                                                                            source_ml_source2_data_property_value2048.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override2049 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2035, _) => {
                                                                                                                                source_ml_source2_data_property_value2035
                                                                                                                            }
                                                                                                                        }, 0f64, (-1f64));
                                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2037.clone(), source_ml_source2_nominal_field2039, source_ml_source2_nominal_field2041, source_ml_source2_nominal_field2043, source_ml_source2_nominal_override2049, Box::new(source_ml_source2_nominal_field2047.clone()))
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        })))
                                                                                    }
                                                                                }
                                                                                self::JavaScriptCallInstruction::NumberBinary(destination_6, instruction_left, instruction_right, operator) => {
                                                                                    {
                                                                                        let left = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_left))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                                        {
                                                                                            let right = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_right))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                                            match left.clone() {
                                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Number(left_value) => {
                                                                                                    match right.clone() {
                                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Null => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Number(right_value) => {
                                                                                                            {
                                                                                                                let op = operator.clone();
                                                                                                                {
                                                                                                                    let first = left_value;
                                                                                                                    {
                                                                                                                        let second = right_value;
                                                                                                                        if ((op == (self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]))) || (op == (self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])))) {
                                                                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Boolean(if (op == (self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]))) {
                                                                                                                                (first < second)
                                                                                                                            } else {
                                                                                                                                (first == second)
                                                                                                                            })))))
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let value_2 = if (op == (self::ml_string::Text::from_units(vec![97, 100, 100]))) {
                                                                                                                                    (first + second)
                                                                                                                                } else {
                                                                                                                                    if (op == (self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116]))) {
                                                                                                                                        (first - second)
                                                                                                                                    } else {
                                                                                                                                        if (op == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121]))) {
                                                                                                                                            (first * second)
                                                                                                                                        } else {
                                                                                                                                            if (op == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101]))) {
                                                                                                                                                (first / second)
                                                                                                                                            } else {
                                                                                                                                                if (op == (self::ml_string::Text::from_units(vec![114, 101, 109, 97, 105, 110, 100, 101, 114]))) {
                                                                                                                                                    (first % second)
                                                                                                                                                } else {
                                                                                                                                                    self::ml_big_integer_conversion::power_java_script_number(first,second)
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Number(value_2)))))
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Text(_) => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::JavaScriptCallInstruction::SameValue(destination_7, left_2, right_2) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register(next.clone(), destination_7, self::JavaScriptSemanticValue::Boolean(self::test_java_script_semantic_same_value((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                                }
                                                                                self::JavaScriptCallInstruction::StrictEqual(destination_8, left_3, right_3) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register(next.clone(), destination_8, self::JavaScriptSemanticValue::Boolean(self::test_java_script_call_strict_equal((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                                }
                                                                                self::JavaScriptCallInstruction::RequireWrite(_) => {
                                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                                }
                                                                                self::JavaScriptCallInstruction::Primitive(_, _, _) => {
                                                                                    self::execute_java_script_call_primitive_instruction(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                                }
                                                                                self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                                }
                                                                                self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                                }
                                                                                self::JavaScriptCallInstruction::PropertyKey(destination_9, input, realm) => {
                                                                                    self::accept_java_script_call_key(program.clone(), current.clone(), destination_9, realm.clone(), self::begin_java_script_property_key_conversion(shared_constructor_field3.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(input))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field2.clone()))
                                                                                }
                                                                                self::JavaScriptCallInstruction::PropertyDefinition(destination_10, target_3, key, attributes, realm_2) => {
                                                                                    self::accept_java_script_call_definition(program.clone(), current.clone(), destination_10, realm_2.clone(), self::begin_java_script_property_definition(shared_constructor_field3.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(target_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(attributes))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field2.clone()))
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
                                            self::JavaScriptCallPending::WaitCall(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptCallPending::WaitGet(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptCallPending::WaitSet(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptCallPending::WaitKey(destination_11, realm_3, state) => {
                                                let state = *state;
                                                self::accept_java_script_call_key(program.clone(), machine.clone(), destination_11, realm_3.clone(), self::step_java_script_property_key_conversion(shared_constructor_field3.clone(), state.clone()))
                                            }
                                            self::JavaScriptCallPending::WaitDefinition(destination_12, realm_4, state_2) => {
                                                let state_2 = *state_2;
                                                self::accept_java_script_call_definition(program.clone(), machine.clone(), destination_12, realm_4.clone(), self::step_java_script_property_definition(shared_constructor_field3.clone(), state_2.clone()))
                                            }
                                            self::JavaScriptCallPending::WaitNumber(destination_13, realm_5, state_3) => {
                                                let state_3 = *state_3;
                                                self::step_java_script_call_number(program.clone(), machine.clone(), destination_13, realm_5.clone(), state_3.clone())
                                            }
                                            self::JavaScriptCallPending::WaitError(_, _, _) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
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
