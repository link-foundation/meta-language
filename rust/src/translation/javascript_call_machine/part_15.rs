// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlTemporaryBeginJavaScriptCallMachineLoop59Result;
use super::accept_java_script_call_definition;
use super::accept_java_script_call_key;
use super::begin_java_script_property_definition;
use super::begin_java_script_property_key_conversion;
use super::convert_java_script_semantic_boolean;
use super::create_java_script_call_frame;
use super::execute_java_script_ordinary_call_property;
use super::find_java_script_call_function;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_string;
use super::push_java_script_call;
use super::read_java_script_call_machine_error;
use super::read_java_script_call_program_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_call_frame;
use super::return_java_script_call_value;
use super::source_ml_temporary_begin_java_script_call_machine_loop59;
use super::step_java_script_property_definition;
use super::step_java_script_property_key_conversion;
use super::test_java_script_semantic_same_value;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_call_machine(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine) -> self::JavaScriptCallMachineTransition {
    {
        let error = self::read_java_script_call_machine_error(program.clone(), machine.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1432, _) => {
                    let source_ml_source1_data_property_value1432 = *source_ml_source1_data_property_value1432;
                    source_ml_source1_data_property_value1432.clone()
                }
            }), error.clone())
        } else {
            {
                let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1433) => source_ml_source1_data_property_value1433.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1434) => source_ml_source1_data_property_value1434.as_slice() }).len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1435, _) => {
                            let source_ml_source1_data_property_value1435 = *source_ml_source1_data_property_value1435;
                            source_ml_source1_data_property_value1435.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    {
                        let pending = match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1436) => {
                                let source_ml_source1_data_property_value1436 = *source_ml_source1_data_property_value1436;
                                source_ml_source1_data_property_value1436.clone()
                            }
                        };
                        match pending.clone() {
                            self::JavaScriptCallPending::CallIdle => {
                                {
                                    let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1455, _, _) => source_ml_source1_data_property_value1455.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1456, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value1456.clone()
                                        }
                                    })));
                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1457, _) => {
                                                let source_ml_source1_data_property_value1457 = *source_ml_source1_data_property_value1457;
                                                source_ml_source1_data_property_value1457.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                    } else {
                                        {
                                            let instruction = self::ml_array::optional_at(&match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value1458) => {
                                                    source_ml_source1_data_property_value1458
                                                }
                                            }, self::ml_array::number_index(match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1459, _, _, _, _) => {
                                                    source_ml_source1_data_property_value1459
                                                }
                                            }));
                                            if match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::return_java_script_call_value(program.clone(), machine.clone(), self::JavaScriptSemanticValue::Undefined)
                                            } else {
                                                {
                                                    let next = {
                                                        let source_ml_source1_nominal_spread1461 = frame.clone().expect("a proven present value");
                                                        {
                                                            let source_ml_source1_nominal_field1462 = match source_ml_source1_nominal_spread1461.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1463, _, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value1463.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field1464 = match source_ml_source1_nominal_spread1461.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1465, _, _, _, _) => {
                                                                        source_ml_source1_data_property_value1465
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field1466 = match source_ml_source1_nominal_spread1461.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1467, _, _, _) => {
                                                                            source_ml_source1_data_property_value1467
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field1468 = match source_ml_source1_nominal_spread1461.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1469, _, _) => {
                                                                                source_ml_source1_data_property_value1469
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_field1470 = match source_ml_source1_nominal_spread1461.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1471, _) => {
                                                                                    source_ml_source1_data_property_value1471
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field1472 = match source_ml_source1_nominal_spread1461.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1473) => {
                                                                                        let source_ml_source1_data_property_value1473 = *source_ml_source1_data_property_value1473;
                                                                                        source_ml_source1_data_property_value1473.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_override1474 = (match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1460, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1460
                                                                                        }
                                                                                    } + 1f64);
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1462.clone(), source_ml_source1_nominal_override1474, source_ml_source1_nominal_field1466, source_ml_source1_nominal_field1468, source_ml_source1_nominal_field1470, Box::new(source_ml_source1_nominal_field1472.clone()))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let current = self::replace_java_script_call_frame(match machine.clone() {
                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1475, _) => {
                                                                let source_ml_source1_data_property_value1475 = *source_ml_source1_data_property_value1475;
                                                                source_ml_source1_data_property_value1475.clone()
                                                            }
                                                        }, match machine.clone() {
                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1476) => {
                                                                source_ml_source1_data_property_value1476
                                                            }
                                                        }, next.clone());
                                                        {
                                                            let registers = match frame.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1477, _, _, _) => {
                                                                    source_ml_source1_data_property_value1477
                                                                }
                                                            };
                                                            match instruction.clone().expect("a proven present value") {
                                                                self::JavaScriptCallInstruction::Constant(destination, value) => {
                                                                    let value = *value;
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1487, _) => {
                                                                            let source_ml_source1_data_property_value1487 = *source_ml_source1_data_property_value1487;
                                                                            source_ml_source1_data_property_value1487.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1488) => {
                                                                            source_ml_source1_data_property_value1488
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination, value.clone()))))
                                                                }
                                                                self::JavaScriptCallInstruction::Copy(destination_2, source) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1489, _) => {
                                                                            let source_ml_source1_data_property_value1489 = *source_ml_source1_data_property_value1489;
                                                                            source_ml_source1_data_property_value1489.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1490) => {
                                                                            source_ml_source1_data_property_value1490
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_2, (self::ml_array::optional_at(&registers, self::ml_array::number_index(source))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                }
                                                                self::JavaScriptCallInstruction::Argument(destination_3, index) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1491, _) => {
                                                                            let source_ml_source1_data_property_value1491 = *source_ml_source1_data_property_value1491;
                                                                            source_ml_source1_data_property_value1491.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1492) => {
                                                                            source_ml_source1_data_property_value1492
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_3, (self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1493, _, _) => {
                                                                            source_ml_source1_data_property_value1493
                                                                        }
                                                                    }, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                }
                                                                self::JavaScriptCallInstruction::Jump(target) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1494, _) => {
                                                                            let source_ml_source1_data_property_value1494 = *source_ml_source1_data_property_value1494;
                                                                            source_ml_source1_data_property_value1494.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1495) => {
                                                                            source_ml_source1_data_property_value1495
                                                                        }
                                                                    }, {
                                                                        let source_ml_source1_nominal_spread1496 = next.clone();
                                                                        {
                                                                            let source_ml_source1_nominal_field1497 = match source_ml_source1_nominal_spread1496.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1498, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value1498.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field1499 = match source_ml_source1_nominal_spread1496.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1500, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1500
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1501 = match source_ml_source1_nominal_spread1496.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1502, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1502
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1503 = match source_ml_source1_nominal_spread1496.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1504, _, _) => {
                                                                                                source_ml_source1_data_property_value1504
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1505 = match source_ml_source1_nominal_spread1496.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1506, _) => {
                                                                                                    source_ml_source1_data_property_value1506
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1507 = match source_ml_source1_nominal_spread1496.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1508) => {
                                                                                                        let source_ml_source1_data_property_value1508 = *source_ml_source1_data_property_value1508;
                                                                                                        source_ml_source1_data_property_value1508.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_override1509 = target;
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1497.clone(), source_ml_source1_nominal_override1509, source_ml_source1_nominal_field1501, source_ml_source1_nominal_field1503, source_ml_source1_nominal_field1505, Box::new(source_ml_source1_nominal_field1507.clone()))
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
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1510, _) => {
                                                                            let source_ml_source1_data_property_value1510 = *source_ml_source1_data_property_value1510;
                                                                            source_ml_source1_data_property_value1510.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1511) => {
                                                                            source_ml_source1_data_property_value1511
                                                                        }
                                                                    }, {
                                                                        let source_ml_source1_nominal_spread1512 = next.clone();
                                                                        {
                                                                            let source_ml_source1_nominal_field1513 = match source_ml_source1_nominal_spread1512.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1514, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value1514.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field1515 = match source_ml_source1_nominal_spread1512.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1516, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1516
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1517 = match source_ml_source1_nominal_spread1512.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1518, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1518
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1519 = match source_ml_source1_nominal_spread1512.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1520, _, _) => {
                                                                                                source_ml_source1_data_property_value1520
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1521 = match source_ml_source1_nominal_spread1512.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1522, _) => {
                                                                                                    source_ml_source1_data_property_value1522
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1523 = match source_ml_source1_nominal_spread1512.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1524) => {
                                                                                                        let source_ml_source1_data_property_value1524 = *source_ml_source1_data_property_value1524;
                                                                                                        source_ml_source1_data_property_value1524.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_override1525 = if self::convert_java_script_semantic_boolean((self::ml_array::optional_at(&registers, self::ml_array::number_index(condition))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)) {
                                                                                                        yes
                                                                                                    } else {
                                                                                                        no
                                                                                                    };
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1513.clone(), source_ml_source1_nominal_override1525, source_ml_source1_nominal_field1517, source_ml_source1_nominal_field1519, source_ml_source1_nominal_field1521, Box::new(source_ml_source1_nominal_field1523.clone()))
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
                                                                        self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1526, _) => {
                                                                                let source_ml_source1_data_property_value1526 = *source_ml_source1_data_property_value1526;
                                                                                source_ml_source1_data_property_value1526.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1527) => {
                                                                                source_ml_source1_data_property_value1527
                                                                            }
                                                                        }, {
                                                                            let source_ml_source1_nominal_spread1528 = next.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field1529 = match source_ml_source1_nominal_spread1528.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1530, _, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1530.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1531 = match source_ml_source1_nominal_spread1528.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1532, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1532
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1533 = match source_ml_source1_nominal_spread1528.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1534, _, _, _) => {
                                                                                                source_ml_source1_data_property_value1534
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1535 = match source_ml_source1_nominal_spread1528.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1536, _, _) => {
                                                                                                    source_ml_source1_data_property_value1536
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1537 = match source_ml_source1_nominal_spread1528.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1538, _) => {
                                                                                                        source_ml_source1_data_property_value1538
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field1539 = match source_ml_source1_nominal_spread1528.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1540) => {
                                                                                                            let source_ml_source1_data_property_value1540 = *source_ml_source1_data_property_value1540;
                                                                                                            source_ml_source1_data_property_value1540.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_override1541 = self::JavaScriptCallPending::WaitCall(destination_4);
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1529.clone(), source_ml_source1_nominal_field1531, source_ml_source1_nominal_field1533, source_ml_source1_nominal_field1535, source_ml_source1_nominal_field1537, Box::new(source_ml_source1_nominal_override1541.clone()))
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
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1543, _) => {
                                                                            source_ml_source1_data_property_value1543
                                                                        }
                                                                    }).len() as f64)) >= (1024f64)) {
                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![104, 97, 110, 100, 108, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1544, _) => {
                                                                                let source_ml_source1_data_property_value1544 = *source_ml_source1_data_property_value1544;
                                                                                source_ml_source1_data_property_value1544.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1545) => {
                                                                                source_ml_source1_data_property_value1545
                                                                            }
                                                                        }, {
                                                                            let source_ml_source1_nominal_spread1549 = next.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field1550 = match source_ml_source1_nominal_spread1549.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1551, _, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1551.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1552 = match source_ml_source1_nominal_spread1549.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1553, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1553
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1554 = match source_ml_source1_nominal_spread1549.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1555, _, _, _) => {
                                                                                                source_ml_source1_data_property_value1555
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1556 = match source_ml_source1_nominal_spread1549.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1557, _, _) => {
                                                                                                    source_ml_source1_data_property_value1557
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1558 = match source_ml_source1_nominal_spread1549.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1559, _) => {
                                                                                                        source_ml_source1_data_property_value1559
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field1560 = match source_ml_source1_nominal_spread1549.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1561) => {
                                                                                                            let source_ml_source1_data_property_value1561 = *source_ml_source1_data_property_value1561;
                                                                                                            source_ml_source1_data_property_value1561.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_override1562 = {
                                                                                                            let source_ml_source1_concat_value1547 = match frame.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1546, _) => {
                                                                                                                    source_ml_source1_data_property_value1546
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value1548 = vec![self::JavaScriptCallHandler::CallHandler(target_2, destination_5)];
                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallHandler>::new(), source_ml_source1_concat_value1547), source_ml_source1_concat_value1548)
                                                                                                            }
                                                                                                        };
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1550.clone(), source_ml_source1_nominal_field1552, source_ml_source1_nominal_field1554, source_ml_source1_nominal_field1556, source_ml_source1_nominal_override1562, Box::new(source_ml_source1_nominal_field1560.clone()))
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
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1564, _) => {
                                                                            source_ml_source1_data_property_value1564
                                                                        }
                                                                    }).len() as f64)) == (0f64)) {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1565, _) => {
                                                                                let source_ml_source1_data_property_value1565 = *source_ml_source1_data_property_value1565;
                                                                                source_ml_source1_data_property_value1565.clone()
                                                                            }
                                                                        }), self::ml_string::Text::from_units(vec![97, 32, 104, 97, 110, 100, 108, 101, 114, 32, 112, 111, 112, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 104, 97, 110, 100, 108, 101, 114]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1566, _) => {
                                                                                let source_ml_source1_data_property_value1566 = *source_ml_source1_data_property_value1566;
                                                                                source_ml_source1_data_property_value1566.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1567) => {
                                                                                source_ml_source1_data_property_value1567
                                                                            }
                                                                        }, {
                                                                            let source_ml_source1_nominal_spread1570 = next.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field1571 = match source_ml_source1_nominal_spread1570.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1572, _, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1572.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1573 = match source_ml_source1_nominal_spread1570.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1574, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1574
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1575 = match source_ml_source1_nominal_spread1570.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1576, _, _, _) => {
                                                                                                source_ml_source1_data_property_value1576
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1577 = match source_ml_source1_nominal_spread1570.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1578, _, _) => {
                                                                                                    source_ml_source1_data_property_value1578
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1579 = match source_ml_source1_nominal_spread1570.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1580, _) => {
                                                                                                        source_ml_source1_data_property_value1580
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field1581 = match source_ml_source1_nominal_spread1570.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1582) => {
                                                                                                            let source_ml_source1_data_property_value1582 = *source_ml_source1_data_property_value1582;
                                                                                                            source_ml_source1_data_property_value1582.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_override1583 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1569, _) => {
                                                                                                                source_ml_source1_data_property_value1569
                                                                                                            }
                                                                                                        }, 0f64, (-1f64));
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1571.clone(), source_ml_source1_nominal_field1573, source_ml_source1_nominal_field1575, source_ml_source1_nominal_field1577, source_ml_source1_nominal_override1583, Box::new(source_ml_source1_nominal_field1581.clone()))
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
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1590, _) => {
                                                                                            let source_ml_source1_data_property_value1590 = *source_ml_source1_data_property_value1590;
                                                                                            source_ml_source1_data_property_value1590.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1591, _) => {
                                                                                            let source_ml_source1_data_property_value1591 = *source_ml_source1_data_property_value1591;
                                                                                            source_ml_source1_data_property_value1591.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1584) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1592, _) => {
                                                                                            let source_ml_source1_data_property_value1592 = *source_ml_source1_data_property_value1592;
                                                                                            source_ml_source1_data_property_value1592.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Number(left_value) => {
                                                                                    match right.clone() {
                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1599, _) => {
                                                                                                    let source_ml_source1_data_property_value1599 = *source_ml_source1_data_property_value1599;
                                                                                                    source_ml_source1_data_property_value1599.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Null => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1600, _) => {
                                                                                                    let source_ml_source1_data_property_value1600 = *source_ml_source1_data_property_value1600;
                                                                                                    source_ml_source1_data_property_value1600.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1593) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1601, _) => {
                                                                                                    let source_ml_source1_data_property_value1601 = *source_ml_source1_data_property_value1601;
                                                                                                    source_ml_source1_data_property_value1601.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Number(right_value) => {
                                                                                            {
                                                                                                let op = operator.clone();
                                                                                                if ((op == (self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]))) || (op == (self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])))) {
                                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1602, _) => {
                                                                                                            let source_ml_source1_data_property_value1602 = *source_ml_source1_data_property_value1602;
                                                                                                            source_ml_source1_data_property_value1602.clone()
                                                                                                        }
                                                                                                    }, match machine.clone() {
                                                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1603) => {
                                                                                                            source_ml_source1_data_property_value1603
                                                                                                        }
                                                                                                    }, self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Boolean(if (op == (self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]))) {
                                                                                                        (left_value < right_value)
                                                                                                    } else {
                                                                                                        (left_value == right_value)
                                                                                                    })))))
                                                                                                } else {
                                                                                                    {
                                                                                                        let value_2 = if (op == (self::ml_string::Text::from_units(vec![97, 100, 100]))) {
                                                                                                            (left_value + right_value)
                                                                                                        } else {
                                                                                                            if (op == (self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116]))) {
                                                                                                                (left_value - right_value)
                                                                                                            } else {
                                                                                                                if (op == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121]))) {
                                                                                                                    (left_value * right_value)
                                                                                                                } else {
                                                                                                                    if (op == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101]))) {
                                                                                                                        (left_value / right_value)
                                                                                                                    } else {
                                                                                                                        if (op == (self::ml_string::Text::from_units(vec![114, 101, 109, 97, 105, 110, 100, 101, 114]))) {
                                                                                                                            (left_value % right_value)
                                                                                                                        } else {
                                                                                                                            self::ml_big_integer_conversion::power_java_script_number(left_value,right_value)
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        };
                                                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1604, _) => {
                                                                                                                let source_ml_source1_data_property_value1604 = *source_ml_source1_data_property_value1604;
                                                                                                                source_ml_source1_data_property_value1604.clone()
                                                                                                            }
                                                                                                        }, match machine.clone() {
                                                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1605) => {
                                                                                                                source_ml_source1_data_property_value1605
                                                                                                            }
                                                                                                        }, self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Number(value_2)))))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1594) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1606, _) => {
                                                                                                    let source_ml_source1_data_property_value1606 = *source_ml_source1_data_property_value1606;
                                                                                                    source_ml_source1_data_property_value1606.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1595) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1607, _) => {
                                                                                                    let source_ml_source1_data_property_value1607 = *source_ml_source1_data_property_value1607;
                                                                                                    source_ml_source1_data_property_value1607.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1596) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1608, _) => {
                                                                                                    let source_ml_source1_data_property_value1608 = *source_ml_source1_data_property_value1608;
                                                                                                    source_ml_source1_data_property_value1608.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1597, source_ml_source1_f1598) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1609, _) => {
                                                                                                    let source_ml_source1_data_property_value1609 = *source_ml_source1_data_property_value1609;
                                                                                                    source_ml_source1_data_property_value1609.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f1585) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1610, _) => {
                                                                                            let source_ml_source1_data_property_value1610 = *source_ml_source1_data_property_value1610;
                                                                                            source_ml_source1_data_property_value1610.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f1586) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1611, _) => {
                                                                                            let source_ml_source1_data_property_value1611 = *source_ml_source1_data_property_value1611;
                                                                                            source_ml_source1_data_property_value1611.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1587) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1612, _) => {
                                                                                            let source_ml_source1_data_property_value1612 = *source_ml_source1_data_property_value1612;
                                                                                            source_ml_source1_data_property_value1612.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f1588, source_ml_source1_f1589) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1613, _) => {
                                                                                            let source_ml_source1_data_property_value1613 = *source_ml_source1_data_property_value1613;
                                                                                            source_ml_source1_data_property_value1613.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::JavaScriptCallInstruction::SameValue(destination_7, left_2, right_2) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1614, _) => {
                                                                            let source_ml_source1_data_property_value1614 = *source_ml_source1_data_property_value1614;
                                                                            source_ml_source1_data_property_value1614.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1615) => {
                                                                            source_ml_source1_data_property_value1615
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_7, self::JavaScriptSemanticValue::Boolean(self::test_java_script_semantic_same_value((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                }
                                                                self::JavaScriptCallInstruction::OrdinaryGet(source_ml_source1_f1478, source_ml_source1_f1479, source_ml_source1_f1480, source_ml_source1_f1481) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::OrdinarySet(source_ml_source1_f1482, source_ml_source1_f1483, source_ml_source1_f1484, source_ml_source1_f1485, source_ml_source1_f1486) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::PropertyKey(destination_8, input, realm) => {
                                                                    self::accept_java_script_call_key(program.clone(), current.clone(), destination_8, realm.clone(), self::begin_java_script_property_key_conversion(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1616, _) => {
                                                                            let source_ml_source1_data_property_value1616 = *source_ml_source1_data_property_value1616;
                                                                            source_ml_source1_data_property_value1616.clone()
                                                                        }
                                                                    }, (self::ml_array::optional_at(&registers, self::ml_array::number_index(input))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match program.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source1_data_property_value1617) => {
                                                                            let source_ml_source1_data_property_value1617 = *source_ml_source1_data_property_value1617;
                                                                            source_ml_source1_data_property_value1617.clone()
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptCallInstruction::PropertyDefinition(destination_9, target_3, key, attributes, realm_2) => {
                                                                    self::accept_java_script_call_definition(program.clone(), current.clone(), destination_9, realm_2.clone(), self::begin_java_script_property_definition(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1618, _) => {
                                                                            let source_ml_source1_data_property_value1618 = *source_ml_source1_data_property_value1618;
                                                                            source_ml_source1_data_property_value1618.clone()
                                                                        }
                                                                    }, (self::ml_array::optional_at(&registers, self::ml_array::number_index(target_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(attributes))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match program.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source1_data_property_value1619) => {
                                                                            let source_ml_source1_data_property_value1619 = *source_ml_source1_data_property_value1619;
                                                                            source_ml_source1_data_property_value1619.clone()
                                                                        }
                                                                    }))
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
                            self::JavaScriptCallPending::WaitCall(source_ml_source1_f1437) => {
                                {
                                    let source_ml_source1_f1620 = source_ml_source1_f1437;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1632, _) => {
                                            let source_ml_source1_data_property_value1632 = *source_ml_source1_data_property_value1632;
                                            source_ml_source1_data_property_value1632.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitGet(source_ml_source1_f1438) => {
                                {
                                    let source_ml_source1_f1634 = source_ml_source1_f1438;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1645, _) => {
                                            let source_ml_source1_data_property_value1645 = *source_ml_source1_data_property_value1645;
                                            source_ml_source1_data_property_value1645.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitSet(source_ml_source1_f1439) => {
                                {
                                    let source_ml_source1_f1648 = source_ml_source1_f1439;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1658, _) => {
                                            let source_ml_source1_data_property_value1658 = *source_ml_source1_data_property_value1658;
                                            source_ml_source1_data_property_value1658.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitKey(destination_10, realm_3, state) => {
                                let state = *state;
                                self::accept_java_script_call_key(program.clone(), machine.clone(), destination_10, realm_3.clone(), self::step_java_script_property_key_conversion(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1659, _) => {
                                        let source_ml_source1_data_property_value1659 = *source_ml_source1_data_property_value1659;
                                        source_ml_source1_data_property_value1659.clone()
                                    }
                                }, state.clone()))
                            }
                            self::JavaScriptCallPending::WaitDefinition(destination_11, realm_4, state_2) => {
                                let state_2 = *state_2;
                                self::accept_java_script_call_definition(program.clone(), machine.clone(), destination_11, realm_4.clone(), self::step_java_script_property_definition(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1660, _) => {
                                        let source_ml_source1_data_property_value1660 = *source_ml_source1_data_property_value1660;
                                        source_ml_source1_data_property_value1660.clone()
                                    }
                                }, state_2.clone()))
                            }
                            self::JavaScriptCallPending::WaitError(source_ml_source1_f1440, source_ml_source1_f1441, source_ml_source1_f1442) => {
                                {
                                    let source_ml_source1_f1670 = source_ml_source1_f1440.clone();
                                    {
                                        let source_ml_source1_f1671 = source_ml_source1_f1441.clone();
                                        {
                                            let source_ml_source1_f1672 = source_ml_source1_f1442.clone();
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1673, _) => {
                                                    let source_ml_source1_data_property_value1673 = *source_ml_source1_data_property_value1673;
                                                    source_ml_source1_data_property_value1673.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
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
pub fn begin_java_script_call_machine(program: self::JavaScriptCallProgram, heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, receiver: self::JavaScriptSemanticValue, arguments_: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallMachineTransition {
    {
        let error = self::read_java_script_call_program_error(program.clone(), heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), error.clone())
        } else {
            {
                let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1674, _, _) => source_ml_source1_data_property_value1674.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), identity.clone())));
                if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 101, 110, 116, 114, 121, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))
                } else {
                    if (((arguments_.len() as f64)) > (1048576f64)) {
                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 97, 108, 108, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                    } else {
                        {
                            let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), receiver_error.clone())
                            } else {
                                {
                                    let source_ml_temporary_values58 = arguments_.clone();
                                    {
                                        let source_ml_temporary_index58 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source1_s1675 = self::source_ml_temporary_begin_java_script_call_machine_loop59(heap.clone(), source_ml_temporary_values58, source_ml_temporary_index58.clone());
                                            match source_ml_source1_s1675.clone() {
                                                self::MlTemporaryBeginJavaScriptCallMachineLoop59Result::MlTemporaryBeginJavaScriptCallMachineLoop59Done => {
                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(heap.clone()), vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_)])))
                                                }
                                                self::MlTemporaryBeginJavaScriptCallMachineLoop59Result::MlTemporaryBeginJavaScriptCallMachineLoop59Return(source_ml_temporary_result59) => {
                                                    let source_ml_temporary_result59 = *source_ml_temporary_result59;
                                                    source_ml_temporary_result59.clone()
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
