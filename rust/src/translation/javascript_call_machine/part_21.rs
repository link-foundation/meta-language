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
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Result;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result;
use super::find_java_script_call_function;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_number_call_source_operation;
use super::read_java_script_number_call_state_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_call_frame;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop141;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop142;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop143;
use super::test_java_script_call_coordinate;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_machine_error_loop140(mut program: self::JavaScriptCallProgram, mut machine: self::JavaScriptCallMachine, mut slots: f64, mut argument_slots: f64, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result {
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
                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                                } else {
                                    {
                                        let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2418, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value2418.clone()
                                            }
                                        })));
                                        if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                        } else {
                                            if (((((match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2421, _, _, _) => {
                                                    source_ml_source2_data_property_value2421
                                                }
                                            }).len() as f64)) != (match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2422, _, _) => {
                                                    source_ml_source2_data_property_value2422
                                                }
                                            })) || !(self::test_java_script_call_coordinate(match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2423, _, _, _, _) => {
                                                    source_ml_source2_data_property_value2423
                                                }
                                            }, (((match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2424) => {
                                                    source_ml_source2_data_property_value2424
                                                }
                                            }).len() as f64) + 1f64)))) {
                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                            } else {
                                                {
                                                    let slots_2 = (slots + ((match frame.clone().expect("a proven present value") {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2425, _, _, _) => {
                                                            source_ml_source2_data_property_value2425
                                                        }
                                                    }).len() as f64));
                                                    {
                                                        let argument_slots_2 = (argument_slots + ((match frame.clone().expect("a proven present value") {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2426, _, _) => {
                                                                source_ml_source2_data_property_value2426
                                                            }
                                                        }).len() as f64));
                                                        if (((slots_2 > (1048576f64)) || (argument_slots_2 > (1048576f64))) || ((((match frame.clone().expect("a proven present value") {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2428, _) => {
                                                                source_ml_source2_data_property_value2428
                                                            }
                                                        }).len() as f64)) > (1024f64))) {
                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                        } else {
                                                            {
                                                                let source_ml_source1_temporary_values137 = match frame.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2429, _, _, _) => {
                                                                        source_ml_source2_data_property_value2429
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_temporary_index137 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let source_ml_source2_s2430 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop141(machine.clone(), source_ml_source1_temporary_values137, source_ml_source1_temporary_index137.clone());
                                                                        match source_ml_source2_s2430.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Done => {
                                                                                {
                                                                                    let source_ml_source1_temporary_values138 = match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2431, _, _) => {
                                                                                            source_ml_source2_data_property_value2431
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_temporary_index138 = self::ml::Big::from_i128(0);
                                                                                        {
                                                                                            let source_ml_source2_s2432 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop142(machine.clone(), source_ml_source1_temporary_values138, source_ml_source1_temporary_index138.clone());
                                                                                            match source_ml_source2_s2432.clone() {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Done => {
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_values139 = match frame.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2433, _) => {
                                                                                                                source_ml_source2_data_property_value2433
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_index139 = self::ml::Big::from_i128(0);
                                                                                                            {
                                                                                                                let source_ml_source2_s2434 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop143(Some(fn_.clone().expect("a proven present value")), source_ml_source1_temporary_values139, source_ml_source1_temporary_index139.clone());
                                                                                                                match source_ml_source2_s2434.clone() {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Done => {
                                                                                                                        {
                                                                                                                            let pending = match frame.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2435) => {
                                                                                                                                    let source_ml_source2_data_property_value2435 = *source_ml_source2_data_property_value2435;
                                                                                                                                    source_ml_source2_data_property_value2435.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_s2451 = match pending.clone() {
                                                                                                                                    self::JavaScriptCallPending::CallIdle => {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitCall(destination) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2461, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2461
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitGet(destination_2) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_2, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2462, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2462
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitSet(destination_3) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_3, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2463, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2463
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitKey(destination_4, _, _) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_4, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2464, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2464
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitDefinition(destination_5, _, _) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_5, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2465, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2465
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitNumber(destination_6, _, _) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_6, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2466, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2466
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitError(_, _, _) => {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                match source_ml_source2_s2451.clone() {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next => {
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_s2496 = match pending.clone() {
                                                                                                                                                self::JavaScriptCallPending::CallIdle => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Next
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitCall(_) => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Next
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitGet(_) => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Next
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitSet(_) => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Next
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitKey(_, realm, _) => {
                                                                                                                                                    if !(self::ml_array::includes(&shared_constructor_field2, &realm.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                    } else {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Next
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitDefinition(_, realm_2, _) => {
                                                                                                                                                    if !(self::ml_array::includes(&shared_constructor_field2, &realm_2.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                    } else {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Next
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitNumber(destination_7, realm_3, state) => {
                                                                                                                                                    let state = *state;
                                                                                                                                                    if !(self::ml_array::includes(&shared_constructor_field2, &realm_3.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                    } else {
                                                                                                                                                        {
                                                                                                                                                            let number_state = state.clone();
                                                                                                                                                            {
                                                                                                                                                                let state_error = self::read_java_script_number_call_state_error(shared_constructor_field4.clone(), number_state.clone());
                                                                                                                                                                if (state_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Return(state_error.clone())
                                                                                                                                                                } else {
                                                                                                                                                                    if ((self::read_java_script_number_call_source_operation(fn_.clone().expect("a proven present value"), frame.clone().expect("a proven present value"), destination_7, realm_3.clone(), shared_constructor_field3.clone(), number_state.clone())) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                                    } else {
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_s2519 = if (index < (((shared_constructor_field5.len() as f64) - 1f64))) {
                                                                                                                                                                                match number_state.clone() {
                                                                                                                                                                                    self::JavaScriptNumberCallState::NumberProtocolState(_) => {
                                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Next
                                                                                                                                                                                    }
                                                                                                                                                                                    self::JavaScriptNumberCallState::NumberPrimitiveValue(_) => {
                                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Return(self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 97, 110, 110, 111, 116, 32, 104, 97, 118, 101, 32, 97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108]))
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            } else {
                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Next
                                                                                                                                                                            };
                                                                                                                                                                            match source_ml_source2_s2519.clone() {
                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Next => {
                                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Next
                                                                                                                                                                                }
                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin146Return(source_ml_source1_temporary_result146) => {
                                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Return(source_ml_source1_temporary_result146.clone())
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
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                    } else {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Next
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            match source_ml_source2_s2496.clone() {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Next => {
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
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Return(self::ml_string::Text::from_units(vec![97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108]))
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
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin145Return(source_ml_source1_temporary_result145) => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Return(source_ml_source1_temporary_result145.clone())
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Return(source_ml_source1_temporary_result146_2) => {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Return(source_ml_source1_temporary_result146_2.clone())
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(source_ml_source1_temporary_result146_3) => {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Return(source_ml_source1_temporary_result146_3.clone())
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(source_ml_source1_temporary_result146_4) => {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Return(source_ml_source1_temporary_result146_4.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Return(source_ml_source1_temporary_result146_5) => {
                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Return(source_ml_source1_temporary_result146_5.clone())
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
                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Done
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryThrowJavaScriptCallValueLoop147Result {
    MlSource1TemporaryThrowJavaScriptCallValueLoop147Done,
    MlSource1TemporaryThrowJavaScriptCallValueLoop147Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_throw_java_script_call_value_loop147(mut machine: self::JavaScriptCallMachine, mut value: self::JavaScriptSemanticValue, mut frames: Vec<self::JavaScriptCallFrame>) -> self::MlSource1TemporaryThrowJavaScriptCallValueLoop147Result {
    loop {
        return match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, _) => {
                let shared_constructor_field1 = *shared_constructor_field1;
                if (((frames.len() as f64)) > (0f64)) {
                    {
                        let frame = self::ml_array::optional_at(&frames, self::ml_array::number_index(((frames.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlSource1TemporaryThrowJavaScriptCallValueLoop147Result::MlSource1TemporaryThrowJavaScriptCallValueLoop147Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                        } else {
                            {
                                let handler = self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2559, _) => {
                                        source_ml_source2_data_property_value2559
                                    }
                                }, self::ml_array::number_index((((match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2560, _) => {
                                        source_ml_source2_data_property_value2560
                                    }
                                }).len() as f64) - 1f64)));
                                if !(match (handler.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    {
                                        let restored = {
                                            let source_ml_source2_nominal_spread2565 = self::write_java_script_call_register(frame.clone().expect("a proven present value"), match handler.clone().expect("a proven present value") {
                                                self::JavaScriptCallHandler::CallHandler(_, source_ml_source2_data_property_value2561) => {
                                                    source_ml_source2_data_property_value2561
                                                }
                                            }, value.clone());
                                            {
                                                let source_ml_source2_nominal_field2566 = match source_ml_source2_nominal_spread2565.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2567, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value2567.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field2568 = match source_ml_source2_nominal_spread2565.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2569, _, _, _, _) => {
                                                            source_ml_source2_data_property_value2569
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field2570 = match source_ml_source2_nominal_spread2565.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2571, _, _, _) => {
                                                                source_ml_source2_data_property_value2571
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field2572 = match source_ml_source2_nominal_spread2565.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2573, _, _) => {
                                                                    source_ml_source2_data_property_value2573
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field2574 = match source_ml_source2_nominal_spread2565.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2575, _) => {
                                                                        source_ml_source2_data_property_value2575
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field2576 = match source_ml_source2_nominal_spread2565.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2577) => {
                                                                            let source_ml_source2_data_property_value2577 = *source_ml_source2_data_property_value2577;
                                                                            source_ml_source2_data_property_value2577.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_override2578 = match handler.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallHandler::CallHandler(source_ml_source2_data_property_value2562, _) => {
                                                                                source_ml_source2_data_property_value2562
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override2579 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2564, _) => {
                                                                                    source_ml_source2_data_property_value2564
                                                                                }
                                                                            }, 0f64, (-1f64));
                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2566.clone(), source_ml_source2_nominal_override2578, source_ml_source2_nominal_field2570, source_ml_source2_nominal_field2572, source_ml_source2_nominal_override2579, Box::new(source_ml_source2_nominal_field2576.clone()))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        };
                                        self::MlSource1TemporaryThrowJavaScriptCallValueLoop147Result::MlSource1TemporaryThrowJavaScriptCallValueLoop147Return(Box::new(self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), frames, restored.clone())))))
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
                    self::MlSource1TemporaryThrowJavaScriptCallValueLoop147Result::MlSource1TemporaryThrowJavaScriptCallValueLoop147Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPushJavaScriptCallLoop149Result {
    MlSource1TemporaryPushJavaScriptCallLoop149Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_call_loop149(mut slots: f64, mut argument_slots: f64, mut source_ml_source1_temporary_values148: Vec<self::JavaScriptCallFrame>, mut source_ml_source1_temporary_index148: self::ml::Big) -> self::MlSource1TemporaryPushJavaScriptCallLoop149Result {
    loop {
        return if (source_ml_source1_temporary_index148 < (self::ml::Big::from_u128(source_ml_source1_temporary_values148.len() as u128))) {
            {
                let active = self::ml_array::at(&source_ml_source1_temporary_values148, source_ml_source1_temporary_index148.to_index());
                {
                    let slots_2 = (slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2581, _, _, _) => source_ml_source2_data_property_value2581.as_slice() }).len() as f64));
                    {
                        let argument_slots_2 = (argument_slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2582, _, _) => source_ml_source2_data_property_value2582.as_slice() }).len() as f64));
                        {
                            let source_ml_source1_temporary_index148_2 = source_ml_source1_temporary_index148.add(&self::ml::Big::from_i128(1));
                            {
                                (slots, argument_slots, source_ml_source1_temporary_index148) = (slots_2, argument_slots_2, source_ml_source1_temporary_index148_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPushJavaScriptCallLoop149Result::MlSource1TemporaryPushJavaScriptCallLoop149Done(slots, argument_slots)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryBeginJavaScriptCallMachineLoop151Result {
    MlSource1TemporaryBeginJavaScriptCallMachineLoop151Done,
    MlSource1TemporaryBeginJavaScriptCallMachineLoop151Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_begin_java_script_call_machine_loop151(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values150: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index150: self::ml::Big) -> self::MlSource1TemporaryBeginJavaScriptCallMachineLoop151Result {
    loop {
        return if (source_ml_source1_temporary_index150 < (self::ml::Big::from_u128(source_ml_source1_temporary_values150.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values150, source_ml_source1_temporary_index150.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryBeginJavaScriptCallMachineLoop151Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop151Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))))
                } else {
                    {
                        let source_ml_source1_temporary_index150_2 = source_ml_source1_temporary_index150.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index150 = source_ml_source1_temporary_index150_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryBeginJavaScriptCallMachineLoop151Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop151Done
        };
    }
}
