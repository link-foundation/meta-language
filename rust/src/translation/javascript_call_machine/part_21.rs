// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptNumberCallState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Result;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Result;
use super::find_java_script_call_function;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_number_call_source_operation;
use super::read_java_script_number_call_state_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_call_frame;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop143;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop144;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop145;
use super::test_java_script_call_coordinate;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_machine_error_loop142(mut program: self::JavaScriptCallProgram, mut machine: self::JavaScriptCallMachine, mut slots: f64, mut argument_slots: f64, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result {
    loop {
        return match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(shared_constructor_field4, shared_constructor_field5) => {
                let shared_constructor_field4 = *shared_constructor_field4;
                match program.clone() {
                    self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                        let shared_constructor_field3 = *shared_constructor_field3;
                        if (index < ((shared_constructor_field5.len() as f64))) {
                            {
                                let frame = self::ml_array::optional_at(&shared_constructor_field5, self::ml_array::number_index(index));
                                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                                } else {
                                    {
                                        let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2440, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value2440.clone()
                                            }
                                        })));
                                        if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                        } else {
                                            if (((((match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2443, _, _, _) => {
                                                    source_ml_source2_data_property_value2443
                                                }
                                            }).len() as f64)) != (match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2444, _, _) => {
                                                    source_ml_source2_data_property_value2444
                                                }
                                            })) || !(self::test_java_script_call_coordinate(match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2445, _, _, _, _) => {
                                                    source_ml_source2_data_property_value2445
                                                }
                                            }, (((match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2446) => {
                                                    source_ml_source2_data_property_value2446
                                                }
                                            }).len() as f64) + 1f64)))) {
                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                            } else {
                                                {
                                                    let slots_2 = (slots + ((match frame.clone().expect("a proven present value") {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2447, _, _, _) => {
                                                            source_ml_source2_data_property_value2447
                                                        }
                                                    }).len() as f64));
                                                    {
                                                        let argument_slots_2 = (argument_slots + ((match frame.clone().expect("a proven present value") {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2448, _, _) => {
                                                                source_ml_source2_data_property_value2448
                                                            }
                                                        }).len() as f64));
                                                        if (((slots_2 > (1048576f64)) || (argument_slots_2 > (1048576f64))) || ((((match frame.clone().expect("a proven present value") {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2450, _) => {
                                                                source_ml_source2_data_property_value2450
                                                            }
                                                        }).len() as f64)) > (1024f64))) {
                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                        } else {
                                                            {
                                                                let source_ml_source1_temporary_values139 = match frame.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2451, _, _, _) => {
                                                                        source_ml_source2_data_property_value2451
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_temporary_index139 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let source_ml_source2_s2452 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop143(machine.clone(), source_ml_source1_temporary_values139, source_ml_source1_temporary_index139.clone());
                                                                        match source_ml_source2_s2452.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Done => {
                                                                                {
                                                                                    let source_ml_source1_temporary_values140 = match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2453, _, _) => {
                                                                                            source_ml_source2_data_property_value2453
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_temporary_index140 = self::ml::Big::from_i128(0);
                                                                                        {
                                                                                            let source_ml_source2_s2454 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop144(machine.clone(), source_ml_source1_temporary_values140, source_ml_source1_temporary_index140.clone());
                                                                                            match source_ml_source2_s2454.clone() {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Done => {
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_values141 = match frame.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2455, _) => {
                                                                                                                source_ml_source2_data_property_value2455
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_index141 = self::ml::Big::from_i128(0);
                                                                                                            {
                                                                                                                let source_ml_source2_s2456 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop145(Some(fn_.clone().expect("a proven present value")), source_ml_source1_temporary_values141, source_ml_source1_temporary_index141.clone());
                                                                                                                match source_ml_source2_s2456.clone() {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Done => {
                                                                                                                        {
                                                                                                                            let pending = match frame.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2457) => {
                                                                                                                                    let source_ml_source2_data_property_value2457 = *source_ml_source2_data_property_value2457;
                                                                                                                                    source_ml_source2_data_property_value2457.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_s2473 = match pending.clone() {
                                                                                                                                    self::JavaScriptCallPending::CallIdle => {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Next
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitCall(destination) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2483, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2483
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitGet(destination_2) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_2, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2484, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2484
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitSet(destination_3) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_3, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2485, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2485
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitKey(destination_4, _, _) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_4, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2486, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2486
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitDefinition(destination_5, _, _) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_5, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2487, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2487
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitNumber(destination_6, _, _) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_6, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2488, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2488
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitError(_, _, _) => {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Next
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                match source_ml_source2_s2473.clone() {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Next => {
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_s2518 = match pending.clone() {
                                                                                                                                                self::JavaScriptCallPending::CallIdle => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitCall(_) => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitGet(_) => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitSet(_) => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitKey(_, realm, _) => {
                                                                                                                                                    if !(self::ml_array::includes(&shared_constructor_field2, &realm.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                    } else {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitDefinition(_, realm_2, _) => {
                                                                                                                                                    if !(self::ml_array::includes(&shared_constructor_field2, &realm_2.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                    } else {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitNumber(destination_7, realm_3, state) => {
                                                                                                                                                    let state = *state;
                                                                                                                                                    if !(self::ml_array::includes(&shared_constructor_field2, &realm_3.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                    } else {
                                                                                                                                                        {
                                                                                                                                                            let number_state = state.clone();
                                                                                                                                                            {
                                                                                                                                                                let state_error = self::read_java_script_number_call_state_error(shared_constructor_field4.clone(), number_state.clone());
                                                                                                                                                                if (state_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(state_error.clone())
                                                                                                                                                                } else {
                                                                                                                                                                    if ((self::read_java_script_number_call_source_operation(fn_.clone().expect("a proven present value"), frame.clone().expect("a proven present value"), destination_7, realm_3.clone(), shared_constructor_field3.clone(), number_state.clone())) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                                    } else {
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_s2541 = if (index < (((shared_constructor_field5.len() as f64) - 1f64))) {
                                                                                                                                                                                match number_state.clone() {
                                                                                                                                                                                    self::JavaScriptNumberCallState::NumberProtocolState(_) => {
                                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Next
                                                                                                                                                                                    }
                                                                                                                                                                                    self::JavaScriptNumberCallState::NumberPrimitiveValue(_) => {
                                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Return(self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 97, 110, 110, 111, 116, 32, 104, 97, 118, 101, 32, 97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108]))
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            } else {
                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Next
                                                                                                                                                                            };
                                                                                                                                                                            match source_ml_source2_s2541.clone() {
                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Next => {
                                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                                                                }
                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Return(source_ml_source1_temporary_result148) => {
                                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(source_ml_source1_temporary_result148.clone())
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitError(realm_4, _, _) => {
                                                                                                                                                    if !(self::ml_array::includes(&shared_constructor_field2, &realm_4.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                    } else {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            match source_ml_source2_s2518.clone() {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next => {
                                                                                                                                                    if ((index < (((shared_constructor_field5.len() as f64) - 1f64))) && (match pending.clone() {
                                                                                                                                                        self::JavaScriptCallPending::CallIdle => {
                                                                                                                                                            true
                                                                                                                                                        }
                                                                                                                                                        _ => {
                                                                                                                                                            false
                                                                                                                                                        }
                                                                                                                                                    } || match pending.clone() {
                                                                                                                                                        self::JavaScriptCallPending::WaitError(_, _, _) => {
                                                                                                                                                            true
                                                                                                                                                        }
                                                                                                                                                        _ => {
                                                                                                                                                            false
                                                                                                                                                        }
                                                                                                                                                    })) {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(self::ml_string::Text::from_units(vec![97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108]))
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
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(source_ml_source1_temporary_result147) => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(source_ml_source1_temporary_result147.clone())
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Return(source_ml_source1_temporary_result148_2) => {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(source_ml_source1_temporary_result148_2.clone())
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Return(source_ml_source1_temporary_result148_3) => {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(source_ml_source1_temporary_result148_3.clone())
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Return(source_ml_source1_temporary_result148_4) => {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(source_ml_source1_temporary_result148_4.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(source_ml_source1_temporary_result148_5) => {
                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(source_ml_source1_temporary_result148_5.clone())
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
                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Done
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryThrowJavaScriptCallValueLoop149Result {
    MlSource1TemporaryThrowJavaScriptCallValueLoop149Done,
    MlSource1TemporaryThrowJavaScriptCallValueLoop149Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_throw_java_script_call_value_loop149(mut machine: self::JavaScriptCallMachine, mut value: self::JavaScriptSemanticValue, mut frames: Vec<self::JavaScriptCallFrame>) -> self::MlSource1TemporaryThrowJavaScriptCallValueLoop149Result {
    loop {
        return match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, _) => {
                let shared_constructor_field1 = *shared_constructor_field1;
                if (((frames.len() as f64)) > (0f64)) {
                    {
                        let frame = self::ml_array::optional_at(&frames, self::ml_array::number_index(((frames.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlSource1TemporaryThrowJavaScriptCallValueLoop149Result::MlSource1TemporaryThrowJavaScriptCallValueLoop149Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                        } else {
                            {
                                let handler = self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2581, _) => {
                                        source_ml_source2_data_property_value2581
                                    }
                                }, self::ml_array::number_index((((match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2582, _) => {
                                        source_ml_source2_data_property_value2582
                                    }
                                }).len() as f64) - 1f64)));
                                if !(match (handler.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    {
                                        let restored = {
                                            let source_ml_source2_nominal_spread2587 = self::write_java_script_call_register(frame.clone().expect("a proven present value"), match handler.clone().expect("a proven present value") {
                                                self::JavaScriptCallHandler::CallHandler(_, source_ml_source2_data_property_value2583) => {
                                                    source_ml_source2_data_property_value2583
                                                }
                                            }, value.clone());
                                            {
                                                let source_ml_source2_nominal_field2588 = match source_ml_source2_nominal_spread2587.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2589, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value2589.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field2590 = match source_ml_source2_nominal_spread2587.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2591, _, _, _, _) => {
                                                            source_ml_source2_data_property_value2591
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field2592 = match source_ml_source2_nominal_spread2587.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2593, _, _, _) => {
                                                                source_ml_source2_data_property_value2593
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field2594 = match source_ml_source2_nominal_spread2587.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2595, _, _) => {
                                                                    source_ml_source2_data_property_value2595
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field2596 = match source_ml_source2_nominal_spread2587.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2597, _) => {
                                                                        source_ml_source2_data_property_value2597
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field2598 = match source_ml_source2_nominal_spread2587.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2599) => {
                                                                            let source_ml_source2_data_property_value2599 = *source_ml_source2_data_property_value2599;
                                                                            source_ml_source2_data_property_value2599.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_override2600 = match handler.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallHandler::CallHandler(source_ml_source2_data_property_value2584, _) => {
                                                                                source_ml_source2_data_property_value2584
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override2601 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2586, _) => {
                                                                                    source_ml_source2_data_property_value2586
                                                                                }
                                                                            }, 0f64, (-1f64));
                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2588.clone(), source_ml_source2_nominal_override2600, source_ml_source2_nominal_field2592, source_ml_source2_nominal_field2594, source_ml_source2_nominal_override2601, Box::new(source_ml_source2_nominal_field2598.clone()))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        };
                                        self::MlSource1TemporaryThrowJavaScriptCallValueLoop149Result::MlSource1TemporaryThrowJavaScriptCallValueLoop149Return(Box::new(self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), frames, restored.clone())))))
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
                    self::MlSource1TemporaryThrowJavaScriptCallValueLoop149Result::MlSource1TemporaryThrowJavaScriptCallValueLoop149Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPushJavaScriptCallLoop151Result {
    MlSource1TemporaryPushJavaScriptCallLoop151Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_call_loop151(mut slots: f64, mut argument_slots: f64, mut source_ml_source1_temporary_values150: Vec<self::JavaScriptCallFrame>, mut source_ml_source1_temporary_index150: self::ml::Big) -> self::MlSource1TemporaryPushJavaScriptCallLoop151Result {
    loop {
        return if (source_ml_source1_temporary_index150 < (self::ml::Big::from_u128(source_ml_source1_temporary_values150.len() as u128))) {
            {
                let active = self::ml_array::at(&source_ml_source1_temporary_values150, source_ml_source1_temporary_index150.to_index());
                {
                    let slots_2 = (slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2603, _, _, _) => source_ml_source2_data_property_value2603.as_slice() }).len() as f64));
                    {
                        let argument_slots_2 = (argument_slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2604, _, _) => source_ml_source2_data_property_value2604.as_slice() }).len() as f64));
                        {
                            let source_ml_source1_temporary_index150_2 = source_ml_source1_temporary_index150.add(&self::ml::Big::from_i128(1));
                            {
                                (slots, argument_slots, source_ml_source1_temporary_index150) = (slots_2, argument_slots_2, source_ml_source1_temporary_index150_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPushJavaScriptCallLoop151Result::MlSource1TemporaryPushJavaScriptCallLoop151Done(slots, argument_slots)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryBeginJavaScriptCallMachineLoop153Result {
    MlSource1TemporaryBeginJavaScriptCallMachineLoop153Done,
    MlSource1TemporaryBeginJavaScriptCallMachineLoop153Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_begin_java_script_call_machine_loop153(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values152: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index152: self::ml::Big) -> self::MlSource1TemporaryBeginJavaScriptCallMachineLoop153Result {
    loop {
        return if (source_ml_source1_temporary_index152 < (self::ml::Big::from_u128(source_ml_source1_temporary_values152.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values152, source_ml_source1_temporary_index152.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryBeginJavaScriptCallMachineLoop153Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop153Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))))
                } else {
                    {
                        let source_ml_source1_temporary_index152_2 = source_ml_source1_temporary_index152.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index152 = source_ml_source1_temporary_index152_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryBeginJavaScriptCallMachineLoop153Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop153Done
        };
    }
}
