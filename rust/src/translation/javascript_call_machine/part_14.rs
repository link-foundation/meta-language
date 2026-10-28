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
            let source_ml_source1_nominal_spread1304 = frame.clone();
            {
                let source_ml_source1_nominal_field1305 = match source_ml_source1_nominal_spread1304.clone() {
                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1306, _, _, _, _, _) => {
                        source_ml_source1_data_property_value1306.clone()
                    }
                };
                {
                    let source_ml_source1_nominal_field1307 = match source_ml_source1_nominal_spread1304.clone() {
                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1308, _, _, _, _) => {
                            source_ml_source1_data_property_value1308
                        }
                    };
                    {
                        let source_ml_source1_nominal_field1309 = match source_ml_source1_nominal_spread1304.clone() {
                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1310, _, _, _) => {
                                source_ml_source1_data_property_value1310
                            }
                        };
                        {
                            let source_ml_source1_nominal_field1311 = match source_ml_source1_nominal_spread1304.clone() {
                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1312, _, _) => {
                                    source_ml_source1_data_property_value1312
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1313 = match source_ml_source1_nominal_spread1304.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1314, _) => {
                                        source_ml_source1_data_property_value1314
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1315 = match source_ml_source1_nominal_spread1304.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1316) => {
                                            let source_ml_source1_data_property_value1316 = *source_ml_source1_data_property_value1316;
                                            source_ml_source1_data_property_value1316.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_override1317 = (match frame.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1303, _, _, _, _) => {
                                                source_ml_source1_data_property_value1303
                                            }
                                        } + 1f64);
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1305.clone(), source_ml_source1_nominal_override1317, source_ml_source1_nominal_field1309, source_ml_source1_nominal_field1311, source_ml_source1_nominal_field1313, Box::new(source_ml_source1_nominal_field1315.clone()))
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
                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1318, _, _, _) => {
                    source_ml_source1_data_property_value1318
                }
            };
            match instruction.clone() {
                self::JavaScriptCallInstruction::Constant(source_ml_source1_f1319, source_ml_source1_f1320) => {
                    let source_ml_source1_f1320 = *source_ml_source1_f1320;
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1352, _) => {
                            let source_ml_source1_data_property_value1352 = *source_ml_source1_data_property_value1352;
                            source_ml_source1_data_property_value1352.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Copy(source_ml_source1_f1321, source_ml_source1_f1322) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1353, _) => {
                            let source_ml_source1_data_property_value1353 = *source_ml_source1_data_property_value1353;
                            source_ml_source1_data_property_value1353.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Argument(source_ml_source1_f1323, source_ml_source1_f1324) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1354, _) => {
                            let source_ml_source1_data_property_value1354 = *source_ml_source1_data_property_value1354;
                            source_ml_source1_data_property_value1354.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Jump(source_ml_source1_f1325) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1355, _) => {
                            let source_ml_source1_data_property_value1355 = *source_ml_source1_data_property_value1355;
                            source_ml_source1_data_property_value1355.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Branch(source_ml_source1_f1326, source_ml_source1_f1327, source_ml_source1_f1328) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1356, _) => {
                            let source_ml_source1_data_property_value1356 = *source_ml_source1_data_property_value1356;
                            source_ml_source1_data_property_value1356.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Call(source_ml_source1_f1329, source_ml_source1_f1330, source_ml_source1_f1331, source_ml_source1_f1332) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1357, _) => {
                            let source_ml_source1_data_property_value1357 = *source_ml_source1_data_property_value1357;
                            source_ml_source1_data_property_value1357.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Return(source_ml_source1_f1333) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1358, _) => {
                            let source_ml_source1_data_property_value1358 = *source_ml_source1_data_property_value1358;
                            source_ml_source1_data_property_value1358.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::Throw(source_ml_source1_f1334) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1359, _) => {
                            let source_ml_source1_data_property_value1359 = *source_ml_source1_data_property_value1359;
                            source_ml_source1_data_property_value1359.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::PushHandler(source_ml_source1_f1335, source_ml_source1_f1336) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1360, _) => {
                            let source_ml_source1_data_property_value1360 = *source_ml_source1_data_property_value1360;
                            source_ml_source1_data_property_value1360.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::PopHandler => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1361, _) => {
                            let source_ml_source1_data_property_value1361 = *source_ml_source1_data_property_value1361;
                            source_ml_source1_data_property_value1361.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::NumberBinary(source_ml_source1_f1337, source_ml_source1_f1338, source_ml_source1_f1339, source_ml_source1_f1340) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1362, _) => {
                            let source_ml_source1_data_property_value1362 = *source_ml_source1_data_property_value1362;
                            source_ml_source1_data_property_value1362.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::SameValue(source_ml_source1_f1341, source_ml_source1_f1342, source_ml_source1_f1343) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1363, _) => {
                            let source_ml_source1_data_property_value1363 = *source_ml_source1_data_property_value1363;
                            source_ml_source1_data_property_value1363.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::OrdinaryGet(destination, instruction_object, key, receiver) => {
                    {
                        let object = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_object))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        match object.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1370, _) => {
                                        let source_ml_source1_data_property_value1370 = *source_ml_source1_data_property_value1370;
                                        source_ml_source1_data_property_value1370.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1371, _) => {
                                        let source_ml_source1_data_property_value1371 = *source_ml_source1_data_property_value1371;
                                        source_ml_source1_data_property_value1371.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1364) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1372, _) => {
                                        let source_ml_source1_data_property_value1372 = *source_ml_source1_data_property_value1372;
                                        source_ml_source1_data_property_value1372.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1365) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1373, _) => {
                                        let source_ml_source1_data_property_value1373 = *source_ml_source1_data_property_value1373;
                                        source_ml_source1_data_property_value1373.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1366) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1374, _) => {
                                        let source_ml_source1_data_property_value1374 = *source_ml_source1_data_property_value1374;
                                        source_ml_source1_data_property_value1374.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1367) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1375, _) => {
                                        let source_ml_source1_data_property_value1375 = *source_ml_source1_data_property_value1375;
                                        source_ml_source1_data_property_value1375.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1368) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1376, _) => {
                                        let source_ml_source1_data_property_value1376 = *source_ml_source1_data_property_value1376;
                                        source_ml_source1_data_property_value1376.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f1369) => {
                                {
                                    let plan = self::plan_java_script_ordinary_get(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1377, _) => {
                                            let source_ml_source1_data_property_value1377 = *source_ml_source1_data_property_value1377;
                                            source_ml_source1_data_property_value1377.clone()
                                        }
                                    }, identity.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                    if (((plan.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1378, _) => {
                                                let source_ml_source1_data_property_value1378 = *source_ml_source1_data_property_value1378;
                                                source_ml_source1_data_property_value1378.clone()
                                            }
                                        }), (plan.clone()).3)
                                    } else {
                                        if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1379, _) => {
                                                    let source_ml_source1_data_property_value1379 = *source_ml_source1_data_property_value1379;
                                                    source_ml_source1_data_property_value1379.clone()
                                                }
                                            }, match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1380) => {
                                                    source_ml_source1_data_property_value1380
                                                }
                                            }, self::write_java_script_call_register(next.clone(), destination, (plan.clone()).5))))
                                        } else {
                                            if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1381, _) => {
                                                        let source_ml_source1_data_property_value1381 = *source_ml_source1_data_property_value1381;
                                                        source_ml_source1_data_property_value1381.clone()
                                                    }
                                                }, match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1382) => {
                                                        source_ml_source1_data_property_value1382
                                                    }
                                                }, {
                                                    let source_ml_source1_nominal_spread1383 = next.clone();
                                                    {
                                                        let source_ml_source1_nominal_field1384 = match source_ml_source1_nominal_spread1383.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1385, _, _, _, _, _) => {
                                                                source_ml_source1_data_property_value1385.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field1386 = match source_ml_source1_nominal_spread1383.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1387, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value1387
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field1388 = match source_ml_source1_nominal_spread1383.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1389, _, _, _) => {
                                                                        source_ml_source1_data_property_value1389
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field1390 = match source_ml_source1_nominal_spread1383.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1391, _, _) => {
                                                                            source_ml_source1_data_property_value1391
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field1392 = match source_ml_source1_nominal_spread1383.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1393, _) => {
                                                                                source_ml_source1_data_property_value1393
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_field1394 = match source_ml_source1_nominal_spread1383.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1395) => {
                                                                                    let source_ml_source1_data_property_value1395 = *source_ml_source1_data_property_value1395;
                                                                                    source_ml_source1_data_property_value1395.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_override1396 = self::JavaScriptCallPending::WaitGet(destination);
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1384.clone(), source_ml_source1_nominal_field1386, source_ml_source1_nominal_field1388, source_ml_source1_nominal_field1390, source_ml_source1_nominal_field1392, Box::new(source_ml_source1_nominal_override1396.clone()))
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
                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1397, _) => {
                                                        let source_ml_source1_data_property_value1397 = *source_ml_source1_data_property_value1397;
                                                        source_ml_source1_data_property_value1397.clone()
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
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1404, _) => {
                                        let source_ml_source1_data_property_value1404 = *source_ml_source1_data_property_value1404;
                                        source_ml_source1_data_property_value1404.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1405, _) => {
                                        let source_ml_source1_data_property_value1405 = *source_ml_source1_data_property_value1405;
                                        source_ml_source1_data_property_value1405.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1398) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1406, _) => {
                                        let source_ml_source1_data_property_value1406 = *source_ml_source1_data_property_value1406;
                                        source_ml_source1_data_property_value1406.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1399) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1407, _) => {
                                        let source_ml_source1_data_property_value1407 = *source_ml_source1_data_property_value1407;
                                        source_ml_source1_data_property_value1407.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1400) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1408, _) => {
                                        let source_ml_source1_data_property_value1408 = *source_ml_source1_data_property_value1408;
                                        source_ml_source1_data_property_value1408.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1401) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1409, _) => {
                                        let source_ml_source1_data_property_value1409 = *source_ml_source1_data_property_value1409;
                                        source_ml_source1_data_property_value1409.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1402) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1410, _) => {
                                        let source_ml_source1_data_property_value1410 = *source_ml_source1_data_property_value1410;
                                        source_ml_source1_data_property_value1410.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                            }
                            self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f1403) => {
                                {
                                    let plan_2 = self::plan_java_script_ordinary_set(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1411, _) => {
                                            let source_ml_source1_data_property_value1411 = *source_ml_source1_data_property_value1411;
                                            source_ml_source1_data_property_value1411.clone()
                                        }
                                    }, identity_2.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(value))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                    if (((plan_2.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1412, _) => {
                                                let source_ml_source1_data_property_value1412 = *source_ml_source1_data_property_value1412;
                                                source_ml_source1_data_property_value1412.clone()
                                            }
                                        }), (plan_2.clone()).3)
                                    } else {
                                        if (((plan_2.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1413, _) => {
                                                    let source_ml_source1_data_property_value1413 = *source_ml_source1_data_property_value1413;
                                                    source_ml_source1_data_property_value1413.clone()
                                                }
                                            }, match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1414) => {
                                                    source_ml_source1_data_property_value1414
                                                }
                                            }, {
                                                let source_ml_source1_nominal_spread1415 = next.clone();
                                                {
                                                    let source_ml_source1_nominal_field1416 = match source_ml_source1_nominal_spread1415.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1417, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value1417.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field1418 = match source_ml_source1_nominal_spread1415.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1419, _, _, _, _) => {
                                                                source_ml_source1_data_property_value1419
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field1420 = match source_ml_source1_nominal_spread1415.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1421, _, _, _) => {
                                                                    source_ml_source1_data_property_value1421
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field1422 = match source_ml_source1_nominal_spread1415.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1423, _, _) => {
                                                                        source_ml_source1_data_property_value1423
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field1424 = match source_ml_source1_nominal_spread1415.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1425, _) => {
                                                                            source_ml_source1_data_property_value1425
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field1426 = match source_ml_source1_nominal_spread1415.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1427) => {
                                                                                let source_ml_source1_data_property_value1427 = *source_ml_source1_data_property_value1427;
                                                                                source_ml_source1_data_property_value1427.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_override1428 = self::JavaScriptCallPending::WaitSet(destination_2);
                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1416.clone(), source_ml_source1_nominal_field1418, source_ml_source1_nominal_field1420, source_ml_source1_nominal_field1422, source_ml_source1_nominal_field1424, Box::new(source_ml_source1_nominal_override1428.clone()))
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
                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1429) => {
                                                    source_ml_source1_data_property_value1429
                                                }
                                            }, self::write_java_script_call_register(next.clone(), destination_2, self::JavaScriptSemanticValue::Boolean((plan_2.clone()).6)))))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptCallInstruction::PropertyKey(source_ml_source1_f1344, source_ml_source1_f1345, source_ml_source1_f1346) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1430, _) => {
                            let source_ml_source1_data_property_value1430 = *source_ml_source1_data_property_value1430;
                            source_ml_source1_data_property_value1430.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
                self::JavaScriptCallInstruction::PropertyDefinition(source_ml_source1_f1347, source_ml_source1_f1348, source_ml_source1_f1349, source_ml_source1_f1350, source_ml_source1_f1351) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1431, _) => {
                            let source_ml_source1_data_property_value1431 = *source_ml_source1_data_property_value1431;
                            source_ml_source1_data_property_value1431.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                }
            }
        }
    }
}
