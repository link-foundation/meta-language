// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::resume_java_script_property_definition;
use super::resume_java_script_property_key_conversion;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_key(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptPropertyKeyConversionTransition) -> self::JavaScriptCallMachineTransition {
    {
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1358) => source_ml_source2_data_property_value1358.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1359) => source_ml_source2_data_property_value1359.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1360, _) => {
                    let source_ml_source2_data_property_value1360 = *source_ml_source2_data_property_value1360;
                    source_ml_source2_data_property_value1360.clone()
                }
            }), self::ml_string::Text::from_units(vec![107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
        } else {
            match transition.clone() {
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(state) => {
                    let state = *state;
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1361, _) => {
                            let source_ml_source2_data_property_value1361 = *source_ml_source2_data_property_value1361;
                            source_ml_source2_data_property_value1361.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1362) => {
                            source_ml_source2_data_property_value1362
                        }
                    }, {
                        let source_ml_source2_nominal_spread1363 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source2_nominal_field1364 = match source_ml_source2_nominal_spread1363.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1365, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1365.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field1366 = match source_ml_source2_nominal_spread1363.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1367, _, _, _, _) => {
                                        source_ml_source2_data_property_value1367
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1368 = match source_ml_source2_nominal_spread1363.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1369, _, _, _) => {
                                            source_ml_source2_data_property_value1369
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1370 = match source_ml_source2_nominal_spread1363.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1371, _, _) => {
                                                source_ml_source2_data_property_value1371
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1372 = match source_ml_source2_nominal_spread1363.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1373, _) => {
                                                    source_ml_source2_data_property_value1373
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1374 = match source_ml_source2_nominal_spread1363.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1375) => {
                                                        let source_ml_source2_data_property_value1375 = *source_ml_source2_data_property_value1375;
                                                        source_ml_source2_data_property_value1375.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override1376 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1364.clone(), source_ml_source2_nominal_field1366, source_ml_source2_nominal_field1368, source_ml_source2_nominal_field1370, source_ml_source2_nominal_field1372, Box::new(source_ml_source2_nominal_override1376.clone()))
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
                    self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1377, _) => {
                            let source_ml_source2_data_property_value1377 = *source_ml_source2_data_property_value1377;
                            source_ml_source2_data_property_value1377.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1378) => {
                            source_ml_source2_data_property_value1378
                        }
                    }, {
                        let source_ml_source2_nominal_spread1379 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source2_nominal_field1380 = match source_ml_source2_nominal_spread1379.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1381, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1381.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field1382 = match source_ml_source2_nominal_spread1379.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1383, _, _, _, _) => {
                                        source_ml_source2_data_property_value1383
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1384 = match source_ml_source2_nominal_spread1379.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1385, _, _, _) => {
                                            source_ml_source2_data_property_value1385
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1386 = match source_ml_source2_nominal_spread1379.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1387, _, _) => {
                                                source_ml_source2_data_property_value1387
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1388 = match source_ml_source2_nominal_spread1379.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1389, _) => {
                                                    source_ml_source2_data_property_value1389
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1390 = match source_ml_source2_nominal_spread1379.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1391) => {
                                                        let source_ml_source2_data_property_value1391 = *source_ml_source2_data_property_value1391;
                                                        source_ml_source2_data_property_value1391.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override1392 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state_2.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1380.clone(), source_ml_source2_nominal_field1382, source_ml_source2_nominal_field1384, source_ml_source2_nominal_field1386, source_ml_source2_nominal_field1388, Box::new(source_ml_source2_nominal_override1392.clone()))
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
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1393, _) => {
                            let source_ml_source2_data_property_value1393 = *source_ml_source2_data_property_value1393;
                            source_ml_source2_data_property_value1393.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1394) => {
                            source_ml_source2_data_property_value1394
                        }
                    }, self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, key.clone()))))
                }
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyThrow(value) => {
                    let value = *value;
                    self::throw_java_script_call_value(machine.clone(), value.clone())
                }
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyIntrinsicError(error_class, reason) => {
                    self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                }
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(reason_2) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1395, _) => {
                            let source_ml_source2_data_property_value1395 = *source_ml_source2_data_property_value1395;
                            source_ml_source2_data_property_value1395.clone()
                        }
                    }), reason_2.clone())
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_definition(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptPropertyDefinitionTransition) -> self::JavaScriptCallMachineTransition {
    {
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1396) => source_ml_source2_data_property_value1396.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1397) => source_ml_source2_data_property_value1397.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1398, _) => {
                    let source_ml_source2_data_property_value1398 = *source_ml_source2_data_property_value1398;
                    source_ml_source2_data_property_value1398.clone()
                }
            }), self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
        } else {
            match transition.clone() {
                self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(state) => {
                    let state = *state;
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1400, _) => {
                            let source_ml_source2_data_property_value1400 = *source_ml_source2_data_property_value1400;
                            source_ml_source2_data_property_value1400.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1401) => {
                            source_ml_source2_data_property_value1401
                        }
                    }, {
                        let source_ml_source2_nominal_spread1402 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source2_nominal_field1403 = match source_ml_source2_nominal_spread1402.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1404, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1404.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field1405 = match source_ml_source2_nominal_spread1402.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1406, _, _, _, _) => {
                                        source_ml_source2_data_property_value1406
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1407 = match source_ml_source2_nominal_spread1402.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1408, _, _, _) => {
                                            source_ml_source2_data_property_value1408
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1409 = match source_ml_source2_nominal_spread1402.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1410, _, _) => {
                                                source_ml_source2_data_property_value1410
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1411 = match source_ml_source2_nominal_spread1402.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1412, _) => {
                                                    source_ml_source2_data_property_value1412
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1413 = match source_ml_source2_nominal_spread1402.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1414) => {
                                                        let source_ml_source2_data_property_value1414 = *source_ml_source2_data_property_value1414;
                                                        source_ml_source2_data_property_value1414.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override1415 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1403.clone(), source_ml_source2_nominal_field1405, source_ml_source2_nominal_field1407, source_ml_source2_nominal_field1409, source_ml_source2_nominal_field1411, Box::new(source_ml_source2_nominal_override1415.clone()))
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
                    self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1416, _) => {
                            let source_ml_source2_data_property_value1416 = *source_ml_source2_data_property_value1416;
                            source_ml_source2_data_property_value1416.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1417) => {
                            source_ml_source2_data_property_value1417
                        }
                    }, {
                        let source_ml_source2_nominal_spread1418 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source2_nominal_field1419 = match source_ml_source2_nominal_spread1418.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1420, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1420.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field1421 = match source_ml_source2_nominal_spread1418.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1422, _, _, _, _) => {
                                        source_ml_source2_data_property_value1422
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1423 = match source_ml_source2_nominal_spread1418.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1424, _, _, _) => {
                                            source_ml_source2_data_property_value1424
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1425 = match source_ml_source2_nominal_spread1418.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1426, _, _) => {
                                                source_ml_source2_data_property_value1426
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1427 = match source_ml_source2_nominal_spread1418.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1428, _) => {
                                                    source_ml_source2_data_property_value1428
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1429 = match source_ml_source2_nominal_spread1418.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1430) => {
                                                        let source_ml_source2_data_property_value1430 = *source_ml_source2_data_property_value1430;
                                                        source_ml_source2_data_property_value1430.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override1431 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state_2.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1419.clone(), source_ml_source2_nominal_field1421, source_ml_source2_nominal_field1423, source_ml_source2_nominal_field1425, source_ml_source2_nominal_field1427, Box::new(source_ml_source2_nominal_override1431.clone()))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }), callee.clone(), receiver.clone(), arguments)
                }
                self::JavaScriptPropertyDefinitionTransition::DefinitionCompleted(heap, value, source_ml_source2_f1399) => {
                    let heap = *heap;
                    let value = *value;
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1432) => {
                            source_ml_source2_data_property_value1432
                        }
                    }, self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                }
                self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(value_2) => {
                    let value_2 = *value_2;
                    self::throw_java_script_call_value(machine.clone(), value_2.clone())
                }
                self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(error_class, reason) => {
                    self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                }
                self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(reason_2) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1433, _) => {
                            let source_ml_source2_data_property_value1433 = *source_ml_source2_data_property_value1433;
                            source_ml_source2_data_property_value1433.clone()
                        }
                    }), reason_2.clone())
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn return_java_script_call_value(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    {
        let frames = self::ml_array::slice((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1435) => source_ml_source2_data_property_value1435.as_slice() }), 0f64, (-1f64));
        {
            let parent = self::ml_array::optional_at(&frames, self::ml_array::number_index(((frames.len() as f64) - 1f64)));
            if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                self::JavaScriptCallMachineTransition::MachineCompleted(Box::new(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1436, _) => {
                        let source_ml_source2_data_property_value1436 = *source_ml_source2_data_property_value1436;
                        source_ml_source2_data_property_value1436.clone()
                    }
                }), Box::new(value.clone()))
            } else {
                {
                    let current = self::JavaScriptCallMachine::CallMachine(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1437, _) => {
                            let source_ml_source2_data_property_value1437 = *source_ml_source2_data_property_value1437;
                            source_ml_source2_data_property_value1437.clone()
                        }
                    }), frames.clone());
                    {
                        let pending = match parent.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1438) => {
                                let source_ml_source2_data_property_value1438 = *source_ml_source2_data_property_value1438;
                                source_ml_source2_data_property_value1438.clone()
                            }
                        };
                        match pending.clone() {
                            self::JavaScriptCallPending::CallIdle => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1442, _) => {
                                        let source_ml_source2_data_property_value1442 = *source_ml_source2_data_property_value1442;
                                        source_ml_source2_data_property_value1442.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                            }
                            self::JavaScriptCallPending::WaitCall(destination) => {
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1443, _) => {
                                        let source_ml_source2_data_property_value1443 = *source_ml_source2_data_property_value1443;
                                        source_ml_source2_data_property_value1443.clone()
                                    }
                                }, frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination, value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitGet(destination_2) => {
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1444, _) => {
                                        let source_ml_source2_data_property_value1444 = *source_ml_source2_data_property_value1444;
                                        source_ml_source2_data_property_value1444.clone()
                                    }
                                }, frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination_2, value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitSet(destination_3) => {
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1445, _) => {
                                        let source_ml_source2_data_property_value1445 = *source_ml_source2_data_property_value1445;
                                        source_ml_source2_data_property_value1445.clone()
                                    }
                                }, frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination_3, self::JavaScriptSemanticValue::Boolean(true)))))
                            }
                            self::JavaScriptCallPending::WaitKey(destination_4, realm, state) => {
                                let state = *state;
                                self::accept_java_script_call_key(program.clone(), current.clone(), destination_4, realm.clone(), self::resume_java_script_property_key_conversion(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1446, _) => {
                                        let source_ml_source2_data_property_value1446 = *source_ml_source2_data_property_value1446;
                                        source_ml_source2_data_property_value1446.clone()
                                    }
                                }, state.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitDefinition(destination_5, realm_2, state_2) => {
                                let state_2 = *state_2;
                                self::accept_java_script_call_definition(program.clone(), current.clone(), destination_5, realm_2.clone(), self::resume_java_script_property_definition(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1447, _) => {
                                        let source_ml_source2_data_property_value1447 = *source_ml_source2_data_property_value1447;
                                        source_ml_source2_data_property_value1447.clone()
                                    }
                                }, state_2.clone(), self::JavaScriptSemanticCompletion::NormalCompletion(Box::new(value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitError(source_ml_source2_f1439, source_ml_source2_f1440, source_ml_source2_f1441) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1448, _) => {
                                        let source_ml_source2_data_property_value1448 = *source_ml_source2_data_property_value1448;
                                        source_ml_source2_data_property_value1448.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                            }
                        }
                    }
                }
            }
        }
    }
}
