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
use super::test_java_script_call_strict_equal;
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
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1467, _) => {
                    let source_ml_source1_data_property_value1467 = *source_ml_source1_data_property_value1467;
                    source_ml_source1_data_property_value1467.clone()
                }
            }), error.clone())
        } else {
            {
                let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1468) => source_ml_source1_data_property_value1468.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1469) => source_ml_source1_data_property_value1469.as_slice() }).len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1470, _) => {
                            let source_ml_source1_data_property_value1470 = *source_ml_source1_data_property_value1470;
                            source_ml_source1_data_property_value1470.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    {
                        let pending = match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1471) => {
                                let source_ml_source1_data_property_value1471 = *source_ml_source1_data_property_value1471;
                                source_ml_source1_data_property_value1471.clone()
                            }
                        };
                        match pending.clone() {
                            self::JavaScriptCallPending::CallIdle => {
                                {
                                    let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1490, _, _) => source_ml_source1_data_property_value1490.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1491, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value1491.clone()
                                        }
                                    })));
                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1492, _) => {
                                                let source_ml_source1_data_property_value1492 = *source_ml_source1_data_property_value1492;
                                                source_ml_source1_data_property_value1492.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                    } else {
                                        {
                                            let instruction = self::ml_array::optional_at(&match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value1493) => {
                                                    source_ml_source1_data_property_value1493
                                                }
                                            }, self::ml_array::number_index(match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1494, _, _, _, _) => {
                                                    source_ml_source1_data_property_value1494
                                                }
                                            }));
                                            if match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::return_java_script_call_value(program.clone(), machine.clone(), self::JavaScriptSemanticValue::Undefined)
                                            } else {
                                                {
                                                    let next = {
                                                        let source_ml_source1_nominal_spread1496 = frame.clone().expect("a proven present value");
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
                                                                                    let source_ml_source1_nominal_override1509 = (match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1495, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1495
                                                                                        }
                                                                                    } + 1f64);
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1497.clone(), source_ml_source1_nominal_override1509, source_ml_source1_nominal_field1501, source_ml_source1_nominal_field1503, source_ml_source1_nominal_field1505, Box::new(source_ml_source1_nominal_field1507.clone()))
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
                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1510, _) => {
                                                                let source_ml_source1_data_property_value1510 = *source_ml_source1_data_property_value1510;
                                                                source_ml_source1_data_property_value1510.clone()
                                                            }
                                                        }, match machine.clone() {
                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1511) => {
                                                                source_ml_source1_data_property_value1511
                                                            }
                                                        }, next.clone());
                                                        {
                                                            let registers = match frame.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1512, _, _, _) => {
                                                                    source_ml_source1_data_property_value1512
                                                                }
                                                            };
                                                            match instruction.clone().expect("a proven present value") {
                                                                self::JavaScriptCallInstruction::Constant(destination, value) => {
                                                                    let value = *value;
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1523, _) => {
                                                                            let source_ml_source1_data_property_value1523 = *source_ml_source1_data_property_value1523;
                                                                            source_ml_source1_data_property_value1523.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1524) => {
                                                                            source_ml_source1_data_property_value1524
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination, value.clone()))))
                                                                }
                                                                self::JavaScriptCallInstruction::Copy(destination_2, source) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1525, _) => {
                                                                            let source_ml_source1_data_property_value1525 = *source_ml_source1_data_property_value1525;
                                                                            source_ml_source1_data_property_value1525.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1526) => {
                                                                            source_ml_source1_data_property_value1526
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_2, (self::ml_array::optional_at(&registers, self::ml_array::number_index(source))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                }
                                                                self::JavaScriptCallInstruction::Argument(destination_3, index) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1527, _) => {
                                                                            let source_ml_source1_data_property_value1527 = *source_ml_source1_data_property_value1527;
                                                                            source_ml_source1_data_property_value1527.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1528) => {
                                                                            source_ml_source1_data_property_value1528
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_3, (self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1529, _, _) => {
                                                                            source_ml_source1_data_property_value1529
                                                                        }
                                                                    }, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                }
                                                                self::JavaScriptCallInstruction::Jump(target) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1530, _) => {
                                                                            let source_ml_source1_data_property_value1530 = *source_ml_source1_data_property_value1530;
                                                                            source_ml_source1_data_property_value1530.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1531) => {
                                                                            source_ml_source1_data_property_value1531
                                                                        }
                                                                    }, {
                                                                        let source_ml_source1_nominal_spread1532 = next.clone();
                                                                        {
                                                                            let source_ml_source1_nominal_field1533 = match source_ml_source1_nominal_spread1532.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1534, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value1534.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field1535 = match source_ml_source1_nominal_spread1532.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1536, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1536
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1537 = match source_ml_source1_nominal_spread1532.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1538, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1538
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1539 = match source_ml_source1_nominal_spread1532.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1540, _, _) => {
                                                                                                source_ml_source1_data_property_value1540
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1541 = match source_ml_source1_nominal_spread1532.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1542, _) => {
                                                                                                    source_ml_source1_data_property_value1542
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1543 = match source_ml_source1_nominal_spread1532.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1544) => {
                                                                                                        let source_ml_source1_data_property_value1544 = *source_ml_source1_data_property_value1544;
                                                                                                        source_ml_source1_data_property_value1544.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_override1545 = target;
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1533.clone(), source_ml_source1_nominal_override1545, source_ml_source1_nominal_field1537, source_ml_source1_nominal_field1539, source_ml_source1_nominal_field1541, Box::new(source_ml_source1_nominal_field1543.clone()))
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
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1546, _) => {
                                                                            let source_ml_source1_data_property_value1546 = *source_ml_source1_data_property_value1546;
                                                                            source_ml_source1_data_property_value1546.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1547) => {
                                                                            source_ml_source1_data_property_value1547
                                                                        }
                                                                    }, {
                                                                        let source_ml_source1_nominal_spread1548 = next.clone();
                                                                        {
                                                                            let source_ml_source1_nominal_field1549 = match source_ml_source1_nominal_spread1548.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1550, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value1550.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field1551 = match source_ml_source1_nominal_spread1548.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1552, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1552
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1553 = match source_ml_source1_nominal_spread1548.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1554, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1554
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1555 = match source_ml_source1_nominal_spread1548.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1556, _, _) => {
                                                                                                source_ml_source1_data_property_value1556
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1557 = match source_ml_source1_nominal_spread1548.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1558, _) => {
                                                                                                    source_ml_source1_data_property_value1558
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1559 = match source_ml_source1_nominal_spread1548.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1560) => {
                                                                                                        let source_ml_source1_data_property_value1560 = *source_ml_source1_data_property_value1560;
                                                                                                        source_ml_source1_data_property_value1560.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_override1561 = if self::convert_java_script_semantic_boolean((self::ml_array::optional_at(&registers, self::ml_array::number_index(condition))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)) {
                                                                                                        yes
                                                                                                    } else {
                                                                                                        no
                                                                                                    };
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1549.clone(), source_ml_source1_nominal_override1561, source_ml_source1_nominal_field1553, source_ml_source1_nominal_field1555, source_ml_source1_nominal_field1557, Box::new(source_ml_source1_nominal_field1559.clone()))
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
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1562, _) => {
                                                                                let source_ml_source1_data_property_value1562 = *source_ml_source1_data_property_value1562;
                                                                                source_ml_source1_data_property_value1562.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1563) => {
                                                                                source_ml_source1_data_property_value1563
                                                                            }
                                                                        }, {
                                                                            let source_ml_source1_nominal_spread1564 = next.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field1565 = match source_ml_source1_nominal_spread1564.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1566, _, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1566.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1567 = match source_ml_source1_nominal_spread1564.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1568, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1568
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1569 = match source_ml_source1_nominal_spread1564.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1570, _, _, _) => {
                                                                                                source_ml_source1_data_property_value1570
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1571 = match source_ml_source1_nominal_spread1564.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1572, _, _) => {
                                                                                                    source_ml_source1_data_property_value1572
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1573 = match source_ml_source1_nominal_spread1564.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1574, _) => {
                                                                                                        source_ml_source1_data_property_value1574
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field1575 = match source_ml_source1_nominal_spread1564.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1576) => {
                                                                                                            let source_ml_source1_data_property_value1576 = *source_ml_source1_data_property_value1576;
                                                                                                            source_ml_source1_data_property_value1576.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_override1577 = self::JavaScriptCallPending::WaitCall(destination_4);
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1565.clone(), source_ml_source1_nominal_field1567, source_ml_source1_nominal_field1569, source_ml_source1_nominal_field1571, source_ml_source1_nominal_field1573, Box::new(source_ml_source1_nominal_override1577.clone()))
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
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1579, _) => {
                                                                            source_ml_source1_data_property_value1579
                                                                        }
                                                                    }).len() as f64)) >= (1024f64)) {
                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![104, 97, 110, 100, 108, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1580, _) => {
                                                                                let source_ml_source1_data_property_value1580 = *source_ml_source1_data_property_value1580;
                                                                                source_ml_source1_data_property_value1580.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1581) => {
                                                                                source_ml_source1_data_property_value1581
                                                                            }
                                                                        }, {
                                                                            let source_ml_source1_nominal_spread1585 = next.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field1586 = match source_ml_source1_nominal_spread1585.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1587, _, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1587.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1588 = match source_ml_source1_nominal_spread1585.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1589, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1589
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1590 = match source_ml_source1_nominal_spread1585.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1591, _, _, _) => {
                                                                                                source_ml_source1_data_property_value1591
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1592 = match source_ml_source1_nominal_spread1585.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1593, _, _) => {
                                                                                                    source_ml_source1_data_property_value1593
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1594 = match source_ml_source1_nominal_spread1585.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1595, _) => {
                                                                                                        source_ml_source1_data_property_value1595
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field1596 = match source_ml_source1_nominal_spread1585.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1597) => {
                                                                                                            let source_ml_source1_data_property_value1597 = *source_ml_source1_data_property_value1597;
                                                                                                            source_ml_source1_data_property_value1597.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_override1598 = {
                                                                                                            let source_ml_source1_concat_value1583 = match frame.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1582, _) => {
                                                                                                                    source_ml_source1_data_property_value1582
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value1584 = vec![self::JavaScriptCallHandler::CallHandler(target_2, destination_5)];
                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallHandler>::new(), source_ml_source1_concat_value1583), source_ml_source1_concat_value1584)
                                                                                                            }
                                                                                                        };
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1586.clone(), source_ml_source1_nominal_field1588, source_ml_source1_nominal_field1590, source_ml_source1_nominal_field1592, source_ml_source1_nominal_override1598, Box::new(source_ml_source1_nominal_field1596.clone()))
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
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1600, _) => {
                                                                            source_ml_source1_data_property_value1600
                                                                        }
                                                                    }).len() as f64)) == (0f64)) {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1601, _) => {
                                                                                let source_ml_source1_data_property_value1601 = *source_ml_source1_data_property_value1601;
                                                                                source_ml_source1_data_property_value1601.clone()
                                                                            }
                                                                        }), self::ml_string::Text::from_units(vec![97, 32, 104, 97, 110, 100, 108, 101, 114, 32, 112, 111, 112, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 104, 97, 110, 100, 108, 101, 114]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1602, _) => {
                                                                                let source_ml_source1_data_property_value1602 = *source_ml_source1_data_property_value1602;
                                                                                source_ml_source1_data_property_value1602.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1603) => {
                                                                                source_ml_source1_data_property_value1603
                                                                            }
                                                                        }, {
                                                                            let source_ml_source1_nominal_spread1606 = next.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field1607 = match source_ml_source1_nominal_spread1606.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1608, _, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1608.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1609 = match source_ml_source1_nominal_spread1606.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1610, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1610
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1611 = match source_ml_source1_nominal_spread1606.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1612, _, _, _) => {
                                                                                                source_ml_source1_data_property_value1612
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1613 = match source_ml_source1_nominal_spread1606.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1614, _, _) => {
                                                                                                    source_ml_source1_data_property_value1614
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1615 = match source_ml_source1_nominal_spread1606.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1616, _) => {
                                                                                                        source_ml_source1_data_property_value1616
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field1617 = match source_ml_source1_nominal_spread1606.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1618) => {
                                                                                                            let source_ml_source1_data_property_value1618 = *source_ml_source1_data_property_value1618;
                                                                                                            source_ml_source1_data_property_value1618.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_override1619 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1605, _) => {
                                                                                                                source_ml_source1_data_property_value1605
                                                                                                            }
                                                                                                        }, 0f64, (-1f64));
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1607.clone(), source_ml_source1_nominal_field1609, source_ml_source1_nominal_field1611, source_ml_source1_nominal_field1613, source_ml_source1_nominal_override1619, Box::new(source_ml_source1_nominal_field1617.clone()))
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
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1626, _) => {
                                                                                            let source_ml_source1_data_property_value1626 = *source_ml_source1_data_property_value1626;
                                                                                            source_ml_source1_data_property_value1626.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1627, _) => {
                                                                                            let source_ml_source1_data_property_value1627 = *source_ml_source1_data_property_value1627;
                                                                                            source_ml_source1_data_property_value1627.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1620) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1628, _) => {
                                                                                            let source_ml_source1_data_property_value1628 = *source_ml_source1_data_property_value1628;
                                                                                            source_ml_source1_data_property_value1628.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Number(left_value) => {
                                                                                    match right.clone() {
                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1635, _) => {
                                                                                                    let source_ml_source1_data_property_value1635 = *source_ml_source1_data_property_value1635;
                                                                                                    source_ml_source1_data_property_value1635.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Null => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1636, _) => {
                                                                                                    let source_ml_source1_data_property_value1636 = *source_ml_source1_data_property_value1636;
                                                                                                    source_ml_source1_data_property_value1636.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1629) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1637, _) => {
                                                                                                    let source_ml_source1_data_property_value1637 = *source_ml_source1_data_property_value1637;
                                                                                                    source_ml_source1_data_property_value1637.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Number(right_value) => {
                                                                                            {
                                                                                                let op = operator.clone();
                                                                                                if ((op == (self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]))) || (op == (self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])))) {
                                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1638, _) => {
                                                                                                            let source_ml_source1_data_property_value1638 = *source_ml_source1_data_property_value1638;
                                                                                                            source_ml_source1_data_property_value1638.clone()
                                                                                                        }
                                                                                                    }, match machine.clone() {
                                                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1639) => {
                                                                                                            source_ml_source1_data_property_value1639
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
                                                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1640, _) => {
                                                                                                                let source_ml_source1_data_property_value1640 = *source_ml_source1_data_property_value1640;
                                                                                                                source_ml_source1_data_property_value1640.clone()
                                                                                                            }
                                                                                                        }, match machine.clone() {
                                                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1641) => {
                                                                                                                source_ml_source1_data_property_value1641
                                                                                                            }
                                                                                                        }, self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Number(value_2)))))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1630) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1642, _) => {
                                                                                                    let source_ml_source1_data_property_value1642 = *source_ml_source1_data_property_value1642;
                                                                                                    source_ml_source1_data_property_value1642.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1631) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1643, _) => {
                                                                                                    let source_ml_source1_data_property_value1643 = *source_ml_source1_data_property_value1643;
                                                                                                    source_ml_source1_data_property_value1643.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1632) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1644, _) => {
                                                                                                    let source_ml_source1_data_property_value1644 = *source_ml_source1_data_property_value1644;
                                                                                                    source_ml_source1_data_property_value1644.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1633, source_ml_source1_f1634) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1645, _) => {
                                                                                                    let source_ml_source1_data_property_value1645 = *source_ml_source1_data_property_value1645;
                                                                                                    source_ml_source1_data_property_value1645.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f1621) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1646, _) => {
                                                                                            let source_ml_source1_data_property_value1646 = *source_ml_source1_data_property_value1646;
                                                                                            source_ml_source1_data_property_value1646.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f1622) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1647, _) => {
                                                                                            let source_ml_source1_data_property_value1647 = *source_ml_source1_data_property_value1647;
                                                                                            source_ml_source1_data_property_value1647.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1623) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1648, _) => {
                                                                                            let source_ml_source1_data_property_value1648 = *source_ml_source1_data_property_value1648;
                                                                                            source_ml_source1_data_property_value1648.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f1624, source_ml_source1_f1625) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1649, _) => {
                                                                                            let source_ml_source1_data_property_value1649 = *source_ml_source1_data_property_value1649;
                                                                                            source_ml_source1_data_property_value1649.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::JavaScriptCallInstruction::SameValue(destination_7, left_2, right_2) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1650, _) => {
                                                                            let source_ml_source1_data_property_value1650 = *source_ml_source1_data_property_value1650;
                                                                            source_ml_source1_data_property_value1650.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1651) => {
                                                                            source_ml_source1_data_property_value1651
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_7, self::JavaScriptSemanticValue::Boolean(self::test_java_script_semantic_same_value((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                }
                                                                self::JavaScriptCallInstruction::StrictEqual(destination_8, left_3, right_3) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1652, _) => {
                                                                            let source_ml_source1_data_property_value1652 = *source_ml_source1_data_property_value1652;
                                                                            source_ml_source1_data_property_value1652.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1653) => {
                                                                            source_ml_source1_data_property_value1653
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_8, self::JavaScriptSemanticValue::Boolean(self::test_java_script_call_strict_equal((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                }
                                                                self::JavaScriptCallInstruction::RequireWrite(source_ml_source1_f1513) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::OrdinaryGet(source_ml_source1_f1514, source_ml_source1_f1515, source_ml_source1_f1516, source_ml_source1_f1517) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::OrdinarySet(source_ml_source1_f1518, source_ml_source1_f1519, source_ml_source1_f1520, source_ml_source1_f1521, source_ml_source1_f1522) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::PropertyKey(destination_9, input, realm) => {
                                                                    self::accept_java_script_call_key(program.clone(), current.clone(), destination_9, realm.clone(), self::begin_java_script_property_key_conversion(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1654, _) => {
                                                                            let source_ml_source1_data_property_value1654 = *source_ml_source1_data_property_value1654;
                                                                            source_ml_source1_data_property_value1654.clone()
                                                                        }
                                                                    }, (self::ml_array::optional_at(&registers, self::ml_array::number_index(input))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match program.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source1_data_property_value1655) => {
                                                                            let source_ml_source1_data_property_value1655 = *source_ml_source1_data_property_value1655;
                                                                            source_ml_source1_data_property_value1655.clone()
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptCallInstruction::PropertyDefinition(destination_10, target_3, key, attributes, realm_2) => {
                                                                    self::accept_java_script_call_definition(program.clone(), current.clone(), destination_10, realm_2.clone(), self::begin_java_script_property_definition(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1656, _) => {
                                                                            let source_ml_source1_data_property_value1656 = *source_ml_source1_data_property_value1656;
                                                                            source_ml_source1_data_property_value1656.clone()
                                                                        }
                                                                    }, (self::ml_array::optional_at(&registers, self::ml_array::number_index(target_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(attributes))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match program.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source1_data_property_value1657) => {
                                                                            let source_ml_source1_data_property_value1657 = *source_ml_source1_data_property_value1657;
                                                                            source_ml_source1_data_property_value1657.clone()
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
                            self::JavaScriptCallPending::WaitCall(source_ml_source1_f1472) => {
                                {
                                    let source_ml_source1_f1658 = source_ml_source1_f1472;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1670, _) => {
                                            let source_ml_source1_data_property_value1670 = *source_ml_source1_data_property_value1670;
                                            source_ml_source1_data_property_value1670.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitGet(source_ml_source1_f1473) => {
                                {
                                    let source_ml_source1_f1672 = source_ml_source1_f1473;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1683, _) => {
                                            let source_ml_source1_data_property_value1683 = *source_ml_source1_data_property_value1683;
                                            source_ml_source1_data_property_value1683.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitSet(source_ml_source1_f1474) => {
                                {
                                    let source_ml_source1_f1686 = source_ml_source1_f1474;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1696, _) => {
                                            let source_ml_source1_data_property_value1696 = *source_ml_source1_data_property_value1696;
                                            source_ml_source1_data_property_value1696.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitKey(destination_11, realm_3, state) => {
                                let state = *state;
                                self::accept_java_script_call_key(program.clone(), machine.clone(), destination_11, realm_3.clone(), self::step_java_script_property_key_conversion(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1697, _) => {
                                        let source_ml_source1_data_property_value1697 = *source_ml_source1_data_property_value1697;
                                        source_ml_source1_data_property_value1697.clone()
                                    }
                                }, state.clone()))
                            }
                            self::JavaScriptCallPending::WaitDefinition(destination_12, realm_4, state_2) => {
                                let state_2 = *state_2;
                                self::accept_java_script_call_definition(program.clone(), machine.clone(), destination_12, realm_4.clone(), self::step_java_script_property_definition(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1698, _) => {
                                        let source_ml_source1_data_property_value1698 = *source_ml_source1_data_property_value1698;
                                        source_ml_source1_data_property_value1698.clone()
                                    }
                                }, state_2.clone()))
                            }
                            self::JavaScriptCallPending::WaitError(source_ml_source1_f1475, source_ml_source1_f1476, source_ml_source1_f1477) => {
                                {
                                    let source_ml_source1_f1708 = source_ml_source1_f1475.clone();
                                    {
                                        let source_ml_source1_f1709 = source_ml_source1_f1476.clone();
                                        {
                                            let source_ml_source1_f1710 = source_ml_source1_f1477.clone();
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1711, _) => {
                                                    let source_ml_source1_data_property_value1711 = *source_ml_source1_data_property_value1711;
                                                    source_ml_source1_data_property_value1711.clone()
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
                let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1712, _, _) => source_ml_source1_data_property_value1712.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), identity.clone())));
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
                                            let source_ml_source1_s1713 = self::source_ml_temporary_begin_java_script_call_machine_loop59(heap.clone(), source_ml_temporary_values58, source_ml_temporary_index58.clone());
                                            match source_ml_source1_s1713.clone() {
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
