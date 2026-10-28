// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptCallMachineErrorJoin53;
use super::MlTemporaryReadJavaScriptCallMachineErrorJoin54;
use super::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result;
use super::MlTemporaryReadJavaScriptCallMachineErrorLoop50Result;
use super::MlTemporaryReadJavaScriptCallMachineErrorLoop51Result;
use super::MlTemporaryReadJavaScriptCallMachineErrorLoop52Result;
use super::find_java_script_call_function;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_call_frame;
use super::source_ml_temporary_read_java_script_call_machine_error_loop50;
use super::source_ml_temporary_read_java_script_call_machine_error_loop51;
use super::source_ml_temporary_read_java_script_call_machine_error_loop52;
use super::test_java_script_call_coordinate;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_machine_error_loop49(mut program: self::JavaScriptCallProgram, mut machine: self::JavaScriptCallMachine, mut slots: f64, mut argument_slots: f64, mut index: f64) -> self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result {
    loop {
        return if (index < (((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1987) => source_ml_source1_data_property_value1987.as_slice() }).len() as f64))) {
            {
                let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1988) => source_ml_source1_data_property_value1988.as_slice() }), self::ml_array::number_index(index));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                } else {
                    {
                        let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1989, _, _) => source_ml_source1_data_property_value1989.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1990, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1990.clone()
                            }
                        })));
                        if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                        } else {
                            if (((((match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1993, _, _, _) => {
                                    source_ml_source1_data_property_value1993
                                }
                            }).len() as f64)) != (match fn_.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value1994, _, _) => {
                                    source_ml_source1_data_property_value1994
                                }
                            })) || !(self::test_java_script_call_coordinate(match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1995, _, _, _, _) => {
                                    source_ml_source1_data_property_value1995
                                }
                            }, (((match fn_.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value1996) => {
                                    source_ml_source1_data_property_value1996
                                }
                            }).len() as f64) + 1f64)))) {
                                self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                            } else {
                                {
                                    let slots_2 = (slots + ((match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1997, _, _, _) => {
                                            source_ml_source1_data_property_value1997
                                        }
                                    }).len() as f64));
                                    {
                                        let argument_slots_2 = (argument_slots + ((match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1998, _, _) => {
                                                source_ml_source1_data_property_value1998
                                            }
                                        }).len() as f64));
                                        if (((slots_2 > (1048576f64)) || (argument_slots_2 > (1048576f64))) || ((((match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2000, _) => {
                                                source_ml_source1_data_property_value2000
                                            }
                                        }).len() as f64)) > (1024f64))) {
                                            self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                        } else {
                                            {
                                                let source_ml_temporary_values46 = match frame.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2001, _, _, _) => {
                                                        source_ml_source1_data_property_value2001
                                                    }
                                                };
                                                {
                                                    let source_ml_temporary_index46 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s2002 = self::source_ml_temporary_read_java_script_call_machine_error_loop50(machine.clone(), source_ml_temporary_values46, source_ml_temporary_index46.clone());
                                                        match source_ml_source1_s2002.clone() {
                                                            self::MlTemporaryReadJavaScriptCallMachineErrorLoop50Result::MlTemporaryReadJavaScriptCallMachineErrorLoop50Done => {
                                                                {
                                                                    let source_ml_temporary_values47 = match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value2003, _, _) => {
                                                                            source_ml_source1_data_property_value2003
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_temporary_index47 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let source_ml_source1_s2004 = self::source_ml_temporary_read_java_script_call_machine_error_loop51(machine.clone(), source_ml_temporary_values47, source_ml_temporary_index47.clone());
                                                                            match source_ml_source1_s2004.clone() {
                                                                                self::MlTemporaryReadJavaScriptCallMachineErrorLoop51Result::MlTemporaryReadJavaScriptCallMachineErrorLoop51Done => {
                                                                                    {
                                                                                        let source_ml_temporary_values48 = match frame.clone().expect("a proven present value") {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2005, _) => {
                                                                                                source_ml_source1_data_property_value2005
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_temporary_index48 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source1_s2006 = self::source_ml_temporary_read_java_script_call_machine_error_loop52(Some(fn_.clone().expect("a proven present value")), source_ml_temporary_values48, source_ml_temporary_index48.clone());
                                                                                                match source_ml_source1_s2006.clone() {
                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop52Result::MlTemporaryReadJavaScriptCallMachineErrorLoop52Done => {
                                                                                                        {
                                                                                                            let pending = match frame.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value2007) => {
                                                                                                                    let source_ml_source1_data_property_value2007 = *source_ml_source1_data_property_value2007;
                                                                                                                    source_ml_source1_data_property_value2007.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_s2020 = match pending.clone() {
                                                                                                                    self::JavaScriptCallPending::CallIdle => {
                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitCall(destination) => {
                                                                                                                        if !(self::test_java_script_call_coordinate(destination, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2028, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2028
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitGet(destination_2) => {
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_2, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2029, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2029
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitSet(destination_3) => {
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_3, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2030, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2030
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitKey(destination_4, source_ml_source1_f2021, source_ml_source1_f2022) => {
                                                                                                                        let source_ml_source1_f2022 = *source_ml_source1_f2022;
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_4, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2031, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2031
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitDefinition(destination_5, source_ml_source1_f2023, source_ml_source1_f2024) => {
                                                                                                                        let source_ml_source1_f2024 = *source_ml_source1_f2024;
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_5, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2032, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2032
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitError(source_ml_source1_f2025, source_ml_source1_f2026, source_ml_source1_f2027) => {
                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                    }
                                                                                                                };
                                                                                                                match source_ml_source1_s2020.clone() {
                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next => {
                                                                                                                        {
                                                                                                                            let source_ml_source1_s2048 = match pending.clone() {
                                                                                                                                self::JavaScriptCallPending::CallIdle => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitCall(source_ml_source1_f2049) => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitGet(source_ml_source1_f2050) => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitSet(source_ml_source1_f2051) => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitKey(source_ml_source1_f2052, realm, source_ml_source1_f2053) => {
                                                                                                                                    let source_ml_source1_f2053 = *source_ml_source1_f2053;
                                                                                                                                    if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value2059, _) => source_ml_source1_data_property_value2059.as_slice() }), &realm.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                    } else {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitDefinition(source_ml_source1_f2054, realm_2, source_ml_source1_f2055) => {
                                                                                                                                    let source_ml_source1_f2055 = *source_ml_source1_f2055;
                                                                                                                                    if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value2061, _) => source_ml_source1_data_property_value2061.as_slice() }), &realm_2.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                    } else {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitError(realm_3, source_ml_source1_f2056, source_ml_source1_f2057) => {
                                                                                                                                    if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value2063, _) => source_ml_source1_data_property_value2063.as_slice() }), &realm_3.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                    } else {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            };
                                                                                                                            match source_ml_source1_s2048.clone() {
                                                                                                                                self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next => {
                                                                                                                                    if ((index < ((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value2065) => source_ml_source1_data_property_value2065.as_slice() }).len() as f64) - 1f64))) && (match pending.clone() {
                                                                                                                                        self::JavaScriptCallPending::CallIdle => {
                                                                                                                                            true
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitCall(source_ml_source1_f2066) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitGet(source_ml_source1_f2067) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitSet(source_ml_source1_f2068) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitKey(source_ml_source1_f2069, source_ml_source1_f2070, source_ml_source1_f2071) => {
                                                                                                                                            let source_ml_source1_f2071 = *source_ml_source1_f2071;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitDefinition(source_ml_source1_f2072, source_ml_source1_f2073, source_ml_source1_f2074) => {
                                                                                                                                            let source_ml_source1_f2074 = *source_ml_source1_f2074;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitError(source_ml_source1_f2075, source_ml_source1_f2076, source_ml_source1_f2077) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                    } || match pending.clone() {
                                                                                                                                        self::JavaScriptCallPending::CallIdle => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitCall(source_ml_source1_f2078) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitGet(source_ml_source1_f2079) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitSet(source_ml_source1_f2080) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitKey(source_ml_source1_f2081, source_ml_source1_f2082, source_ml_source1_f2083) => {
                                                                                                                                            let source_ml_source1_f2083 = *source_ml_source1_f2083;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitDefinition(source_ml_source1_f2084, source_ml_source1_f2085, source_ml_source1_f2086) => {
                                                                                                                                            let source_ml_source1_f2086 = *source_ml_source1_f2086;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitError(source_ml_source1_f2087, source_ml_source1_f2088, source_ml_source1_f2089) => {
                                                                                                                                            true
                                                                                                                                        }
                                                                                                                                    })) {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(self::ml_string::Text::from_units(vec![97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108]))
                                                                                                                                    } else {
                                                                                                                                        {
                                                                                                                                            let index_2 = (index + 1f64);
                                                                                                                                            {
                                                                                                                                                (slots, argument_slots, index) = (slots_2, argument_slots_2, index_2);
                                                                                                                                                continue;
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Return(source_ml_temporary_result54) => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(source_ml_temporary_result54.clone())
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(source_ml_temporary_result54_2) => {
                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(source_ml_temporary_result54_2.clone())
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop52Result::MlTemporaryReadJavaScriptCallMachineErrorLoop52Return(source_ml_temporary_result54_3) => {
                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(source_ml_temporary_result54_3.clone())
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlTemporaryReadJavaScriptCallMachineErrorLoop51Result::MlTemporaryReadJavaScriptCallMachineErrorLoop51Return(source_ml_temporary_result54_4) => {
                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(source_ml_temporary_result54_4.clone())
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporaryReadJavaScriptCallMachineErrorLoop50Result::MlTemporaryReadJavaScriptCallMachineErrorLoop50Return(source_ml_temporary_result54_5) => {
                                                                self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(source_ml_temporary_result54_5.clone())
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
        } else {
            self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryThrowJavaScriptCallValueLoop55Result {
    MlTemporaryThrowJavaScriptCallValueLoop55Done,
    MlTemporaryThrowJavaScriptCallValueLoop55Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_throw_java_script_call_value_loop55(mut machine: self::JavaScriptCallMachine, mut value: self::JavaScriptSemanticValue, mut frames: Vec<self::JavaScriptCallFrame>) -> self::MlTemporaryThrowJavaScriptCallValueLoop55Result {
    loop {
        return if (((frames.len() as f64)) > (0f64)) {
            {
                let frame = self::ml_array::optional_at(&frames, self::ml_array::number_index(((frames.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlTemporaryThrowJavaScriptCallValueLoop55Result::MlTemporaryThrowJavaScriptCallValueLoop55Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value2090, _) => {
                            let source_ml_source1_data_property_value2090 = *source_ml_source1_data_property_value2090;
                            source_ml_source1_data_property_value2090.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                } else {
                    {
                        let handler = self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2091, _) => {
                                source_ml_source1_data_property_value2091
                            }
                        }, self::ml_array::number_index((((match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2092, _) => {
                                source_ml_source1_data_property_value2092
                            }
                        }).len() as f64) - 1f64)));
                        if !(match (handler.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                            {
                                let restored = {
                                    let source_ml_source1_nominal_spread2097 = self::write_java_script_call_register(frame.clone().expect("a proven present value"), match handler.clone().expect("a proven present value") {
                                        self::JavaScriptCallHandler::CallHandler(_, source_ml_source1_data_property_value2093) => {
                                            source_ml_source1_data_property_value2093
                                        }
                                    }, value.clone());
                                    {
                                        let source_ml_source1_nominal_field2098 = match source_ml_source1_nominal_spread2097.clone() {
                                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value2099, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2099.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field2100 = match source_ml_source1_nominal_spread2097.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value2101, _, _, _, _) => {
                                                    source_ml_source1_data_property_value2101
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field2102 = match source_ml_source1_nominal_spread2097.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2103, _, _, _) => {
                                                        source_ml_source1_data_property_value2103
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_field2104 = match source_ml_source1_nominal_spread2097.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value2105, _, _) => {
                                                            source_ml_source1_data_property_value2105
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field2106 = match source_ml_source1_nominal_spread2097.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2107, _) => {
                                                                source_ml_source1_data_property_value2107
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field2108 = match source_ml_source1_nominal_spread2097.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value2109) => {
                                                                    let source_ml_source1_data_property_value2109 = *source_ml_source1_data_property_value2109;
                                                                    source_ml_source1_data_property_value2109.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_override2110 = match handler.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallHandler::CallHandler(source_ml_source1_data_property_value2094, _) => {
                                                                        source_ml_source1_data_property_value2094
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_override2111 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2096, _) => {
                                                                            source_ml_source1_data_property_value2096
                                                                        }
                                                                    }, 0f64, (-1f64));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field2098.clone(), source_ml_source1_nominal_override2110, source_ml_source1_nominal_field2102, source_ml_source1_nominal_field2104, source_ml_source1_nominal_override2111, Box::new(source_ml_source1_nominal_field2108.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                                self::MlTemporaryThrowJavaScriptCallValueLoop55Result::MlTemporaryThrowJavaScriptCallValueLoop55Return(Box::new(self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value2112, _) => {
                                        let source_ml_source1_data_property_value2112 = *source_ml_source1_data_property_value2112;
                                        source_ml_source1_data_property_value2112.clone()
                                    }
                                }, frames, restored.clone())))))
                            }
                        } else {
                            {
                                let frames_2 = self::ml_array::slice(&frames, 0f64, (-1f64));
                                {
                                    frames = frames_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryThrowJavaScriptCallValueLoop55Result::MlTemporaryThrowJavaScriptCallValueLoop55Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPushJavaScriptCallLoop57Result {
    MlTemporaryPushJavaScriptCallLoop57Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_push_java_script_call_loop57(mut slots: f64, mut argument_slots: f64, mut source_ml_temporary_values56: Vec<self::JavaScriptCallFrame>, mut source_ml_temporary_index56: self::ml::Big) -> self::MlTemporaryPushJavaScriptCallLoop57Result {
    loop {
        return if (source_ml_temporary_index56 < (self::ml::Big::from_u128(source_ml_temporary_values56.len() as u128))) {
            {
                let active = self::ml_array::at(&source_ml_temporary_values56, source_ml_temporary_index56.to_index());
                {
                    let slots_2 = (slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2113, _, _, _) => source_ml_source1_data_property_value2113.as_slice() }).len() as f64));
                    {
                        let argument_slots_2 = (argument_slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value2114, _, _) => source_ml_source1_data_property_value2114.as_slice() }).len() as f64));
                        {
                            let source_ml_temporary_index56_2 = source_ml_temporary_index56.add(&self::ml::Big::from_i128(1));
                            {
                                (slots, argument_slots, source_ml_temporary_index56) = (slots_2, argument_slots_2, source_ml_temporary_index56_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPushJavaScriptCallLoop57Result::MlTemporaryPushJavaScriptCallLoop57Done(slots, argument_slots)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryBeginJavaScriptCallMachineLoop59Result {
    MlTemporaryBeginJavaScriptCallMachineLoop59Done,
    MlTemporaryBeginJavaScriptCallMachineLoop59Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_begin_java_script_call_machine_loop59(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_temporary_values58: Vec<self::JavaScriptSemanticValue>, mut source_ml_temporary_index58: self::ml::Big) -> self::MlTemporaryBeginJavaScriptCallMachineLoop59Result {
    loop {
        return if (source_ml_temporary_index58 < (self::ml::Big::from_u128(source_ml_temporary_values58.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_temporary_values58, source_ml_temporary_index58.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlTemporaryBeginJavaScriptCallMachineLoop59Result::MlTemporaryBeginJavaScriptCallMachineLoop59Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))))
                } else {
                    {
                        let source_ml_temporary_index58_2 = source_ml_temporary_index58.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index58 = source_ml_temporary_index58_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryBeginJavaScriptCallMachineLoop59Result::MlTemporaryBeginJavaScriptCallMachineLoop59Done
        };
    }
}
