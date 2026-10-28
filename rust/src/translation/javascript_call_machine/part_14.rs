// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::plan_java_script_ordinary_set;
use super::push_java_script_call;
use super::replace_java_script_call_frame;
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
                self::JavaScriptCallInstruction::OrdinaryGet(destination, instruction_object, key, receiver) => {
                    {
                        let object = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_object))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        match object.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1386, _) => {
                                        let source_ml_source1_data_property_value1386 = *source_ml_source1_data_property_value1386;
                                        source_ml_source1_data_property_value1386.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1387, _) => {
                                        let source_ml_source1_data_property_value1387 = *source_ml_source1_data_property_value1387;
                                        source_ml_source1_data_property_value1387.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1380) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1388, _) => {
                                        let source_ml_source1_data_property_value1388 = *source_ml_source1_data_property_value1388;
                                        source_ml_source1_data_property_value1388.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1381) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1389, _) => {
                                        let source_ml_source1_data_property_value1389 = *source_ml_source1_data_property_value1389;
                                        source_ml_source1_data_property_value1389.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1382) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1390, _) => {
                                        let source_ml_source1_data_property_value1390 = *source_ml_source1_data_property_value1390;
                                        source_ml_source1_data_property_value1390.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1383) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1391, _) => {
                                        let source_ml_source1_data_property_value1391 = *source_ml_source1_data_property_value1391;
                                        source_ml_source1_data_property_value1391.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1384) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1392, _) => {
                                        let source_ml_source1_data_property_value1392 = *source_ml_source1_data_property_value1392;
                                        source_ml_source1_data_property_value1392.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f1385) => {
                                {
                                    let plan = self::plan_java_script_ordinary_get(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1393, _) => {
                                            let source_ml_source1_data_property_value1393 = *source_ml_source1_data_property_value1393;
                                            source_ml_source1_data_property_value1393.clone()
                                        }
                                    }, identity.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                    if (((plan.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1394, _) => {
                                                let source_ml_source1_data_property_value1394 = *source_ml_source1_data_property_value1394;
                                                source_ml_source1_data_property_value1394.clone()
                                            }
                                        }), (plan.clone()).3)
                                    } else {
                                        if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1395, _) => {
                                                    let source_ml_source1_data_property_value1395 = *source_ml_source1_data_property_value1395;
                                                    source_ml_source1_data_property_value1395.clone()
                                                }
                                            }, match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1396) => {
                                                    source_ml_source1_data_property_value1396
                                                }
                                            }, self::write_java_script_call_register(next.clone(), destination, (plan.clone()).5))))
                                        } else {
                                            if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1397, _) => {
                                                        let source_ml_source1_data_property_value1397 = *source_ml_source1_data_property_value1397;
                                                        source_ml_source1_data_property_value1397.clone()
                                                    }
                                                }, match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1398) => {
                                                        source_ml_source1_data_property_value1398
                                                    }
                                                }, {
                                                    let source_ml_source1_nominal_spread1399 = next.clone();
                                                    {
                                                        let source_ml_source1_nominal_field1400 = match source_ml_source1_nominal_spread1399.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1401, _, _, _, _, _) => {
                                                                source_ml_source1_data_property_value1401.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field1402 = match source_ml_source1_nominal_spread1399.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1403, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value1403
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field1404 = match source_ml_source1_nominal_spread1399.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1405, _, _, _) => {
                                                                        source_ml_source1_data_property_value1405
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field1406 = match source_ml_source1_nominal_spread1399.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1407, _, _) => {
                                                                            source_ml_source1_data_property_value1407
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field1408 = match source_ml_source1_nominal_spread1399.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1409, _) => {
                                                                                source_ml_source1_data_property_value1409
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_field1410 = match source_ml_source1_nominal_spread1399.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1411) => {
                                                                                    let source_ml_source1_data_property_value1411 = *source_ml_source1_data_property_value1411;
                                                                                    source_ml_source1_data_property_value1411.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_override1412 = self::JavaScriptCallPending::WaitGet(destination);
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1400.clone(), source_ml_source1_nominal_field1402, source_ml_source1_nominal_field1404, source_ml_source1_nominal_field1406, source_ml_source1_nominal_field1408, Box::new(source_ml_source1_nominal_override1412.clone()))
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
                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1413, _) => {
                                                        let source_ml_source1_data_property_value1413 = *source_ml_source1_data_property_value1413;
                                                        source_ml_source1_data_property_value1413.clone()
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
                self::JavaScriptCallInstruction::OrdinarySet(destination_2, instruction_object_2, key_2, value, receiver_2) => {
                    {
                        let object_2 = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_object_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        match object_2.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1420, _) => {
                                        let source_ml_source1_data_property_value1420 = *source_ml_source1_data_property_value1420;
                                        source_ml_source1_data_property_value1420.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1421, _) => {
                                        let source_ml_source1_data_property_value1421 = *source_ml_source1_data_property_value1421;
                                        source_ml_source1_data_property_value1421.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1414) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1422, _) => {
                                        let source_ml_source1_data_property_value1422 = *source_ml_source1_data_property_value1422;
                                        source_ml_source1_data_property_value1422.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1415) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1423, _) => {
                                        let source_ml_source1_data_property_value1423 = *source_ml_source1_data_property_value1423;
                                        source_ml_source1_data_property_value1423.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1416) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1424, _) => {
                                        let source_ml_source1_data_property_value1424 = *source_ml_source1_data_property_value1424;
                                        source_ml_source1_data_property_value1424.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1417) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1425, _) => {
                                        let source_ml_source1_data_property_value1425 = *source_ml_source1_data_property_value1425;
                                        source_ml_source1_data_property_value1425.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1418) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1426, _) => {
                                        let source_ml_source1_data_property_value1426 = *source_ml_source1_data_property_value1426;
                                        source_ml_source1_data_property_value1426.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f1419) => {
                                {
                                    let plan_2 = self::plan_java_script_ordinary_set(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1427, _) => {
                                            let source_ml_source1_data_property_value1427 = *source_ml_source1_data_property_value1427;
                                            source_ml_source1_data_property_value1427.clone()
                                        }
                                    }, identity_2.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(value))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                    if (((plan_2.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1428, _) => {
                                                let source_ml_source1_data_property_value1428 = *source_ml_source1_data_property_value1428;
                                                source_ml_source1_data_property_value1428.clone()
                                            }
                                        }), (plan_2.clone()).3)
                                    } else {
                                        if (((plan_2.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1429, _) => {
                                                    let source_ml_source1_data_property_value1429 = *source_ml_source1_data_property_value1429;
                                                    source_ml_source1_data_property_value1429.clone()
                                                }
                                            }, match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1430) => {
                                                    source_ml_source1_data_property_value1430
                                                }
                                            }, {
                                                let source_ml_source1_nominal_spread1431 = next.clone();
                                                {
                                                    let source_ml_source1_nominal_field1432 = match source_ml_source1_nominal_spread1431.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1433, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value1433.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field1434 = match source_ml_source1_nominal_spread1431.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1435, _, _, _, _) => {
                                                                source_ml_source1_data_property_value1435
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field1436 = match source_ml_source1_nominal_spread1431.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1437, _, _, _) => {
                                                                    source_ml_source1_data_property_value1437
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field1438 = match source_ml_source1_nominal_spread1431.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1439, _, _) => {
                                                                        source_ml_source1_data_property_value1439
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field1440 = match source_ml_source1_nominal_spread1431.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1441, _) => {
                                                                            source_ml_source1_data_property_value1441
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field1442 = match source_ml_source1_nominal_spread1431.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1443) => {
                                                                                let source_ml_source1_data_property_value1443 = *source_ml_source1_data_property_value1443;
                                                                                source_ml_source1_data_property_value1443.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_override1444 = self::JavaScriptCallPending::WaitSet(destination_2);
                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1432.clone(), source_ml_source1_nominal_field1434, source_ml_source1_nominal_field1436, source_ml_source1_nominal_field1438, source_ml_source1_nominal_field1440, Box::new(source_ml_source1_nominal_override1444.clone()))
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
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1445) => {
                                                    source_ml_source1_data_property_value1445
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
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1446, _) => {
                            let source_ml_source1_data_property_value1446 = *source_ml_source1_data_property_value1446;
                            source_ml_source1_data_property_value1446.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::PropertyDefinition(source_ml_source1_f1362, source_ml_source1_f1363, source_ml_source1_f1364, source_ml_source1_f1365, source_ml_source1_f1366) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1447, _) => {
                            let source_ml_source1_data_property_value1447 = *source_ml_source1_data_property_value1447;
                            source_ml_source1_data_property_value1447.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
            }
        }
    }
}
