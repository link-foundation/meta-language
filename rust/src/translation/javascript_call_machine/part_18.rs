// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryBeginJavaScriptCallMachineLoop149Result;
use super::accept_java_script_whitespace_unit;
use super::create_java_script_call_frame;
use super::find_java_script_call_function;
use super::match_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_call_machine_error;
use super::read_java_script_call_program_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_begin_java_script_call_machine_loop149;
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
                let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1953, _, _) => source_ml_source2_data_property_value1953.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), identity.clone())));
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
                                    let source_ml_source1_temporary_values148 = arguments_.clone();
                                    {
                                        let source_ml_source1_temporary_index148 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s1954 = self::source_ml_source1_temporary_begin_java_script_call_machine_loop149(heap.clone(), source_ml_source1_temporary_values148, source_ml_source1_temporary_index148.clone());
                                            match source_ml_source2_s1954.clone() {
                                                self::MlSource1TemporaryBeginJavaScriptCallMachineLoop149Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop149Done => {
                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(heap.clone()), vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_)])))
                                                }
                                                self::MlSource1TemporaryBeginJavaScriptCallMachineLoop149Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop149Return(source_ml_source1_temporary_result149) => {
                                                    let source_ml_source1_temporary_result149 = *source_ml_source1_temporary_result149;
                                                    source_ml_source1_temporary_result149.clone()
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
                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1955, _) => {
                    let source_ml_source2_data_property_value1955 = *source_ml_source2_data_property_value1955;
                    source_ml_source2_data_property_value1955.clone()
                }
            }), error.clone())
        } else {
            {
                let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1956) => source_ml_source2_data_property_value1956.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value1957) => source_ml_source2_data_property_value1957.as_slice() }).len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1958, _) => {
                            let source_ml_source2_data_property_value1958 = *source_ml_source2_data_property_value1958;
                            source_ml_source2_data_property_value1958.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    if {
                        let source_ml_source2_s1960 = match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1973) => {
                                let source_ml_source2_data_property_value1973 = *source_ml_source2_data_property_value1973;
                                source_ml_source2_data_property_value1973.clone()
                            }
                        };
                        match source_ml_source2_s1960.clone() {
                            self::JavaScriptCallPending::CallIdle => {
                                true
                            }
                            self::JavaScriptCallPending::WaitCall(source_ml_source2_f1961) => {
                                true
                            }
                            self::JavaScriptCallPending::WaitGet(source_ml_source2_f1962) => {
                                true
                            }
                            self::JavaScriptCallPending::WaitSet(source_ml_source2_f1963) => {
                                true
                            }
                            self::JavaScriptCallPending::WaitKey(source_ml_source2_f1964, source_ml_source2_f1965, source_ml_source2_f1966) => {
                                let source_ml_source2_f1966 = *source_ml_source2_f1966;
                                true
                            }
                            self::JavaScriptCallPending::WaitDefinition(source_ml_source2_f1967, source_ml_source2_f1968, source_ml_source2_f1969) => {
                                let source_ml_source2_f1969 = *source_ml_source2_f1969;
                                true
                            }
                            self::JavaScriptCallPending::WaitError(source_ml_source2_f1970, source_ml_source2_f1971, source_ml_source2_f1972) => {
                                false
                            }
                        }
                    } {
                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1974, _) => {
                                let source_ml_source2_data_property_value1974 = *source_ml_source2_data_property_value1974;
                                source_ml_source2_data_property_value1974.clone()
                            }
                        }), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))
                    } else {
                        if {
                            let source_ml_source2_comparison_operand1979 = match match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value1975) => {
                                    let source_ml_source2_data_property_value1975 = *source_ml_source2_data_property_value1975;
                                    source_ml_source2_data_property_value1975.clone()
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
                                self::JavaScriptCallPending::WaitKey(_, source_ml_source2_data_property_value1976, _) => {
                                    Some(source_ml_source2_data_property_value1976.clone())
                                }
                                self::JavaScriptCallPending::WaitDefinition(_, source_ml_source2_data_property_value1977, _) => {
                                    Some(source_ml_source2_data_property_value1977.clone())
                                }
                                self::JavaScriptCallPending::WaitError(source_ml_source2_data_property_value1978, _, _) => {
                                    Some(source_ml_source2_data_property_value1978.clone())
                                }
                            };
                            {
                                let source_ml_source2_comparison_operand1980 = realm.clone();
                                !(match (source_ml_source2_comparison_operand1979.clone(), source_ml_source2_comparison_operand1980.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                            }
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1981, _) => {
                                    let source_ml_source2_data_property_value1981 = *source_ml_source2_data_property_value1981;
                                    source_ml_source2_data_property_value1981.clone()
                                }
                            }), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 114, 101, 97, 108, 109]))
                        } else {
                            {
                                let value_error = self::read_java_script_ordinary_value_error(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1982, _) => {
                                        let source_ml_source2_data_property_value1982 = *source_ml_source2_data_property_value1982;
                                        source_ml_source2_data_property_value1982.clone()
                                    }
                                }, value.clone());
                                if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1983, _) => {
                                            let source_ml_source2_data_property_value1983 = *source_ml_source2_data_property_value1983;
                                            source_ml_source2_data_property_value1983.clone()
                                        }
                                    }), value_error.clone())
                                } else {
                                    match value.clone() {
                                        self::JavaScriptSemanticValue::Undefined => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1990, _) => {
                                                    let source_ml_source2_data_property_value1990 = *source_ml_source2_data_property_value1990;
                                                    source_ml_source2_data_property_value1990.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Null => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1991, _) => {
                                                    let source_ml_source2_data_property_value1991 = *source_ml_source2_data_property_value1991;
                                                    source_ml_source2_data_property_value1991.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1984) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1992, _) => {
                                                    let source_ml_source2_data_property_value1992 = *source_ml_source2_data_property_value1992;
                                                    source_ml_source2_data_property_value1992.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Number(source_ml_source2_f1985) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1993, _) => {
                                                    let source_ml_source2_data_property_value1993 = *source_ml_source2_data_property_value1993;
                                                    source_ml_source2_data_property_value1993.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f1986) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1994, _) => {
                                                    let source_ml_source2_data_property_value1994 = *source_ml_source2_data_property_value1994;
                                                    source_ml_source2_data_property_value1994.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Text(source_ml_source2_f1987) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1995, _) => {
                                                    let source_ml_source2_data_property_value1995 = *source_ml_source2_data_property_value1995;
                                                    source_ml_source2_data_property_value1995.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1988) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1996, _) => {
                                                    let source_ml_source2_data_property_value1996 = *source_ml_source2_data_property_value1996;
                                                    source_ml_source2_data_property_value1996.clone()
                                                }
                                            }), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                        }
                                        self::JavaScriptSemanticValue::Object(source_ml_source2_f1989, callable) => {
                                            if callable {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value1997, _) => {
                                                        let source_ml_source2_data_property_value1997 = *source_ml_source2_data_property_value1997;
                                                        source_ml_source2_data_property_value1997.clone()
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
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(units: &[f64], mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
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
            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result {
    MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Done,
    MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_semantic_same_value_loop2(left_units: &[f64], right_units: &[f64], mut at: f64) -> self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result {
    loop {
        return if (at < ((left_units.len() as f64))) {
            if (((self::ml_array::optional_at(left_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))) != ((self::ml_array::optional_at(right_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64)))) {
                self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Return(false)
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
            self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop6(mut count: self::ml::Big, mut value: self::ml::Big, mut factor: self::ml::Big) -> self::ml::Big {
    loop {
        return if (count > (self::ml::Big::from_i128(0))) {
            {
                let value_3 = if ((self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let value_2 = value.mul(&factor);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let count_2 = self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (count_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let factor_2 = factor.mul(&factor);
                            {
                                (count, value, factor) = (count_2.clone(), value_3.clone(), factor_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (count, value) = (count_2.clone(), value_3.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop7(mut count: self::ml::Big, mut value: self::ml::Big) -> self::ml::Big {
    loop {
        return if (((count > (self::ml::Big::from_i128(0))) && (value != (self::ml::Big::from_i128(0)))) && (value != (self::ml::Big::from_i128(-1)))) {
            {
                let value_2 = if (value < (self::ml::Big::from_i128(0))) {
                    self::ml::divide(&value.sub(&self::ml::Big::from_i128(1)), &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                } else {
                    self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                };
                {
                    let count_2 = count.sub(&self::ml::Big::from_i128(1));
                    {
                        (count, value) = (count_2.clone(), value_2.clone());
                        continue;
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin5 {
    MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin5Next(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop8Result {
    MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop8Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop8(mut operation: f64, mut negative_left: bool, mut negative_right: bool, mut first: self::ml::Big, mut second: self::ml::Big, mut weight: self::ml::Big, mut result: self::ml::Big) -> self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop8Result {
    loop {
        return if ((first > (self::ml::Big::from_i128(0))) || (second > (self::ml::Big::from_i128(0)))) {
            {
                let a = if negative_left {
                    (self::ml::Big::from_i128(1)).sub(&self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                } else {
                    self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                };
                {
                    let b = if negative_right {
                        (self::ml::Big::from_i128(1)).sub(&self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                    } else {
                        self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                    };
                    {
                        let selected = if (operation == (0f64)) {
                            ((a == (self::ml::Big::from_i128(1))) && (b == (self::ml::Big::from_i128(1))))
                        } else {
                            if (operation == (1f64)) {
                                ((a == (self::ml::Big::from_i128(1))) || (b == (self::ml::Big::from_i128(1))))
                            } else {
                                (a != b)
                            }
                        };
                        {
                            let result_3 = if selected {
                                {
                                    let result_2 = result.add(&weight);
                                    result_2.clone()
                                }
                            } else {
                                result.clone()
                            };
                            {
                                let first_2 = self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                {
                                    let second_2 = self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                    {
                                        let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                                        {
                                            (first, second, weight, result) = (first_2.clone(), second_2.clone(), weight_2.clone(), result_3.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop8Result::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop8Done(weight.clone(), result.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop10(mut count: f64, mut factor: f64, mut position: f64) -> f64 {
    loop {
        return if (position < count) {
            {
                let factor_2 = (factor * 2f64);
                {
                    let position_2 = (position + 1f64);
                    {
                        (factor, position) = (factor_2, position_2);
                        continue;
                    }
                }
            }
        } else {
            factor
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop11(mut operation: f64, mut first: f64, mut second: f64, mut result: f64, mut weight: f64, mut position: f64) -> f64 {
    loop {
        return if (position < (32f64)) {
            {
                let first_bit = (first % 2f64);
                {
                    let second_bit = (second % 2f64);
                    {
                        let result_3 = if (((((operation == (0f64)) && (first_bit == (1f64))) && (second_bit == (1f64))) || ((operation == (1f64)) && ((first_bit == (1f64)) || (second_bit == (1f64))))) || ((operation == (2f64)) && (first_bit != second_bit))) {
                            {
                                let result_2 = (result + weight);
                                result_2
                            }
                        } else {
                            result
                        };
                        {
                            let first_2 = ((first - first_bit) / 2f64);
                            {
                                let second_2 = ((second - second_bit) / 2f64);
                                {
                                    let weight_2 = (weight * 2f64);
                                    {
                                        let position_2 = (position + 1f64);
                                        {
                                            (first, second, result, weight, position) = (first_2, second_2, result_3, weight_2, position_2);
                                            continue;
                                        }
                                    }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Result {
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop49(mut remaining: self::ml::Big, mut shifted: self::ml::Big, mut weight: self::ml::Big, mut quotient: self::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Result {
    loop {
        return if (weight > (self::ml::Big::from_i128(1))) {
            {
                let shifted_2 = self::ml::divide(&shifted, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let weight_2 = self::ml::divide(&weight, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (shifted_2 <= remaining) {
                        {
                            let remaining_2 = remaining.sub(&shifted_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (remaining, shifted, weight, quotient) = (remaining_2.clone(), shifted_2.clone(), weight_2.clone(), quotient_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            (shifted, weight) = (shifted_2.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Done(remaining.clone(), quotient.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDivideJavaScriptBigIntegerLoop48Result {
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop48Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop48(mut remaining: self::ml::Big, mut shifted: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop48Result {
    loop {
        return if (shifted <= remaining) {
            {
                let shifted_2 = shifted.mul(&self::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                    {
                        (shifted, weight) = (shifted_2.clone(), weight_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop48Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop48Done(shifted.clone(), weight.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_power_java_script_big_integer_loop50(mut result: self::ml::Big, mut weight: self::ml::Big, mut remaining: self::ml::Big) -> self::ml::Big {
    loop {
        return if (remaining > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(&remaining, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let result_2 = result.mul(&weight);
                        result_2.clone()
                    }
                } else {
                    result.clone()
                };
                {
                    let remaining_2 = self::ml::divide(&remaining, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (remaining_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let weight_2 = weight.mul(&weight);
                            {
                                (result, weight, remaining) = (result_3.clone(), weight_2.clone(), remaining_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (result, remaining) = (result_3.clone(), remaining_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Result {
    MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Done(f64),
    MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_integer_width_loop62(mut rest: self::ml::Big, mut width: f64) -> self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Result {
    loop {
        return if (rest > (self::ml::Big::from_i128(0))) {
            if (width >= (4096f64)) {
                self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Return((-1f64))
            } else {
                {
                    let rest_2 = self::ml::divide(&rest, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    {
                        let width_2 = (width + 1f64);
                        {
                            (rest, width) = (rest_2.clone(), width_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Done(width)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64 {
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(self::ml::Big),
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Return(Box<self::JavaScriptCallPrimitiveResult>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63 {
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63Next,
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_slice_utf16_text_loop65(text: &[f64], mut last: f64, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryMatchUtf16TextLoop69Result {
    MlSource1TemporaryMatchUtf16TextLoop69Done,
    MlSource1TemporaryMatchUtf16TextLoop69Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_match_utf16_text_loop69(text: &[f64], search: &[f64], mut start: f64, mut at: f64) -> self::MlSource1TemporaryMatchUtf16TextLoop69Result {
    loop {
        return if (at < ((search.len() as f64))) {
            if {
                let source_ml_source2_comparison_operand2001 = self::ml_array::optional_at(text, self::ml_array::number_index((start + at)));
                {
                    let source_ml_source2_comparison_operand2002 = self::ml_array::optional_at(search, self::ml_array::number_index(at));
                    !(match (source_ml_source2_comparison_operand2001.clone(), source_ml_source2_comparison_operand2002.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                }
            } {
                self::MlSource1TemporaryMatchUtf16TextLoop69Result::MlSource1TemporaryMatchUtf16TextLoop69Return(false)
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
            self::MlSource1TemporaryMatchUtf16TextLoop69Result::MlSource1TemporaryMatchUtf16TextLoop69Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryIndexUtf16TextLoop70Result {
    MlSource1TemporaryIndexUtf16TextLoop70Done,
    MlSource1TemporaryIndexUtf16TextLoop70Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_index_utf16_text_loop70(text: &[f64], search: &[f64], mut at: f64) -> self::MlSource1TemporaryIndexUtf16TextLoop70Result {
    loop {
        return if (at <= (((text.len() as f64) - (search.len() as f64)))) {
            if self::match_utf16_text(text, search, at) {
                self::MlSource1TemporaryIndexUtf16TextLoop70Result::MlSource1TemporaryIndexUtf16TextLoop70Return(at)
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
            self::MlSource1TemporaryIndexUtf16TextLoop70Result::MlSource1TemporaryIndexUtf16TextLoop70Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLastIndexUtf16TextLoop71Result {
    MlSource1TemporaryLastIndexUtf16TextLoop71Done,
    MlSource1TemporaryLastIndexUtf16TextLoop71Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_last_index_utf16_text_loop71(text: &[f64], search: &[f64], mut at: f64) -> self::MlSource1TemporaryLastIndexUtf16TextLoop71Result {
    loop {
        return if (at >= (0f64)) {
            if self::match_utf16_text(text, search, at) {
                self::MlSource1TemporaryLastIndexUtf16TextLoop71Result::MlSource1TemporaryLastIndexUtf16TextLoop71Return(at)
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
            self::MlSource1TemporaryLastIndexUtf16TextLoop71Result::MlSource1TemporaryLastIndexUtf16TextLoop71Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompareUtf16TextLoop72Result {
    MlSource1TemporaryCompareUtf16TextLoop72Done,
    MlSource1TemporaryCompareUtf16TextLoop72Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compare_utf16_text_loop72(left: &[f64], right: &[f64], mut count: f64, mut at: f64) -> self::MlSource1TemporaryCompareUtf16TextLoop72Result {
    loop {
        return if (at < count) {
            {
                let a = (self::ml_array::optional_at(left, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let b = (self::ml_array::optional_at(right, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                    if (a != b) {
                        self::MlSource1TemporaryCompareUtf16TextLoop72Result::MlSource1TemporaryCompareUtf16TextLoop72Return(if (a < b) {
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
            self::MlSource1TemporaryCompareUtf16TextLoop72Result::MlSource1TemporaryCompareUtf16TextLoop72Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_trim_utf16_text_loop76(text: &[f64], mut first: f64, mut last: f64) -> f64 {
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
pub fn source_ml_source1_temporary_trim_utf16_text_loop77(text: &[f64], mut first: f64, mut last: f64) -> f64 {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_character_units_utf16_text_loop78(values: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let integer = if value.is_finite() {
                        value.trunc()
                    } else {
                        0f64
                    };
                    {
                        let residue = (integer % 65536f64);
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![if (residue < (0f64)) {
                                (residue + 65536f64)
                            } else {
                                if (residue == (0f64)) {
                                    0f64
                                } else {
                                    residue
                                }
                            }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2, at_2);
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result {
    MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Done(Vec<f64>),
    MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box<self::JavaScriptCallPrimitiveResult>),
}
