// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlTemporaryBeginJavaScriptCallMachineLoop91Result;
use super::accept_java_script_whitespace_unit;
use super::create_java_script_call_frame;
use super::find_java_script_call_function;
use super::index_utf16_text;
use super::match_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_call_machine_error;
use super::read_java_script_call_program_error;
use super::read_java_script_ordinary_value_error;
use super::slice_utf16_text;
use super::source_ml_temporary_begin_java_script_call_machine_loop91;
use super::throw_java_script_call_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_call_machine(program: self::JavaScriptCallProgram, heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, receiver: self::JavaScriptSemanticValue, arguments_: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallMachineTransition {
    {
        let error = self::read_java_script_call_program_error(program.clone(), heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), error.clone())
        } else {
            {
                let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1857, _, _) => source_ml_source1_data_property_value1857.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), identity.clone())));
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
                                    let source_ml_temporary_values90 = arguments_.clone();
                                    {
                                        let source_ml_temporary_index90 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source1_s1858 = self::source_ml_temporary_begin_java_script_call_machine_loop91(heap.clone(), source_ml_temporary_values90, source_ml_temporary_index90.clone());
                                            match source_ml_source1_s1858.clone() {
                                                self::MlTemporaryBeginJavaScriptCallMachineLoop91Result::MlTemporaryBeginJavaScriptCallMachineLoop91Done => {
                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(heap.clone()), vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_)])))
                                                }
                                                self::MlTemporaryBeginJavaScriptCallMachineLoop91Result::MlTemporaryBeginJavaScriptCallMachineLoop91Return(source_ml_temporary_result91) => {
                                                    let source_ml_temporary_result91 = *source_ml_temporary_result91;
                                                    source_ml_temporary_result91.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_error_allocation(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, realm: self::ml::Big, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    {
        let error = self::read_java_script_call_machine_error(program.clone(), machine.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1859, _) => {
                    let source_ml_source1_data_property_value1859 = *source_ml_source1_data_property_value1859;
                    source_ml_source1_data_property_value1859.clone()
                }
            }), error.clone())
        } else {
            {
                let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1860) => source_ml_source1_data_property_value1860.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1861) => source_ml_source1_data_property_value1861.as_slice() }).len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1862, _) => {
                            let source_ml_source1_data_property_value1862 = *source_ml_source1_data_property_value1862;
                            source_ml_source1_data_property_value1862.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    if {
                        let source_ml_source1_s1864 = match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1877) => {
                                let source_ml_source1_data_property_value1877 = *source_ml_source1_data_property_value1877;
                                source_ml_source1_data_property_value1877.clone()
                            }
                        };
                        match source_ml_source1_s1864.clone() {
                            self::JavaScriptCallPending::CallIdle => {
                                true
                            }
                            self::JavaScriptCallPending::WaitCall(source_ml_source1_f1865) => {
                                true
                            }
                            self::JavaScriptCallPending::WaitGet(source_ml_source1_f1866) => {
                                true
                            }
                            self::JavaScriptCallPending::WaitSet(source_ml_source1_f1867) => {
                                true
                            }
                            self::JavaScriptCallPending::WaitKey(source_ml_source1_f1868, source_ml_source1_f1869, source_ml_source1_f1870) => {
                                let source_ml_source1_f1870 = *source_ml_source1_f1870;
                                true
                            }
                            self::JavaScriptCallPending::WaitDefinition(source_ml_source1_f1871, source_ml_source1_f1872, source_ml_source1_f1873) => {
                                let source_ml_source1_f1873 = *source_ml_source1_f1873;
                                true
                            }
                            self::JavaScriptCallPending::WaitError(source_ml_source1_f1874, source_ml_source1_f1875, source_ml_source1_f1876) => {
                                false
                            }
                        }
                    } {
                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1878, _) => {
                                let source_ml_source1_data_property_value1878 = *source_ml_source1_data_property_value1878;
                                source_ml_source1_data_property_value1878.clone()
                            }
                        }), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))
                    } else {
                        if {
                            let source_ml_source1_comparison_operand1883 = match match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1879) => {
                                    let source_ml_source1_data_property_value1879 = *source_ml_source1_data_property_value1879;
                                    source_ml_source1_data_property_value1879.clone()
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
                                self::JavaScriptCallPending::WaitKey(_, source_ml_source1_data_property_value1880, _) => {
                                    Some(source_ml_source1_data_property_value1880.clone())
                                }
                                self::JavaScriptCallPending::WaitDefinition(_, source_ml_source1_data_property_value1881, _) => {
                                    Some(source_ml_source1_data_property_value1881.clone())
                                }
                                self::JavaScriptCallPending::WaitError(source_ml_source1_data_property_value1882, _, _) => {
                                    Some(source_ml_source1_data_property_value1882.clone())
                                }
                            };
                            {
                                let source_ml_source1_comparison_operand1884 = realm.clone();
                                !(match (source_ml_source1_comparison_operand1883.clone(), source_ml_source1_comparison_operand1884.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                            }
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1885, _) => {
                                    let source_ml_source1_data_property_value1885 = *source_ml_source1_data_property_value1885;
                                    source_ml_source1_data_property_value1885.clone()
                                }
                            }), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 114, 101, 97, 108, 109]))
                        } else {
                            {
                                let value_error = self::read_java_script_ordinary_value_error(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1886, _) => {
                                        let source_ml_source1_data_property_value1886 = *source_ml_source1_data_property_value1886;
                                        source_ml_source1_data_property_value1886.clone()
                                    }
                                }, value.clone());
                                if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1887, _) => {
                                            let source_ml_source1_data_property_value1887 = *source_ml_source1_data_property_value1887;
                                            source_ml_source1_data_property_value1887.clone()
                                        }
                                    }), value_error.clone())
                                } else {
                                    match value.clone() {
                                        self::JavaScriptSemanticValue::Undefined => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1894, _) => {
                                                    let source_ml_source1_data_property_value1894 = *source_ml_source1_data_property_value1894;
                                                    source_ml_source1_data_property_value1894.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Null => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1895, _) => {
                                                    let source_ml_source1_data_property_value1895 = *source_ml_source1_data_property_value1895;
                                                    source_ml_source1_data_property_value1895.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1888) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1896, _) => {
                                                    let source_ml_source1_data_property_value1896 = *source_ml_source1_data_property_value1896;
                                                    source_ml_source1_data_property_value1896.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1889) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1897, _) => {
                                                    let source_ml_source1_data_property_value1897 = *source_ml_source1_data_property_value1897;
                                                    source_ml_source1_data_property_value1897.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1890) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1898, _) => {
                                                    let source_ml_source1_data_property_value1898 = *source_ml_source1_data_property_value1898;
                                                    source_ml_source1_data_property_value1898.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1891) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1899, _) => {
                                                    let source_ml_source1_data_property_value1899 = *source_ml_source1_data_property_value1899;
                                                    source_ml_source1_data_property_value1899.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1892) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1900, _) => {
                                                    let source_ml_source1_data_property_value1900 = *source_ml_source1_data_property_value1900;
                                                    source_ml_source1_data_property_value1900.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1893, callable) => {
                                            if callable {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1901, _) => {
                                                        let source_ml_source1_data_property_value1901 = *source_ml_source1_data_property_value1901;
                                                        source_ml_source1_data_property_value1901.clone()
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
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Done(Vec<f64>),
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsLoop6Result {
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop6Done(f64, f64),
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop6Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_utf8_string_units_loop6(bytes: &[f64], mut index: f64, mut scalar: f64, mut remaining: f64) -> self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop6Result {
    loop {
        return if (remaining > (0f64)) {
            {
                let next = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((next < (128f64)) || (next > (191f64))) {
                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop6Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop6Return(vec![(-1f64)])
                } else {
                    {
                        let scalar_2 = (((scalar * 64f64) + next) - 128f64);
                        {
                            let index_2 = (index + 1f64);
                            {
                                let remaining_2 = (remaining - 1f64);
                                {
                                    (index, scalar, remaining) = (index_2, scalar_2, remaining_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop6Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop6Done(index, scalar)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsJoin5 {
    MlTemporaryReadJavaScriptUtf8StringUnitsJoin5Next(f64, f64, f64),
    MlTemporaryReadJavaScriptUtf8StringUnitsJoin5Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_utf8_string_units_loop4(bytes: &[f64], mut units: Vec<f64>, mut index: f64) -> self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let scalar = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                {
                    let index_2 = (index + 1f64);
                    {
                        let remaining = 0f64;
                        {
                            let minimum = 0f64;
                            {
                                let source_ml_source1_s1902 = if ((scalar >= (194f64)) && (scalar <= (223f64))) {
                                    {
                                        let scalar_2 = (scalar - 192f64);
                                        {
                                            let remaining_2 = 1f64;
                                            {
                                                let minimum_2 = 128f64;
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin5::MlTemporaryReadJavaScriptUtf8StringUnitsJoin5Next(scalar_2, remaining_2, minimum_2)
                                            }
                                        }
                                    }
                                } else {
                                    if ((scalar >= (224f64)) && (scalar <= (239f64))) {
                                        {
                                            let scalar_3 = (scalar - 224f64);
                                            {
                                                let remaining_3 = 2f64;
                                                {
                                                    let minimum_3 = 2048f64;
                                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin5::MlTemporaryReadJavaScriptUtf8StringUnitsJoin5Next(scalar_3, remaining_3, minimum_3)
                                                }
                                            }
                                        }
                                    } else {
                                        if ((scalar >= (240f64)) && (scalar <= (244f64))) {
                                            {
                                                let scalar_4 = (scalar - 240f64);
                                                {
                                                    let remaining_4 = 3f64;
                                                    {
                                                        let minimum_4 = 65536f64;
                                                        self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin5::MlTemporaryReadJavaScriptUtf8StringUnitsJoin5Next(scalar_4, remaining_4, minimum_4)
                                                    }
                                                }
                                            }
                                        } else {
                                            if (scalar >= (128f64)) {
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin5::MlTemporaryReadJavaScriptUtf8StringUnitsJoin5Return(vec![(-1f64)])
                                            } else {
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin5::MlTemporaryReadJavaScriptUtf8StringUnitsJoin5Next(scalar, remaining, minimum)
                                            }
                                        }
                                    }
                                };
                                match source_ml_source1_s1902.clone() {
                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin5::MlTemporaryReadJavaScriptUtf8StringUnitsJoin5Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source1_s1903 = self::source_ml_temporary_read_java_script_utf8_string_units_loop6(bytes, index_2, scalar_5, remaining_5);
                                            match source_ml_source1_s1903.clone() {
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop6Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop6Done(index_3, scalar_6) => {
                                                    if (((scalar_6 < minimum_5) || (scalar_6 > (1114111f64))) || ((scalar_6 >= (55296f64)) && (scalar_6 <= (57343f64)))) {
                                                        self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Return(vec![(-1f64)])
                                                    } else {
                                                        if (scalar_6 <= (65535f64)) {
                                                            {
                                                                let units_2 = { let (ml_append_right, ml_append_left) = (vec![scalar_6], units); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    (units, index) = (units_2, index_3);
                                                                    continue;
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let units_3 = { let (ml_append_right, ml_append_left) = (vec![(55296f64 + (((scalar_6 - 65536f64) / 1024f64)).floor())], units); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let units_4 = { let (ml_append_right, ml_append_left) = (vec![(56320f64 + ((scalar_6 - 65536f64) % 1024f64))], units_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                    {
                                                                        (units, index) = (units_4, index_3);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop6Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop6Return(source_ml_temporary_result6) => {
                                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Return(source_ml_temporary_result6)
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin5::MlTemporaryReadJavaScriptUtf8StringUnitsJoin5Return(source_ml_temporary_result6_2) => {
                                        self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Return(source_ml_temporary_result6_2)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Done(units)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsLoop3Result {
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop3Done,
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_utf8_string_units_loop3(bytes: &[f64], mut index: f64) -> self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop3Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (255f64))) {
                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop3Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop3Return(vec![(-1f64)])
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop3Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop3Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_slice_utf16_text_loop7(text: &[f64], mut last: f64, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_points_utf16_text_loop8(text: &[f64], mut result: Vec<Vec<f64>>, mut at: f64) -> Vec<Vec<f64>> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let first = (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                    if ((((first >= (55296f64)) && (first <= (56319f64))) && (second >= (56320f64))) && (second <= (57343f64))) {
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![vec![first, second]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    let at_3 = (at_2 + 1f64);
                                    {
                                        (result, at) = (result_2, at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let result_3 = { let (ml_append_right, ml_append_left) = (vec![vec![first]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_4 = (at + 1f64);
                                {
                                    (result, at) = (result_3, at_4);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_split_utf16_text_loop9(text: &[f64], mut count: f64, mut result: Vec<Vec<f64>>, mut at: f64) -> Vec<Vec<f64>> {
    loop {
        return if ((at < ((text.len() as f64))) && (((result.len() as f64)) < count)) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporarySplitUtf16TextLoop10Result {
    MlTemporarySplitUtf16TextLoop10Done(Vec<Vec<f64>>, f64),
    MlTemporarySplitUtf16TextLoop10Return(Vec<Vec<f64>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_split_utf16_text_loop10(text: &[f64], separator: &[f64], mut count: f64, mut result: Vec<Vec<f64>>, mut start: f64) -> self::MlTemporarySplitUtf16TextLoop10Result {
    loop {
        return {
            let found = self::index_utf16_text(text, separator, start);
            if (found < (0f64)) {
                self::MlTemporarySplitUtf16TextLoop10Result::MlTemporarySplitUtf16TextLoop10Done(result, start)
            } else {
                {
                    let result_2 = { let (ml_append_right, ml_append_left) = (vec![self::slice_utf16_text(text, start, found)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                    if (((result_2.len() as f64)) == count) {
                        self::MlTemporarySplitUtf16TextLoop10Result::MlTemporarySplitUtf16TextLoop10Return(result_2)
                    } else {
                        {
                            let start_2 = (found + (separator.len() as f64));
                            {
                                (result, start) = (result_2, start_2);
                                continue;
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryMatchUtf16TextLoop11Result {
    MlTemporaryMatchUtf16TextLoop11Done,
    MlTemporaryMatchUtf16TextLoop11Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_match_utf16_text_loop11(text: &[f64], search: &[f64], mut start: f64, mut at: f64) -> self::MlTemporaryMatchUtf16TextLoop11Result {
    loop {
        return if (at < ((search.len() as f64))) {
            if {
                let source_ml_source1_comparison_operand1904 = self::ml_array::optional_at(text, self::ml_array::number_index((start + at)));
                {
                    let source_ml_source1_comparison_operand1905 = self::ml_array::optional_at(search, self::ml_array::number_index(at));
                    !(match (source_ml_source1_comparison_operand1904.clone(), source_ml_source1_comparison_operand1905.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                }
            } {
                self::MlTemporaryMatchUtf16TextLoop11Result::MlTemporaryMatchUtf16TextLoop11Return(false)
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
            self::MlTemporaryMatchUtf16TextLoop11Result::MlTemporaryMatchUtf16TextLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryIndexUtf16TextLoop12Result {
    MlTemporaryIndexUtf16TextLoop12Done,
    MlTemporaryIndexUtf16TextLoop12Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_index_utf16_text_loop12(text: &[f64], search: &[f64], mut at: f64) -> self::MlTemporaryIndexUtf16TextLoop12Result {
    loop {
        return if (at <= (((text.len() as f64) - (search.len() as f64)))) {
            if self::match_utf16_text(text, search, at) {
                self::MlTemporaryIndexUtf16TextLoop12Result::MlTemporaryIndexUtf16TextLoop12Return(at)
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
            self::MlTemporaryIndexUtf16TextLoop12Result::MlTemporaryIndexUtf16TextLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLastIndexUtf16TextLoop13Result {
    MlTemporaryLastIndexUtf16TextLoop13Done,
    MlTemporaryLastIndexUtf16TextLoop13Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_last_index_utf16_text_loop13(text: &[f64], search: &[f64], mut at: f64) -> self::MlTemporaryLastIndexUtf16TextLoop13Result {
    loop {
        return if (at >= (0f64)) {
            if self::match_utf16_text(text, search, at) {
                self::MlTemporaryLastIndexUtf16TextLoop13Result::MlTemporaryLastIndexUtf16TextLoop13Return(at)
            } else {
                {
                    let at_2 = (at - 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryLastIndexUtf16TextLoop13Result::MlTemporaryLastIndexUtf16TextLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareUtf16TextLoop14Result {
    MlTemporaryCompareUtf16TextLoop14Done,
    MlTemporaryCompareUtf16TextLoop14Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compare_utf16_text_loop14(left: &[f64], right: &[f64], mut count: f64, mut at: f64) -> self::MlTemporaryCompareUtf16TextLoop14Result {
    loop {
        return if (at < count) {
            {
                let a = (self::ml_array::optional_at(left, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let b = (self::ml_array::optional_at(right, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                    if (a != b) {
                        self::MlTemporaryCompareUtf16TextLoop14Result::MlTemporaryCompareUtf16TextLoop14Return(if (a < b) {
                            (-1f64)
                        } else {
                            1f64
                        })
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
            }
        } else {
            self::MlTemporaryCompareUtf16TextLoop14Result::MlTemporaryCompareUtf16TextLoop14Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_join_utf16_text_loop16(separator: &[f64], mut result: Vec<f64>, mut offset: f64) -> Vec<f64> {
    loop {
        return if (offset < ((separator.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(separator, self::ml_array::number_index(offset))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let offset_2 = (offset + 1f64);
                    {
                        (result, offset) = (result_2, offset_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_join_utf16_text_loop17(mut result: Vec<f64>, value: &[f64], mut offset: f64) -> Vec<f64> {
    loop {
        return if (offset < ((value.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(value, self::ml_array::number_index(offset))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let offset_2 = (offset + 1f64);
                    {
                        (result, offset) = (result_2, offset_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_join_utf16_text_loop15(values: &[Vec<f64>], separator: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let result_3 = if (at != (0f64)) {
                    {
                        let offset = 0f64;
                        {
                            let result_2 = self::source_ml_temporary_join_utf16_text_loop16(separator, result, offset);
                            result_2
                        }
                    }
                } else {
                    result
                };
                {
                    let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| Vec::<f64>::new());
                    {
                        let offset_2 = 0f64;
                        {
                            let result_4 = self::source_ml_temporary_join_utf16_text_loop17(result_3, &(value), offset_2);
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_4, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_trim_utf16_text_loop18(text: &[f64], mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((first < last) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(text, self::ml_array::number_index(first))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let first_2 = (first + 1f64);
                {
                    first = first_2;
                    continue;
                }
            }
        } else {
            first
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_trim_utf16_text_loop19(text: &[f64], mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((last > first) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(text, self::ml_array::number_index((last - 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let last_2 = (last - 1f64);
                {
                    last = last_2;
                    continue;
                }
            }
        } else {
            last
        };
    }
}
