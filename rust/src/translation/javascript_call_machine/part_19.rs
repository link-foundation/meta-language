// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
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
            let source_ml_source2_nominal_spread1446 = frame.clone();
            {
                let source_ml_source2_nominal_field1447 = match source_ml_source2_nominal_spread1446.clone() {
                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1448, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1448.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field1449 = match source_ml_source2_nominal_spread1446.clone() {
                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1450, _, _, _, _) => {
                            source_ml_source2_data_property_value1450
                        }
                    };
                    {
                        let source_ml_source2_nominal_field1451 = match source_ml_source2_nominal_spread1446.clone() {
                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1452, _, _, _) => {
                                source_ml_source2_data_property_value1452
                            }
                        };
                        {
                            let source_ml_source2_nominal_field1453 = match source_ml_source2_nominal_spread1446.clone() {
                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1454, _, _) => {
                                    source_ml_source2_data_property_value1454
                                }
                            };
                            {
                                let source_ml_source2_nominal_field1455 = match source_ml_source2_nominal_spread1446.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1456, _) => {
                                        source_ml_source2_data_property_value1456
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1457 = match source_ml_source2_nominal_spread1446.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1458) => {
                                            let source_ml_source2_data_property_value1458 = *source_ml_source2_data_property_value1458;
                                            source_ml_source2_data_property_value1458.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override1459 = (match frame.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1445, _, _, _, _) => {
                                                source_ml_source2_data_property_value1445
                                            }
                                        } + 1f64);
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1447.clone(), source_ml_source2_nominal_override1459, source_ml_source2_nominal_field1451, source_ml_source2_nominal_field1453, source_ml_source2_nominal_field1455, Box::new(source_ml_source2_nominal_field1457.clone()))
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
                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1460, _, _, _) => {
                    source_ml_source2_data_property_value1460
                }
            };
            match instruction.clone() {
                self::JavaScriptCallInstruction::Constant(source_ml_source2_f1461, source_ml_source2_f1462) => {
                    let source_ml_source2_f1462 = *source_ml_source2_f1462;
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1500, _) => {
                            let source_ml_source2_data_property_value1500 = *source_ml_source2_data_property_value1500;
                            source_ml_source2_data_property_value1500.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Copy(source_ml_source2_f1463, source_ml_source2_f1464) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1501, _) => {
                            let source_ml_source2_data_property_value1501 = *source_ml_source2_data_property_value1501;
                            source_ml_source2_data_property_value1501.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Argument(source_ml_source2_f1465, source_ml_source2_f1466) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1502, _) => {
                            let source_ml_source2_data_property_value1502 = *source_ml_source2_data_property_value1502;
                            source_ml_source2_data_property_value1502.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Jump(source_ml_source2_f1467) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1503, _) => {
                            let source_ml_source2_data_property_value1503 = *source_ml_source2_data_property_value1503;
                            source_ml_source2_data_property_value1503.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Branch(source_ml_source2_f1468, source_ml_source2_f1469, source_ml_source2_f1470) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1504, _) => {
                            let source_ml_source2_data_property_value1504 = *source_ml_source2_data_property_value1504;
                            source_ml_source2_data_property_value1504.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Call(source_ml_source2_f1471, source_ml_source2_f1472, source_ml_source2_f1473, source_ml_source2_f1474) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1505, _) => {
                            let source_ml_source2_data_property_value1505 = *source_ml_source2_data_property_value1505;
                            source_ml_source2_data_property_value1505.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Return(source_ml_source2_f1475) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1506, _) => {
                            let source_ml_source2_data_property_value1506 = *source_ml_source2_data_property_value1506;
                            source_ml_source2_data_property_value1506.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Throw(source_ml_source2_f1476) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1507, _) => {
                            let source_ml_source2_data_property_value1507 = *source_ml_source2_data_property_value1507;
                            source_ml_source2_data_property_value1507.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::PushHandler(source_ml_source2_f1477, source_ml_source2_f1478) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1508, _) => {
                            let source_ml_source2_data_property_value1508 = *source_ml_source2_data_property_value1508;
                            source_ml_source2_data_property_value1508.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::PopHandler => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1509, _) => {
                            let source_ml_source2_data_property_value1509 = *source_ml_source2_data_property_value1509;
                            source_ml_source2_data_property_value1509.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::NumberBinary(source_ml_source2_f1479, source_ml_source2_f1480, source_ml_source2_f1481, source_ml_source2_f1482) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1510, _) => {
                            let source_ml_source2_data_property_value1510 = *source_ml_source2_data_property_value1510;
                            source_ml_source2_data_property_value1510.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::SameValue(source_ml_source2_f1483, source_ml_source2_f1484, source_ml_source2_f1485) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1511, _) => {
                            let source_ml_source2_data_property_value1511 = *source_ml_source2_data_property_value1511;
                            source_ml_source2_data_property_value1511.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::StrictEqual(source_ml_source2_f1486, source_ml_source2_f1487, source_ml_source2_f1488) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1512, _) => {
                            let source_ml_source2_data_property_value1512 = *source_ml_source2_data_property_value1512;
                            source_ml_source2_data_property_value1512.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::RequireWrite(instruction_success) => {
                    {
                        let success = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_success))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        {
                            let owner = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1513, _, _) => source_ml_source2_data_property_value1513.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1514, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1514.clone()
                                }
                            })));
                            if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1515, _) => {
                                        let source_ml_source2_data_property_value1515 = *source_ml_source2_data_property_value1515;
                                        source_ml_source2_data_property_value1515.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                            } else {
                                match success.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1522, _) => {
                                                let source_ml_source2_data_property_value1522 = *source_ml_source2_data_property_value1522;
                                                source_ml_source2_data_property_value1522.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1523, _) => {
                                                let source_ml_source2_data_property_value1523 = *source_ml_source2_data_property_value1523;
                                                source_ml_source2_data_property_value1523.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(value) => {
                                        {
                                            let current = self::replace_java_script_call_frame(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1524, _) => {
                                                    let source_ml_source2_data_property_value1524 = *source_ml_source2_data_property_value1524;
                                                    source_ml_source2_data_property_value1524.clone()
                                                }
                                            }, match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1525) => {
                                                    source_ml_source2_data_property_value1525
                                                }
                                            }, next.clone());
                                            if value {
                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(current.clone()))
                                            } else {
                                                self::request_java_script_call_error(current.clone(), match owner.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1526, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value1526.clone()
                                                    }
                                                }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100])))
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Number(source_ml_source2_f1516) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1527, _) => {
                                                let source_ml_source2_data_property_value1527 = *source_ml_source2_data_property_value1527;
                                                source_ml_source2_data_property_value1527.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f1517) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1528, _) => {
                                                let source_ml_source2_data_property_value1528 = *source_ml_source2_data_property_value1528;
                                                source_ml_source2_data_property_value1528.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f1518) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1529, _) => {
                                                let source_ml_source2_data_property_value1529 = *source_ml_source2_data_property_value1529;
                                                source_ml_source2_data_property_value1529.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1519) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1530, _) => {
                                                let source_ml_source2_data_property_value1530 = *source_ml_source2_data_property_value1530;
                                                source_ml_source2_data_property_value1530.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f1520, source_ml_source2_f1521) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1531, _) => {
                                                let source_ml_source2_data_property_value1531 = *source_ml_source2_data_property_value1531;
                                                source_ml_source2_data_property_value1531.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptCallInstruction::Primitive(source_ml_source2_f1489, source_ml_source2_f1490, source_ml_source2_f1491) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1532, _) => {
                            let source_ml_source2_data_property_value1532 = *source_ml_source2_data_property_value1532;
                            source_ml_source2_data_property_value1532.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::OrdinaryGet(destination, instruction_object, key, receiver) => {
                    {
                        let object = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_object))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        match object.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1539, _) => {
                                        let source_ml_source2_data_property_value1539 = *source_ml_source2_data_property_value1539;
                                        source_ml_source2_data_property_value1539.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1540, _) => {
                                        let source_ml_source2_data_property_value1540 = *source_ml_source2_data_property_value1540;
                                        source_ml_source2_data_property_value1540.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1533) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1541, _) => {
                                        let source_ml_source2_data_property_value1541 = *source_ml_source2_data_property_value1541;
                                        source_ml_source2_data_property_value1541.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source2_f1534) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1542, _) => {
                                        let source_ml_source2_data_property_value1542 = *source_ml_source2_data_property_value1542;
                                        source_ml_source2_data_property_value1542.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source2_f1535) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1543, _) => {
                                        let source_ml_source2_data_property_value1543 = *source_ml_source2_data_property_value1543;
                                        source_ml_source2_data_property_value1543.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source2_f1536) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1544, _) => {
                                        let source_ml_source2_data_property_value1544 = *source_ml_source2_data_property_value1544;
                                        source_ml_source2_data_property_value1544.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1537) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1545, _) => {
                                        let source_ml_source2_data_property_value1545 = *source_ml_source2_data_property_value1545;
                                        source_ml_source2_data_property_value1545.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Object(identity, source_ml_source2_f1538) => {
                                {
                                    let plan = self::plan_java_script_ordinary_get(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1546, _) => {
                                            let source_ml_source2_data_property_value1546 = *source_ml_source2_data_property_value1546;
                                            source_ml_source2_data_property_value1546.clone()
                                        }
                                    }, identity.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                    if (((plan.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1547, _) => {
                                                let source_ml_source2_data_property_value1547 = *source_ml_source2_data_property_value1547;
                                                source_ml_source2_data_property_value1547.clone()
                                            }
                                        }), (plan.clone()).3)
                                    } else {
                                        if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1548, _) => {
                                                    let source_ml_source2_data_property_value1548 = *source_ml_source2_data_property_value1548;
                                                    source_ml_source2_data_property_value1548.clone()
                                                }
                                            }, match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1549) => {
                                                    source_ml_source2_data_property_value1549
                                                }
                                            }, self::write_java_script_call_register(next.clone(), destination, (plan.clone()).5))))
                                        } else {
                                            if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1550, _) => {
                                                        let source_ml_source2_data_property_value1550 = *source_ml_source2_data_property_value1550;
                                                        source_ml_source2_data_property_value1550.clone()
                                                    }
                                                }, match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1551) => {
                                                        source_ml_source2_data_property_value1551
                                                    }
                                                }, {
                                                    let source_ml_source2_nominal_spread1552 = next.clone();
                                                    {
                                                        let source_ml_source2_nominal_field1553 = match source_ml_source2_nominal_spread1552.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1554, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value1554.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field1555 = match source_ml_source2_nominal_spread1552.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1556, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value1556
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field1557 = match source_ml_source2_nominal_spread1552.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1558, _, _, _) => {
                                                                        source_ml_source2_data_property_value1558
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field1559 = match source_ml_source2_nominal_spread1552.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1560, _, _) => {
                                                                            source_ml_source2_data_property_value1560
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field1561 = match source_ml_source2_nominal_spread1552.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1562, _) => {
                                                                                source_ml_source2_data_property_value1562
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field1563 = match source_ml_source2_nominal_spread1552.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1564) => {
                                                                                    let source_ml_source2_data_property_value1564 = *source_ml_source2_data_property_value1564;
                                                                                    source_ml_source2_data_property_value1564.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override1565 = self::JavaScriptCallPending::WaitGet(destination);
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1553.clone(), source_ml_source2_nominal_field1555, source_ml_source2_nominal_field1557, source_ml_source2_nominal_field1559, source_ml_source2_nominal_field1561, Box::new(source_ml_source2_nominal_override1565.clone()))
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
                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1566, _) => {
                                                        let source_ml_source2_data_property_value1566 = *source_ml_source2_data_property_value1566;
                                                        source_ml_source2_data_property_value1566.clone()
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
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1573, _) => {
                                        let source_ml_source2_data_property_value1573 = *source_ml_source2_data_property_value1573;
                                        source_ml_source2_data_property_value1573.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1574, _) => {
                                        let source_ml_source2_data_property_value1574 = *source_ml_source2_data_property_value1574;
                                        source_ml_source2_data_property_value1574.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1567) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1575, _) => {
                                        let source_ml_source2_data_property_value1575 = *source_ml_source2_data_property_value1575;
                                        source_ml_source2_data_property_value1575.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source2_f1568) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1576, _) => {
                                        let source_ml_source2_data_property_value1576 = *source_ml_source2_data_property_value1576;
                                        source_ml_source2_data_property_value1576.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source2_f1569) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1577, _) => {
                                        let source_ml_source2_data_property_value1577 = *source_ml_source2_data_property_value1577;
                                        source_ml_source2_data_property_value1577.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source2_f1570) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1578, _) => {
                                        let source_ml_source2_data_property_value1578 = *source_ml_source2_data_property_value1578;
                                        source_ml_source2_data_property_value1578.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1571) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1579, _) => {
                                        let source_ml_source2_data_property_value1579 = *source_ml_source2_data_property_value1579;
                                        source_ml_source2_data_property_value1579.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Object(identity_2, source_ml_source2_f1572) => {
                                {
                                    let plan_2 = self::plan_java_script_ordinary_set(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1580, _) => {
                                            let source_ml_source2_data_property_value1580 = *source_ml_source2_data_property_value1580;
                                            source_ml_source2_data_property_value1580.clone()
                                        }
                                    }, identity_2.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(value_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                    if (((plan_2.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1581, _) => {
                                                let source_ml_source2_data_property_value1581 = *source_ml_source2_data_property_value1581;
                                                source_ml_source2_data_property_value1581.clone()
                                            }
                                        }), (plan_2.clone()).3)
                                    } else {
                                        if (((plan_2.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1582, _) => {
                                                    let source_ml_source2_data_property_value1582 = *source_ml_source2_data_property_value1582;
                                                    source_ml_source2_data_property_value1582.clone()
                                                }
                                            }, match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1583) => {
                                                    source_ml_source2_data_property_value1583
                                                }
                                            }, {
                                                let source_ml_source2_nominal_spread1584 = next.clone();
                                                {
                                                    let source_ml_source2_nominal_field1585 = match source_ml_source2_nominal_spread1584.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1586, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value1586.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1587 = match source_ml_source2_nominal_spread1584.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1588, _, _, _, _) => {
                                                                source_ml_source2_data_property_value1588
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field1589 = match source_ml_source2_nominal_spread1584.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1590, _, _, _) => {
                                                                    source_ml_source2_data_property_value1590
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field1591 = match source_ml_source2_nominal_spread1584.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1592, _, _) => {
                                                                        source_ml_source2_data_property_value1592
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field1593 = match source_ml_source2_nominal_spread1584.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1594, _) => {
                                                                            source_ml_source2_data_property_value1594
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field1595 = match source_ml_source2_nominal_spread1584.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1596) => {
                                                                                let source_ml_source2_data_property_value1596 = *source_ml_source2_data_property_value1596;
                                                                                source_ml_source2_data_property_value1596.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override1597 = self::JavaScriptCallPending::WaitSet(destination_2);
                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1585.clone(), source_ml_source2_nominal_field1587, source_ml_source2_nominal_field1589, source_ml_source2_nominal_field1591, source_ml_source2_nominal_field1593, Box::new(source_ml_source2_nominal_override1597.clone()))
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
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1598) => {
                                                    source_ml_source2_data_property_value1598
                                                }
                                            }, self::write_java_script_call_register(next.clone(), destination_2, self::JavaScriptSemanticValue::Boolean((plan_2.clone()).6)))))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptCallInstruction::PropertyKey(source_ml_source2_f1492, source_ml_source2_f1493, source_ml_source2_f1494) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1599, _) => {
                            let source_ml_source2_data_property_value1599 = *source_ml_source2_data_property_value1599;
                            source_ml_source2_data_property_value1599.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::PropertyDefinition(source_ml_source2_f1495, source_ml_source2_f1496, source_ml_source2_f1497, source_ml_source2_f1498, source_ml_source2_f1499) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1600, _) => {
                            let source_ml_source2_data_property_value1600 = *source_ml_source2_data_property_value1600;
                            source_ml_source2_data_property_value1600.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
            }
        }
    }
}
