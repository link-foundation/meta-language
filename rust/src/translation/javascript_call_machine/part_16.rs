// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
use super::execute_java_script_call_primitive;
use super::find_java_script_call_function;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::plan_java_script_ordinary_set;
use super::push_java_script_call;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_ordinary_call_property(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, frame: self::JavaScriptCallFrame, instruction: self::JavaScriptCallInstruction) -> self::JavaScriptCallMachineTransition {
    {
        let next = {
            let source_ml_source1_nominal_spread1371 = frame.clone();
            {
                let source_ml_source1_nominal_field1372 = match source_ml_source1_nominal_spread1371.clone() {
                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1373, _, _, _, _, _) => {
                        source_ml_source1_data_property_value1373.clone()
                    }
                };
                {
                    let source_ml_source1_nominal_field1374 = match source_ml_source1_nominal_spread1371.clone() {
                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1375, _, _, _, _) => {
                            source_ml_source1_data_property_value1375
                        }
                    };
                    {
                        let source_ml_source1_nominal_field1376 = match source_ml_source1_nominal_spread1371.clone() {
                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1377, _, _, _) => {
                                source_ml_source1_data_property_value1377
                            }
                        };
                        {
                            let source_ml_source1_nominal_field1378 = match source_ml_source1_nominal_spread1371.clone() {
                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1379, _, _) => {
                                    source_ml_source1_data_property_value1379
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1380 = match source_ml_source1_nominal_spread1371.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1381, _) => {
                                        source_ml_source1_data_property_value1381
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1382 = match source_ml_source1_nominal_spread1371.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1383) => {
                                            let source_ml_source1_data_property_value1383 = *source_ml_source1_data_property_value1383;
                                            source_ml_source1_data_property_value1383.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_override1384 = (match frame.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1370, _, _, _, _) => {
                                                source_ml_source1_data_property_value1370
                                            }
                                        } + 1f64);
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1372.clone(), source_ml_source1_nominal_override1384, source_ml_source1_nominal_field1376, source_ml_source1_nominal_field1378, source_ml_source1_nominal_field1380, Box::new(source_ml_source1_nominal_field1382.clone()))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
        {
            let registers = match frame.clone() {
                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1385, _, _, _) => {
                    source_ml_source1_data_property_value1385
                }
            };
            match instruction.clone() {
                self::JavaScriptCallInstruction::Constant(source_ml_source1_f1386, source_ml_source1_f1387) => {
                    let source_ml_source1_f1387 = *source_ml_source1_f1387;
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1425, _) => {
                            let source_ml_source1_data_property_value1425 = *source_ml_source1_data_property_value1425;
                            source_ml_source1_data_property_value1425.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Copy(source_ml_source1_f1388, source_ml_source1_f1389) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1426, _) => {
                            let source_ml_source1_data_property_value1426 = *source_ml_source1_data_property_value1426;
                            source_ml_source1_data_property_value1426.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Argument(source_ml_source1_f1390, source_ml_source1_f1391) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1427, _) => {
                            let source_ml_source1_data_property_value1427 = *source_ml_source1_data_property_value1427;
                            source_ml_source1_data_property_value1427.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Jump(source_ml_source1_f1392) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1428, _) => {
                            let source_ml_source1_data_property_value1428 = *source_ml_source1_data_property_value1428;
                            source_ml_source1_data_property_value1428.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Branch(source_ml_source1_f1393, source_ml_source1_f1394, source_ml_source1_f1395) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1429, _) => {
                            let source_ml_source1_data_property_value1429 = *source_ml_source1_data_property_value1429;
                            source_ml_source1_data_property_value1429.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Call(source_ml_source1_f1396, source_ml_source1_f1397, source_ml_source1_f1398, source_ml_source1_f1399) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1430, _) => {
                            let source_ml_source1_data_property_value1430 = *source_ml_source1_data_property_value1430;
                            source_ml_source1_data_property_value1430.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Return(source_ml_source1_f1400) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1431, _) => {
                            let source_ml_source1_data_property_value1431 = *source_ml_source1_data_property_value1431;
                            source_ml_source1_data_property_value1431.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Throw(source_ml_source1_f1401) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1432, _) => {
                            let source_ml_source1_data_property_value1432 = *source_ml_source1_data_property_value1432;
                            source_ml_source1_data_property_value1432.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::PushHandler(source_ml_source1_f1402, source_ml_source1_f1403) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1433, _) => {
                            let source_ml_source1_data_property_value1433 = *source_ml_source1_data_property_value1433;
                            source_ml_source1_data_property_value1433.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::PopHandler => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1434, _) => {
                            let source_ml_source1_data_property_value1434 = *source_ml_source1_data_property_value1434;
                            source_ml_source1_data_property_value1434.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::NumberBinary(source_ml_source1_f1404, source_ml_source1_f1405, source_ml_source1_f1406, source_ml_source1_f1407) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1435, _) => {
                            let source_ml_source1_data_property_value1435 = *source_ml_source1_data_property_value1435;
                            source_ml_source1_data_property_value1435.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::SameValue(source_ml_source1_f1408, source_ml_source1_f1409, source_ml_source1_f1410) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1436, _) => {
                            let source_ml_source1_data_property_value1436 = *source_ml_source1_data_property_value1436;
                            source_ml_source1_data_property_value1436.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::StrictEqual(source_ml_source1_f1411, source_ml_source1_f1412, source_ml_source1_f1413) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1437, _) => {
                            let source_ml_source1_data_property_value1437 = *source_ml_source1_data_property_value1437;
                            source_ml_source1_data_property_value1437.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::RequireWrite(instruction_success) => {
                    {
                        let success = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_success))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        {
                            let owner = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1438, _, _) => source_ml_source1_data_property_value1438.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1439, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1439.clone()
                                }
                            })));
                            if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1440, _) => {
                                        let source_ml_source1_data_property_value1440 = *source_ml_source1_data_property_value1440;
                                        source_ml_source1_data_property_value1440.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                            } else {
                                match success.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1447, _) => {
                                                let source_ml_source1_data_property_value1447 = *source_ml_source1_data_property_value1447;
                                                source_ml_source1_data_property_value1447.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1448, _) => {
                                                let source_ml_source1_data_property_value1448 = *source_ml_source1_data_property_value1448;
                                                source_ml_source1_data_property_value1448.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(value) => {
                                        {
                                            let current = self::replace_java_script_call_frame(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1449, _) => {
                                                    let source_ml_source1_data_property_value1449 = *source_ml_source1_data_property_value1449;
                                                    source_ml_source1_data_property_value1449.clone()
                                                }
                                            }, match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1450) => {
                                                    source_ml_source1_data_property_value1450
                                                }
                                            }, next.clone());
                                            if value {
                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(current.clone()))
                                            } else {
                                                self::request_java_script_call_error(current.clone(), match owner.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1451, _, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value1451.clone()
                                                    }
                                                }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100])))
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Number(source_ml_source1_f1441) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1452, _) => {
                                                let source_ml_source1_data_property_value1452 = *source_ml_source1_data_property_value1452;
                                                source_ml_source1_data_property_value1452.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f1442) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1453, _) => {
                                                let source_ml_source1_data_property_value1453 = *source_ml_source1_data_property_value1453;
                                                source_ml_source1_data_property_value1453.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source1_f1443) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1454, _) => {
                                                let source_ml_source1_data_property_value1454 = *source_ml_source1_data_property_value1454;
                                                source_ml_source1_data_property_value1454.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1444) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1455, _) => {
                                                let source_ml_source1_data_property_value1455 = *source_ml_source1_data_property_value1455;
                                                source_ml_source1_data_property_value1455.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source1_f1445, source_ml_source1_f1446) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1456, _) => {
                                                let source_ml_source1_data_property_value1456 = *source_ml_source1_data_property_value1456;
                                                source_ml_source1_data_property_value1456.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptCallInstruction::Primitive(source_ml_source1_f1414, source_ml_source1_f1415, source_ml_source1_f1416) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1457, _) => {
                            let source_ml_source1_data_property_value1457 = *source_ml_source1_data_property_value1457;
                            source_ml_source1_data_property_value1457.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::OrdinaryGet(destination, instruction_object, key, receiver) => {
                    {
                        let object = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_object))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        match object.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1464, _) => {
                                        let source_ml_source1_data_property_value1464 = *source_ml_source1_data_property_value1464;
                                        source_ml_source1_data_property_value1464.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1465, _) => {
                                        let source_ml_source1_data_property_value1465 = *source_ml_source1_data_property_value1465;
                                        source_ml_source1_data_property_value1465.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1458) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1466, _) => {
                                        let source_ml_source1_data_property_value1466 = *source_ml_source1_data_property_value1466;
                                        source_ml_source1_data_property_value1466.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1459) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1467, _) => {
                                        let source_ml_source1_data_property_value1467 = *source_ml_source1_data_property_value1467;
                                        source_ml_source1_data_property_value1467.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1460) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1468, _) => {
                                        let source_ml_source1_data_property_value1468 = *source_ml_source1_data_property_value1468;
                                        source_ml_source1_data_property_value1468.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1461) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1469, _) => {
                                        let source_ml_source1_data_property_value1469 = *source_ml_source1_data_property_value1469;
                                        source_ml_source1_data_property_value1469.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1462) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1470, _) => {
                                        let source_ml_source1_data_property_value1470 = *source_ml_source1_data_property_value1470;
                                        source_ml_source1_data_property_value1470.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f1463) => {
                                {
                                    let plan = self::plan_java_script_ordinary_get(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1471, _) => {
                                            let source_ml_source1_data_property_value1471 = *source_ml_source1_data_property_value1471;
                                            source_ml_source1_data_property_value1471.clone()
                                        }
                                    }, identity.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                    if (((plan.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1472, _) => {
                                                let source_ml_source1_data_property_value1472 = *source_ml_source1_data_property_value1472;
                                                source_ml_source1_data_property_value1472.clone()
                                            }
                                        }), (plan.clone()).3)
                                    } else {
                                        if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1473, _) => {
                                                    let source_ml_source1_data_property_value1473 = *source_ml_source1_data_property_value1473;
                                                    source_ml_source1_data_property_value1473.clone()
                                                }
                                            }, match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1474) => {
                                                    source_ml_source1_data_property_value1474
                                                }
                                            }, self::write_java_script_call_register(next.clone(), destination, (plan.clone()).5))))
                                        } else {
                                            if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1475, _) => {
                                                        let source_ml_source1_data_property_value1475 = *source_ml_source1_data_property_value1475;
                                                        source_ml_source1_data_property_value1475.clone()
                                                    }
                                                }, match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1476) => {
                                                        source_ml_source1_data_property_value1476
                                                    }
                                                }, {
                                                    let source_ml_source1_nominal_spread1477 = next.clone();
                                                    {
                                                        let source_ml_source1_nominal_field1478 = match source_ml_source1_nominal_spread1477.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1479, _, _, _, _, _) => {
                                                                source_ml_source1_data_property_value1479.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field1480 = match source_ml_source1_nominal_spread1477.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1481, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value1481
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field1482 = match source_ml_source1_nominal_spread1477.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1483, _, _, _) => {
                                                                        source_ml_source1_data_property_value1483
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field1484 = match source_ml_source1_nominal_spread1477.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1485, _, _) => {
                                                                            source_ml_source1_data_property_value1485
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field1486 = match source_ml_source1_nominal_spread1477.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1487, _) => {
                                                                                source_ml_source1_data_property_value1487
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_field1488 = match source_ml_source1_nominal_spread1477.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1489) => {
                                                                                    let source_ml_source1_data_property_value1489 = *source_ml_source1_data_property_value1489;
                                                                                    source_ml_source1_data_property_value1489.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_override1490 = self::JavaScriptCallPending::WaitGet(destination);
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1478.clone(), source_ml_source1_nominal_field1480, source_ml_source1_nominal_field1482, source_ml_source1_nominal_field1484, source_ml_source1_nominal_field1486, Box::new(source_ml_source1_nominal_override1490.clone()))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }), (plan.clone()).2, (plan.clone()).4, (plan.clone()).1)
                                            } else {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1491, _) => {
                                                        let source_ml_source1_data_property_value1491 = *source_ml_source1_data_property_value1491;
                                                        source_ml_source1_data_property_value1491.clone()
                                                    }
                                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114]))
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
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1498, _) => {
                                        let source_ml_source1_data_property_value1498 = *source_ml_source1_data_property_value1498;
                                        source_ml_source1_data_property_value1498.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1499, _) => {
                                        let source_ml_source1_data_property_value1499 = *source_ml_source1_data_property_value1499;
                                        source_ml_source1_data_property_value1499.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1492) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1500, _) => {
                                        let source_ml_source1_data_property_value1500 = *source_ml_source1_data_property_value1500;
                                        source_ml_source1_data_property_value1500.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1493) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1501, _) => {
                                        let source_ml_source1_data_property_value1501 = *source_ml_source1_data_property_value1501;
                                        source_ml_source1_data_property_value1501.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1494) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1502, _) => {
                                        let source_ml_source1_data_property_value1502 = *source_ml_source1_data_property_value1502;
                                        source_ml_source1_data_property_value1502.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1495) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1503, _) => {
                                        let source_ml_source1_data_property_value1503 = *source_ml_source1_data_property_value1503;
                                        source_ml_source1_data_property_value1503.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1496) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1504, _) => {
                                        let source_ml_source1_data_property_value1504 = *source_ml_source1_data_property_value1504;
                                        source_ml_source1_data_property_value1504.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f1497) => {
                                {
                                    let plan_2 = self::plan_java_script_ordinary_set(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1505, _) => {
                                            let source_ml_source1_data_property_value1505 = *source_ml_source1_data_property_value1505;
                                            source_ml_source1_data_property_value1505.clone()
                                        }
                                    }, identity_2.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(value_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                    if (((plan_2.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1506, _) => {
                                                let source_ml_source1_data_property_value1506 = *source_ml_source1_data_property_value1506;
                                                source_ml_source1_data_property_value1506.clone()
                                            }
                                        }), (plan_2.clone()).3)
                                    } else {
                                        if (((plan_2.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1507, _) => {
                                                    let source_ml_source1_data_property_value1507 = *source_ml_source1_data_property_value1507;
                                                    source_ml_source1_data_property_value1507.clone()
                                                }
                                            }, match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1508) => {
                                                    source_ml_source1_data_property_value1508
                                                }
                                            }, {
                                                let source_ml_source1_nominal_spread1509 = next.clone();
                                                {
                                                    let source_ml_source1_nominal_field1510 = match source_ml_source1_nominal_spread1509.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1511, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value1511.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field1512 = match source_ml_source1_nominal_spread1509.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1513, _, _, _, _) => {
                                                                source_ml_source1_data_property_value1513
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field1514 = match source_ml_source1_nominal_spread1509.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1515, _, _, _) => {
                                                                    source_ml_source1_data_property_value1515
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field1516 = match source_ml_source1_nominal_spread1509.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1517, _, _) => {
                                                                        source_ml_source1_data_property_value1517
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field1518 = match source_ml_source1_nominal_spread1509.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1519, _) => {
                                                                            source_ml_source1_data_property_value1519
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field1520 = match source_ml_source1_nominal_spread1509.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1521) => {
                                                                                let source_ml_source1_data_property_value1521 = *source_ml_source1_data_property_value1521;
                                                                                source_ml_source1_data_property_value1521.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_override1522 = self::JavaScriptCallPending::WaitSet(destination_2);
                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1510.clone(), source_ml_source1_nominal_field1512, source_ml_source1_nominal_field1514, source_ml_source1_nominal_field1516, source_ml_source1_nominal_field1518, Box::new(source_ml_source1_nominal_override1522.clone()))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }), (plan_2.clone()).2, (plan_2.clone()).5, (plan_2.clone()).1)
                                        } else {
                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame((plan_2.clone()).4, match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1523) => {
                                                    source_ml_source1_data_property_value1523
                                                }
                                            }, self::write_java_script_call_register(next.clone(), destination_2, self::JavaScriptSemanticValue::Boolean((plan_2.clone()).6)))))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptCallInstruction::PropertyKey(source_ml_source1_f1417, source_ml_source1_f1418, source_ml_source1_f1419) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1524, _) => {
                            let source_ml_source1_data_property_value1524 = *source_ml_source1_data_property_value1524;
                            source_ml_source1_data_property_value1524.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::PropertyDefinition(source_ml_source1_f1420, source_ml_source1_f1421, source_ml_source1_f1422, source_ml_source1_f1423, source_ml_source1_f1424) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1525, _) => {
                            let source_ml_source1_data_property_value1525 = *source_ml_source1_data_property_value1525;
                            source_ml_source1_data_property_value1525.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_primitive_instruction(machine: self::JavaScriptCallMachine, frame: self::JavaScriptCallFrame, instruction: self::JavaScriptCallInstruction) -> self::JavaScriptCallMachineTransition {
    match instruction.clone() {
        self::JavaScriptCallInstruction::Constant(source_ml_source1_f1526, source_ml_source1_f1527) => {
            let source_ml_source1_f1527 = *source_ml_source1_f1527;
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1572, _) => {
                    let source_ml_source1_data_property_value1572 = *source_ml_source1_data_property_value1572;
                    source_ml_source1_data_property_value1572.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Copy(source_ml_source1_f1528, source_ml_source1_f1529) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1573, _) => {
                    let source_ml_source1_data_property_value1573 = *source_ml_source1_data_property_value1573;
                    source_ml_source1_data_property_value1573.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Argument(source_ml_source1_f1530, source_ml_source1_f1531) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1574, _) => {
                    let source_ml_source1_data_property_value1574 = *source_ml_source1_data_property_value1574;
                    source_ml_source1_data_property_value1574.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Jump(source_ml_source1_f1532) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1575, _) => {
                    let source_ml_source1_data_property_value1575 = *source_ml_source1_data_property_value1575;
                    source_ml_source1_data_property_value1575.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Branch(source_ml_source1_f1533, source_ml_source1_f1534, source_ml_source1_f1535) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1576, _) => {
                    let source_ml_source1_data_property_value1576 = *source_ml_source1_data_property_value1576;
                    source_ml_source1_data_property_value1576.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Call(source_ml_source1_f1536, source_ml_source1_f1537, source_ml_source1_f1538, source_ml_source1_f1539) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1577, _) => {
                    let source_ml_source1_data_property_value1577 = *source_ml_source1_data_property_value1577;
                    source_ml_source1_data_property_value1577.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Return(source_ml_source1_f1540) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1578, _) => {
                    let source_ml_source1_data_property_value1578 = *source_ml_source1_data_property_value1578;
                    source_ml_source1_data_property_value1578.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Throw(source_ml_source1_f1541) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1579, _) => {
                    let source_ml_source1_data_property_value1579 = *source_ml_source1_data_property_value1579;
                    source_ml_source1_data_property_value1579.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::PushHandler(source_ml_source1_f1542, source_ml_source1_f1543) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1580, _) => {
                    let source_ml_source1_data_property_value1580 = *source_ml_source1_data_property_value1580;
                    source_ml_source1_data_property_value1580.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::PopHandler => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1581, _) => {
                    let source_ml_source1_data_property_value1581 = *source_ml_source1_data_property_value1581;
                    source_ml_source1_data_property_value1581.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::NumberBinary(source_ml_source1_f1544, source_ml_source1_f1545, source_ml_source1_f1546, source_ml_source1_f1547) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1582, _) => {
                    let source_ml_source1_data_property_value1582 = *source_ml_source1_data_property_value1582;
                    source_ml_source1_data_property_value1582.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::SameValue(source_ml_source1_f1548, source_ml_source1_f1549, source_ml_source1_f1550) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1583, _) => {
                    let source_ml_source1_data_property_value1583 = *source_ml_source1_data_property_value1583;
                    source_ml_source1_data_property_value1583.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::StrictEqual(source_ml_source1_f1551, source_ml_source1_f1552, source_ml_source1_f1553) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1584, _) => {
                    let source_ml_source1_data_property_value1584 = *source_ml_source1_data_property_value1584;
                    source_ml_source1_data_property_value1584.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::RequireWrite(source_ml_source1_f1554) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1585, _) => {
                    let source_ml_source1_data_property_value1585 = *source_ml_source1_data_property_value1585;
                    source_ml_source1_data_property_value1585.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Primitive(destination, operation, arguments) => {
            {
                let result = self::execute_java_script_call_primitive(&(operation), { let ml_values7 = arguments; let ml_callback7 = { let frame = frame.clone(); std::rc::Rc::new(move |index: f64| -> self::JavaScriptSemanticValue { (self::ml_array::optional_at((match &frame { self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1586, _, _, _) => source_ml_source1_data_property_value1586.as_slice() }), self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values7.iter().cloned().enumerate().map(|(ml_index7, ml_value7)| ml_callback7(ml_value7)).collect::<Vec<_>>() });
                match result.clone() {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(value) => {
                        let value = *value;
                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1587, _) => {
                                let source_ml_source1_data_property_value1587 = *source_ml_source1_data_property_value1587;
                                source_ml_source1_data_property_value1587.clone()
                            }
                        }, match machine.clone() {
                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1588) => {
                                source_ml_source1_data_property_value1588
                            }
                        }, self::write_java_script_call_register({
                            let source_ml_source1_nominal_spread1590 = frame.clone();
                            {
                                let source_ml_source1_nominal_field1591 = match source_ml_source1_nominal_spread1590.clone() {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1592, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value1592.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1593 = match source_ml_source1_nominal_spread1590.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1594, _, _, _, _) => {
                                            source_ml_source1_data_property_value1594
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1595 = match source_ml_source1_nominal_spread1590.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1596, _, _, _) => {
                                                source_ml_source1_data_property_value1596
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1597 = match source_ml_source1_nominal_spread1590.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1598, _, _) => {
                                                    source_ml_source1_data_property_value1598
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field1599 = match source_ml_source1_nominal_spread1590.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1600, _) => {
                                                        source_ml_source1_data_property_value1600
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_field1601 = match source_ml_source1_nominal_spread1590.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1602) => {
                                                            let source_ml_source1_data_property_value1602 = *source_ml_source1_data_property_value1602;
                                                            source_ml_source1_data_property_value1602.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_override1603 = (match frame.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1589, _, _, _, _) => {
                                                                source_ml_source1_data_property_value1589
                                                            }
                                                        } + 1f64);
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1591.clone(), source_ml_source1_nominal_override1603, source_ml_source1_nominal_field1595, source_ml_source1_nominal_field1597, source_ml_source1_nominal_field1599, Box::new(source_ml_source1_nominal_field1601.clone()))
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
                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1604, _) => {
                                let source_ml_source1_data_property_value1604 = *source_ml_source1_data_property_value1604;
                                source_ml_source1_data_property_value1604.clone()
                            }
                        }), reason.clone())
                    }
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(reason_2) => {
                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_2.clone())
                    }
                }
            }
        }
        self::JavaScriptCallInstruction::OrdinaryGet(source_ml_source1_f1555, source_ml_source1_f1556, source_ml_source1_f1557, source_ml_source1_f1558) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1605, _) => {
                    let source_ml_source1_data_property_value1605 = *source_ml_source1_data_property_value1605;
                    source_ml_source1_data_property_value1605.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::OrdinarySet(source_ml_source1_f1559, source_ml_source1_f1560, source_ml_source1_f1561, source_ml_source1_f1562, source_ml_source1_f1563) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1606, _) => {
                    let source_ml_source1_data_property_value1606 = *source_ml_source1_data_property_value1606;
                    source_ml_source1_data_property_value1606.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::PropertyKey(source_ml_source1_f1564, source_ml_source1_f1565, source_ml_source1_f1566) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1607, _) => {
                    let source_ml_source1_data_property_value1607 = *source_ml_source1_data_property_value1607;
                    source_ml_source1_data_property_value1607.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::PropertyDefinition(source_ml_source1_f1567, source_ml_source1_f1568, source_ml_source1_f1569, source_ml_source1_f1570, source_ml_source1_f1571) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1608, _) => {
                    let source_ml_source1_data_property_value1608 = *source_ml_source1_data_property_value1608;
                    source_ml_source1_data_property_value1608.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
    }
}
