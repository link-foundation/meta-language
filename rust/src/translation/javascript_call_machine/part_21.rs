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
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1705, _) => {
                    let source_ml_source2_data_property_value1705 = *source_ml_source2_data_property_value1705;
                    source_ml_source2_data_property_value1705.clone()
                }
            }), error.clone())
        } else {
            {
                let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1706) => source_ml_source2_data_property_value1706.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1707) => source_ml_source2_data_property_value1707.as_slice() }).len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1708, _) => {
                            let source_ml_source2_data_property_value1708 = *source_ml_source2_data_property_value1708;
                            source_ml_source2_data_property_value1708.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    {
                        let pending = match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1709) => {
                                let source_ml_source2_data_property_value1709 = *source_ml_source2_data_property_value1709;
                                source_ml_source2_data_property_value1709.clone()
                            }
                        };
                        match pending.clone() {
                            self::JavaScriptCallPending::CallIdle => {
                                {
                                    let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1728, _, _) => source_ml_source2_data_property_value1728.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1729, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1729.clone()
                                        }
                                    })));
                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1730, _) => {
                                                let source_ml_source2_data_property_value1730 = *source_ml_source2_data_property_value1730;
                                                source_ml_source2_data_property_value1730.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                    } else {
                                        {
                                            let instruction = self::ml_array::optional_at(&match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value1731) => {
                                                    source_ml_source2_data_property_value1731
                                                }
                                            }, self::ml_array::number_index(match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1732, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1732
                                                }
                                            }));
                                            if match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::return_java_script_call_value(program.clone(), machine.clone(), self::JavaScriptSemanticValue::Undefined)
                                            } else {
                                                {
                                                    let next = {
                                                        let source_ml_source2_nominal_spread1734 = frame.clone().expect("a proven present value");
                                                        {
                                                            let source_ml_source2_nominal_field1735 = match source_ml_source2_nominal_spread1734.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1736, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value1736.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field1737 = match source_ml_source2_nominal_spread1734.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1738, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value1738
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field1739 = match source_ml_source2_nominal_spread1734.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1740, _, _, _) => {
                                                                            source_ml_source2_data_property_value1740
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field1741 = match source_ml_source2_nominal_spread1734.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1742, _, _) => {
                                                                                source_ml_source2_data_property_value1742
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field1743 = match source_ml_source2_nominal_spread1734.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1744, _) => {
                                                                                    source_ml_source2_data_property_value1744
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1745 = match source_ml_source2_nominal_spread1734.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1746) => {
                                                                                        let source_ml_source2_data_property_value1746 = *source_ml_source2_data_property_value1746;
                                                                                        source_ml_source2_data_property_value1746.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override1747 = (match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1733, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1733
                                                                                        }
                                                                                    } + 1f64);
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1735.clone(), source_ml_source2_nominal_override1747, source_ml_source2_nominal_field1739, source_ml_source2_nominal_field1741, source_ml_source2_nominal_field1743, Box::new(source_ml_source2_nominal_field1745.clone()))
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
                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1748, _) => {
                                                                let source_ml_source2_data_property_value1748 = *source_ml_source2_data_property_value1748;
                                                                source_ml_source2_data_property_value1748.clone()
                                                            }
                                                        }, match machine.clone() {
                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1749) => {
                                                                source_ml_source2_data_property_value1749
                                                            }
                                                        }, next.clone());
                                                        {
                                                            let registers = match frame.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1750, _, _, _) => {
                                                                    source_ml_source2_data_property_value1750
                                                                }
                                                            };
                                                            match instruction.clone().expect("a proven present value") {
                                                                self::JavaScriptCallInstruction::Constant(destination, value) => {
                                                                    let value = *value;
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1764, _) => {
                                                                            let source_ml_source2_data_property_value1764 = *source_ml_source2_data_property_value1764;
                                                                            source_ml_source2_data_property_value1764.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1765) => {
                                                                            source_ml_source2_data_property_value1765
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination, value.clone()))))
                                                                }
                                                                self::JavaScriptCallInstruction::Copy(destination_2, source) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1766, _) => {
                                                                            let source_ml_source2_data_property_value1766 = *source_ml_source2_data_property_value1766;
                                                                            source_ml_source2_data_property_value1766.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1767) => {
                                                                            source_ml_source2_data_property_value1767
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_2, (self::ml_array::optional_at(&registers, self::ml_array::number_index(source))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                }
                                                                self::JavaScriptCallInstruction::Argument(destination_3, index) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1768, _) => {
                                                                            let source_ml_source2_data_property_value1768 = *source_ml_source2_data_property_value1768;
                                                                            source_ml_source2_data_property_value1768.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1769) => {
                                                                            source_ml_source2_data_property_value1769
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_3, (self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1770, _, _) => {
                                                                            source_ml_source2_data_property_value1770
                                                                        }
                                                                    }, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                }
                                                                self::JavaScriptCallInstruction::Jump(target) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1771, _) => {
                                                                            let source_ml_source2_data_property_value1771 = *source_ml_source2_data_property_value1771;
                                                                            source_ml_source2_data_property_value1771.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1772) => {
                                                                            source_ml_source2_data_property_value1772
                                                                        }
                                                                    }, {
                                                                        let source_ml_source2_nominal_spread1773 = next.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field1774 = match source_ml_source2_nominal_spread1773.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1775, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1775.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1776 = match source_ml_source2_nominal_spread1773.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1777, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1777
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1778 = match source_ml_source2_nominal_spread1773.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1779, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1779
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1780 = match source_ml_source2_nominal_spread1773.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1781, _, _) => {
                                                                                                source_ml_source2_data_property_value1781
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1782 = match source_ml_source2_nominal_spread1773.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1783, _) => {
                                                                                                    source_ml_source2_data_property_value1783
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1784 = match source_ml_source2_nominal_spread1773.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1785) => {
                                                                                                        let source_ml_source2_data_property_value1785 = *source_ml_source2_data_property_value1785;
                                                                                                        source_ml_source2_data_property_value1785.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override1786 = target;
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1774.clone(), source_ml_source2_nominal_override1786, source_ml_source2_nominal_field1778, source_ml_source2_nominal_field1780, source_ml_source2_nominal_field1782, Box::new(source_ml_source2_nominal_field1784.clone()))
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
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1787, _) => {
                                                                            let source_ml_source2_data_property_value1787 = *source_ml_source2_data_property_value1787;
                                                                            source_ml_source2_data_property_value1787.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1788) => {
                                                                            source_ml_source2_data_property_value1788
                                                                        }
                                                                    }, {
                                                                        let source_ml_source2_nominal_spread1789 = next.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field1790 = match source_ml_source2_nominal_spread1789.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1791, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1791.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1792 = match source_ml_source2_nominal_spread1789.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1793, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1793
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1794 = match source_ml_source2_nominal_spread1789.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1795, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1795
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1796 = match source_ml_source2_nominal_spread1789.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1797, _, _) => {
                                                                                                source_ml_source2_data_property_value1797
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1798 = match source_ml_source2_nominal_spread1789.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1799, _) => {
                                                                                                    source_ml_source2_data_property_value1799
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1800 = match source_ml_source2_nominal_spread1789.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1801) => {
                                                                                                        let source_ml_source2_data_property_value1801 = *source_ml_source2_data_property_value1801;
                                                                                                        source_ml_source2_data_property_value1801.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override1802 = if self::convert_java_script_semantic_boolean((self::ml_array::optional_at(&registers, self::ml_array::number_index(condition))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)) {
                                                                                                        yes
                                                                                                    } else {
                                                                                                        no
                                                                                                    };
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1790.clone(), source_ml_source2_nominal_override1802, source_ml_source2_nominal_field1794, source_ml_source2_nominal_field1796, source_ml_source2_nominal_field1798, Box::new(source_ml_source2_nominal_field1800.clone()))
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
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1803, _) => {
                                                                                let source_ml_source2_data_property_value1803 = *source_ml_source2_data_property_value1803;
                                                                                source_ml_source2_data_property_value1803.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1804) => {
                                                                                source_ml_source2_data_property_value1804
                                                                            }
                                                                        }, {
                                                                            let source_ml_source2_nominal_spread1805 = next.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field1806 = match source_ml_source2_nominal_spread1805.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1807, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1807.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1808 = match source_ml_source2_nominal_spread1805.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1809, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1809
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1810 = match source_ml_source2_nominal_spread1805.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1811, _, _, _) => {
                                                                                                source_ml_source2_data_property_value1811
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1812 = match source_ml_source2_nominal_spread1805.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1813, _, _) => {
                                                                                                    source_ml_source2_data_property_value1813
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1814 = match source_ml_source2_nominal_spread1805.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1815, _) => {
                                                                                                        source_ml_source2_data_property_value1815
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field1816 = match source_ml_source2_nominal_spread1805.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1817) => {
                                                                                                            let source_ml_source2_data_property_value1817 = *source_ml_source2_data_property_value1817;
                                                                                                            source_ml_source2_data_property_value1817.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override1818 = self::JavaScriptCallPending::WaitCall(destination_4);
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1806.clone(), source_ml_source2_nominal_field1808, source_ml_source2_nominal_field1810, source_ml_source2_nominal_field1812, source_ml_source2_nominal_field1814, Box::new(source_ml_source2_nominal_override1818.clone()))
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
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1820, _) => {
                                                                            source_ml_source2_data_property_value1820
                                                                        }
                                                                    }).len() as f64)) >= (1024f64)) {
                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![104, 97, 110, 100, 108, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1821, _) => {
                                                                                let source_ml_source2_data_property_value1821 = *source_ml_source2_data_property_value1821;
                                                                                source_ml_source2_data_property_value1821.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1822) => {
                                                                                source_ml_source2_data_property_value1822
                                                                            }
                                                                        }, {
                                                                            let source_ml_source2_nominal_spread1826 = next.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field1827 = match source_ml_source2_nominal_spread1826.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1828, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1828.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1829 = match source_ml_source2_nominal_spread1826.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1830, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1830
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1831 = match source_ml_source2_nominal_spread1826.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1832, _, _, _) => {
                                                                                                source_ml_source2_data_property_value1832
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1833 = match source_ml_source2_nominal_spread1826.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1834, _, _) => {
                                                                                                    source_ml_source2_data_property_value1834
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1835 = match source_ml_source2_nominal_spread1826.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1836, _) => {
                                                                                                        source_ml_source2_data_property_value1836
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field1837 = match source_ml_source2_nominal_spread1826.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1838) => {
                                                                                                            let source_ml_source2_data_property_value1838 = *source_ml_source2_data_property_value1838;
                                                                                                            source_ml_source2_data_property_value1838.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override1839 = {
                                                                                                            let source_ml_source2_concat_value1824 = match frame.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1823, _) => {
                                                                                                                    source_ml_source2_data_property_value1823
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_concat_value1825 = vec![self::JavaScriptCallHandler::CallHandler(target_2, destination_5)];
                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallHandler>::new(), source_ml_source2_concat_value1824), source_ml_source2_concat_value1825)
                                                                                                            }
                                                                                                        };
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1827.clone(), source_ml_source2_nominal_field1829, source_ml_source2_nominal_field1831, source_ml_source2_nominal_field1833, source_ml_source2_nominal_override1839, Box::new(source_ml_source2_nominal_field1837.clone()))
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
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1841, _) => {
                                                                            source_ml_source2_data_property_value1841
                                                                        }
                                                                    }).len() as f64)) == (0f64)) {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1842, _) => {
                                                                                let source_ml_source2_data_property_value1842 = *source_ml_source2_data_property_value1842;
                                                                                source_ml_source2_data_property_value1842.clone()
                                                                            }
                                                                        }), self::ml_string::Text::from_units(vec![97, 32, 104, 97, 110, 100, 108, 101, 114, 32, 112, 111, 112, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 104, 97, 110, 100, 108, 101, 114]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1843, _) => {
                                                                                let source_ml_source2_data_property_value1843 = *source_ml_source2_data_property_value1843;
                                                                                source_ml_source2_data_property_value1843.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1844) => {
                                                                                source_ml_source2_data_property_value1844
                                                                            }
                                                                        }, {
                                                                            let source_ml_source2_nominal_spread1847 = next.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field1848 = match source_ml_source2_nominal_spread1847.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1849, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1849.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1850 = match source_ml_source2_nominal_spread1847.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1851, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1851
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1852 = match source_ml_source2_nominal_spread1847.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1853, _, _, _) => {
                                                                                                source_ml_source2_data_property_value1853
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1854 = match source_ml_source2_nominal_spread1847.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1855, _, _) => {
                                                                                                    source_ml_source2_data_property_value1855
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1856 = match source_ml_source2_nominal_spread1847.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1857, _) => {
                                                                                                        source_ml_source2_data_property_value1857
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field1858 = match source_ml_source2_nominal_spread1847.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1859) => {
                                                                                                            let source_ml_source2_data_property_value1859 = *source_ml_source2_data_property_value1859;
                                                                                                            source_ml_source2_data_property_value1859.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override1860 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1846, _) => {
                                                                                                                source_ml_source2_data_property_value1846
                                                                                                            }
                                                                                                        }, 0f64, (-1f64));
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1848.clone(), source_ml_source2_nominal_field1850, source_ml_source2_nominal_field1852, source_ml_source2_nominal_field1854, source_ml_source2_nominal_override1860, Box::new(source_ml_source2_nominal_field1858.clone()))
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
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1867, _) => {
                                                                                            let source_ml_source2_data_property_value1867 = *source_ml_source2_data_property_value1867;
                                                                                            source_ml_source2_data_property_value1867.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1868, _) => {
                                                                                            let source_ml_source2_data_property_value1868 = *source_ml_source2_data_property_value1868;
                                                                                            source_ml_source2_data_property_value1868.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1861) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1869, _) => {
                                                                                            let source_ml_source2_data_property_value1869 = *source_ml_source2_data_property_value1869;
                                                                                            source_ml_source2_data_property_value1869.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Number(left_value) => {
                                                                                    match right.clone() {
                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1876, _) => {
                                                                                                    let source_ml_source2_data_property_value1876 = *source_ml_source2_data_property_value1876;
                                                                                                    source_ml_source2_data_property_value1876.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Null => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1877, _) => {
                                                                                                    let source_ml_source2_data_property_value1877 = *source_ml_source2_data_property_value1877;
                                                                                                    source_ml_source2_data_property_value1877.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1870) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1878, _) => {
                                                                                                    let source_ml_source2_data_property_value1878 = *source_ml_source2_data_property_value1878;
                                                                                                    source_ml_source2_data_property_value1878.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Number(right_value) => {
                                                                                            {
                                                                                                let op = operator.clone();
                                                                                                if ((op == (self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]))) || (op == (self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])))) {
                                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1879, _) => {
                                                                                                            let source_ml_source2_data_property_value1879 = *source_ml_source2_data_property_value1879;
                                                                                                            source_ml_source2_data_property_value1879.clone()
                                                                                                        }
                                                                                                    }, match machine.clone() {
                                                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1880) => {
                                                                                                            source_ml_source2_data_property_value1880
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
                                                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1881, _) => {
                                                                                                                let source_ml_source2_data_property_value1881 = *source_ml_source2_data_property_value1881;
                                                                                                                source_ml_source2_data_property_value1881.clone()
                                                                                                            }
                                                                                                        }, match machine.clone() {
                                                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1882) => {
                                                                                                                source_ml_source2_data_property_value1882
                                                                                                            }
                                                                                                        }, self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Number(value_2)))))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f1871) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1883, _) => {
                                                                                                    let source_ml_source2_data_property_value1883 = *source_ml_source2_data_property_value1883;
                                                                                                    source_ml_source2_data_property_value1883.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Text(source_ml_source2_f1872) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1884, _) => {
                                                                                                    let source_ml_source2_data_property_value1884 = *source_ml_source2_data_property_value1884;
                                                                                                    source_ml_source2_data_property_value1884.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1873) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1885, _) => {
                                                                                                    let source_ml_source2_data_property_value1885 = *source_ml_source2_data_property_value1885;
                                                                                                    source_ml_source2_data_property_value1885.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_f1874, source_ml_source2_f1875) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1886, _) => {
                                                                                                    let source_ml_source2_data_property_value1886 = *source_ml_source2_data_property_value1886;
                                                                                                    source_ml_source2_data_property_value1886.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f1862) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1887, _) => {
                                                                                            let source_ml_source2_data_property_value1887 = *source_ml_source2_data_property_value1887;
                                                                                            source_ml_source2_data_property_value1887.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Text(source_ml_source2_f1863) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1888, _) => {
                                                                                            let source_ml_source2_data_property_value1888 = *source_ml_source2_data_property_value1888;
                                                                                            source_ml_source2_data_property_value1888.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1864) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1889, _) => {
                                                                                            let source_ml_source2_data_property_value1889 = *source_ml_source2_data_property_value1889;
                                                                                            source_ml_source2_data_property_value1889.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_f1865, source_ml_source2_f1866) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1890, _) => {
                                                                                            let source_ml_source2_data_property_value1890 = *source_ml_source2_data_property_value1890;
                                                                                            source_ml_source2_data_property_value1890.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::JavaScriptCallInstruction::SameValue(destination_7, left_2, right_2) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1891, _) => {
                                                                            let source_ml_source2_data_property_value1891 = *source_ml_source2_data_property_value1891;
                                                                            source_ml_source2_data_property_value1891.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1892) => {
                                                                            source_ml_source2_data_property_value1892
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_7, self::JavaScriptSemanticValue::Boolean(self::test_java_script_semantic_same_value((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                }
                                                                self::JavaScriptCallInstruction::StrictEqual(destination_8, left_3, right_3) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1893, _) => {
                                                                            let source_ml_source2_data_property_value1893 = *source_ml_source2_data_property_value1893;
                                                                            source_ml_source2_data_property_value1893.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1894) => {
                                                                            source_ml_source2_data_property_value1894
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_8, self::JavaScriptSemanticValue::Boolean(self::test_java_script_call_strict_equal((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                }
                                                                self::JavaScriptCallInstruction::RequireWrite(source_ml_source2_f1751) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::Primitive(source_ml_source2_f1752, source_ml_source2_f1753, source_ml_source2_f1754) => {
                                                                    self::execute_java_script_call_primitive_instruction(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::OrdinaryGet(source_ml_source2_f1755, source_ml_source2_f1756, source_ml_source2_f1757, source_ml_source2_f1758) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::OrdinarySet(source_ml_source2_f1759, source_ml_source2_f1760, source_ml_source2_f1761, source_ml_source2_f1762, source_ml_source2_f1763) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::PropertyKey(destination_9, input, realm) => {
                                                                    self::accept_java_script_call_key(program.clone(), current.clone(), destination_9, realm.clone(), self::begin_java_script_property_key_conversion(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1895, _) => {
                                                                            let source_ml_source2_data_property_value1895 = *source_ml_source2_data_property_value1895;
                                                                            source_ml_source2_data_property_value1895.clone()
                                                                        }
                                                                    }, (self::ml_array::optional_at(&registers, self::ml_array::number_index(input))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match program.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source2_data_property_value1896) => {
                                                                            let source_ml_source2_data_property_value1896 = *source_ml_source2_data_property_value1896;
                                                                            source_ml_source2_data_property_value1896.clone()
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptCallInstruction::PropertyDefinition(destination_10, target_3, key, attributes, realm_2) => {
                                                                    self::accept_java_script_call_definition(program.clone(), current.clone(), destination_10, realm_2.clone(), self::begin_java_script_property_definition(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1897, _) => {
                                                                            let source_ml_source2_data_property_value1897 = *source_ml_source2_data_property_value1897;
                                                                            source_ml_source2_data_property_value1897.clone()
                                                                        }
                                                                    }, (self::ml_array::optional_at(&registers, self::ml_array::number_index(target_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(attributes))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match program.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source2_data_property_value1898) => {
                                                                            let source_ml_source2_data_property_value1898 = *source_ml_source2_data_property_value1898;
                                                                            source_ml_source2_data_property_value1898.clone()
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
                            self::JavaScriptCallPending::WaitCall(source_ml_source2_f1710) => {
                                {
                                    let source_ml_source2_f1899 = source_ml_source2_f1710;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1911, _) => {
                                            let source_ml_source2_data_property_value1911 = *source_ml_source2_data_property_value1911;
                                            source_ml_source2_data_property_value1911.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitGet(source_ml_source2_f1711) => {
                                {
                                    let source_ml_source2_f1913 = source_ml_source2_f1711;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1924, _) => {
                                            let source_ml_source2_data_property_value1924 = *source_ml_source2_data_property_value1924;
                                            source_ml_source2_data_property_value1924.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitSet(source_ml_source2_f1712) => {
                                {
                                    let source_ml_source2_f1927 = source_ml_source2_f1712;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1937, _) => {
                                            let source_ml_source2_data_property_value1937 = *source_ml_source2_data_property_value1937;
                                            source_ml_source2_data_property_value1937.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitKey(destination_11, realm_3, state) => {
                                let state = *state;
                                self::accept_java_script_call_key(program.clone(), machine.clone(), destination_11, realm_3.clone(), self::step_java_script_property_key_conversion(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1938, _) => {
                                        let source_ml_source2_data_property_value1938 = *source_ml_source2_data_property_value1938;
                                        source_ml_source2_data_property_value1938.clone()
                                    }
                                }, state.clone()))
                            }
                            self::JavaScriptCallPending::WaitDefinition(destination_12, realm_4, state_2) => {
                                let state_2 = *state_2;
                                self::accept_java_script_call_definition(program.clone(), machine.clone(), destination_12, realm_4.clone(), self::step_java_script_property_definition(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1939, _) => {
                                        let source_ml_source2_data_property_value1939 = *source_ml_source2_data_property_value1939;
                                        source_ml_source2_data_property_value1939.clone()
                                    }
                                }, state_2.clone()))
                            }
                            self::JavaScriptCallPending::WaitError(source_ml_source2_f1713, source_ml_source2_f1714, source_ml_source2_f1715) => {
                                {
                                    let source_ml_source2_f1949 = source_ml_source2_f1713.clone();
                                    {
                                        let source_ml_source2_f1950 = source_ml_source2_f1714.clone();
                                        {
                                            let source_ml_source2_f1951 = source_ml_source2_f1715.clone();
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1952, _) => {
                                                    let source_ml_source2_data_property_value1952 = *source_ml_source2_data_property_value1952;
                                                    source_ml_source2_data_property_value1952.clone()
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
