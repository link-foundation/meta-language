// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
use super::execute_java_script_call_primitive;
use super::find_java_script_call_function;
use super::ml_array;
use super::ml_string;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_primitive_instruction(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, frame: self::JavaScriptCallFrame, instruction: self::JavaScriptCallInstruction) -> self::JavaScriptCallMachineTransition {
    match instruction.clone() {
        self::JavaScriptCallInstruction::Constant(source_ml_source2_f1601, source_ml_source2_f1602) => {
            let source_ml_source2_f1602 = *source_ml_source2_f1602;
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1647, _) => {
                    let source_ml_source2_data_property_value1647 = *source_ml_source2_data_property_value1647;
                    source_ml_source2_data_property_value1647.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Copy(source_ml_source2_f1603, source_ml_source2_f1604) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1648, _) => {
                    let source_ml_source2_data_property_value1648 = *source_ml_source2_data_property_value1648;
                    source_ml_source2_data_property_value1648.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Argument(source_ml_source2_f1605, source_ml_source2_f1606) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1649, _) => {
                    let source_ml_source2_data_property_value1649 = *source_ml_source2_data_property_value1649;
                    source_ml_source2_data_property_value1649.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Jump(source_ml_source2_f1607) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1650, _) => {
                    let source_ml_source2_data_property_value1650 = *source_ml_source2_data_property_value1650;
                    source_ml_source2_data_property_value1650.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Branch(source_ml_source2_f1608, source_ml_source2_f1609, source_ml_source2_f1610) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1651, _) => {
                    let source_ml_source2_data_property_value1651 = *source_ml_source2_data_property_value1651;
                    source_ml_source2_data_property_value1651.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Call(source_ml_source2_f1611, source_ml_source2_f1612, source_ml_source2_f1613, source_ml_source2_f1614) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1652, _) => {
                    let source_ml_source2_data_property_value1652 = *source_ml_source2_data_property_value1652;
                    source_ml_source2_data_property_value1652.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Return(source_ml_source2_f1615) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1653, _) => {
                    let source_ml_source2_data_property_value1653 = *source_ml_source2_data_property_value1653;
                    source_ml_source2_data_property_value1653.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Throw(source_ml_source2_f1616) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1654, _) => {
                    let source_ml_source2_data_property_value1654 = *source_ml_source2_data_property_value1654;
                    source_ml_source2_data_property_value1654.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::PushHandler(source_ml_source2_f1617, source_ml_source2_f1618) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1655, _) => {
                    let source_ml_source2_data_property_value1655 = *source_ml_source2_data_property_value1655;
                    source_ml_source2_data_property_value1655.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::PopHandler => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1656, _) => {
                    let source_ml_source2_data_property_value1656 = *source_ml_source2_data_property_value1656;
                    source_ml_source2_data_property_value1656.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::NumberBinary(source_ml_source2_f1619, source_ml_source2_f1620, source_ml_source2_f1621, source_ml_source2_f1622) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1657, _) => {
                    let source_ml_source2_data_property_value1657 = *source_ml_source2_data_property_value1657;
                    source_ml_source2_data_property_value1657.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::SameValue(source_ml_source2_f1623, source_ml_source2_f1624, source_ml_source2_f1625) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1658, _) => {
                    let source_ml_source2_data_property_value1658 = *source_ml_source2_data_property_value1658;
                    source_ml_source2_data_property_value1658.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::StrictEqual(source_ml_source2_f1626, source_ml_source2_f1627, source_ml_source2_f1628) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1659, _) => {
                    let source_ml_source2_data_property_value1659 = *source_ml_source2_data_property_value1659;
                    source_ml_source2_data_property_value1659.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::RequireWrite(source_ml_source2_f1629) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1660, _) => {
                    let source_ml_source2_data_property_value1660 = *source_ml_source2_data_property_value1660;
                    source_ml_source2_data_property_value1660.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::Primitive(destination, operation, arguments) => {
            {
                let result = self::execute_java_script_call_primitive(&(operation), { let ml_values7 = arguments; let ml_callback7 = { let frame = frame.clone(); std::rc::Rc::new(move |index: f64| -> self::JavaScriptSemanticValue { (self::ml_array::optional_at((match &frame { self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1661, _, _, _) => source_ml_source2_data_property_value1661.as_slice() }), self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values7.iter().cloned().enumerate().map(|(ml_index7, ml_value7)| ml_callback7(ml_value7)).collect::<Vec<_>>() });
                match result.clone() {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(value) => {
                        let value = *value;
                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1662, _) => {
                                let source_ml_source2_data_property_value1662 = *source_ml_source2_data_property_value1662;
                                source_ml_source2_data_property_value1662.clone()
                            }
                        }, match machine.clone() {
                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1663) => {
                                source_ml_source2_data_property_value1663
                            }
                        }, self::write_java_script_call_register({
                            let source_ml_source2_nominal_spread1665 = frame.clone();
                            {
                                let source_ml_source2_nominal_field1666 = match source_ml_source2_nominal_spread1665.clone() {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1667, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1667.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1668 = match source_ml_source2_nominal_spread1665.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1669, _, _, _, _) => {
                                            source_ml_source2_data_property_value1669
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1670 = match source_ml_source2_nominal_spread1665.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1671, _, _, _) => {
                                                source_ml_source2_data_property_value1671
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1672 = match source_ml_source2_nominal_spread1665.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1673, _, _) => {
                                                    source_ml_source2_data_property_value1673
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1674 = match source_ml_source2_nominal_spread1665.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1675, _) => {
                                                        source_ml_source2_data_property_value1675
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1676 = match source_ml_source2_nominal_spread1665.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1677) => {
                                                            let source_ml_source2_data_property_value1677 = *source_ml_source2_data_property_value1677;
                                                            source_ml_source2_data_property_value1677.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_override1678 = (match frame.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1664, _, _, _, _) => {
                                                                source_ml_source2_data_property_value1664
                                                            }
                                                        } + 1f64);
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1666.clone(), source_ml_source2_nominal_override1678, source_ml_source2_nominal_field1670, source_ml_source2_nominal_field1672, source_ml_source2_nominal_field1674, Box::new(source_ml_source2_nominal_field1676.clone()))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }, destination, value.clone()))))
                    }
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(reason) => {
                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1679, _) => {
                                let source_ml_source2_data_property_value1679 = *source_ml_source2_data_property_value1679;
                                source_ml_source2_data_property_value1679.clone()
                            }
                        }), reason.clone())
                    }
                    self::JavaScriptCallPrimitiveResult::PrimitiveError(error_class, reason_2) => {
                        {
                            let owner = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1680, _, _) => source_ml_source2_data_property_value1680.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1681, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1681.clone()
                                }
                            })));
                            if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1682, _) => {
                                        let source_ml_source2_data_property_value1682 = *source_ml_source2_data_property_value1682;
                                        source_ml_source2_data_property_value1682.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 102, 97, 105, 108, 117, 114, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                            } else {
                                self::request_java_script_call_error(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1683, _) => {
                                        let source_ml_source2_data_property_value1683 = *source_ml_source2_data_property_value1683;
                                        source_ml_source2_data_property_value1683.clone()
                                    }
                                }, match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1684) => {
                                        source_ml_source2_data_property_value1684
                                    }
                                }, {
                                    let source_ml_source2_nominal_spread1686 = frame.clone();
                                    {
                                        let source_ml_source2_nominal_field1687 = match source_ml_source2_nominal_spread1686.clone() {
                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1688, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value1688.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1689 = match source_ml_source2_nominal_spread1686.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1690, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1690
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field1691 = match source_ml_source2_nominal_spread1686.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1692, _, _, _) => {
                                                        source_ml_source2_data_property_value1692
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field1693 = match source_ml_source2_nominal_spread1686.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value1694, _, _) => {
                                                            source_ml_source2_data_property_value1694
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field1695 = match source_ml_source2_nominal_spread1686.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value1696, _) => {
                                                                source_ml_source2_data_property_value1696
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field1697 = match source_ml_source2_nominal_spread1686.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1698) => {
                                                                    let source_ml_source2_data_property_value1698 = *source_ml_source2_data_property_value1698;
                                                                    source_ml_source2_data_property_value1698.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_override1699 = (match frame.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value1685, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value1685
                                                                    }
                                                                } + 1f64);
                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1687.clone(), source_ml_source2_nominal_override1699, source_ml_source2_nominal_field1691, source_ml_source2_nominal_field1693, source_ml_source2_nominal_field1695, Box::new(source_ml_source2_nominal_field1697.clone()))
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }), match owner.clone().expect("a proven present value") {
                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1700, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1700.clone()
                                    }
                                }, &(error_class), &(reason_2))
                            }
                        }
                    }
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(reason_3) => {
                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_3.clone())
                    }
                }
            }
        }
        self::JavaScriptCallInstruction::OrdinaryGet(source_ml_source2_f1630, source_ml_source2_f1631, source_ml_source2_f1632, source_ml_source2_f1633) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1701, _) => {
                    let source_ml_source2_data_property_value1701 = *source_ml_source2_data_property_value1701;
                    source_ml_source2_data_property_value1701.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::OrdinarySet(source_ml_source2_f1634, source_ml_source2_f1635, source_ml_source2_f1636, source_ml_source2_f1637, source_ml_source2_f1638) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1702, _) => {
                    let source_ml_source2_data_property_value1702 = *source_ml_source2_data_property_value1702;
                    source_ml_source2_data_property_value1702.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::PropertyKey(source_ml_source2_f1639, source_ml_source2_f1640, source_ml_source2_f1641) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1703, _) => {
                    let source_ml_source2_data_property_value1703 = *source_ml_source2_data_property_value1703;
                    source_ml_source2_data_property_value1703.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
        self::JavaScriptCallInstruction::PropertyDefinition(source_ml_source2_f1642, source_ml_source2_f1643, source_ml_source2_f1644, source_ml_source2_f1645, source_ml_source2_f1646) => {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1704, _) => {
                    let source_ml_source2_data_property_value1704 = *source_ml_source2_data_property_value1704;
                    source_ml_source2_data_property_value1704.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
        }
    }
}
