// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_call_machine_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::test_complete_java_script_property_descriptor;
use super::test_java_script_semantic_same_value;
use super::throw_java_script_call_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_error_allocation(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, realm: self::ml::Big, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    {
        let error = self::read_java_script_call_machine_error(program.clone(), machine.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1714, _) => {
                    let source_ml_source1_data_property_value1714 = *source_ml_source1_data_property_value1714;
                    source_ml_source1_data_property_value1714.clone()
                }
            }), error.clone())
        } else {
            {
                let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1715) => source_ml_source1_data_property_value1715.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1716) => source_ml_source1_data_property_value1716.as_slice() }).len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1717, _) => {
                            let source_ml_source1_data_property_value1717 = *source_ml_source1_data_property_value1717;
                            source_ml_source1_data_property_value1717.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    if {
                        let source_ml_source1_s1719 = match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1732) => {
                                let source_ml_source1_data_property_value1732 = *source_ml_source1_data_property_value1732;
                                source_ml_source1_data_property_value1732.clone()
                            }
                        };
                        match source_ml_source1_s1719.clone() {
                            self::JavaScriptCallPending::CallIdle => {
                                true
                            }
                            self::JavaScriptCallPending::WaitCall(source_ml_source1_f1720) => {
                                true
                            }
                            self::JavaScriptCallPending::WaitGet(source_ml_source1_f1721) => {
                                true
                            }
                            self::JavaScriptCallPending::WaitSet(source_ml_source1_f1722) => {
                                true
                            }
                            self::JavaScriptCallPending::WaitKey(source_ml_source1_f1723, source_ml_source1_f1724, source_ml_source1_f1725) => {
                                let source_ml_source1_f1725 = *source_ml_source1_f1725;
                                true
                            }
                            self::JavaScriptCallPending::WaitDefinition(source_ml_source1_f1726, source_ml_source1_f1727, source_ml_source1_f1728) => {
                                let source_ml_source1_f1728 = *source_ml_source1_f1728;
                                true
                            }
                            self::JavaScriptCallPending::WaitError(source_ml_source1_f1729, source_ml_source1_f1730, source_ml_source1_f1731) => {
                                false
                            }
                        }
                    } {
                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1733, _) => {
                                let source_ml_source1_data_property_value1733 = *source_ml_source1_data_property_value1733;
                                source_ml_source1_data_property_value1733.clone()
                            }
                        }), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))
                    } else {
                        if {
                            let source_ml_source1_comparison_operand1738 = match match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1734) => {
                                    let source_ml_source1_data_property_value1734 = *source_ml_source1_data_property_value1734;
                                    source_ml_source1_data_property_value1734.clone()
                                }
                            } {
                                self::JavaScriptCallPending::CallIdle => {
                                    None::<self::ml::Big>
                                }
                                self::JavaScriptCallPending::WaitCall(_) => {
                                    None::<self::ml::Big>
                                }
                                self::JavaScriptCallPending::WaitGet(_) => {
                                    None::<self::ml::Big>
                                }
                                self::JavaScriptCallPending::WaitSet(_) => {
                                    None::<self::ml::Big>
                                }
                                self::JavaScriptCallPending::WaitKey(_, source_ml_source1_data_property_value1735, _) => {
                                    Some(source_ml_source1_data_property_value1735.clone())
                                }
                                self::JavaScriptCallPending::WaitDefinition(_, source_ml_source1_data_property_value1736, _) => {
                                    Some(source_ml_source1_data_property_value1736.clone())
                                }
                                self::JavaScriptCallPending::WaitError(source_ml_source1_data_property_value1737, _, _) => {
                                    Some(source_ml_source1_data_property_value1737.clone())
                                }
                            };
                            {
                                let source_ml_source1_comparison_operand1739 = realm.clone();
                                !(match (source_ml_source1_comparison_operand1738.clone(), source_ml_source1_comparison_operand1739.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                            }
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1740, _) => {
                                    let source_ml_source1_data_property_value1740 = *source_ml_source1_data_property_value1740;
                                    source_ml_source1_data_property_value1740.clone()
                                }
                            }), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 114, 101, 97, 108, 109]))
                        } else {
                            {
                                let value_error = self::read_java_script_ordinary_value_error(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1741, _) => {
                                        let source_ml_source1_data_property_value1741 = *source_ml_source1_data_property_value1741;
                                        source_ml_source1_data_property_value1741.clone()
                                    }
                                }, value.clone());
                                if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1742, _) => {
                                            let source_ml_source1_data_property_value1742 = *source_ml_source1_data_property_value1742;
                                            source_ml_source1_data_property_value1742.clone()
                                        }
                                    }), value_error.clone())
                                } else {
                                    match value.clone() {
                                        self::JavaScriptSemanticValue::Undefined => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1749, _) => {
                                                    let source_ml_source1_data_property_value1749 = *source_ml_source1_data_property_value1749;
                                                    source_ml_source1_data_property_value1749.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Null => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1750, _) => {
                                                    let source_ml_source1_data_property_value1750 = *source_ml_source1_data_property_value1750;
                                                    source_ml_source1_data_property_value1750.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1743) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1751, _) => {
                                                    let source_ml_source1_data_property_value1751 = *source_ml_source1_data_property_value1751;
                                                    source_ml_source1_data_property_value1751.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1744) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1752, _) => {
                                                    let source_ml_source1_data_property_value1752 = *source_ml_source1_data_property_value1752;
                                                    source_ml_source1_data_property_value1752.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1745) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1753, _) => {
                                                    let source_ml_source1_data_property_value1753 = *source_ml_source1_data_property_value1753;
                                                    source_ml_source1_data_property_value1753.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1746) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1754, _) => {
                                                    let source_ml_source1_data_property_value1754 = *source_ml_source1_data_property_value1754;
                                                    source_ml_source1_data_property_value1754.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1747) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1755, _) => {
                                                    let source_ml_source1_data_property_value1755 = *source_ml_source1_data_property_value1755;
                                                    source_ml_source1_data_property_value1755.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1748, callable) => {
                                            if callable {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1756, _) => {
                                                        let source_ml_source1_data_property_value1756 = *source_ml_source1_data_property_value1756;
                                                        source_ml_source1_data_property_value1756.clone()
                                                    }
                                                }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                            } else {
                                                self::throw_java_script_call_value(machine.clone(), value.clone())
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(units: &[f64], mut at: f64) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Done,
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_same_value_loop2(left_units: &[f64], right_units: &[f64], mut at: f64) -> self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    loop {
        return if (at < ((left_units.len() as f64))) {
            if (((self::ml_array::optional_at(left_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))) != ((self::ml_array::optional_at(right_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64)))) {
                self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(false)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result {
    MlTemporaryFindJavaScriptCallCodeFunctionLoop3Done,
    MlTemporaryFindJavaScriptCallCodeFunctionLoop3Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_call_code_function_loop3(mut program: self::JavaScriptCallProgram, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let entry = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1757, _, _) => source_ml_source1_data_property_value1757.as_slice() }), self::ml_array::number_index(middle));
                    if match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Return((-1f64))
                    } else {
                        if ((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value1759, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1759.clone()
                            }
                        }) == identity) {
                            self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Return(middle)
                        } else {
                            if ((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value1761, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1761.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin4 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin4Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin4Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>)),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop5Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop5Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop5Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_literal_property_index_loop5(key: &[f64], mut value: f64, mut at: f64) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop5Result {
    loop {
        return if (at < ((key.len() as f64))) {
            {
                let digit = (match self::ml_array::optional_at(key, self::ml_array::number_index(at)) { Some(ml_present) => ml_present, None => f64::NAN } - 48f64);
                if ((!(self::ml_math::is_integer(digit)) || (digit < (0f64))) || (digit > (9f64))) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop5Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop5Return((-1f64))
                } else {
                    {
                        let value_2 = ((value * 10f64) + digit);
                        {
                            let at_2 = (at + 1f64);
                            {
                                (value, at) = (value_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop5Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop5Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptOrdinaryObjectLoop6Result {
    MlTemporaryFindJavaScriptOrdinaryObjectLoop6Done,
    MlTemporaryFindJavaScriptOrdinaryObjectLoop6Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_ordinary_object_loop6(mut objects: Vec<self::JavaScriptOrdinaryObject>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryFindJavaScriptOrdinaryObjectLoop6Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = self::ml_array::optional_at(&objects, self::ml_array::number_index(middle));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryFindJavaScriptOrdinaryObjectLoop6Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop6Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1763, _, _, _, _) => {
                                source_ml_source1_data_property_value1763.clone()
                            }
                        }) == identity) {
                            self::MlTemporaryFindJavaScriptOrdinaryObjectLoop6Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop6Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1765, _, _, _, _) => {
                                    source_ml_source1_data_property_value1765.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptOrdinaryObjectLoop6Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptOrdinarySymbolLoop7Result {
    MlTemporaryTestJavaScriptOrdinarySymbolLoop7Done,
    MlTemporaryTestJavaScriptOrdinarySymbolLoop7Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_ordinary_symbol_loop7(mut symbols: Vec<self::ml::Big>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryTestJavaScriptOrdinarySymbolLoop7Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = self::ml_array::optional_at(&symbols, self::ml_array::number_index(middle));
                    if {
                        let source_ml_source1_comparison_operand1766 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand1767 = None::<()>;
                            match (source_ml_source1_comparison_operand1766.clone(), source_ml_source1_comparison_operand1767.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                        }
                    } {
                        self::MlTemporaryTestJavaScriptOrdinarySymbolLoop7Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop7Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlTemporaryTestJavaScriptOrdinarySymbolLoop7Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop7Return(true)
                        } else {
                            if ((symbol.clone().expect("a proven present value")) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptOrdinarySymbolLoop7Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptOrdinaryPropertyLoop8Result {
    MlTemporaryFindJavaScriptOrdinaryPropertyLoop8Done,
    MlTemporaryFindJavaScriptOrdinaryPropertyLoop8Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_ordinary_property_loop8(mut properties: Vec<self::JavaScriptOrdinaryProperty>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop8Result {
    loop {
        return if (at < ((properties.len() as f64))) {
            {
                let property = self::ml_array::optional_at(&properties, self::ml_array::number_index(at));
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value({
                    let source_ml_source1_nullish_field_receiver1768 = (property.clone(),);
                    if match (source_ml_source1_nullish_field_receiver1768.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present1769 = ((source_ml_source1_nullish_field_receiver1768.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present1769.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1770, _) => {
                                    let source_ml_source1_data_property_value1770 = *source_ml_source1_data_property_value1770;
                                    source_ml_source1_data_property_value1770.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                    }
                }, key.clone())) {
                    self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop8Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop8Return(at)
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop8Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop16(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut prototype: self::JavaScriptSemanticValue, mut depth: f64) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Result {
    loop {
        return match prototype.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Done
            }
            self::JavaScriptSemanticValue::Null => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Done
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1771) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Done
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f1772) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Done
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1773) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Done
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f1774) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Done
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1775) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Done
            }
            self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f1776) => {
                {
                    let depth_2 = (depth + 1f64);
                    if ((identity == (match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1778, _, _, _, _) => {
                            source_ml_source1_data_property_value1778.clone()
                        }
                    })) || (depth_2 > (((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1780, _) => source_ml_source1_data_property_value1780.as_slice() }).len() as f64)))) {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 104, 97, 105, 110, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 121, 99, 108, 105, 99]))
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1781, _) => {
                                    source_ml_source1_data_property_value1781
                                }
                            }, identity.clone());
                            {
                                let parent = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1782, _) => source_ml_source1_data_property_value1782.as_slice() }), self::ml_array::number_index(index));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                } else {
                                    {
                                        let prototype_2 = match parent.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1783, _, _, _) => {
                                                let source_ml_source1_data_property_value1783 = *source_ml_source1_data_property_value1783;
                                                source_ml_source1_data_property_value1783.clone()
                                            }
                                        };
                                        {
                                            (prototype, depth) = (prototype_2.clone(), depth_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin18 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin18Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin18Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop15(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut at: f64) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result {
    loop {
        return if (at < (((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1785) => source_ml_source1_data_property_value1785.as_slice() }).len() as f64))) {
            {
                let property = self::ml_array::optional_at((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1786) => source_ml_source1_data_property_value1786.as_slice() }), self::ml_array::number_index(at));
                if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_key_error(heap.clone(), match property.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1787, _) => {
                                let source_ml_source1_data_property_value1787 = *source_ml_source1_data_property_value1787;
                                source_ml_source1_data_property_value1787.clone()
                            }
                        });
                        if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(key_error.clone())
                        } else {
                            if ((self::find_java_script_ordinary_property(match object.clone() {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1790) => {
                                    source_ml_source1_data_property_value1790
                                }
                            }, match property.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1791, _) => {
                                    let source_ml_source1_data_property_value1791 = *source_ml_source1_data_property_value1791;
                                    source_ml_source1_data_property_value1791.clone()
                                }
                            })) != at) {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 119, 105, 116, 104, 111, 117, 116, 32, 114, 101, 111, 114, 100, 101, 114, 105, 110, 103, 32, 116, 104, 101, 105, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121]))
                            } else {
                                {
                                    let descriptor = match property.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1792) => {
                                            source_ml_source1_data_property_value1792.clone()
                                        }
                                    };
                                    if !(self::test_complete_java_script_property_descriptor(descriptor.clone())) {
                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 105, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115]))
                                    } else {
                                        {
                                            let source_ml_source1_s1793 = if (descriptor.clone()).9 {
                                                {
                                                    let error = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).8);
                                                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(error.clone())
                                                    } else {
                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next
                                                    }
                                                }
                                            } else {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next
                                            };
                                            match source_ml_source1_s1793.clone() {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next => {
                                                    {
                                                        let source_ml_source1_s1794 = if (descriptor.clone()).5 {
                                                            {
                                                                let error_2 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4);
                                                                if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin18::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin18Return(error_2.clone())
                                                                } else {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin18::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin18Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin18::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin18Next
                                                        };
                                                        match source_ml_source1_s1794.clone() {
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin18::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin18Next => {
                                                                if (descriptor.clone()).7 {
                                                                    {
                                                                        let error_3 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6);
                                                                        if (error_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(error_3.clone())
                                                                        } else {
                                                                            {
                                                                                let at_2 = (at + 1f64);
                                                                                {
                                                                                    at = at_2;
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    {
                                                                        let at_3 = (at + 1f64);
                                                                        {
                                                                            at = at_3;
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin18::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin18Return(source_ml_temporary_result18) => {
                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(source_ml_temporary_result18.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(source_ml_temporary_result18_2) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(source_ml_temporary_result18_2.clone())
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
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop14(mut heap: self::JavaScriptOrdinaryHeap, mut property_count: f64, mut source_ml_temporary_values11: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_temporary_index11: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result {
    loop {
        return if (source_ml_temporary_index11 < (self::ml::Big::from_u128(source_ml_temporary_values11.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_temporary_values11, source_ml_temporary_index11.to_index());
                if ({
                    let source_ml_source1_s1796 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1804, _, _, _) => {
                            let source_ml_source1_data_property_value1804 = *source_ml_source1_data_property_value1804;
                            source_ml_source1_data_property_value1804.clone()
                        }
                    };
                    match source_ml_source1_s1796.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        self::JavaScriptSemanticValue::Null => {
                            false
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1797) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1798) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1799) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1800) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1801) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1802, source_ml_source1_f1803) => {
                            true
                        }
                    }
                } && {
                    let source_ml_source1_s1806 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1814, _, _, _) => {
                            let source_ml_source1_data_property_value1814 = *source_ml_source1_data_property_value1814;
                            source_ml_source1_data_property_value1814.clone()
                        }
                    };
                    match source_ml_source1_s1806.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        self::JavaScriptSemanticValue::Null => {
                            true
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1807) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1808) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1809) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1810) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1811) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1812, source_ml_source1_f1813) => {
                            false
                        }
                    }
                }) {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]))
                } else {
                    {
                        let prototype_error = self::read_java_script_ordinary_value_error(heap.clone(), match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1815, _, _, _) => {
                                let source_ml_source1_data_property_value1815 = *source_ml_source1_data_property_value1815;
                                source_ml_source1_data_property_value1815.clone()
                            }
                        });
                        if (prototype_error != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(prototype_error.clone())
                        } else {
                            {
                                let property_count_2 = (property_count + ((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1816) => source_ml_source1_data_property_value1816.as_slice() }).len() as f64));
                                if ((property_count_2 > (16384f64)) || ((((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1818) => source_ml_source1_data_property_value1818.as_slice() }).len() as f64)) > (4096f64))) {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                } else {
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source1_s1819 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop15(heap.clone(), object.clone(), at);
                                            match source_ml_source1_s1819.clone() {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done => {
                                                    {
                                                        let prototype = match object.clone() {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1820, _, _, _) => {
                                                                let source_ml_source1_data_property_value1820 = *source_ml_source1_data_property_value1820;
                                                                source_ml_source1_data_property_value1820.clone()
                                                            }
                                                        };
                                                        {
                                                            let depth = 0f64;
                                                            {
                                                                let source_ml_source1_s1821 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop16(heap.clone(), object.clone(), prototype.clone(), depth);
                                                                match source_ml_source1_s1821.clone() {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Done => {
                                                                        {
                                                                            let source_ml_temporary_index11_2 = source_ml_temporary_index11.add(&self::ml::Big::from_i128(1));
                                                                            {
                                                                                (property_count, source_ml_temporary_index11) = (property_count_2, source_ml_temporary_index11_2.clone());
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop16Return(source_ml_temporary_result16) => {
                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_temporary_result16.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(source_ml_temporary_result16_2) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_temporary_result16_2.clone())
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
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(self::ml_string::Text),
}
