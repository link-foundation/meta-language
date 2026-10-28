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
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1279) => source_ml_source1_data_property_value1279.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1280) => source_ml_source1_data_property_value1280.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1281, _) => {
                    let source_ml_source1_data_property_value1281 = *source_ml_source1_data_property_value1281;
                    source_ml_source1_data_property_value1281.clone()
                }
            }), self::ml_string::Text::from_units(vec![107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
        } else {
            match transition.clone() {
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(state) => {
                    let state = *state;
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1282, _) => {
                            let source_ml_source1_data_property_value1282 = *source_ml_source1_data_property_value1282;
                            source_ml_source1_data_property_value1282.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1283) => {
                            source_ml_source1_data_property_value1283
                        }
                    }, {
                        let source_ml_source1_nominal_spread1284 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source1_nominal_field1285 = match source_ml_source1_nominal_spread1284.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1286, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1286.clone()
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1287 = match source_ml_source1_nominal_spread1284.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1288, _, _, _, _) => {
                                        source_ml_source1_data_property_value1288
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1289 = match source_ml_source1_nominal_spread1284.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1290, _, _, _) => {
                                            source_ml_source1_data_property_value1290
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1291 = match source_ml_source1_nominal_spread1284.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1292, _, _) => {
                                                source_ml_source1_data_property_value1292
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1293 = match source_ml_source1_nominal_spread1284.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1294, _) => {
                                                    source_ml_source1_data_property_value1294
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field1295 = match source_ml_source1_nominal_spread1284.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1296) => {
                                                        let source_ml_source1_data_property_value1296 = *source_ml_source1_data_property_value1296;
                                                        source_ml_source1_data_property_value1296.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override1297 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1285.clone(), source_ml_source1_nominal_field1287, source_ml_source1_nominal_field1289, source_ml_source1_nominal_field1291, source_ml_source1_nominal_field1293, Box::new(source_ml_source1_nominal_override1297.clone()))
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
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1298, _) => {
                            let source_ml_source1_data_property_value1298 = *source_ml_source1_data_property_value1298;
                            source_ml_source1_data_property_value1298.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1299) => {
                            source_ml_source1_data_property_value1299
                        }
                    }, {
                        let source_ml_source1_nominal_spread1300 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source1_nominal_field1301 = match source_ml_source1_nominal_spread1300.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1302, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1302.clone()
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1303 = match source_ml_source1_nominal_spread1300.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1304, _, _, _, _) => {
                                        source_ml_source1_data_property_value1304
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1305 = match source_ml_source1_nominal_spread1300.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1306, _, _, _) => {
                                            source_ml_source1_data_property_value1306
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1307 = match source_ml_source1_nominal_spread1300.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1308, _, _) => {
                                                source_ml_source1_data_property_value1308
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1309 = match source_ml_source1_nominal_spread1300.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1310, _) => {
                                                    source_ml_source1_data_property_value1310
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field1311 = match source_ml_source1_nominal_spread1300.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1312) => {
                                                        let source_ml_source1_data_property_value1312 = *source_ml_source1_data_property_value1312;
                                                        source_ml_source1_data_property_value1312.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override1313 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state_2.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1301.clone(), source_ml_source1_nominal_field1303, source_ml_source1_nominal_field1305, source_ml_source1_nominal_field1307, source_ml_source1_nominal_field1309, Box::new(source_ml_source1_nominal_override1313.clone()))
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
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1314, _) => {
                            let source_ml_source1_data_property_value1314 = *source_ml_source1_data_property_value1314;
                            source_ml_source1_data_property_value1314.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1315) => {
                            source_ml_source1_data_property_value1315
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
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1316, _) => {
                            let source_ml_source1_data_property_value1316 = *source_ml_source1_data_property_value1316;
                            source_ml_source1_data_property_value1316.clone()
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
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1317) => source_ml_source1_data_property_value1317.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1318) => source_ml_source1_data_property_value1318.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1319, _) => {
                    let source_ml_source1_data_property_value1319 = *source_ml_source1_data_property_value1319;
                    source_ml_source1_data_property_value1319.clone()
                }
            }), self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
        } else {
            match transition.clone() {
                self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(state) => {
                    let state = *state;
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1321, _) => {
                            let source_ml_source1_data_property_value1321 = *source_ml_source1_data_property_value1321;
                            source_ml_source1_data_property_value1321.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1322) => {
                            source_ml_source1_data_property_value1322
                        }
                    }, {
                        let source_ml_source1_nominal_spread1323 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source1_nominal_field1324 = match source_ml_source1_nominal_spread1323.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1325, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1325.clone()
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1326 = match source_ml_source1_nominal_spread1323.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1327, _, _, _, _) => {
                                        source_ml_source1_data_property_value1327
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1328 = match source_ml_source1_nominal_spread1323.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1329, _, _, _) => {
                                            source_ml_source1_data_property_value1329
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1330 = match source_ml_source1_nominal_spread1323.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1331, _, _) => {
                                                source_ml_source1_data_property_value1331
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1332 = match source_ml_source1_nominal_spread1323.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1333, _) => {
                                                    source_ml_source1_data_property_value1333
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field1334 = match source_ml_source1_nominal_spread1323.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1335) => {
                                                        let source_ml_source1_data_property_value1335 = *source_ml_source1_data_property_value1335;
                                                        source_ml_source1_data_property_value1335.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override1336 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1324.clone(), source_ml_source1_nominal_field1326, source_ml_source1_nominal_field1328, source_ml_source1_nominal_field1330, source_ml_source1_nominal_field1332, Box::new(source_ml_source1_nominal_override1336.clone()))
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
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1337, _) => {
                            let source_ml_source1_data_property_value1337 = *source_ml_source1_data_property_value1337;
                            source_ml_source1_data_property_value1337.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1338) => {
                            source_ml_source1_data_property_value1338
                        }
                    }, {
                        let source_ml_source1_nominal_spread1339 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source1_nominal_field1340 = match source_ml_source1_nominal_spread1339.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1341, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1341.clone()
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1342 = match source_ml_source1_nominal_spread1339.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1343, _, _, _, _) => {
                                        source_ml_source1_data_property_value1343
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1344 = match source_ml_source1_nominal_spread1339.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1345, _, _, _) => {
                                            source_ml_source1_data_property_value1345
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1346 = match source_ml_source1_nominal_spread1339.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1347, _, _) => {
                                                source_ml_source1_data_property_value1347
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1348 = match source_ml_source1_nominal_spread1339.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1349, _) => {
                                                    source_ml_source1_data_property_value1349
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field1350 = match source_ml_source1_nominal_spread1339.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1351) => {
                                                        let source_ml_source1_data_property_value1351 = *source_ml_source1_data_property_value1351;
                                                        source_ml_source1_data_property_value1351.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override1352 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state_2.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1340.clone(), source_ml_source1_nominal_field1342, source_ml_source1_nominal_field1344, source_ml_source1_nominal_field1346, source_ml_source1_nominal_field1348, Box::new(source_ml_source1_nominal_override1352.clone()))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }), callee.clone(), receiver.clone(), arguments)
                }
                self::JavaScriptPropertyDefinitionTransition::DefinitionCompleted(heap, value, source_ml_source1_f1320) => {
                    let heap = *heap;
                    let value = *value;
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1353) => {
                            source_ml_source1_data_property_value1353
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
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1354, _) => {
                            let source_ml_source1_data_property_value1354 = *source_ml_source1_data_property_value1354;
                            source_ml_source1_data_property_value1354.clone()
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
        let frames = self::ml_array::slice((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1356) => source_ml_source1_data_property_value1356.as_slice() }), 0f64, (-1f64));
        {
            let parent = self::ml_array::optional_at(&frames, self::ml_array::number_index(((frames.len() as f64) - 1f64)));
            if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                self::JavaScriptCallMachineTransition::MachineCompleted(Box::new(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1357, _) => {
                        let source_ml_source1_data_property_value1357 = *source_ml_source1_data_property_value1357;
                        source_ml_source1_data_property_value1357.clone()
                    }
                }), Box::new(value.clone()))
            } else {
                {
                    let current = self::JavaScriptCallMachine::CallMachine(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1358, _) => {
                            let source_ml_source1_data_property_value1358 = *source_ml_source1_data_property_value1358;
                            source_ml_source1_data_property_value1358.clone()
                        }
                    }), frames.clone());
                    {
                        let pending = match parent.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1359) => {
                                let source_ml_source1_data_property_value1359 = *source_ml_source1_data_property_value1359;
                                source_ml_source1_data_property_value1359.clone()
                            }
                        };
                        match pending.clone() {
                            self::JavaScriptCallPending::CallIdle => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1363, _) => {
                                        let source_ml_source1_data_property_value1363 = *source_ml_source1_data_property_value1363;
                                        source_ml_source1_data_property_value1363.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                            }
                            self::JavaScriptCallPending::WaitCall(destination) => {
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1364, _) => {
                                        let source_ml_source1_data_property_value1364 = *source_ml_source1_data_property_value1364;
                                        source_ml_source1_data_property_value1364.clone()
                                    }
                                }, frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination, value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitGet(destination_2) => {
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1365, _) => {
                                        let source_ml_source1_data_property_value1365 = *source_ml_source1_data_property_value1365;
                                        source_ml_source1_data_property_value1365.clone()
                                    }
                                }, frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination_2, value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitSet(destination_3) => {
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1366, _) => {
                                        let source_ml_source1_data_property_value1366 = *source_ml_source1_data_property_value1366;
                                        source_ml_source1_data_property_value1366.clone()
                                    }
                                }, frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination_3, self::JavaScriptSemanticValue::Boolean(true)))))
                            }
                            self::JavaScriptCallPending::WaitKey(destination_4, realm, state) => {
                                let state = *state;
                                self::accept_java_script_call_key(program.clone(), current.clone(), destination_4, realm.clone(), self::resume_java_script_property_key_conversion(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1367, _) => {
                                        let source_ml_source1_data_property_value1367 = *source_ml_source1_data_property_value1367;
                                        source_ml_source1_data_property_value1367.clone()
                                    }
                                }, state.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitDefinition(destination_5, realm_2, state_2) => {
                                let state_2 = *state_2;
                                self::accept_java_script_call_definition(program.clone(), current.clone(), destination_5, realm_2.clone(), self::resume_java_script_property_definition(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1368, _) => {
                                        let source_ml_source1_data_property_value1368 = *source_ml_source1_data_property_value1368;
                                        source_ml_source1_data_property_value1368.clone()
                                    }
                                }, state_2.clone(), self::JavaScriptSemanticCompletion::NormalCompletion(Box::new(value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitError(source_ml_source1_f1360, source_ml_source1_f1361, source_ml_source1_f1362) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1369, _) => {
                                        let source_ml_source1_data_property_value1369 = *source_ml_source1_data_property_value1369;
                                        source_ml_source1_data_property_value1369.clone()
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
