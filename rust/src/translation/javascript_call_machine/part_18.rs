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
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1709, _) => {
                    let source_ml_source2_data_property_value1709 = *source_ml_source2_data_property_value1709;
                    source_ml_source2_data_property_value1709.clone()
                }
            }), error.clone())
        } else {
            {
                let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1710) => source_ml_source2_data_property_value1710.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1711) => source_ml_source2_data_property_value1711.as_slice() }).len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1712, _) => {
                            let source_ml_source2_data_property_value1712 = *source_ml_source2_data_property_value1712;
                            source_ml_source2_data_property_value1712.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    {
                        let pending = match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1713) => {
                                let source_ml_source2_data_property_value1713 = *source_ml_source2_data_property_value1713;
                                source_ml_source2_data_property_value1713.clone()
                            }
                        };
                        match pending.clone() {
                            self::JavaScriptCallPending::CallIdle => {
                                {
                                    let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1732, _, _) => source_ml_source2_data_property_value1732.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1733, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1733.clone()
                                        }
                                    })));
                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1734, _) => {
                                                let source_ml_source2_data_property_value1734 = *source_ml_source2_data_property_value1734;
                                                source_ml_source2_data_property_value1734.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                    } else {
                                        {
                                            let instruction = self::ml_array::optional_at(&match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value1735) => {
                                                    source_ml_source2_data_property_value1735
                                                }
                                            }, self::ml_array::number_index(match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1736, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1736
                                                }
                                            }));
                                            if match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::return_java_script_call_value(program.clone(), machine.clone(), self::JavaScriptSemanticValue::Undefined)
                                            } else {
                                                {
                                                    let next = {
                                                        let source_ml_source2_nominal_spread1738 = frame.clone().expect("a proven present value");
                                                        {
                                                            let source_ml_source2_nominal_field1739 = match source_ml_source2_nominal_spread1738.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1740, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value1740.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field1741 = match source_ml_source2_nominal_spread1738.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1742, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value1742
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field1743 = match source_ml_source2_nominal_spread1738.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1744, _, _, _) => {
                                                                            source_ml_source2_data_property_value1744
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field1745 = match source_ml_source2_nominal_spread1738.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1746, _, _) => {
                                                                                source_ml_source2_data_property_value1746
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field1747 = match source_ml_source2_nominal_spread1738.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1748, _) => {
                                                                                    source_ml_source2_data_property_value1748
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1749 = match source_ml_source2_nominal_spread1738.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1750) => {
                                                                                        let source_ml_source2_data_property_value1750 = *source_ml_source2_data_property_value1750;
                                                                                        source_ml_source2_data_property_value1750.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override1751 = (match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1737, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1737
                                                                                        }
                                                                                    } + 1f64);
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1739.clone(), source_ml_source2_nominal_override1751, source_ml_source2_nominal_field1743, source_ml_source2_nominal_field1745, source_ml_source2_nominal_field1747, Box::new(source_ml_source2_nominal_field1749.clone()))
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
                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1752, _) => {
                                                                let source_ml_source2_data_property_value1752 = *source_ml_source2_data_property_value1752;
                                                                source_ml_source2_data_property_value1752.clone()
                                                            }
                                                        }, match machine.clone() {
                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1753) => {
                                                                source_ml_source2_data_property_value1753
                                                            }
                                                        }, next.clone());
                                                        {
                                                            let registers = match frame.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1754, _, _, _) => {
                                                                    source_ml_source2_data_property_value1754
                                                                }
                                                            };
                                                            match instruction.clone().expect("a proven present value") {
                                                                self::JavaScriptCallInstruction::Constant(destination, value) => {
                                                                    let value = *value;
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1768, _) => {
                                                                            let source_ml_source2_data_property_value1768 = *source_ml_source2_data_property_value1768;
                                                                            source_ml_source2_data_property_value1768.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1769) => {
                                                                            source_ml_source2_data_property_value1769
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination, value.clone()))))
                                                                }
                                                                self::JavaScriptCallInstruction::Copy(destination_2, source) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1770, _) => {
                                                                            let source_ml_source2_data_property_value1770 = *source_ml_source2_data_property_value1770;
                                                                            source_ml_source2_data_property_value1770.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1771) => {
                                                                            source_ml_source2_data_property_value1771
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_2, (self::ml_array::optional_at(&registers, self::ml_array::number_index(source))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                }
                                                                self::JavaScriptCallInstruction::Argument(destination_3, index) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1772, _) => {
                                                                            let source_ml_source2_data_property_value1772 = *source_ml_source2_data_property_value1772;
                                                                            source_ml_source2_data_property_value1772.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1773) => {
                                                                            source_ml_source2_data_property_value1773
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_3, (self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1774, _, _) => {
                                                                            source_ml_source2_data_property_value1774
                                                                        }
                                                                    }, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                }
                                                                self::JavaScriptCallInstruction::Jump(target) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1775, _) => {
                                                                            let source_ml_source2_data_property_value1775 = *source_ml_source2_data_property_value1775;
                                                                            source_ml_source2_data_property_value1775.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1776) => {
                                                                            source_ml_source2_data_property_value1776
                                                                        }
                                                                    }, {
                                                                        let source_ml_source2_nominal_spread1777 = next.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field1778 = match source_ml_source2_nominal_spread1777.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1779, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1779.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1780 = match source_ml_source2_nominal_spread1777.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1781, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1781
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1782 = match source_ml_source2_nominal_spread1777.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1783, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1783
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1784 = match source_ml_source2_nominal_spread1777.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1785, _, _) => {
                                                                                                source_ml_source2_data_property_value1785
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1786 = match source_ml_source2_nominal_spread1777.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1787, _) => {
                                                                                                    source_ml_source2_data_property_value1787
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1788 = match source_ml_source2_nominal_spread1777.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1789) => {
                                                                                                        let source_ml_source2_data_property_value1789 = *source_ml_source2_data_property_value1789;
                                                                                                        source_ml_source2_data_property_value1789.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override1790 = target;
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1778.clone(), source_ml_source2_nominal_override1790, source_ml_source2_nominal_field1782, source_ml_source2_nominal_field1784, source_ml_source2_nominal_field1786, Box::new(source_ml_source2_nominal_field1788.clone()))
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
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1791, _) => {
                                                                            let source_ml_source2_data_property_value1791 = *source_ml_source2_data_property_value1791;
                                                                            source_ml_source2_data_property_value1791.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1792) => {
                                                                            source_ml_source2_data_property_value1792
                                                                        }
                                                                    }, {
                                                                        let source_ml_source2_nominal_spread1793 = next.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field1794 = match source_ml_source2_nominal_spread1793.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1795, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1795.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field1796 = match source_ml_source2_nominal_spread1793.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1797, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1797
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1798 = match source_ml_source2_nominal_spread1793.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1799, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1799
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1800 = match source_ml_source2_nominal_spread1793.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1801, _, _) => {
                                                                                                source_ml_source2_data_property_value1801
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1802 = match source_ml_source2_nominal_spread1793.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1803, _) => {
                                                                                                    source_ml_source2_data_property_value1803
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1804 = match source_ml_source2_nominal_spread1793.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1805) => {
                                                                                                        let source_ml_source2_data_property_value1805 = *source_ml_source2_data_property_value1805;
                                                                                                        source_ml_source2_data_property_value1805.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override1806 = if self::convert_java_script_semantic_boolean((self::ml_array::optional_at(&registers, self::ml_array::number_index(condition))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)) {
                                                                                                        yes
                                                                                                    } else {
                                                                                                        no
                                                                                                    };
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1794.clone(), source_ml_source2_nominal_override1806, source_ml_source2_nominal_field1798, source_ml_source2_nominal_field1800, source_ml_source2_nominal_field1802, Box::new(source_ml_source2_nominal_field1804.clone()))
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
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1807, _) => {
                                                                                let source_ml_source2_data_property_value1807 = *source_ml_source2_data_property_value1807;
                                                                                source_ml_source2_data_property_value1807.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1808) => {
                                                                                source_ml_source2_data_property_value1808
                                                                            }
                                                                        }, {
                                                                            let source_ml_source2_nominal_spread1809 = next.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field1810 = match source_ml_source2_nominal_spread1809.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1811, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1811.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1812 = match source_ml_source2_nominal_spread1809.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1813, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1813
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1814 = match source_ml_source2_nominal_spread1809.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1815, _, _, _) => {
                                                                                                source_ml_source2_data_property_value1815
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1816 = match source_ml_source2_nominal_spread1809.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1817, _, _) => {
                                                                                                    source_ml_source2_data_property_value1817
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1818 = match source_ml_source2_nominal_spread1809.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1819, _) => {
                                                                                                        source_ml_source2_data_property_value1819
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field1820 = match source_ml_source2_nominal_spread1809.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1821) => {
                                                                                                            let source_ml_source2_data_property_value1821 = *source_ml_source2_data_property_value1821;
                                                                                                            source_ml_source2_data_property_value1821.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override1822 = self::JavaScriptCallPending::WaitCall(destination_4);
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1810.clone(), source_ml_source2_nominal_field1812, source_ml_source2_nominal_field1814, source_ml_source2_nominal_field1816, source_ml_source2_nominal_field1818, Box::new(source_ml_source2_nominal_override1822.clone()))
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
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1824, _) => {
                                                                            source_ml_source2_data_property_value1824
                                                                        }
                                                                    }).len() as f64)) >= (1024f64)) {
                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![104, 97, 110, 100, 108, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1825, _) => {
                                                                                let source_ml_source2_data_property_value1825 = *source_ml_source2_data_property_value1825;
                                                                                source_ml_source2_data_property_value1825.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1826) => {
                                                                                source_ml_source2_data_property_value1826
                                                                            }
                                                                        }, {
                                                                            let source_ml_source2_nominal_spread1830 = next.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field1831 = match source_ml_source2_nominal_spread1830.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1832, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1832.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1833 = match source_ml_source2_nominal_spread1830.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1834, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1834
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1835 = match source_ml_source2_nominal_spread1830.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1836, _, _, _) => {
                                                                                                source_ml_source2_data_property_value1836
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1837 = match source_ml_source2_nominal_spread1830.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1838, _, _) => {
                                                                                                    source_ml_source2_data_property_value1838
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1839 = match source_ml_source2_nominal_spread1830.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1840, _) => {
                                                                                                        source_ml_source2_data_property_value1840
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field1841 = match source_ml_source2_nominal_spread1830.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1842) => {
                                                                                                            let source_ml_source2_data_property_value1842 = *source_ml_source2_data_property_value1842;
                                                                                                            source_ml_source2_data_property_value1842.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override1843 = {
                                                                                                            let source_ml_source2_concat_value1828 = match frame.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1827, _) => {
                                                                                                                    source_ml_source2_data_property_value1827
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_concat_value1829 = vec![self::JavaScriptCallHandler::CallHandler(target_2, destination_5)];
                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallHandler>::new(), source_ml_source2_concat_value1828), source_ml_source2_concat_value1829)
                                                                                                            }
                                                                                                        };
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1831.clone(), source_ml_source2_nominal_field1833, source_ml_source2_nominal_field1835, source_ml_source2_nominal_field1837, source_ml_source2_nominal_override1843, Box::new(source_ml_source2_nominal_field1841.clone()))
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
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1845, _) => {
                                                                            source_ml_source2_data_property_value1845
                                                                        }
                                                                    }).len() as f64)) == (0f64)) {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1846, _) => {
                                                                                let source_ml_source2_data_property_value1846 = *source_ml_source2_data_property_value1846;
                                                                                source_ml_source2_data_property_value1846.clone()
                                                                            }
                                                                        }), self::ml_string::Text::from_units(vec![97, 32, 104, 97, 110, 100, 108, 101, 114, 32, 112, 111, 112, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 104, 97, 110, 100, 108, 101, 114]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1847, _) => {
                                                                                let source_ml_source2_data_property_value1847 = *source_ml_source2_data_property_value1847;
                                                                                source_ml_source2_data_property_value1847.clone()
                                                                            }
                                                                        }, match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1848) => {
                                                                                source_ml_source2_data_property_value1848
                                                                            }
                                                                        }, {
                                                                            let source_ml_source2_nominal_spread1851 = next.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field1852 = match source_ml_source2_nominal_spread1851.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1853, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value1853.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field1854 = match source_ml_source2_nominal_spread1851.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1855, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value1855
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1856 = match source_ml_source2_nominal_spread1851.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1857, _, _, _) => {
                                                                                                source_ml_source2_data_property_value1857
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1858 = match source_ml_source2_nominal_spread1851.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1859, _, _) => {
                                                                                                    source_ml_source2_data_property_value1859
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1860 = match source_ml_source2_nominal_spread1851.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1861, _) => {
                                                                                                        source_ml_source2_data_property_value1861
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field1862 = match source_ml_source2_nominal_spread1851.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1863) => {
                                                                                                            let source_ml_source2_data_property_value1863 = *source_ml_source2_data_property_value1863;
                                                                                                            source_ml_source2_data_property_value1863.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override1864 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1850, _) => {
                                                                                                                source_ml_source2_data_property_value1850
                                                                                                            }
                                                                                                        }, 0f64, (-1f64));
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1852.clone(), source_ml_source2_nominal_field1854, source_ml_source2_nominal_field1856, source_ml_source2_nominal_field1858, source_ml_source2_nominal_override1864, Box::new(source_ml_source2_nominal_field1862.clone()))
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
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1871, _) => {
                                                                                            let source_ml_source2_data_property_value1871 = *source_ml_source2_data_property_value1871;
                                                                                            source_ml_source2_data_property_value1871.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1872, _) => {
                                                                                            let source_ml_source2_data_property_value1872 = *source_ml_source2_data_property_value1872;
                                                                                            source_ml_source2_data_property_value1872.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1865) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1873, _) => {
                                                                                            let source_ml_source2_data_property_value1873 = *source_ml_source2_data_property_value1873;
                                                                                            source_ml_source2_data_property_value1873.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Number(left_value) => {
                                                                                    match right.clone() {
                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1880, _) => {
                                                                                                    let source_ml_source2_data_property_value1880 = *source_ml_source2_data_property_value1880;
                                                                                                    source_ml_source2_data_property_value1880.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Null => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1881, _) => {
                                                                                                    let source_ml_source2_data_property_value1881 = *source_ml_source2_data_property_value1881;
                                                                                                    source_ml_source2_data_property_value1881.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1874) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1882, _) => {
                                                                                                    let source_ml_source2_data_property_value1882 = *source_ml_source2_data_property_value1882;
                                                                                                    source_ml_source2_data_property_value1882.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Number(right_value) => {
                                                                                            {
                                                                                                let op = operator.clone();
                                                                                                if ((op == (self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]))) || (op == (self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])))) {
                                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1883, _) => {
                                                                                                            let source_ml_source2_data_property_value1883 = *source_ml_source2_data_property_value1883;
                                                                                                            source_ml_source2_data_property_value1883.clone()
                                                                                                        }
                                                                                                    }, match machine.clone() {
                                                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1884) => {
                                                                                                            source_ml_source2_data_property_value1884
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
                                                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1885, _) => {
                                                                                                                let source_ml_source2_data_property_value1885 = *source_ml_source2_data_property_value1885;
                                                                                                                source_ml_source2_data_property_value1885.clone()
                                                                                                            }
                                                                                                        }, match machine.clone() {
                                                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1886) => {
                                                                                                                source_ml_source2_data_property_value1886
                                                                                                            }
                                                                                                        }, self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Number(value_2)))))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f1875) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1887, _) => {
                                                                                                    let source_ml_source2_data_property_value1887 = *source_ml_source2_data_property_value1887;
                                                                                                    source_ml_source2_data_property_value1887.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Text(source_ml_source2_f1876) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1888, _) => {
                                                                                                    let source_ml_source2_data_property_value1888 = *source_ml_source2_data_property_value1888;
                                                                                                    source_ml_source2_data_property_value1888.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1877) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1889, _) => {
                                                                                                    let source_ml_source2_data_property_value1889 = *source_ml_source2_data_property_value1889;
                                                                                                    source_ml_source2_data_property_value1889.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_f1878, source_ml_source2_f1879) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1890, _) => {
                                                                                                    let source_ml_source2_data_property_value1890 = *source_ml_source2_data_property_value1890;
                                                                                                    source_ml_source2_data_property_value1890.clone()
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f1866) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1891, _) => {
                                                                                            let source_ml_source2_data_property_value1891 = *source_ml_source2_data_property_value1891;
                                                                                            source_ml_source2_data_property_value1891.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Text(source_ml_source2_f1867) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1892, _) => {
                                                                                            let source_ml_source2_data_property_value1892 = *source_ml_source2_data_property_value1892;
                                                                                            source_ml_source2_data_property_value1892.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1868) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1893, _) => {
                                                                                            let source_ml_source2_data_property_value1893 = *source_ml_source2_data_property_value1893;
                                                                                            source_ml_source2_data_property_value1893.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_f1869, source_ml_source2_f1870) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1894, _) => {
                                                                                            let source_ml_source2_data_property_value1894 = *source_ml_source2_data_property_value1894;
                                                                                            source_ml_source2_data_property_value1894.clone()
                                                                                        }
                                                                                    }), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::JavaScriptCallInstruction::SameValue(destination_7, left_2, right_2) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1895, _) => {
                                                                            let source_ml_source2_data_property_value1895 = *source_ml_source2_data_property_value1895;
                                                                            source_ml_source2_data_property_value1895.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1896) => {
                                                                            source_ml_source2_data_property_value1896
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_7, self::JavaScriptSemanticValue::Boolean(self::test_java_script_semantic_same_value((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                }
                                                                self::JavaScriptCallInstruction::StrictEqual(destination_8, left_3, right_3) => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1897, _) => {
                                                                            let source_ml_source2_data_property_value1897 = *source_ml_source2_data_property_value1897;
                                                                            source_ml_source2_data_property_value1897.clone()
                                                                        }
                                                                    }, match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1898) => {
                                                                            source_ml_source2_data_property_value1898
                                                                        }
                                                                    }, self::write_java_script_call_register(next.clone(), destination_8, self::JavaScriptSemanticValue::Boolean(self::test_java_script_call_strict_equal((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                }
                                                                self::JavaScriptCallInstruction::RequireWrite(source_ml_source2_f1755) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::Primitive(source_ml_source2_f1756, source_ml_source2_f1757, source_ml_source2_f1758) => {
                                                                    self::execute_java_script_call_primitive_instruction(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::OrdinaryGet(source_ml_source2_f1759, source_ml_source2_f1760, source_ml_source2_f1761, source_ml_source2_f1762) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::OrdinarySet(source_ml_source2_f1763, source_ml_source2_f1764, source_ml_source2_f1765, source_ml_source2_f1766, source_ml_source2_f1767) => {
                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                }
                                                                self::JavaScriptCallInstruction::PropertyKey(destination_9, input, realm) => {
                                                                    self::accept_java_script_call_key(program.clone(), current.clone(), destination_9, realm.clone(), self::begin_java_script_property_key_conversion(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1899, _) => {
                                                                            let source_ml_source2_data_property_value1899 = *source_ml_source2_data_property_value1899;
                                                                            source_ml_source2_data_property_value1899.clone()
                                                                        }
                                                                    }, (self::ml_array::optional_at(&registers, self::ml_array::number_index(input))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match program.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source2_data_property_value1900) => {
                                                                            let source_ml_source2_data_property_value1900 = *source_ml_source2_data_property_value1900;
                                                                            source_ml_source2_data_property_value1900.clone()
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptCallInstruction::PropertyDefinition(destination_10, target_3, key, attributes, realm_2) => {
                                                                    self::accept_java_script_call_definition(program.clone(), current.clone(), destination_10, realm_2.clone(), self::begin_java_script_property_definition(match machine.clone() {
                                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1901, _) => {
                                                                            let source_ml_source2_data_property_value1901 = *source_ml_source2_data_property_value1901;
                                                                            source_ml_source2_data_property_value1901.clone()
                                                                        }
                                                                    }, (self::ml_array::optional_at(&registers, self::ml_array::number_index(target_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(attributes))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match program.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source2_data_property_value1902) => {
                                                                            let source_ml_source2_data_property_value1902 = *source_ml_source2_data_property_value1902;
                                                                            source_ml_source2_data_property_value1902.clone()
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
                            self::JavaScriptCallPending::WaitCall(source_ml_source2_f1714) => {
                                {
                                    let source_ml_source2_f1903 = source_ml_source2_f1714;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1915, _) => {
                                            let source_ml_source2_data_property_value1915 = *source_ml_source2_data_property_value1915;
                                            source_ml_source2_data_property_value1915.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitGet(source_ml_source2_f1715) => {
                                {
                                    let source_ml_source2_f1917 = source_ml_source2_f1715;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1928, _) => {
                                            let source_ml_source2_data_property_value1928 = *source_ml_source2_data_property_value1928;
                                            source_ml_source2_data_property_value1928.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitSet(source_ml_source2_f1716) => {
                                {
                                    let source_ml_source2_f1931 = source_ml_source2_f1716;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1941, _) => {
                                            let source_ml_source2_data_property_value1941 = *source_ml_source2_data_property_value1941;
                                            source_ml_source2_data_property_value1941.clone()
                                        }
                                    }), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                }
                            }
                            self::JavaScriptCallPending::WaitKey(destination_11, realm_3, state) => {
                                let state = *state;
                                self::accept_java_script_call_key(program.clone(), machine.clone(), destination_11, realm_3.clone(), self::step_java_script_property_key_conversion(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1942, _) => {
                                        let source_ml_source2_data_property_value1942 = *source_ml_source2_data_property_value1942;
                                        source_ml_source2_data_property_value1942.clone()
                                    }
                                }, state.clone()))
                            }
                            self::JavaScriptCallPending::WaitDefinition(destination_12, realm_4, state_2) => {
                                let state_2 = *state_2;
                                self::accept_java_script_call_definition(program.clone(), machine.clone(), destination_12, realm_4.clone(), self::step_java_script_property_definition(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1943, _) => {
                                        let source_ml_source2_data_property_value1943 = *source_ml_source2_data_property_value1943;
                                        source_ml_source2_data_property_value1943.clone()
                                    }
                                }, state_2.clone()))
                            }
                            self::JavaScriptCallPending::WaitError(source_ml_source2_f1717, source_ml_source2_f1718, source_ml_source2_f1719) => {
                                {
                                    let source_ml_source2_f1953 = source_ml_source2_f1717.clone();
                                    {
                                        let source_ml_source2_f1954 = source_ml_source2_f1718.clone();
                                        {
                                            let source_ml_source2_f1955 = source_ml_source2_f1719.clone();
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1956, _) => {
                                                    let source_ml_source2_data_property_value1956 = *source_ml_source2_data_property_value1956;
                                                    source_ml_source2_data_property_value1956.clone()
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
