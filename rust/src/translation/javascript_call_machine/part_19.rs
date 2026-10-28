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
        return if (index < (((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value2029) => source_ml_source1_data_property_value2029.as_slice() }).len() as f64))) {
            {
                let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value2030) => source_ml_source1_data_property_value2030.as_slice() }), self::ml_array::number_index(index));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                } else {
                    {
                        let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value2031, _, _) => source_ml_source1_data_property_value2031.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value2032, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2032.clone()
                            }
                        })));
                        if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                        } else {
                            if (((((match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2035, _, _, _) => {
                                    source_ml_source1_data_property_value2035
                                }
                            }).len() as f64)) != (match fn_.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2036, _, _) => {
                                    source_ml_source1_data_property_value2036
                                }
                            })) || !(self::test_java_script_call_coordinate(match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value2037, _, _, _, _) => {
                                    source_ml_source1_data_property_value2037
                                }
                            }, (((match fn_.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2038) => {
                                    source_ml_source1_data_property_value2038
                                }
                            }).len() as f64) + 1f64)))) {
                                self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                            } else {
                                {
                                    let slots_2 = (slots + ((match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2039, _, _, _) => {
                                            source_ml_source1_data_property_value2039
                                        }
                                    }).len() as f64));
                                    {
                                        let argument_slots_2 = (argument_slots + ((match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value2040, _, _) => {
                                                source_ml_source1_data_property_value2040
                                            }
                                        }).len() as f64));
                                        if (((slots_2 > (1048576f64)) || (argument_slots_2 > (1048576f64))) || ((((match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2042, _) => {
                                                source_ml_source1_data_property_value2042
                                            }
                                        }).len() as f64)) > (1024f64))) {
                                            self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                        } else {
                                            {
                                                let source_ml_temporary_values46 = match frame.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2043, _, _, _) => {
                                                        source_ml_source1_data_property_value2043
                                                    }
                                                };
                                                {
                                                    let source_ml_temporary_index46 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s2044 = self::source_ml_temporary_read_java_script_call_machine_error_loop50(machine.clone(), source_ml_temporary_values46, source_ml_temporary_index46.clone());
                                                        match source_ml_source1_s2044.clone() {
                                                            self::MlTemporaryReadJavaScriptCallMachineErrorLoop50Result::MlTemporaryReadJavaScriptCallMachineErrorLoop50Done => {
                                                                {
                                                                    let source_ml_temporary_values47 = match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value2045, _, _) => {
                                                                            source_ml_source1_data_property_value2045
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_temporary_index47 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let source_ml_source1_s2046 = self::source_ml_temporary_read_java_script_call_machine_error_loop51(machine.clone(), source_ml_temporary_values47, source_ml_temporary_index47.clone());
                                                                            match source_ml_source1_s2046.clone() {
                                                                                self::MlTemporaryReadJavaScriptCallMachineErrorLoop51Result::MlTemporaryReadJavaScriptCallMachineErrorLoop51Done => {
                                                                                    {
                                                                                        let source_ml_temporary_values48 = match frame.clone().expect("a proven present value") {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2047, _) => {
                                                                                                source_ml_source1_data_property_value2047
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_temporary_index48 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source1_s2048 = self::source_ml_temporary_read_java_script_call_machine_error_loop52(Some(fn_.clone().expect("a proven present value")), source_ml_temporary_values48, source_ml_temporary_index48.clone());
                                                                                                match source_ml_source1_s2048.clone() {
                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop52Result::MlTemporaryReadJavaScriptCallMachineErrorLoop52Done => {
                                                                                                        {
                                                                                                            let pending = match frame.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value2049) => {
                                                                                                                    let source_ml_source1_data_property_value2049 = *source_ml_source1_data_property_value2049;
                                                                                                                    source_ml_source1_data_property_value2049.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_s2062 = match pending.clone() {
                                                                                                                    self::JavaScriptCallPending::CallIdle => {
                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitCall(destination) => {
                                                                                                                        if !(self::test_java_script_call_coordinate(destination, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2070, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2070
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitGet(destination_2) => {
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_2, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2071, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2071
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitSet(destination_3) => {
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_3, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2072, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2072
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitKey(destination_4, source_ml_source1_f2063, source_ml_source1_f2064) => {
                                                                                                                        let source_ml_source1_f2064 = *source_ml_source1_f2064;
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_4, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2073, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2073
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitDefinition(destination_5, source_ml_source1_f2065, source_ml_source1_f2066) => {
                                                                                                                        let source_ml_source1_f2066 = *source_ml_source1_f2066;
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_5, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2074, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2074
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitError(source_ml_source1_f2067, source_ml_source1_f2068, source_ml_source1_f2069) => {
                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next
                                                                                                                    }
                                                                                                                };
                                                                                                                match source_ml_source1_s2062.clone() {
                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin53::MlTemporaryReadJavaScriptCallMachineErrorJoin53Next => {
                                                                                                                        {
                                                                                                                            let source_ml_source1_s2090 = match pending.clone() {
                                                                                                                                self::JavaScriptCallPending::CallIdle => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitCall(source_ml_source1_f2091) => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitGet(source_ml_source1_f2092) => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitSet(source_ml_source1_f2093) => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitKey(source_ml_source1_f2094, realm, source_ml_source1_f2095) => {
                                                                                                                                    let source_ml_source1_f2095 = *source_ml_source1_f2095;
                                                                                                                                    if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value2101, _) => source_ml_source1_data_property_value2101.as_slice() }), &realm.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                    } else {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitDefinition(source_ml_source1_f2096, realm_2, source_ml_source1_f2097) => {
                                                                                                                                    let source_ml_source1_f2097 = *source_ml_source1_f2097;
                                                                                                                                    if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value2103, _) => source_ml_source1_data_property_value2103.as_slice() }), &realm_2.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                    } else {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitError(realm_3, source_ml_source1_f2098, source_ml_source1_f2099) => {
                                                                                                                                    if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value2105, _) => source_ml_source1_data_property_value2105.as_slice() }), &realm_3.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                    } else {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            };
                                                                                                                            match source_ml_source1_s2090.clone() {
                                                                                                                                self::MlTemporaryReadJavaScriptCallMachineErrorJoin54::MlTemporaryReadJavaScriptCallMachineErrorJoin54Next => {
                                                                                                                                    if ((index < ((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value2107) => source_ml_source1_data_property_value2107.as_slice() }).len() as f64) - 1f64))) && (match pending.clone() {
                                                                                                                                        self::JavaScriptCallPending::CallIdle => {
                                                                                                                                            true
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitCall(source_ml_source1_f2108) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitGet(source_ml_source1_f2109) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitSet(source_ml_source1_f2110) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitKey(source_ml_source1_f2111, source_ml_source1_f2112, source_ml_source1_f2113) => {
                                                                                                                                            let source_ml_source1_f2113 = *source_ml_source1_f2113;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitDefinition(source_ml_source1_f2114, source_ml_source1_f2115, source_ml_source1_f2116) => {
                                                                                                                                            let source_ml_source1_f2116 = *source_ml_source1_f2116;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitError(source_ml_source1_f2117, source_ml_source1_f2118, source_ml_source1_f2119) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                    } || match pending.clone() {
                                                                                                                                        self::JavaScriptCallPending::CallIdle => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitCall(source_ml_source1_f2120) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitGet(source_ml_source1_f2121) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitSet(source_ml_source1_f2122) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitKey(source_ml_source1_f2123, source_ml_source1_f2124, source_ml_source1_f2125) => {
                                                                                                                                            let source_ml_source1_f2125 = *source_ml_source1_f2125;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitDefinition(source_ml_source1_f2126, source_ml_source1_f2127, source_ml_source1_f2128) => {
                                                                                                                                            let source_ml_source1_f2128 = *source_ml_source1_f2128;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitError(source_ml_source1_f2129, source_ml_source1_f2130, source_ml_source1_f2131) => {
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
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value2132, _) => {
                            let source_ml_source1_data_property_value2132 = *source_ml_source1_data_property_value2132;
                            source_ml_source1_data_property_value2132.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                } else {
                    {
                        let handler = self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2133, _) => {
                                source_ml_source1_data_property_value2133
                            }
                        }, self::ml_array::number_index((((match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2134, _) => {
                                source_ml_source1_data_property_value2134
                            }
                        }).len() as f64) - 1f64)));
                        if !(match (handler.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                            {
                                let restored = {
                                    let source_ml_source1_nominal_spread2139 = self::write_java_script_call_register(frame.clone().expect("a proven present value"), match handler.clone().expect("a proven present value") {
                                        self::JavaScriptCallHandler::CallHandler(_, source_ml_source1_data_property_value2135) => {
                                            source_ml_source1_data_property_value2135
                                        }
                                    }, value.clone());
                                    {
                                        let source_ml_source1_nominal_field2140 = match source_ml_source1_nominal_spread2139.clone() {
                                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value2141, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2141.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field2142 = match source_ml_source1_nominal_spread2139.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value2143, _, _, _, _) => {
                                                    source_ml_source1_data_property_value2143
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field2144 = match source_ml_source1_nominal_spread2139.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2145, _, _, _) => {
                                                        source_ml_source1_data_property_value2145
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_field2146 = match source_ml_source1_nominal_spread2139.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value2147, _, _) => {
                                                            source_ml_source1_data_property_value2147
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field2148 = match source_ml_source1_nominal_spread2139.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2149, _) => {
                                                                source_ml_source1_data_property_value2149
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field2150 = match source_ml_source1_nominal_spread2139.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value2151) => {
                                                                    let source_ml_source1_data_property_value2151 = *source_ml_source1_data_property_value2151;
                                                                    source_ml_source1_data_property_value2151.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_override2152 = match handler.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallHandler::CallHandler(source_ml_source1_data_property_value2136, _) => {
                                                                        source_ml_source1_data_property_value2136
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_override2153 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2138, _) => {
                                                                            source_ml_source1_data_property_value2138
                                                                        }
                                                                    }, 0f64, (-1f64));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field2140.clone(), source_ml_source1_nominal_override2152, source_ml_source1_nominal_field2144, source_ml_source1_nominal_field2146, source_ml_source1_nominal_override2153, Box::new(source_ml_source1_nominal_field2150.clone()))
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
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value2154, _) => {
                                        let source_ml_source1_data_property_value2154 = *source_ml_source1_data_property_value2154;
                                        source_ml_source1_data_property_value2154.clone()
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
                    let slots_2 = (slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2155, _, _, _) => source_ml_source1_data_property_value2155.as_slice() }).len() as f64));
                    {
                        let argument_slots_2 = (argument_slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value2156, _, _) => source_ml_source1_data_property_value2156.as_slice() }).len() as f64));
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
