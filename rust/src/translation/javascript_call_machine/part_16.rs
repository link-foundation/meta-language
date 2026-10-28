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
                                        let arguments_ = { let ml_values6 = arguments; let ml_callback6 = { let frame = frame.clone(); std::rc::Rc::new(move |index: f64| -> self::JavaScriptSemanticValue { (self::ml_array::optional_at((match &frame { self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1819, _, _, _) => source_ml_source2_data_property_value1819.as_slice() }), self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values6.iter().cloned().enumerate().map(|(ml_index6, ml_value6)| ml_callback6(ml_value6)).collect::<Vec<_>>() };
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
                                                                let source_ml_source2_nominal_spread1833 = frame.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field1834 = match source_ml_source2_nominal_spread1833.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1835, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value1835.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field1836 = match source_ml_source2_nominal_spread1833.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1837, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value1837
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field1838 = match source_ml_source2_nominal_spread1833.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1839, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1839
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1840 = match source_ml_source2_nominal_spread1833.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1841, _, _) => {
                                                                                        source_ml_source2_data_property_value1841
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1842 = match source_ml_source2_nominal_spread1833.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1843, _) => {
                                                                                            source_ml_source2_data_property_value1843
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1844 = match source_ml_source2_nominal_spread1833.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1845) => {
                                                                                                let source_ml_source2_data_property_value1845 = *source_ml_source2_data_property_value1845;
                                                                                                source_ml_source2_data_property_value1845.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override1846 = (shared_constructor_field6 + 1f64);
                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1834.clone(), source_ml_source2_nominal_override1846, source_ml_source2_nominal_field1838, source_ml_source2_nominal_field1840, source_ml_source2_nominal_field1842, Box::new(source_ml_source2_nominal_field1844.clone()))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            });
                                                            self::accept_java_script_call_number(program.clone(), current.clone(), destination, match owner.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1847, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value1847.clone()
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
                                                                let source_ml_source2_nominal_spread1853 = frame.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field1854 = match source_ml_source2_nominal_spread1853.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1855, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value1855.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field1856 = match source_ml_source2_nominal_spread1853.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1857, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value1857
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field1858 = match source_ml_source2_nominal_spread1853.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1859, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1859
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1860 = match source_ml_source2_nominal_spread1853.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1861, _, _) => {
                                                                                        source_ml_source2_data_property_value1861
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1862 = match source_ml_source2_nominal_spread1853.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1863, _) => {
                                                                                            source_ml_source2_data_property_value1863
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1864 = match source_ml_source2_nominal_spread1853.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1865) => {
                                                                                                let source_ml_source2_data_property_value1865 = *source_ml_source2_data_property_value1865;
                                                                                                source_ml_source2_data_property_value1865.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override1866 = (shared_constructor_field6 + 1f64);
                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1854.clone(), source_ml_source2_nominal_override1866, source_ml_source2_nominal_field1858, source_ml_source2_nominal_field1860, source_ml_source2_nominal_field1862, Box::new(source_ml_source2_nominal_field1864.clone()))
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
                                                                        let source_ml_source2_nominal_spread1874 = frame.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field1875 = match source_ml_source2_nominal_spread1874.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1876, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1876.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1877 = match source_ml_source2_nominal_spread1874.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1878, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1878
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1879 = match source_ml_source2_nominal_spread1874.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1880, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1880
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1881 = match source_ml_source2_nominal_spread1874.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1882, _, _) => {
                                                                                                source_ml_source2_data_property_value1882
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1883 = match source_ml_source2_nominal_spread1874.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1884, _) => {
                                                                                                    source_ml_source2_data_property_value1884
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1885 = match source_ml_source2_nominal_spread1874.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1886) => {
                                                                                                        let source_ml_source2_data_property_value1886 = *source_ml_source2_data_property_value1886;
                                                                                                        source_ml_source2_data_property_value1886.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override1887 = (shared_constructor_field6 + 1f64);
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1875.clone(), source_ml_source2_nominal_override1887, source_ml_source2_nominal_field1879, source_ml_source2_nominal_field1881, source_ml_source2_nominal_field1883, Box::new(source_ml_source2_nominal_field1885.clone()))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }), match owner_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1888, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value1888.clone()
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
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1897) => {
                                                let source_ml_source2_data_property_value1897 = *source_ml_source2_data_property_value1897;
                                                source_ml_source2_data_property_value1897.clone()
                                            }
                                        };
                                        match pending.clone() {
                                            self::JavaScriptCallPending::CallIdle => {
                                                {
                                                    let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1905, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value1905.clone()
                                                        }
                                                    })));
                                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                                    } else {
                                                        {
                                                            let instruction = self::ml_array::optional_at(&match fn_.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value1907) => {
                                                                    source_ml_source2_data_property_value1907
                                                                }
                                                            }, self::ml_array::number_index(match frame.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1908, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value1908
                                                                }
                                                            }));
                                                            if match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                self::return_java_script_call_value(program.clone(), machine.clone(), self::JavaScriptSemanticValue::Undefined)
                                                            } else {
                                                                {
                                                                    let next = {
                                                                        let source_ml_source2_nominal_spread1910 = frame.clone().expect("a proven present value");
                                                                        {
                                                                            let source_ml_source2_nominal_field1911 = match source_ml_source2_nominal_spread1910.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1912, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1912.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1913 = match source_ml_source2_nominal_spread1910.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1914, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1914
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1915 = match source_ml_source2_nominal_spread1910.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1916, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1916
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1917 = match source_ml_source2_nominal_spread1910.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1918, _, _) => {
                                                                                                source_ml_source2_data_property_value1918
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1919 = match source_ml_source2_nominal_spread1910.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1920, _) => {
                                                                                                    source_ml_source2_data_property_value1920
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1921 = match source_ml_source2_nominal_spread1910.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1922) => {
                                                                                                        let source_ml_source2_data_property_value1922 = *source_ml_source2_data_property_value1922;
                                                                                                        source_ml_source2_data_property_value1922.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override1923 = (match frame.clone().expect("a proven present value") {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1909, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value1909
                                                                                                        }
                                                                                                    } + 1f64);
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1911.clone(), source_ml_source2_nominal_override1923, source_ml_source2_nominal_field1915, source_ml_source2_nominal_field1917, source_ml_source2_nominal_field1919, Box::new(source_ml_source2_nominal_field1921.clone()))
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
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1926, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1926
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
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1946, _, _) => {
                                                                                            source_ml_source2_data_property_value1946
                                                                                        }
                                                                                    }, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                                }
                                                                                self::JavaScriptCallInstruction::Jump(target) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                                        let source_ml_source2_nominal_spread1949 = next.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1950 = match source_ml_source2_nominal_spread1949.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1951, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value1951.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1952 = match source_ml_source2_nominal_spread1949.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1953, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value1953
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field1954 = match source_ml_source2_nominal_spread1949.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1955, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value1955
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field1956 = match source_ml_source2_nominal_spread1949.clone() {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1957, _, _) => {
                                                                                                                source_ml_source2_data_property_value1957
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field1958 = match source_ml_source2_nominal_spread1949.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1959, _) => {
                                                                                                                    source_ml_source2_data_property_value1959
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field1960 = match source_ml_source2_nominal_spread1949.clone() {
                                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1961) => {
                                                                                                                        let source_ml_source2_data_property_value1961 = *source_ml_source2_data_property_value1961;
                                                                                                                        source_ml_source2_data_property_value1961.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override1962 = target;
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1950.clone(), source_ml_source2_nominal_override1962, source_ml_source2_nominal_field1954, source_ml_source2_nominal_field1956, source_ml_source2_nominal_field1958, Box::new(source_ml_source2_nominal_field1960.clone()))
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
                                                                                        let source_ml_source2_nominal_spread1965 = next.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1966 = match source_ml_source2_nominal_spread1965.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1967, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value1967.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1968 = match source_ml_source2_nominal_spread1965.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1969, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value1969
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field1970 = match source_ml_source2_nominal_spread1965.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1971, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value1971
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field1972 = match source_ml_source2_nominal_spread1965.clone() {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1973, _, _) => {
                                                                                                                source_ml_source2_data_property_value1973
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field1974 = match source_ml_source2_nominal_spread1965.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1975, _) => {
                                                                                                                    source_ml_source2_data_property_value1975
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field1976 = match source_ml_source2_nominal_spread1965.clone() {
                                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1977) => {
                                                                                                                        let source_ml_source2_data_property_value1977 = *source_ml_source2_data_property_value1977;
                                                                                                                        source_ml_source2_data_property_value1977.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override1978 = if self::convert_java_script_semantic_boolean((self::ml_array::optional_at(&registers, self::ml_array::number_index(condition))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)) {
                                                                                                                        yes
                                                                                                                    } else {
                                                                                                                        no
                                                                                                                    };
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1966.clone(), source_ml_source2_nominal_override1978, source_ml_source2_nominal_field1970, source_ml_source2_nominal_field1972, source_ml_source2_nominal_field1974, Box::new(source_ml_source2_nominal_field1976.clone()))
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
                                                                                            let source_ml_source2_nominal_spread1981 = next.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1982 = match source_ml_source2_nominal_spread1981.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1983, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value1983.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field1984 = match source_ml_source2_nominal_spread1981.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1985, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value1985
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field1986 = match source_ml_source2_nominal_spread1981.clone() {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1987, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value1987
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field1988 = match source_ml_source2_nominal_spread1981.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1989, _, _) => {
                                                                                                                    source_ml_source2_data_property_value1989
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field1990 = match source_ml_source2_nominal_spread1981.clone() {
                                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1991, _) => {
                                                                                                                        source_ml_source2_data_property_value1991
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field1992 = match source_ml_source2_nominal_spread1981.clone() {
                                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1993) => {
                                                                                                                            let source_ml_source2_data_property_value1993 = *source_ml_source2_data_property_value1993;
                                                                                                                            source_ml_source2_data_property_value1993.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override1994 = self::JavaScriptCallPending::WaitCall(destination_4);
                                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1982.clone(), source_ml_source2_nominal_field1984, source_ml_source2_nominal_field1986, source_ml_source2_nominal_field1988, source_ml_source2_nominal_field1990, Box::new(source_ml_source2_nominal_override1994.clone()))
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
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1996, _) => {
                                                                                            source_ml_source2_data_property_value1996
                                                                                        }
                                                                                    }).len() as f64)) >= (1024f64)) {
                                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![104, 97, 110, 100, 108, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                                    } else {
                                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                                            let source_ml_source2_nominal_spread2002 = next.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2003 = match source_ml_source2_nominal_spread2002.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2004, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value2004.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2005 = match source_ml_source2_nominal_spread2002.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2006, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value2006
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2007 = match source_ml_source2_nominal_spread2002.clone() {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2008, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value2008
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2009 = match source_ml_source2_nominal_spread2002.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2010, _, _) => {
                                                                                                                    source_ml_source2_data_property_value2010
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field2011 = match source_ml_source2_nominal_spread2002.clone() {
                                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2012, _) => {
                                                                                                                        source_ml_source2_data_property_value2012
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field2013 = match source_ml_source2_nominal_spread2002.clone() {
                                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2014) => {
                                                                                                                            let source_ml_source2_data_property_value2014 = *source_ml_source2_data_property_value2014;
                                                                                                                            source_ml_source2_data_property_value2014.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override2015 = {
                                                                                                                            let source_ml_source2_concat_value2000 = match frame.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1999, _) => {
                                                                                                                                    source_ml_source2_data_property_value1999
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_concat_value2001 = vec![self::JavaScriptCallHandler::CallHandler(target_2, destination_5)];
                                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallHandler>::new(), source_ml_source2_concat_value2000), source_ml_source2_concat_value2001)
                                                                                                                            }
                                                                                                                        };
                                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2003.clone(), source_ml_source2_nominal_field2005, source_ml_source2_nominal_field2007, source_ml_source2_nominal_field2009, source_ml_source2_nominal_override2015, Box::new(source_ml_source2_nominal_field2013.clone()))
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
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2017, _) => {
                                                                                            source_ml_source2_data_property_value2017
                                                                                        }
                                                                                    }).len() as f64)) == (0f64)) {
                                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 104, 97, 110, 100, 108, 101, 114, 32, 112, 111, 112, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 104, 97, 110, 100, 108, 101, 114]))
                                                                                    } else {
                                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                                            let source_ml_source2_nominal_spread2023 = next.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2024 = match source_ml_source2_nominal_spread2023.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2025, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value2025.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2026 = match source_ml_source2_nominal_spread2023.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2027, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value2027
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2028 = match source_ml_source2_nominal_spread2023.clone() {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2029, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value2029
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2030 = match source_ml_source2_nominal_spread2023.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2031, _, _) => {
                                                                                                                    source_ml_source2_data_property_value2031
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field2032 = match source_ml_source2_nominal_spread2023.clone() {
                                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2033, _) => {
                                                                                                                        source_ml_source2_data_property_value2033
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field2034 = match source_ml_source2_nominal_spread2023.clone() {
                                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2035) => {
                                                                                                                            let source_ml_source2_data_property_value2035 = *source_ml_source2_data_property_value2035;
                                                                                                                            source_ml_source2_data_property_value2035.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override2036 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2022, _) => {
                                                                                                                                source_ml_source2_data_property_value2022
                                                                                                                            }
                                                                                                                        }, 0f64, (-1f64));
                                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2024.clone(), source_ml_source2_nominal_field2026, source_ml_source2_nominal_field2028, source_ml_source2_nominal_field2030, source_ml_source2_nominal_override2036, Box::new(source_ml_source2_nominal_field2034.clone()))
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
