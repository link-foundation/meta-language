// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
use super::accept_java_script_call_definition;
use super::accept_java_script_call_key;
use super::begin_java_script_property_definition;
use super::begin_java_script_property_key_conversion;
use super::convert_java_script_semantic_boolean;
use super::execute_java_script_call_primitive_instruction;
use super::execute_java_script_ordinary_call_property;
use super::find_java_script_call_function;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_string;
use super::push_java_script_call;
use super::read_java_script_call_machine_error;
use super::replace_java_script_call_frame;
use super::return_java_script_call_value;
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
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1609, _) => {
                    let source_ml_source1_data_property_value1609 = *source_ml_source1_data_property_value1609;
                    source_ml_source1_data_property_value1609.clone()
                }
            }), error.clone())
        } else {
            {
                let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1610) => source_ml_source1_data_property_value1610.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1611) => source_ml_source1_data_property_value1611.as_slice() }).len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1612, _) => {
                            let source_ml_source1_data_property_value1612 = *source_ml_source1_data_property_value1612;
                            source_ml_source1_data_property_value1612.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    {
                        let pending = match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1613) => {
                                let source_ml_source1_data_property_value1613 = *source_ml_source1_data_property_value1613;
                                source_ml_source1_data_property_value1613.clone()
                            }
                        };
                        match pending.clone() {
                            self::JavaScriptCallPending::CallIdle => {
                                {
                                    let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1632, _, _) => source_ml_source1_data_property_value1632.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1633, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value1633.clone()
                                        }
                                    })));
                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1634, _) => {
                                                let source_ml_source1_data_property_value1634 = *source_ml_source1_data_property_value1634;
                                                source_ml_source1_data_property_value1634.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                    } else {
                                        {
                                            let instruction = self::ml_array::optional_at(&match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value1635) => {
                                                    source_ml_source1_data_property_value1635
                                                }
                                            }, self::ml_array::number_index(match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1636, _, _, _, _) => {
                                                    source_ml_source1_data_property_value1636
                                                }
                                            }));
                                            if match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::return_java_script_call_value(program.clone(), machine.clone(), self::JavaScriptSemanticValue::Undefined)
                                            } else {
                                                {
                                                    let next = {
                                                        let source_ml_source1_nominal_spread1638 = frame.clone().expect("a proven present value");
                                                        {
                                                            let source_ml_source1_nominal_field1639 = match source_ml_source1_nominal_spread1638.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1640, _, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value1640.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field1641 = match source_ml_source1_nominal_spread1638.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1642, _, _, _, _) => {
                                                                        source_ml_source1_data_property_value1642
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field1643 = match source_ml_source1_nominal_spread1638.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1644, _, _, _) => {
                                                                            source_ml_source1_data_property_value1644
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field1645 = match source_ml_source1_nominal_spread1638.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1646, _, _) => {
                                                                                source_ml_source1_data_property_value1646
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_field1647 = match source_ml_source1_nominal_spread1638.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1648, _) => {
                                                                                    source_ml_source1_data_property_value1648
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field1649 = match source_ml_source1_nominal_spread1638.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1650) => {
                                                                                        let source_ml_source1_data_property_value1650 = *source_ml_source1_data_property_value1650;
                                                                                        source_ml_source1_data_property_value1650.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_override1651 = (match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1637, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1637
                                                                                        }
                                                                                    } + 1f64);
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1639.clone(), source_ml_source1_nominal_override1651, source_ml_source1_nominal_field1643, source_ml_source1_nominal_field1645, source_ml_source1_nominal_field1647, Box::new(source_ml_source1_nominal_field1649.clone()))
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
                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1652, _) => {
                                                                let source_ml_source1_data_property_value1652 = *source_ml_source1_data_property_value1652;
                                                                source_ml_source1_data_property_value1652.clone()
                                                            }
                                                        }, match machine.clone() {
                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1653) => {
                                                                source_ml_source1_data_property_value1653
                                                            }
                                                        }, next.clone());
                                                        {
                                                            let registers = match frame.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1654, _, _, _) => {
                                                                    source_ml_source1_data_property_value1654
                                                                }
                                                            };
                                                            match instruction.clone().expect("a proven present value") {
                                                                self::JavaScriptCallInstruction::Constant(destination, value) => {
                                                                    let value = *value;
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1668, _) => {
                                                                            let source_ml_source1_data_property_value1668 = *source_ml_source1_data_property_value1668;
                                                                            source_ml_source1_data_property_value1668.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1669) => {
                                                                            source_ml_source1_data_property_value1669
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination, value.clone()))))
                                                                }
                                                                self::JavaScriptCallInstruction::Copy(destination_2, source) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1670, _) => {
                                                                            let source_ml_source1_data_property_value1670 = *source_ml_source1_data_property_value1670;
                                                                            source_ml_source1_data_property_value1670.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1671) => {
                                                                            source_ml_source1_data_property_value1671
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_2, (self::ml_array::optional_at(&registers, self::ml_array::number_index(source))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                }
                                                                self::JavaScriptCallInstruction::Argument(destination_3, index) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1672, _) => {
                                                                            let source_ml_source1_data_property_value1672 = *source_ml_source1_data_property_value1672;
                                                                            source_ml_source1_data_property_value1672.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1673) => {
                                                                            source_ml_source1_data_property_value1673
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_3, (self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1674, _, _) => {
                                                                            source_ml_source1_data_property_value1674
                                                                        }
                                                                    }, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                }
                                                                self::JavaScriptCallInstruction::Jump(target) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1675, _) => {
                                                                            let source_ml_source1_data_property_value1675 = *source_ml_source1_data_property_value1675;
                                                                            source_ml_source1_data_property_value1675.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1676) => {
                                                                            source_ml_source1_data_property_value1676
                                                                        }
                                                                    }, {
                                                                        let source_ml_source1_nominal_spread1677 = next.clone();
                                                                        {
                                                                            let source_ml_source1_nominal_field1678 = match source_ml_source1_nominal_spread1677.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1679, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value1679.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field1680 = match source_ml_source1_nominal_spread1677.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1681, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1681
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1682 = match source_ml_source1_nominal_spread1677.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1683, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1683
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1684 = match source_ml_source1_nominal_spread1677.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1685, _, _) => {
                                                                                                source_ml_source1_data_property_value1685
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1686 = match source_ml_source1_nominal_spread1677.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1687, _) => {
                                                                                                    source_ml_source1_data_property_value1687
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1688 = match source_ml_source1_nominal_spread1677.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1689) => {
                                                                                                        let source_ml_source1_data_property_value1689 = *source_ml_source1_data_property_value1689;
                                                                                                        source_ml_source1_data_property_value1689.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_override1690 = target;
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1678.clone(), source_ml_source1_nominal_override1690, source_ml_source1_nominal_field1682, source_ml_source1_nominal_field1684, source_ml_source1_nominal_field1686, Box::new(source_ml_source1_nominal_field1688.clone()))
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
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1691, _) => {
                                                                            let source_ml_source1_data_property_value1691 = *source_ml_source1_data_property_value1691;
                                                                            source_ml_source1_data_property_value1691.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1692) => {
                                                                            source_ml_source1_data_property_value1692
                                                                        }
                                                                    }, {
                                                                        let source_ml_source1_nominal_spread1693 = next.clone();
                                                                        {
                                                                            let source_ml_source1_nominal_field1694 = match source_ml_source1_nominal_spread1693.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1695, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value1695.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field1696 = match source_ml_source1_nominal_spread1693.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1697, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1697
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1698 = match source_ml_source1_nominal_spread1693.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1699, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1699
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1700 = match source_ml_source1_nominal_spread1693.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1701, _, _) => {
                                                                                                source_ml_source1_data_property_value1701
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1702 = match source_ml_source1_nominal_spread1693.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1703, _) => {
                                                                                                    source_ml_source1_data_property_value1703
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1704 = match source_ml_source1_nominal_spread1693.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1705) => {
                                                                                                        let source_ml_source1_data_property_value1705 = *source_ml_source1_data_property_value1705;
                                                                                                        source_ml_source1_data_property_value1705.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_override1706 = if self::convert_java_script_semantic_boolean((self::ml_array::optional_at(&registers, self::ml_array::number_index(condition))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)) {
                                                                                                        yes
                                                                                                    } else {
                                                                                                        no
                                                                                                    };
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1694.clone(), source_ml_source1_nominal_override1706, source_ml_source1_nominal_field1698, source_ml_source1_nominal_field1700, source_ml_source1_nominal_field1702, Box::new(source_ml_source1_nominal_field1704.clone()))
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
                                                                        let arguments_ = { let ml_values8 = arguments; let ml_callback8 = { let registers = registers.clone(); std::rc::Rc::new(move |index_2: f64| -> self::JavaScriptSemanticValue { (self::ml_array::optional_at(&registers, self::ml_array::number_index(index_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values8.iter().cloned().enumerate().map(|(ml_index8, ml_value8)| ml_callback8(ml_value8)).collect::<Vec<_>>() };
                                                                        self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1707, _) => {
                                                                                let source_ml_source1_data_property_value1707 = *source_ml_source1_data_property_value1707;
                                                                                source_ml_source1_data_property_value1707.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1708) => {
                                                                                source_ml_source1_data_property_value1708
                                                                            }
                                                                        }, {
                                                                            let source_ml_source1_nominal_spread1709 = next.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field1710 = match source_ml_source1_nominal_spread1709.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1711, _, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1711.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1712 = match source_ml_source1_nominal_spread1709.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1713, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1713
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1714 = match source_ml_source1_nominal_spread1709.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1715, _, _, _) => {
                                                                                                source_ml_source1_data_property_value1715
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1716 = match source_ml_source1_nominal_spread1709.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1717, _, _) => {
                                                                                                    source_ml_source1_data_property_value1717
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1718 = match source_ml_source1_nominal_spread1709.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1719, _) => {
                                                                                                        source_ml_source1_data_property_value1719
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field1720 = match source_ml_source1_nominal_spread1709.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1721) => {
                                                                                                            let source_ml_source1_data_property_value1721 = *source_ml_source1_data_property_value1721;
                                                                                                            source_ml_source1_data_property_value1721.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_override1722 = self::JavaScriptCallPending::WaitCall(destination_4);
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1710.clone(), source_ml_source1_nominal_field1712, source_ml_source1_nominal_field1714, source_ml_source1_nominal_field1716, source_ml_source1_nominal_field1718, Box::new(source_ml_source1_nominal_override1722.clone()))
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
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1724, _) => {
                                                                            source_ml_source1_data_property_value1724
                                                                        }
                                                                    }).len() as f64)) >= (1024f64)) {
                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![104, 97, 110, 100, 108, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1725, _) => {
                                                                                let source_ml_source1_data_property_value1725 = *source_ml_source1_data_property_value1725;
                                                                                source_ml_source1_data_property_value1725.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1726) => {
                                                                                source_ml_source1_data_property_value1726
                                                                            }
                                                                        }, {
                                                                            let source_ml_source1_nominal_spread1730 = next.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field1731 = match source_ml_source1_nominal_spread1730.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1732, _, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1732.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1733 = match source_ml_source1_nominal_spread1730.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1734, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1734
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1735 = match source_ml_source1_nominal_spread1730.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1736, _, _, _) => {
                                                                                                source_ml_source1_data_property_value1736
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1737 = match source_ml_source1_nominal_spread1730.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1738, _, _) => {
                                                                                                    source_ml_source1_data_property_value1738
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1739 = match source_ml_source1_nominal_spread1730.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1740, _) => {
                                                                                                        source_ml_source1_data_property_value1740
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field1741 = match source_ml_source1_nominal_spread1730.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1742) => {
                                                                                                            let source_ml_source1_data_property_value1742 = *source_ml_source1_data_property_value1742;
                                                                                                            source_ml_source1_data_property_value1742.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_override1743 = {
                                                                                                            let source_ml_source1_concat_value1728 = match frame.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1727, _) => {
                                                                                                                    source_ml_source1_data_property_value1727
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value1729 = vec![self::JavaScriptCallHandler::CallHandler(target_2, destination_5)];
                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallHandler>::new(), source_ml_source1_concat_value1728), source_ml_source1_concat_value1729)
                                                                                                            }
                                                                                                        };
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1731.clone(), source_ml_source1_nominal_field1733, source_ml_source1_nominal_field1735, source_ml_source1_nominal_field1737, source_ml_source1_nominal_override1743, Box::new(source_ml_source1_nominal_field1741.clone()))
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
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1745, _) => {
                                                                            source_ml_source1_data_property_value1745
                                                                        }
                                                                    }).len() as f64)) == (0f64)) {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1746, _) => {
                                                                                let source_ml_source1_data_property_value1746 = *source_ml_source1_data_property_value1746;
                                                                                source_ml_source1_data_property_value1746.clone()
                                                                            }
                                                                        }), self::ml_string::Text::from_units(vec![97, 32, 104, 97, 110, 100, 108, 101, 114, 32, 112, 111, 112, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 104, 97, 110, 100, 108, 101, 114]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1747, _) => {
                                                                                let source_ml_source1_data_property_value1747 = *source_ml_source1_data_property_value1747;
                                                                                source_ml_source1_data_property_value1747.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1748) => {
                                                                                source_ml_source1_data_property_value1748
                                                                            }
                                                                        }, {
                                                                            let source_ml_source1_nominal_spread1751 = next.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field1752 = match source_ml_source1_nominal_spread1751.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1753, _, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value1753.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field1754 = match source_ml_source1_nominal_spread1751.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1755, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value1755
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field1756 = match source_ml_source1_nominal_spread1751.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1757, _, _, _) => {
                                                                                                source_ml_source1_data_property_value1757
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field1758 = match source_ml_source1_nominal_spread1751.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1759, _, _) => {
                                                                                                    source_ml_source1_data_property_value1759
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field1760 = match source_ml_source1_nominal_spread1751.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1761, _) => {
                                                                                                        source_ml_source1_data_property_value1761
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field1762 = match source_ml_source1_nominal_spread1751.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1763) => {
                                                                                                            let source_ml_source1_data_property_value1763 = *source_ml_source1_data_property_value1763;
                                                                                                            source_ml_source1_data_property_value1763.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_override1764 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1750, _) => {
                                                                                                                source_ml_source1_data_property_value1750
                                                                                                            }
                                                                                                        }, 0f64, (-1f64));
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1752.clone(), source_ml_source1_nominal_field1754, source_ml_source1_nominal_field1756, source_ml_source1_nominal_field1758, source_ml_source1_nominal_override1764, Box::new(source_ml_source1_nominal_field1762.clone()))
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
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1771, _) => {
                                                                                            let source_ml_source1_data_property_value1771 = *source_ml_source1_data_property_value1771;
                                                                                            source_ml_source1_data_property_value1771.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1772, _) => {
                                                                                            let source_ml_source1_data_property_value1772 = *source_ml_source1_data_property_value1772;
                                                                                            source_ml_source1_data_property_value1772.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1765) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1773, _) => {
                                                                                            let source_ml_source1_data_property_value1773 = *source_ml_source1_data_property_value1773;
                                                                                            source_ml_source1_data_property_value1773.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Number(left_value) => {
                                                                                    match right.clone() {
                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1780, _) => {
                                                                                                    let source_ml_source1_data_property_value1780 = *source_ml_source1_data_property_value1780;
                                                                                                    source_ml_source1_data_property_value1780.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Null => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1781, _) => {
                                                                                                    let source_ml_source1_data_property_value1781 = *source_ml_source1_data_property_value1781;
                                                                                                    source_ml_source1_data_property_value1781.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1774) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1782, _) => {
                                                                                                    let source_ml_source1_data_property_value1782 = *source_ml_source1_data_property_value1782;
                                                                                                    source_ml_source1_data_property_value1782.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Number(right_value) => {
                                                                                            {
                                                                                                let op = operator.clone();
                                                                                                if ((op == (self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]))) || (op == (self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])))) {
                                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1783, _) => {
                                                                                                            let source_ml_source1_data_property_value1783 = *source_ml_source1_data_property_value1783;
                                                                                                            source_ml_source1_data_property_value1783.clone()
                                                                                                        }
                                                                                                    }, match machine.clone() {
                                                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1784) => {
                                                                                                            source_ml_source1_data_property_value1784
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
                                                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1785, _) => {
                                                                                                                let source_ml_source1_data_property_value1785 = *source_ml_source1_data_property_value1785;
                                                                                                                source_ml_source1_data_property_value1785.clone()
                                                                                                            }
                                                                                                        }, match machine.clone() {
                                                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1786) => {
                                                                                                                source_ml_source1_data_property_value1786
                                                                                                            }
                                                                                                        }, self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Number(value_2)))))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1775) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1787, _) => {
                                                                                                    let source_ml_source1_data_property_value1787 = *source_ml_source1_data_property_value1787;
                                                                                                    source_ml_source1_data_property_value1787.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1776) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1788, _) => {
                                                                                                    let source_ml_source1_data_property_value1788 = *source_ml_source1_data_property_value1788;
                                                                                                    source_ml_source1_data_property_value1788.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1777) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1789, _) => {
                                                                                                    let source_ml_source1_data_property_value1789 = *source_ml_source1_data_property_value1789;
                                                                                                    source_ml_source1_data_property_value1789.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1778, source_ml_source1_f1779) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1790, _) => {
                                                                                                    let source_ml_source1_data_property_value1790 = *source_ml_source1_data_property_value1790;
                                                                                                    source_ml_source1_data_property_value1790.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f1766) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1791, _) => {
                                                                                            let source_ml_source1_data_property_value1791 = *source_ml_source1_data_property_value1791;
                                                                                            source_ml_source1_data_property_value1791.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f1767) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1792, _) => {
                                                                                            let source_ml_source1_data_property_value1792 = *source_ml_source1_data_property_value1792;
                                                                                            source_ml_source1_data_property_value1792.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1768) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1793, _) => {
                                                                                            let source_ml_source1_data_property_value1793 = *source_ml_source1_data_property_value1793;
                                                                                            source_ml_source1_data_property_value1793.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f1769, source_ml_source1_f1770) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1794, _) => {
                                                                                            let source_ml_source1_data_property_value1794 = *source_ml_source1_data_property_value1794;
                                                                                            source_ml_source1_data_property_value1794.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::JavaScriptCallInstruction::SameValue(destination_7, left_2, right_2) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1795, _) => {
                                                                            let source_ml_source1_data_property_value1795 = *source_ml_source1_data_property_value1795;
                                                                            source_ml_source1_data_property_value1795.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1796) => {
                                                                            source_ml_source1_data_property_value1796
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_7, self::JavaScriptSemanticValue::Boolean(self::test_java_script_semantic_same_value((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                }
                                                                self::JavaScriptCallInstruction::StrictEqual(destination_8, left_3, right_3) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1797, _) => {
                                                                            let source_ml_source1_data_property_value1797 = *source_ml_source1_data_property_value1797;
                                                                            source_ml_source1_data_property_value1797.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1798) => {
                                                                            source_ml_source1_data_property_value1798
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_8, self::JavaScriptSemanticValue::Boolean(self::test_java_script_call_strict_equal((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                }
                                                                self::JavaScriptCallInstruction::RequireWrite(source_ml_source1_f1655) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::Primitive(source_ml_source1_f1656, source_ml_source1_f1657, source_ml_source1_f1658) => {
                                                                    self::execute_java_script_call_primitive_instruction(machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::OrdinaryGet(source_ml_source1_f1659, source_ml_source1_f1660, source_ml_source1_f1661, source_ml_source1_f1662) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::OrdinarySet(source_ml_source1_f1663, source_ml_source1_f1664, source_ml_source1_f1665, source_ml_source1_f1666, source_ml_source1_f1667) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::PropertyKey(destination_9, input, realm) => {
                                                                    self::accept_java_script_call_key(program.clone(), current.clone(), destination_9, realm.clone(), self::begin_java_script_property_key_conversion(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1799, _) => {
                                                                            let source_ml_source1_data_property_value1799 = *source_ml_source1_data_property_value1799;
                                                                            source_ml_source1_data_property_value1799.clone()
                                                                        }
                                                                    }, (self::ml_array::optional_at(&registers, self::ml_array::number_index(input))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match program.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source1_data_property_value1800) => {
                                                                            let source_ml_source1_data_property_value1800 = *source_ml_source1_data_property_value1800;
                                                                            source_ml_source1_data_property_value1800.clone()
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptCallInstruction::PropertyDefinition(destination_10, target_3, key, attributes, realm_2) => {
                                                                    self::accept_java_script_call_definition(program.clone(), current.clone(), destination_10, realm_2.clone(), self::begin_java_script_property_definition(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1801, _) => {
                                                                            let source_ml_source1_data_property_value1801 = *source_ml_source1_data_property_value1801;
                                                                            source_ml_source1_data_property_value1801.clone()
                                                                        }
                                                                    }, (self::ml_array::optional_at(&registers, self::ml_array::number_index(target_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(attributes))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match program.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source1_data_property_value1802) => {
                                                                            let source_ml_source1_data_property_value1802 = *source_ml_source1_data_property_value1802;
                                                                            source_ml_source1_data_property_value1802.clone()
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
                            self::JavaScriptCallPending::WaitCall(source_ml_source1_f1614) => {
                                {
                                    let source_ml_source1_f1803 = source_ml_source1_f1614;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1815, _) => {
                                            let source_ml_source1_data_property_value1815 = *source_ml_source1_data_property_value1815;
                                            source_ml_source1_data_property_value1815.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitGet(source_ml_source1_f1615) => {
                                {
                                    let source_ml_source1_f1817 = source_ml_source1_f1615;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1828, _) => {
                                            let source_ml_source1_data_property_value1828 = *source_ml_source1_data_property_value1828;
                                            source_ml_source1_data_property_value1828.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitSet(source_ml_source1_f1616) => {
                                {
                                    let source_ml_source1_f1831 = source_ml_source1_f1616;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1841, _) => {
                                            let source_ml_source1_data_property_value1841 = *source_ml_source1_data_property_value1841;
                                            source_ml_source1_data_property_value1841.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitKey(destination_11, realm_3, state) => {
                                let state = *state;
                                self::accept_java_script_call_key(program.clone(), machine.clone(), destination_11, realm_3.clone(), self::step_java_script_property_key_conversion(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1842, _) => {
                                        let source_ml_source1_data_property_value1842 = *source_ml_source1_data_property_value1842;
                                        source_ml_source1_data_property_value1842.clone()
                                    }
                                }, state.clone()))
                            }
                            self::JavaScriptCallPending::WaitDefinition(destination_12, realm_4, state_2) => {
                                let state_2 = *state_2;
                                self::accept_java_script_call_definition(program.clone(), machine.clone(), destination_12, realm_4.clone(), self::step_java_script_property_definition(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1843, _) => {
                                        let source_ml_source1_data_property_value1843 = *source_ml_source1_data_property_value1843;
                                        source_ml_source1_data_property_value1843.clone()
                                    }
                                }, state_2.clone()))
                            }
                            self::JavaScriptCallPending::WaitError(source_ml_source1_f1617, source_ml_source1_f1618, source_ml_source1_f1619) => {
                                {
                                    let source_ml_source1_f1853 = source_ml_source1_f1617.clone();
                                    {
                                        let source_ml_source1_f1854 = source_ml_source1_f1618.clone();
                                        {
                                            let source_ml_source1_f1855 = source_ml_source1_f1619.clone();
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1856, _) => {
                                                    let source_ml_source1_data_property_value1856 = *source_ml_source1_data_property_value1856;
                                                    source_ml_source1_data_property_value1856.clone()
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
