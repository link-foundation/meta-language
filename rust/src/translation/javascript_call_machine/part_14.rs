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
            let source_ml_source1_nominal_spread1316 = frame.clone();
            {
                let source_ml_source1_nominal_field1317 = match source_ml_source1_nominal_spread1316.clone() {
                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1318, _, _, _, _, _) => {
                        source_ml_source1_data_property_value1318.clone()
                    }
                };
                {
                    let source_ml_source1_nominal_field1319 = match source_ml_source1_nominal_spread1316.clone() {
                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1320, _, _, _, _) => {
                            source_ml_source1_data_property_value1320
                        }
                    };
                    {
                        let source_ml_source1_nominal_field1321 = match source_ml_source1_nominal_spread1316.clone() {
                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1322, _, _, _) => {
                                source_ml_source1_data_property_value1322
                            }
                        };
                        {
                            let source_ml_source1_nominal_field1323 = match source_ml_source1_nominal_spread1316.clone() {
                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1324, _, _) => {
                                    source_ml_source1_data_property_value1324
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1325 = match source_ml_source1_nominal_spread1316.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1326, _) => {
                                        source_ml_source1_data_property_value1326
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1327 = match source_ml_source1_nominal_spread1316.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1328) => {
                                            let source_ml_source1_data_property_value1328 = *source_ml_source1_data_property_value1328;
                                            source_ml_source1_data_property_value1328.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_override1329 = (match frame.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1315, _, _, _, _) => {
                                                source_ml_source1_data_property_value1315
                                            }
                                        } + 1f64);
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1317.clone(), source_ml_source1_nominal_override1329, source_ml_source1_nominal_field1321, source_ml_source1_nominal_field1323, source_ml_source1_nominal_field1325, Box::new(source_ml_source1_nominal_field1327.clone()))
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
                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1330, _, _, _) => {
                    source_ml_source1_data_property_value1330
                }
            };
            match instruction.clone() {
                self::JavaScriptCallInstruction::Constant(source_ml_source1_f1331, source_ml_source1_f1332) => {
                    let source_ml_source1_f1332 = *source_ml_source1_f1332;
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1367, _) => {
                            let source_ml_source1_data_property_value1367 = *source_ml_source1_data_property_value1367;
                            source_ml_source1_data_property_value1367.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Copy(source_ml_source1_f1333, source_ml_source1_f1334) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1368, _) => {
                            let source_ml_source1_data_property_value1368 = *source_ml_source1_data_property_value1368;
                            source_ml_source1_data_property_value1368.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Argument(source_ml_source1_f1335, source_ml_source1_f1336) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1369, _) => {
                            let source_ml_source1_data_property_value1369 = *source_ml_source1_data_property_value1369;
                            source_ml_source1_data_property_value1369.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Jump(source_ml_source1_f1337) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1370, _) => {
                            let source_ml_source1_data_property_value1370 = *source_ml_source1_data_property_value1370;
                            source_ml_source1_data_property_value1370.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Branch(source_ml_source1_f1338, source_ml_source1_f1339, source_ml_source1_f1340) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1371, _) => {
                            let source_ml_source1_data_property_value1371 = *source_ml_source1_data_property_value1371;
                            source_ml_source1_data_property_value1371.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Call(source_ml_source1_f1341, source_ml_source1_f1342, source_ml_source1_f1343, source_ml_source1_f1344) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1372, _) => {
                            let source_ml_source1_data_property_value1372 = *source_ml_source1_data_property_value1372;
                            source_ml_source1_data_property_value1372.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Return(source_ml_source1_f1345) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1373, _) => {
                            let source_ml_source1_data_property_value1373 = *source_ml_source1_data_property_value1373;
                            source_ml_source1_data_property_value1373.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Throw(source_ml_source1_f1346) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1374, _) => {
                            let source_ml_source1_data_property_value1374 = *source_ml_source1_data_property_value1374;
                            source_ml_source1_data_property_value1374.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::PushHandler(source_ml_source1_f1347, source_ml_source1_f1348) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1375, _) => {
                            let source_ml_source1_data_property_value1375 = *source_ml_source1_data_property_value1375;
                            source_ml_source1_data_property_value1375.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::PopHandler => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1376, _) => {
                            let source_ml_source1_data_property_value1376 = *source_ml_source1_data_property_value1376;
                            source_ml_source1_data_property_value1376.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::NumberBinary(source_ml_source1_f1349, source_ml_source1_f1350, source_ml_source1_f1351, source_ml_source1_f1352) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1377, _) => {
                            let source_ml_source1_data_property_value1377 = *source_ml_source1_data_property_value1377;
                            source_ml_source1_data_property_value1377.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::SameValue(source_ml_source1_f1353, source_ml_source1_f1354, source_ml_source1_f1355) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1378, _) => {
                            let source_ml_source1_data_property_value1378 = *source_ml_source1_data_property_value1378;
                            source_ml_source1_data_property_value1378.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::StrictEqual(source_ml_source1_f1356, source_ml_source1_f1357, source_ml_source1_f1358) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1379, _) => {
                            let source_ml_source1_data_property_value1379 = *source_ml_source1_data_property_value1379;
                            source_ml_source1_data_property_value1379.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::RequireWrite(instruction_success) => {
                    {
                        let success = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_success))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        {
                            let owner = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1380, _, _) => source_ml_source1_data_property_value1380.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1381, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1381.clone()
                                }
                            })));
                            if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1382, _) => {
                                        let source_ml_source1_data_property_value1382 = *source_ml_source1_data_property_value1382;
                                        source_ml_source1_data_property_value1382.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                            } else {
                                match success.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1389, _) => {
                                                let source_ml_source1_data_property_value1389 = *source_ml_source1_data_property_value1389;
                                                source_ml_source1_data_property_value1389.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1390, _) => {
                                                let source_ml_source1_data_property_value1390 = *source_ml_source1_data_property_value1390;
                                                source_ml_source1_data_property_value1390.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(value) => {
                                        {
                                            let current = self::replace_java_script_call_frame(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1391, _) => {
                                                    let source_ml_source1_data_property_value1391 = *source_ml_source1_data_property_value1391;
                                                    source_ml_source1_data_property_value1391.clone()
                                                }
                                            }, match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1392) => {
                                                    source_ml_source1_data_property_value1392
                                                }
                                            }, next.clone());
                                            if value {
                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(current.clone()))
                                            } else {
                                                self::request_java_script_call_error(current.clone(), match owner.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1393, _, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value1393.clone()
                                                    }
                                                }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100])))
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Number(source_ml_source1_f1383) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1394, _) => {
                                                let source_ml_source1_data_property_value1394 = *source_ml_source1_data_property_value1394;
                                                source_ml_source1_data_property_value1394.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f1384) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1395, _) => {
                                                let source_ml_source1_data_property_value1395 = *source_ml_source1_data_property_value1395;
                                                source_ml_source1_data_property_value1395.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source1_f1385) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1396, _) => {
                                                let source_ml_source1_data_property_value1396 = *source_ml_source1_data_property_value1396;
                                                source_ml_source1_data_property_value1396.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1386) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1397, _) => {
                                                let source_ml_source1_data_property_value1397 = *source_ml_source1_data_property_value1397;
                                                source_ml_source1_data_property_value1397.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source1_f1387, source_ml_source1_f1388) => {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1398, _) => {
                                                let source_ml_source1_data_property_value1398 = *source_ml_source1_data_property_value1398;
                                                source_ml_source1_data_property_value1398.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptCallInstruction::OrdinaryGet(destination, instruction_object, key, receiver) => {
                    {
                        let object = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_object))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        match object.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1405, _) => {
                                        let source_ml_source1_data_property_value1405 = *source_ml_source1_data_property_value1405;
                                        source_ml_source1_data_property_value1405.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1406, _) => {
                                        let source_ml_source1_data_property_value1406 = *source_ml_source1_data_property_value1406;
                                        source_ml_source1_data_property_value1406.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1399) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1407, _) => {
                                        let source_ml_source1_data_property_value1407 = *source_ml_source1_data_property_value1407;
                                        source_ml_source1_data_property_value1407.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1400) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1408, _) => {
                                        let source_ml_source1_data_property_value1408 = *source_ml_source1_data_property_value1408;
                                        source_ml_source1_data_property_value1408.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1401) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1409, _) => {
                                        let source_ml_source1_data_property_value1409 = *source_ml_source1_data_property_value1409;
                                        source_ml_source1_data_property_value1409.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1402) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1410, _) => {
                                        let source_ml_source1_data_property_value1410 = *source_ml_source1_data_property_value1410;
                                        source_ml_source1_data_property_value1410.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1403) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1411, _) => {
                                        let source_ml_source1_data_property_value1411 = *source_ml_source1_data_property_value1411;
                                        source_ml_source1_data_property_value1411.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f1404) => {
                                {
                                    let plan = self::plan_java_script_ordinary_get(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1412, _) => {
                                            let source_ml_source1_data_property_value1412 = *source_ml_source1_data_property_value1412;
                                            source_ml_source1_data_property_value1412.clone()
                                        }
                                    }, identity.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                    if (((plan.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1413, _) => {
                                                let source_ml_source1_data_property_value1413 = *source_ml_source1_data_property_value1413;
                                                source_ml_source1_data_property_value1413.clone()
                                            }
                                        }), (plan.clone()).3)
                                    } else {
                                        if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1414, _) => {
                                                    let source_ml_source1_data_property_value1414 = *source_ml_source1_data_property_value1414;
                                                    source_ml_source1_data_property_value1414.clone()
                                                }
                                            }, match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1415) => {
                                                    source_ml_source1_data_property_value1415
                                                }
                                            }, self::write_java_script_call_register(next.clone(), destination, (plan.clone()).5))))
                                        } else {
                                            if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1416, _) => {
                                                        let source_ml_source1_data_property_value1416 = *source_ml_source1_data_property_value1416;
                                                        source_ml_source1_data_property_value1416.clone()
                                                    }
                                                }, match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1417) => {
                                                        source_ml_source1_data_property_value1417
                                                    }
                                                }, {
                                                    let source_ml_source1_nominal_spread1418 = next.clone();
                                                    {
                                                        let source_ml_source1_nominal_field1419 = match source_ml_source1_nominal_spread1418.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1420, _, _, _, _, _) => {
                                                                source_ml_source1_data_property_value1420.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field1421 = match source_ml_source1_nominal_spread1418.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1422, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value1422
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field1423 = match source_ml_source1_nominal_spread1418.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1424, _, _, _) => {
                                                                        source_ml_source1_data_property_value1424
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field1425 = match source_ml_source1_nominal_spread1418.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1426, _, _) => {
                                                                            source_ml_source1_data_property_value1426
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field1427 = match source_ml_source1_nominal_spread1418.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1428, _) => {
                                                                                source_ml_source1_data_property_value1428
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_field1429 = match source_ml_source1_nominal_spread1418.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1430) => {
                                                                                    let source_ml_source1_data_property_value1430 = *source_ml_source1_data_property_value1430;
                                                                                    source_ml_source1_data_property_value1430.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_override1431 = self::JavaScriptCallPending::WaitGet(destination);
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1419.clone(), source_ml_source1_nominal_field1421, source_ml_source1_nominal_field1423, source_ml_source1_nominal_field1425, source_ml_source1_nominal_field1427, Box::new(source_ml_source1_nominal_override1431.clone()))
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
                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1432, _) => {
                                                        let source_ml_source1_data_property_value1432 = *source_ml_source1_data_property_value1432;
                                                        source_ml_source1_data_property_value1432.clone()
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
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1439, _) => {
                                        let source_ml_source1_data_property_value1439 = *source_ml_source1_data_property_value1439;
                                        source_ml_source1_data_property_value1439.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1440, _) => {
                                        let source_ml_source1_data_property_value1440 = *source_ml_source1_data_property_value1440;
                                        source_ml_source1_data_property_value1440.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1433) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1441, _) => {
                                        let source_ml_source1_data_property_value1441 = *source_ml_source1_data_property_value1441;
                                        source_ml_source1_data_property_value1441.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1434) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1442, _) => {
                                        let source_ml_source1_data_property_value1442 = *source_ml_source1_data_property_value1442;
                                        source_ml_source1_data_property_value1442.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1435) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1443, _) => {
                                        let source_ml_source1_data_property_value1443 = *source_ml_source1_data_property_value1443;
                                        source_ml_source1_data_property_value1443.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1436) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1444, _) => {
                                        let source_ml_source1_data_property_value1444 = *source_ml_source1_data_property_value1444;
                                        source_ml_source1_data_property_value1444.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1437) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1445, _) => {
                                        let source_ml_source1_data_property_value1445 = *source_ml_source1_data_property_value1445;
                                        source_ml_source1_data_property_value1445.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f1438) => {
                                {
                                    let plan_2 = self::plan_java_script_ordinary_set(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1446, _) => {
                                            let source_ml_source1_data_property_value1446 = *source_ml_source1_data_property_value1446;
                                            source_ml_source1_data_property_value1446.clone()
                                        }
                                    }, identity_2.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(value_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                    if (((plan_2.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1447, _) => {
                                                let source_ml_source1_data_property_value1447 = *source_ml_source1_data_property_value1447;
                                                source_ml_source1_data_property_value1447.clone()
                                            }
                                        }), (plan_2.clone()).3)
                                    } else {
                                        if (((plan_2.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1448, _) => {
                                                    let source_ml_source1_data_property_value1448 = *source_ml_source1_data_property_value1448;
                                                    source_ml_source1_data_property_value1448.clone()
                                                }
                                            }, match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1449) => {
                                                    source_ml_source1_data_property_value1449
                                                }
                                            }, {
                                                let source_ml_source1_nominal_spread1450 = next.clone();
                                                {
                                                    let source_ml_source1_nominal_field1451 = match source_ml_source1_nominal_spread1450.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1452, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value1452.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field1453 = match source_ml_source1_nominal_spread1450.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1454, _, _, _, _) => {
                                                                source_ml_source1_data_property_value1454
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field1455 = match source_ml_source1_nominal_spread1450.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1456, _, _, _) => {
                                                                    source_ml_source1_data_property_value1456
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field1457 = match source_ml_source1_nominal_spread1450.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1458, _, _) => {
                                                                        source_ml_source1_data_property_value1458
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field1459 = match source_ml_source1_nominal_spread1450.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1460, _) => {
                                                                            source_ml_source1_data_property_value1460
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field1461 = match source_ml_source1_nominal_spread1450.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1462) => {
                                                                                let source_ml_source1_data_property_value1462 = *source_ml_source1_data_property_value1462;
                                                                                source_ml_source1_data_property_value1462.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_override1463 = self::JavaScriptCallPending::WaitSet(destination_2);
                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1451.clone(), source_ml_source1_nominal_field1453, source_ml_source1_nominal_field1455, source_ml_source1_nominal_field1457, source_ml_source1_nominal_field1459, Box::new(source_ml_source1_nominal_override1463.clone()))
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
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1464) => {
                                                    source_ml_source1_data_property_value1464
                                                }
                                            }, self::write_java_script_call_register(next.clone(), destination_2, self::JavaScriptSemanticValue::Boolean((plan_2.clone()).6)))))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptCallInstruction::PropertyKey(source_ml_source1_f1359, source_ml_source1_f1360, source_ml_source1_f1361) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1465, _) => {
                            let source_ml_source1_data_property_value1465 = *source_ml_source1_data_property_value1465;
                            source_ml_source1_data_property_value1465.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::PropertyDefinition(source_ml_source1_f1362, source_ml_source1_f1363, source_ml_source1_f1364, source_ml_source1_f1365, source_ml_source1_f1366) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1466, _) => {
                            let source_ml_source1_data_property_value1466 = *source_ml_source1_data_property_value1466;
                            source_ml_source1_data_property_value1466.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
            }
        }
    }
}
