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
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1448, _) => {
                    let source_ml_source1_data_property_value1448 = *source_ml_source1_data_property_value1448;
                    source_ml_source1_data_property_value1448.clone()
                }
            }), error.clone())
        } else {
            {
                let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1449) => source_ml_source1_data_property_value1449.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1450) => source_ml_source1_data_property_value1450.as_slice() }).len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1451, _) => {
                            let source_ml_source1_data_property_value1451 = *source_ml_source1_data_property_value1451;
                            source_ml_source1_data_property_value1451.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    {
                        let pending = match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1452) => {
                                let source_ml_source1_data_property_value1452 = *source_ml_source1_data_property_value1452;
                                source_ml_source1_data_property_value1452.clone()
                            }
                        };
                        match pending.clone() {
                            self::JavaScriptCallPending::CallIdle => {
                                {
                                    let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1471, _, _) => source_ml_source1_data_property_value1471.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1472, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value1472.clone()
                                        }
                                    })));
                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1473, _) => {
                                                let source_ml_source1_data_property_value1473 = *source_ml_source1_data_property_value1473;
                                                source_ml_source1_data_property_value1473.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                    } else {
                                        {
                                            let instruction = self::ml_array::optional_at(&match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value1474) => {
                                                    source_ml_source1_data_property_value1474
                                                }
                                            }, self::ml_array::number_index(match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1475, _, _, _, _) => {
                                                    source_ml_source1_data_property_value1475
                                                }
                                            }));
                                            if match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::return_java_script_call_value(program.clone(), machine.clone(), self::JavaScriptSemanticValue::Undefined)
                                            } else {
                                                {
                                                    let next = {
                                                        let source_ml_source1_nominal_spread1477 = frame.clone().expect("a proven present value");
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
                                                                                    let source_ml_source1_nominal_override1490 = (match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1476, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1476
                                                                                        }
                                                                                    } + 1f64);
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1478.clone(), source_ml_source1_nominal_override1490, source_ml_source1_nominal_field1482, source_ml_source1_nominal_field1484, source_ml_source1_nominal_field1486, Box::new(source_ml_source1_nominal_field1488.clone()))
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
                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1491, _) => {
                                                                let source_ml_source1_data_property_value1491 = *source_ml_source1_data_property_value1491;
                                                                source_ml_source1_data_property_value1491.clone()
                                                            }
                                                        }, match machine.clone() {
                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1492) => {
                                                                source_ml_source1_data_property_value1492
                                                            }
                                                        }, next.clone());
                                                        {
                                                            let registers = match frame.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1493, _, _, _) => {
                                                                    source_ml_source1_data_property_value1493
                                                                }
                                                            };
                                                            match instruction.clone().expect("a proven present value") {
                                                                self::JavaScriptCallInstruction::Constant(destination, value) => {
                                                                    let value = *value;
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1503, _) => {
                                                                            let source_ml_source1_data_property_value1503 = *source_ml_source1_data_property_value1503;
                                                                            source_ml_source1_data_property_value1503.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1504) => {
                                                                            source_ml_source1_data_property_value1504
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination, value.clone()))))
                                                                }
                                                                self::JavaScriptCallInstruction::Copy(destination_2, source) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1505, _) => {
                                                                            let source_ml_source1_data_property_value1505 = *source_ml_source1_data_property_value1505;
                                                                            source_ml_source1_data_property_value1505.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1506) => {
                                                                            source_ml_source1_data_property_value1506
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_2, (self::ml_array::optional_at(&registers, self::ml_array::number_index(source))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                }
                                                                self::JavaScriptCallInstruction::Argument(destination_3, index) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1507, _) => {
                                                                            let source_ml_source1_data_property_value1507 = *source_ml_source1_data_property_value1507;
                                                                            source_ml_source1_data_property_value1507.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1508) => {
                                                                            source_ml_source1_data_property_value1508
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_3, (self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1509, _, _) => {
                                                                            source_ml_source1_data_property_value1509
                                                                        }
                                                                    }, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                }
                                                                self::JavaScriptCallInstruction::Jump(target) => {
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
                                                                                                    let source_ml_source1_nominal_override1525 = target;
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
                                                                self::JavaScriptCallInstruction::Branch(condition, yes, no) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
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
                                                                                                    let source_ml_source1_nominal_override1541 = if self::convert_java_script_semantic_boolean((self::ml_array::optional_at(&registers, self::ml_array::number_index(condition))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)) {
                                                                                                        yes
                                                                                                    } else {
                                                                                                        no
                                                                                                    };
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1529.clone(), source_ml_source1_nominal_override1541, source_ml_source1_nominal_field1533, source_ml_source1_nominal_field1535, source_ml_source1_nominal_field1537, Box::new(source_ml_source1_nominal_field1539.clone()))
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
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1542, _) => {
                                                                                let source_ml_source1_data_property_value1542 = *source_ml_source1_data_property_value1542;
                                                                                source_ml_source1_data_property_value1542.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1543) => {
                                                                                source_ml_source1_data_property_value1543
                                                                            }
                                                                        }, {
                                                                            let source_ml_source1_nominal_spread1544 = next.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field1545 = match source_ml_source1_nominal_spread1544.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1546, _, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1546.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1547 = match source_ml_source1_nominal_spread1544.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1548, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1548
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1549 = match source_ml_source1_nominal_spread1544.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1550, _, _, _) => {
                                                                                                source_ml_source1_data_property_value1550
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1551 = match source_ml_source1_nominal_spread1544.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1552, _, _) => {
                                                                                                    source_ml_source1_data_property_value1552
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1553 = match source_ml_source1_nominal_spread1544.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1554, _) => {
                                                                                                        source_ml_source1_data_property_value1554
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field1555 = match source_ml_source1_nominal_spread1544.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1556) => {
                                                                                                            let source_ml_source1_data_property_value1556 = *source_ml_source1_data_property_value1556;
                                                                                                            source_ml_source1_data_property_value1556.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_override1557 = self::JavaScriptCallPending::WaitCall(destination_4);
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1545.clone(), source_ml_source1_nominal_field1547, source_ml_source1_nominal_field1549, source_ml_source1_nominal_field1551, source_ml_source1_nominal_field1553, Box::new(source_ml_source1_nominal_override1557.clone()))
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
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1559, _) => {
                                                                            source_ml_source1_data_property_value1559
                                                                        }
                                                                    }).len() as f64)) >= (1024f64)) {
                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![104, 97, 110, 100, 108, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1560, _) => {
                                                                                let source_ml_source1_data_property_value1560 = *source_ml_source1_data_property_value1560;
                                                                                source_ml_source1_data_property_value1560.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1561) => {
                                                                                source_ml_source1_data_property_value1561
                                                                            }
                                                                        }, {
                                                                            let source_ml_source1_nominal_spread1565 = next.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field1566 = match source_ml_source1_nominal_spread1565.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1567, _, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1567.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1568 = match source_ml_source1_nominal_spread1565.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1569, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1569
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1570 = match source_ml_source1_nominal_spread1565.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1571, _, _, _) => {
                                                                                                source_ml_source1_data_property_value1571
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1572 = match source_ml_source1_nominal_spread1565.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1573, _, _) => {
                                                                                                    source_ml_source1_data_property_value1573
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1574 = match source_ml_source1_nominal_spread1565.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1575, _) => {
                                                                                                        source_ml_source1_data_property_value1575
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field1576 = match source_ml_source1_nominal_spread1565.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1577) => {
                                                                                                            let source_ml_source1_data_property_value1577 = *source_ml_source1_data_property_value1577;
                                                                                                            source_ml_source1_data_property_value1577.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_override1578 = {
                                                                                                            let source_ml_source1_concat_value1563 = match frame.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1562, _) => {
                                                                                                                    source_ml_source1_data_property_value1562
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value1564 = vec![self::JavaScriptCallHandler::CallHandler(target_2, destination_5)];
                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallHandler>::new(), source_ml_source1_concat_value1563), source_ml_source1_concat_value1564)
                                                                                                            }
                                                                                                        };
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1566.clone(), source_ml_source1_nominal_field1568, source_ml_source1_nominal_field1570, source_ml_source1_nominal_field1572, source_ml_source1_nominal_override1578, Box::new(source_ml_source1_nominal_field1576.clone()))
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
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1580, _) => {
                                                                            source_ml_source1_data_property_value1580
                                                                        }
                                                                    }).len() as f64)) == (0f64)) {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1581, _) => {
                                                                                let source_ml_source1_data_property_value1581 = *source_ml_source1_data_property_value1581;
                                                                                source_ml_source1_data_property_value1581.clone()
                                                                            }
                                                                        }), self::ml_string::Text::from_units(vec![97, 32, 104, 97, 110, 100, 108, 101, 114, 32, 112, 111, 112, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 104, 97, 110, 100, 108, 101, 114]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1582, _) => {
                                                                                let source_ml_source1_data_property_value1582 = *source_ml_source1_data_property_value1582;
                                                                                source_ml_source1_data_property_value1582.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1583) => {
                                                                                source_ml_source1_data_property_value1583
                                                                            }
                                                                        }, {
                                                                            let source_ml_source1_nominal_spread1586 = next.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field1587 = match source_ml_source1_nominal_spread1586.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1588, _, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1588.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1589 = match source_ml_source1_nominal_spread1586.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1590, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1590
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1591 = match source_ml_source1_nominal_spread1586.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1592, _, _, _) => {
                                                                                                source_ml_source1_data_property_value1592
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1593 = match source_ml_source1_nominal_spread1586.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1594, _, _) => {
                                                                                                    source_ml_source1_data_property_value1594
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1595 = match source_ml_source1_nominal_spread1586.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1596, _) => {
                                                                                                        source_ml_source1_data_property_value1596
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field1597 = match source_ml_source1_nominal_spread1586.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1598) => {
                                                                                                            let source_ml_source1_data_property_value1598 = *source_ml_source1_data_property_value1598;
                                                                                                            source_ml_source1_data_property_value1598.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_override1599 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1585, _) => {
                                                                                                                source_ml_source1_data_property_value1585
                                                                                                            }
                                                                                                        }, 0f64, (-1f64));
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1587.clone(), source_ml_source1_nominal_field1589, source_ml_source1_nominal_field1591, source_ml_source1_nominal_field1593, source_ml_source1_nominal_override1599, Box::new(source_ml_source1_nominal_field1597.clone()))
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
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1606, _) => {
                                                                                            let source_ml_source1_data_property_value1606 = *source_ml_source1_data_property_value1606;
                                                                                            source_ml_source1_data_property_value1606.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1607, _) => {
                                                                                            let source_ml_source1_data_property_value1607 = *source_ml_source1_data_property_value1607;
                                                                                            source_ml_source1_data_property_value1607.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1600) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1608, _) => {
                                                                                            let source_ml_source1_data_property_value1608 = *source_ml_source1_data_property_value1608;
                                                                                            source_ml_source1_data_property_value1608.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Number(left_value) => {
                                                                                    match right.clone() {
                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1615, _) => {
                                                                                                    let source_ml_source1_data_property_value1615 = *source_ml_source1_data_property_value1615;
                                                                                                    source_ml_source1_data_property_value1615.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Null => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1616, _) => {
                                                                                                    let source_ml_source1_data_property_value1616 = *source_ml_source1_data_property_value1616;
                                                                                                    source_ml_source1_data_property_value1616.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1609) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1617, _) => {
                                                                                                    let source_ml_source1_data_property_value1617 = *source_ml_source1_data_property_value1617;
                                                                                                    source_ml_source1_data_property_value1617.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Number(right_value) => {
                                                                                            {
                                                                                                let op = operator.clone();
                                                                                                if ((op == (self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]))) || (op == (self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])))) {
                                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1618, _) => {
                                                                                                            let source_ml_source1_data_property_value1618 = *source_ml_source1_data_property_value1618;
                                                                                                            source_ml_source1_data_property_value1618.clone()
                                                                                                        }
                                                                                                    }, match machine.clone() {
                                                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1619) => {
                                                                                                            source_ml_source1_data_property_value1619
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
                                                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1620, _) => {
                                                                                                                let source_ml_source1_data_property_value1620 = *source_ml_source1_data_property_value1620;
                                                                                                                source_ml_source1_data_property_value1620.clone()
                                                                                                            }
                                                                                                        }, match machine.clone() {
                                                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1621) => {
                                                                                                                source_ml_source1_data_property_value1621
                                                                                                            }
                                                                                                        }, self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Number(value_2)))))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1610) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1622, _) => {
                                                                                                    let source_ml_source1_data_property_value1622 = *source_ml_source1_data_property_value1622;
                                                                                                    source_ml_source1_data_property_value1622.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1611) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1623, _) => {
                                                                                                    let source_ml_source1_data_property_value1623 = *source_ml_source1_data_property_value1623;
                                                                                                    source_ml_source1_data_property_value1623.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1612) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1624, _) => {
                                                                                                    let source_ml_source1_data_property_value1624 = *source_ml_source1_data_property_value1624;
                                                                                                    source_ml_source1_data_property_value1624.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1613, source_ml_source1_f1614) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1625, _) => {
                                                                                                    let source_ml_source1_data_property_value1625 = *source_ml_source1_data_property_value1625;
                                                                                                    source_ml_source1_data_property_value1625.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f1601) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1626, _) => {
                                                                                            let source_ml_source1_data_property_value1626 = *source_ml_source1_data_property_value1626;
                                                                                            source_ml_source1_data_property_value1626.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f1602) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1627, _) => {
                                                                                            let source_ml_source1_data_property_value1627 = *source_ml_source1_data_property_value1627;
                                                                                            source_ml_source1_data_property_value1627.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1603) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1628, _) => {
                                                                                            let source_ml_source1_data_property_value1628 = *source_ml_source1_data_property_value1628;
                                                                                            source_ml_source1_data_property_value1628.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f1604, source_ml_source1_f1605) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1629, _) => {
                                                                                            let source_ml_source1_data_property_value1629 = *source_ml_source1_data_property_value1629;
                                                                                            source_ml_source1_data_property_value1629.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::JavaScriptCallInstruction::SameValue(destination_7, left_2, right_2) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1630, _) => {
                                                                            let source_ml_source1_data_property_value1630 = *source_ml_source1_data_property_value1630;
                                                                            source_ml_source1_data_property_value1630.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1631) => {
                                                                            source_ml_source1_data_property_value1631
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_7, self::JavaScriptSemanticValue::Boolean(self::test_java_script_semantic_same_value((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                }
                                                                self::JavaScriptCallInstruction::StrictEqual(destination_8, left_3, right_3) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1632, _) => {
                                                                            let source_ml_source1_data_property_value1632 = *source_ml_source1_data_property_value1632;
                                                                            source_ml_source1_data_property_value1632.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1633) => {
                                                                            source_ml_source1_data_property_value1633
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_8, self::JavaScriptSemanticValue::Boolean(self::test_java_script_call_strict_equal((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                }
                                                                self::JavaScriptCallInstruction::OrdinaryGet(source_ml_source1_f1494, source_ml_source1_f1495, source_ml_source1_f1496, source_ml_source1_f1497) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::OrdinarySet(source_ml_source1_f1498, source_ml_source1_f1499, source_ml_source1_f1500, source_ml_source1_f1501, source_ml_source1_f1502) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::PropertyKey(destination_9, input, realm) => {
                                                                    self::accept_java_script_call_key(program.clone(), current.clone(), destination_9, realm.clone(), self::begin_java_script_property_key_conversion(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1634, _) => {
                                                                            let source_ml_source1_data_property_value1634 = *source_ml_source1_data_property_value1634;
                                                                            source_ml_source1_data_property_value1634.clone()
                                                                        }
                                                                    }, (self::ml_array::optional_at(&registers, self::ml_array::number_index(input))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match program.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source1_data_property_value1635) => {
                                                                            let source_ml_source1_data_property_value1635 = *source_ml_source1_data_property_value1635;
                                                                            source_ml_source1_data_property_value1635.clone()
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptCallInstruction::PropertyDefinition(destination_10, target_3, key, attributes, realm_2) => {
                                                                    self::accept_java_script_call_definition(program.clone(), current.clone(), destination_10, realm_2.clone(), self::begin_java_script_property_definition(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1636, _) => {
                                                                            let source_ml_source1_data_property_value1636 = *source_ml_source1_data_property_value1636;
                                                                            source_ml_source1_data_property_value1636.clone()
                                                                        }
                                                                    }, (self::ml_array::optional_at(&registers, self::ml_array::number_index(target_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(attributes))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match program.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source1_data_property_value1637) => {
                                                                            let source_ml_source1_data_property_value1637 = *source_ml_source1_data_property_value1637;
                                                                            source_ml_source1_data_property_value1637.clone()
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
                            self::JavaScriptCallPending::WaitCall(source_ml_source1_f1453) => {
                                {
                                    let source_ml_source1_f1638 = source_ml_source1_f1453;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1650, _) => {
                                            let source_ml_source1_data_property_value1650 = *source_ml_source1_data_property_value1650;
                                            source_ml_source1_data_property_value1650.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitGet(source_ml_source1_f1454) => {
                                {
                                    let source_ml_source1_f1652 = source_ml_source1_f1454;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1663, _) => {
                                            let source_ml_source1_data_property_value1663 = *source_ml_source1_data_property_value1663;
                                            source_ml_source1_data_property_value1663.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitSet(source_ml_source1_f1455) => {
                                {
                                    let source_ml_source1_f1666 = source_ml_source1_f1455;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1676, _) => {
                                            let source_ml_source1_data_property_value1676 = *source_ml_source1_data_property_value1676;
                                            source_ml_source1_data_property_value1676.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitKey(destination_11, realm_3, state) => {
                                let state = *state;
                                self::accept_java_script_call_key(program.clone(), machine.clone(), destination_11, realm_3.clone(), self::step_java_script_property_key_conversion(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1677, _) => {
                                        let source_ml_source1_data_property_value1677 = *source_ml_source1_data_property_value1677;
                                        source_ml_source1_data_property_value1677.clone()
                                    }
                                }, state.clone()))
                            }
                            self::JavaScriptCallPending::WaitDefinition(destination_12, realm_4, state_2) => {
                                let state_2 = *state_2;
                                self::accept_java_script_call_definition(program.clone(), machine.clone(), destination_12, realm_4.clone(), self::step_java_script_property_definition(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1678, _) => {
                                        let source_ml_source1_data_property_value1678 = *source_ml_source1_data_property_value1678;
                                        source_ml_source1_data_property_value1678.clone()
                                    }
                                }, state_2.clone()))
                            }
                            self::JavaScriptCallPending::WaitError(source_ml_source1_f1456, source_ml_source1_f1457, source_ml_source1_f1458) => {
                                {
                                    let source_ml_source1_f1688 = source_ml_source1_f1456.clone();
                                    {
                                        let source_ml_source1_f1689 = source_ml_source1_f1457.clone();
                                        {
                                            let source_ml_source1_f1690 = source_ml_source1_f1458.clone();
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1691, _) => {
                                                    let source_ml_source1_data_property_value1691 = *source_ml_source1_data_property_value1691;
                                                    source_ml_source1_data_property_value1691.clone()
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
                let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1692, _, _) => source_ml_source1_data_property_value1692.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), identity.clone())));
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
                                            let source_ml_source1_s1693 = self::source_ml_temporary_begin_java_script_call_machine_loop59(heap.clone(), source_ml_temporary_values58, source_ml_temporary_index58.clone());
                                            match source_ml_source1_s1693.clone() {
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
