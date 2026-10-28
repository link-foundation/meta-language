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
        return if (index < (((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value2008) => source_ml_source1_data_property_value2008.as_slice() }).len() as f64))) {
            {
                let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value2009) => source_ml_source1_data_property_value2009.as_slice() }), self::ml_array::number_index(index));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                } else {
                    {
                        let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value2010, _, _) => source_ml_source1_data_property_value2010.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value2011, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2011.clone()
                            }
                        })));
                        if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                        } else {
                            if (((((match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2014, _, _, _) => {
                                    source_ml_source1_data_property_value2014
                                }
                            }).len() as f64)) != (match fn_.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2015, _, _) => {
                                    source_ml_source1_data_property_value2015
                                }
                            })) || !(self::test_java_script_call_coordinate(match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value2016, _, _, _, _) => {
                                    source_ml_source1_data_property_value2016
                                }
                            }, (((match fn_.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2017) => {
                                    source_ml_source1_data_property_value2017
                                }
                            }).len() as f64) + 1f64)))) {
                                self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                            } else {
                                {
                                    let slots_2 = (slots + ((match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2018, _, _, _) => {
                                            source_ml_source1_data_property_value2018
                                        }
                                    }).len() as f64));
                                    {
                                        let argument_slots_2 = (argument_slots + ((match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value2019, _, _) => {
                                                source_ml_source1_data_property_value2019
                                            }
                                        }).len() as f64));
                                        if (((slots_2 > (1048576f64)) || (argument_slots_2 > (1048576f64))) || ((((match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2021, _) => {
                                                source_ml_source1_data_property_value2021
                                            }
                                        }).len() as f64)) > (1024f64))) {
                                            self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                        } else {
                                            {
                                                let source_ml_temporary_values46 = match frame.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2022, _, _, _) => {
                                                        source_ml_source1_data_property_value2022
                                                    }
                                                };
                                                {
                                                    let source_ml_temporary_index46 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s2023 = self::source_ml_temporary_read_java_script_call_machine_error_loop50(machine.clone(), source_ml_temporary_values46, source_ml_temporary_index46.clone());
                                                        match source_ml_source1_s2023.clone() {
                                                            self::MlTemporaryReadJavaScriptCallMachineErrorLoop50Result::MlTemporaryReadJavaScriptCallMachineErrorLoop50Done => {
                                                                {
                                                                    let source_ml_temporary_values47 = match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value2024, _, _) => {
                                                                            source_ml_source1_data_property_value2024
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_temporary_index47 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let source_ml_source1_s2025 = self::source_ml_temporary_read_java_script_call_machine_error_loop51(machine.clone(), source_ml_temporary_values47, source_ml_temporary_index47.clone());
                                                                            match source_ml_source1_s2025.clone() {
                                                                                self::MlTemporaryReadJavaScriptCallMachineErrorLoop51Result::MlTemporaryReadJavaScriptCallMachineErrorLoop51Done => {
                                                                                    {
                                                                                        let source_ml_temporary_values48 = match frame.clone().expect("a proven present value") {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2026, _) => {
                                                                                                source_ml_source1_data_property_value2026
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_temporary_index48 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source1_s2027 = self::source_ml_temporary_read_java_script_call_machine_error_loop52(Some(fn_.clone().expect("a proven present value")), source_ml_temporary_values48, source_ml_temporary_index48.clone());
                                                                                                match source_ml_source1_s2027.clone() {
                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop52Result::MlTemporaryReadJavaScriptCallMachineErrorLoop52Done => {
                                                                                                        {
                                                                                                            let pending = match frame.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value2028) => {
                                                                                                                    let source_ml_source1_data_property_value2028 = *source_ml_source1_data_property_value2028;
                                                                                                                    source_ml_source1_data_property_value2028.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_s2041 = match pending.clone() {
                                                                                                                    self::JavaScriptCallPending::CallIdle => {
                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitCall(destination) => {
                                                                                                                        if !(self::test_java_script_call_coordinate(destination, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2049, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2049
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitGet(destination_2) => {
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_2, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2050, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2050
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitSet(destination_3) => {
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_3, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2051, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2051
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitKey(destination_4, source_ml_source1_f2042, source_ml_source1_f2043) => {
                                                                                                                        let source_ml_source1_f2043 = *source_ml_source1_f2043;
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_4, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2052, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2052
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitDefinition(destination_5, source_ml_source1_f2044, source_ml_source1_f2045) => {
                                                                                                                        let source_ml_source1_f2045 = *source_ml_source1_f2045;
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_5, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2053, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2053
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitError(source_ml_source1_f2046, source_ml_source1_f2047, source_ml_source1_f2048) => {
                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                    }
                                                                                                                };
                                                                                                                match source_ml_source1_s2041.clone() {
                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next => {
                                                                                                                        {
                                                                                                                            let source_ml_source1_s2069 = match pending.clone() {
                                                                                                                                self::JavaScriptCallPending::CallIdle => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitCall(source_ml_source1_f2070) => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitGet(source_ml_source1_f2071) => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitSet(source_ml_source1_f2072) => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitKey(source_ml_source1_f2073, realm, source_ml_source1_f2074) => {
                                                                                                                                    let source_ml_source1_f2074 = *source_ml_source1_f2074;
                                                                                                                                    if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value2080, _) => source_ml_source1_data_property_value2080.as_slice() }), &realm.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                    } else {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitDefinition(source_ml_source1_f2075, realm_2, source_ml_source1_f2076) => {
                                                                                                                                    let source_ml_source1_f2076 = *source_ml_source1_f2076;
                                                                                                                                    if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value2082, _) => source_ml_source1_data_property_value2082.as_slice() }), &realm_2.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                    } else {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitError(realm_3, source_ml_source1_f2077, source_ml_source1_f2078) => {
                                                                                                                                    if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value2084, _) => source_ml_source1_data_property_value2084.as_slice() }), &realm_3.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                    } else {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            };
                                                                                                                            match source_ml_source1_s2069.clone() {
                                                                                                                                self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next => {
                                                                                                                                    if ((index < ((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value2086) => source_ml_source1_data_property_value2086.as_slice() }).len() as f64) - 1f64))) && (match pending.clone() {
                                                                                                                                        self::JavaScriptCallPending::CallIdle => {
                                                                                                                                            true
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitCall(source_ml_source1_f2087) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitGet(source_ml_source1_f2088) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitSet(source_ml_source1_f2089) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitKey(source_ml_source1_f2090, source_ml_source1_f2091, source_ml_source1_f2092) => {
                                                                                                                                            let source_ml_source1_f2092 = *source_ml_source1_f2092;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitDefinition(source_ml_source1_f2093, source_ml_source1_f2094, source_ml_source1_f2095) => {
                                                                                                                                            let source_ml_source1_f2095 = *source_ml_source1_f2095;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitError(source_ml_source1_f2096, source_ml_source1_f2097, source_ml_source1_f2098) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                    } || match pending.clone() {
                                                                                                                                        self::JavaScriptCallPending::CallIdle => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitCall(source_ml_source1_f2099) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitGet(source_ml_source1_f2100) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitSet(source_ml_source1_f2101) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitKey(source_ml_source1_f2102, source_ml_source1_f2103, source_ml_source1_f2104) => {
                                                                                                                                            let source_ml_source1_f2104 = *source_ml_source1_f2104;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitDefinition(source_ml_source1_f2105, source_ml_source1_f2106, source_ml_source1_f2107) => {
                                                                                                                                            let source_ml_source1_f2107 = *source_ml_source1_f2107;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitError(source_ml_source1_f2108, source_ml_source1_f2109, source_ml_source1_f2110) => {
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
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value2111, _) => {
                            let source_ml_source1_data_property_value2111 = *source_ml_source1_data_property_value2111;
                            source_ml_source1_data_property_value2111.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                } else {
                    {
                        let handler = self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2112, _) => {
                                source_ml_source1_data_property_value2112
                            }
                        }, self::ml_array::number_index((((match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2113, _) => {
                                source_ml_source1_data_property_value2113
                            }
                        }).len() as f64) - 1f64)));
                        if !(match (handler.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                            {
                                let restored = {
                                    let source_ml_source1_nominal_spread2118 = self::write_java_script_call_register(frame.clone().expect("a proven present value"), match handler.clone().expect("a proven present value") {
                                        self::JavaScriptCallHandler::CallHandler(_, source_ml_source1_data_property_value2114) => {
                                            source_ml_source1_data_property_value2114
                                        }
                                    }, value.clone());
                                    {
                                        let source_ml_source1_nominal_field2119 = match source_ml_source1_nominal_spread2118.clone() {
                                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value2120, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2120.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field2121 = match source_ml_source1_nominal_spread2118.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value2122, _, _, _, _) => {
                                                    source_ml_source1_data_property_value2122
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field2123 = match source_ml_source1_nominal_spread2118.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2124, _, _, _) => {
                                                        source_ml_source1_data_property_value2124
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_field2125 = match source_ml_source1_nominal_spread2118.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value2126, _, _) => {
                                                            source_ml_source1_data_property_value2126
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field2127 = match source_ml_source1_nominal_spread2118.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2128, _) => {
                                                                source_ml_source1_data_property_value2128
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field2129 = match source_ml_source1_nominal_spread2118.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value2130) => {
                                                                    let source_ml_source1_data_property_value2130 = *source_ml_source1_data_property_value2130;
                                                                    source_ml_source1_data_property_value2130.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_override2131 = match handler.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallHandler::CallHandler(source_ml_source1_data_property_value2115, _) => {
                                                                        source_ml_source1_data_property_value2115
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_override2132 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2117, _) => {
                                                                            source_ml_source1_data_property_value2117
                                                                        }
                                                                    }, 0f64, (-1f64));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field2119.clone(), source_ml_source1_nominal_override2131, source_ml_source1_nominal_field2123, source_ml_source1_nominal_field2125, source_ml_source1_nominal_override2132, Box::new(source_ml_source1_nominal_field2129.clone()))
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
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value2133, _) => {
                                        let source_ml_source1_data_property_value2133 = *source_ml_source1_data_property_value2133;
                                        source_ml_source1_data_property_value2133.clone()
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
                    let slots_2 = (slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2134, _, _, _) => source_ml_source1_data_property_value2134.as_slice() }).len() as f64));
                    {
                        let argument_slots_2 = (argument_slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value2135, _, _) => source_ml_source1_data_property_value2135.as_slice() }).len() as f64));
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
