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
use super::JavaScriptTextCallState;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin149;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin150;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Result;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Result;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop146Result;
use super::find_java_script_call_function;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_number_call_source_operation;
use super::read_java_script_number_call_state_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_text_call_source_operation;
use super::read_java_script_text_call_state_error;
use super::replace_java_script_call_frame;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop144;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop145;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop146;
use super::test_java_script_call_coordinate;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_machine_error_loop143(mut program: self::JavaScriptCallProgram, mut machine: self::JavaScriptCallMachine, mut slots: f64, mut argument_slots: f64, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result {
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
                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                                } else {
                                    {
                                        let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2624, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value2624.clone()
                                            }
                                        })));
                                        if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                        } else {
                                            if (((((match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2627, _, _, _) => {
                                                    source_ml_source2_data_property_value2627
                                                }
                                            }).len() as f64)) != (match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2628, _, _) => {
                                                    source_ml_source2_data_property_value2628
                                                }
                                            })) || !(self::test_java_script_call_coordinate(match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2629, _, _, _, _) => {
                                                    source_ml_source2_data_property_value2629
                                                }
                                            }, (((match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2630) => {
                                                    source_ml_source2_data_property_value2630
                                                }
                                            }).len() as f64) + 1f64)))) {
                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                            } else {
                                                {
                                                    let slots_2 = (slots + ((match frame.clone().expect("a proven present value") {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2631, _, _, _) => {
                                                            source_ml_source2_data_property_value2631
                                                        }
                                                    }).len() as f64));
                                                    {
                                                        let argument_slots_2 = (argument_slots + ((match frame.clone().expect("a proven present value") {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2632, _, _) => {
                                                                source_ml_source2_data_property_value2632
                                                            }
                                                        }).len() as f64));
                                                        if (((slots_2 > (1048576f64)) || (argument_slots_2 > (1048576f64))) || ((((match frame.clone().expect("a proven present value") {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2634, _) => {
                                                                source_ml_source2_data_property_value2634
                                                            }
                                                        }).len() as f64)) > (1024f64))) {
                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                        } else {
                                                            {
                                                                let source_ml_source1_temporary_values140 = match frame.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2635, _, _, _) => {
                                                                        source_ml_source2_data_property_value2635
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_temporary_index140 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let source_ml_source2_s2636 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop144(machine.clone(), source_ml_source1_temporary_values140, source_ml_source1_temporary_index140.clone());
                                                                        match source_ml_source2_s2636.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Done => {
                                                                                {
                                                                                    let source_ml_source1_temporary_values141 = match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2637, _, _) => {
                                                                                            source_ml_source2_data_property_value2637
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_temporary_index141 = self::ml::Big::from_i128(0);
                                                                                        {
                                                                                            let source_ml_source2_s2638 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop145(machine.clone(), source_ml_source1_temporary_values141, source_ml_source1_temporary_index141.clone());
                                                                                            match source_ml_source2_s2638.clone() {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Done => {
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_values142 = match frame.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2639, _) => {
                                                                                                                source_ml_source2_data_property_value2639
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_index142 = self::ml::Big::from_i128(0);
                                                                                                            {
                                                                                                                let source_ml_source2_s2640 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop146(Some(fn_.clone().expect("a proven present value")), source_ml_source1_temporary_values142, source_ml_source1_temporary_index142.clone());
                                                                                                                match source_ml_source2_s2640.clone() {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop146Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop146Done => {
                                                                                                                        {
                                                                                                                            let pending = match frame.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2641) => {
                                                                                                                                    let source_ml_source2_data_property_value2641 = *source_ml_source2_data_property_value2641;
                                                                                                                                    source_ml_source2_data_property_value2641.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_s2660 = match pending.clone() {
                                                                                                                                    self::JavaScriptCallPending::CallIdle => {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitCall(destination) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2672, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2672
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitGet(destination_2) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_2, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2673, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2673
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitSet(destination_3) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_3, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2674, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2674
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitKey(destination_4, _, _) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_4, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2675, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2675
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitDefinition(destination_5, _, _) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_5, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2676, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2676
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitNumber(destination_6, _, _) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_6, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2677, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2677
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitText(destination_7, _, _) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_7, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2678, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2678
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitError(_, _, _) => {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                match source_ml_source2_s2660.clone() {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next => {
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_s2722 = match pending.clone() {
                                                                                                                                                self::JavaScriptCallPending::CallIdle => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Next
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitCall(_) => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Next
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitGet(_) => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Next
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitSet(_) => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Next
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitKey(_, realm, _) => {
                                                                                                                                                    if !(self::ml_array::includes(&shared_constructor_field2, &realm.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                    } else {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Next
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitDefinition(_, realm_2, _) => {
                                                                                                                                                    if !(self::ml_array::includes(&shared_constructor_field2, &realm_2.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                    } else {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Next
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitNumber(destination_8, realm_3, state) => {
                                                                                                                                                    let state = *state;
                                                                                                                                                    if !(self::ml_array::includes(&shared_constructor_field2, &realm_3.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                    } else {
                                                                                                                                                        {
                                                                                                                                                            let state_error = self::read_java_script_number_call_state_error(shared_constructor_field4.clone(), state.clone());
                                                                                                                                                            if (state_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Return(state_error.clone())
                                                                                                                                                            } else {
                                                                                                                                                                if ((self::read_java_script_number_call_source_operation(fn_.clone().expect("a proven present value"), frame.clone().expect("a proven present value"), destination_8, realm_3.clone(), shared_constructor_field3.clone(), state.clone())) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                                } else {
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_s2745 = if (index < (((shared_constructor_field5.len() as f64) - 1f64))) {
                                                                                                                                                                            match state.clone() {
                                                                                                                                                                                self::JavaScriptNumberCallState::NumberProtocolState(_) => {
                                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin149::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin149Next
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptNumberCallState::NumberPrimitiveValue(_) => {
                                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin149::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin149Return(self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 97, 110, 110, 111, 116, 32, 104, 97, 118, 101, 32, 97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108]))
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin149::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin149Next
                                                                                                                                                                        };
                                                                                                                                                                        match source_ml_source2_s2745.clone() {
                                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin149::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin149Next => {
                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Next
                                                                                                                                                                            }
                                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin149::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin149Return(source_ml_source1_temporary_result149) => {
                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Return(source_ml_source1_temporary_result149.clone())
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitText(destination_9, realm_4, state_2) => {
                                                                                                                                                    let state_2 = *state_2;
                                                                                                                                                    if !(self::ml_array::includes(&shared_constructor_field2, &realm_4.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                    } else {
                                                                                                                                                        {
                                                                                                                                                            let state_error_2 = self::read_java_script_text_call_state_error(shared_constructor_field4.clone(), state_2.clone());
                                                                                                                                                            if (state_error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Return(state_error_2.clone())
                                                                                                                                                            } else {
                                                                                                                                                                if ((self::read_java_script_text_call_source_operation(fn_.clone().expect("a proven present value"), frame.clone().expect("a proven present value"), destination_9, realm_4.clone(), shared_constructor_field3.clone(), state_2.clone())) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                                } else {
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_s2759 = if (index < (((shared_constructor_field5.len() as f64) - 1f64))) {
                                                                                                                                                                            match state_2.clone() {
                                                                                                                                                                                self::JavaScriptTextCallState::TextProtocolState(_) => {
                                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin150::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin150Next
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptTextCallState::TextPrimitiveValue(_) => {
                                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin150::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin150Return(self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 115, 116, 114, 105, 110, 103, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 97, 110, 110, 111, 116, 32, 104, 97, 118, 101, 32, 97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108]))
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin150::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin150Next
                                                                                                                                                                        };
                                                                                                                                                                        match source_ml_source2_s2759.clone() {
                                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin150::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin150Next => {
                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Next
                                                                                                                                                                            }
                                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin150::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin150Return(source_ml_source1_temporary_result150) => {
                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Return(source_ml_source1_temporary_result150.clone())
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                                self::JavaScriptCallPending::WaitError(realm_5, _, _) => {
                                                                                                                                                    if !(self::ml_array::includes(&shared_constructor_field2, &realm_5.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                                    } else {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Next
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            match source_ml_source2_s2722.clone() {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Next => {
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
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(self::ml_string::Text::from_units(vec![97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108]))
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
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Return(source_ml_source1_temporary_result148) => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(source_ml_source1_temporary_result148.clone())
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(source_ml_source1_temporary_result150_2) => {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(source_ml_source1_temporary_result150_2.clone())
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop146Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop146Return(source_ml_source1_temporary_result150_3) => {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(source_ml_source1_temporary_result150_3.clone())
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Return(source_ml_source1_temporary_result150_4) => {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(source_ml_source1_temporary_result150_4.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Return(source_ml_source1_temporary_result150_5) => {
                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(source_ml_source1_temporary_result150_5.clone())
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
                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Done
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryThrowJavaScriptCallValueLoop151Result {
    MlSource1TemporaryThrowJavaScriptCallValueLoop151Done,
    MlSource1TemporaryThrowJavaScriptCallValueLoop151Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_throw_java_script_call_value_loop151(mut machine: self::JavaScriptCallMachine, mut value: self::JavaScriptSemanticValue, mut frames: Vec<self::JavaScriptCallFrame>) -> self::MlSource1TemporaryThrowJavaScriptCallValueLoop151Result {
    loop {
        return match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, _) => {
                let shared_constructor_field1 = *shared_constructor_field1;
                if (((frames.len() as f64)) > (0f64)) {
                    {
                        let frame = self::ml_array::optional_at(&frames, self::ml_array::number_index(((frames.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlSource1TemporaryThrowJavaScriptCallValueLoop151Result::MlSource1TemporaryThrowJavaScriptCallValueLoop151Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                        } else {
                            {
                                let handler = self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2805, _) => {
                                        source_ml_source2_data_property_value2805
                                    }
                                }, self::ml_array::number_index((((match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2806, _) => {
                                        source_ml_source2_data_property_value2806
                                    }
                                }).len() as f64) - 1f64)));
                                if !(match (handler.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    {
                                        let restored = {
                                            let source_ml_source2_nominal_spread2811 = self::write_java_script_call_register(frame.clone().expect("a proven present value"), match handler.clone().expect("a proven present value") {
                                                self::JavaScriptCallHandler::CallHandler(_, source_ml_source2_data_property_value2807) => {
                                                    source_ml_source2_data_property_value2807
                                                }
                                            }, value.clone());
                                            {
                                                let source_ml_source2_nominal_field2812 = match source_ml_source2_nominal_spread2811.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2813, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value2813.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field2814 = match source_ml_source2_nominal_spread2811.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2815, _, _, _, _) => {
                                                            source_ml_source2_data_property_value2815
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field2816 = match source_ml_source2_nominal_spread2811.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2817, _, _, _) => {
                                                                source_ml_source2_data_property_value2817
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field2818 = match source_ml_source2_nominal_spread2811.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2819, _, _) => {
                                                                    source_ml_source2_data_property_value2819
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field2820 = match source_ml_source2_nominal_spread2811.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2821, _) => {
                                                                        source_ml_source2_data_property_value2821
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field2822 = match source_ml_source2_nominal_spread2811.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2823) => {
                                                                            let source_ml_source2_data_property_value2823 = *source_ml_source2_data_property_value2823;
                                                                            source_ml_source2_data_property_value2823.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_override2824 = match handler.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallHandler::CallHandler(source_ml_source2_data_property_value2808, _) => {
                                                                                source_ml_source2_data_property_value2808
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override2825 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2810, _) => {
                                                                                    source_ml_source2_data_property_value2810
                                                                                }
                                                                            }, 0f64, (-1f64));
                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2812.clone(), source_ml_source2_nominal_override2824, source_ml_source2_nominal_field2816, source_ml_source2_nominal_field2818, source_ml_source2_nominal_override2825, Box::new(source_ml_source2_nominal_field2822.clone()))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        };
                                        self::MlSource1TemporaryThrowJavaScriptCallValueLoop151Result::MlSource1TemporaryThrowJavaScriptCallValueLoop151Return(Box::new(self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), frames, restored.clone())))))
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
                    self::MlSource1TemporaryThrowJavaScriptCallValueLoop151Result::MlSource1TemporaryThrowJavaScriptCallValueLoop151Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPushJavaScriptCallLoop153Result {
    MlSource1TemporaryPushJavaScriptCallLoop153Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_call_loop153(mut slots: f64, mut argument_slots: f64, mut source_ml_source1_temporary_values152: Vec<self::JavaScriptCallFrame>, mut source_ml_source1_temporary_index152: self::ml::Big) -> self::MlSource1TemporaryPushJavaScriptCallLoop153Result {
    loop {
        return if (source_ml_source1_temporary_index152 < (self::ml::Big::from_u128(source_ml_source1_temporary_values152.len() as u128))) {
            {
                let active = self::ml_array::at(&source_ml_source1_temporary_values152, source_ml_source1_temporary_index152.to_index());
                {
                    let slots_2 = (slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2827, _, _, _) => source_ml_source2_data_property_value2827.as_slice() }).len() as f64));
                    {
                        let argument_slots_2 = (argument_slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2828, _, _) => source_ml_source2_data_property_value2828.as_slice() }).len() as f64));
                        {
                            let source_ml_source1_temporary_index152_2 = source_ml_source1_temporary_index152.add(&self::ml::Big::from_i128(1));
                            {
                                (slots, argument_slots, source_ml_source1_temporary_index152) = (slots_2, argument_slots_2, source_ml_source1_temporary_index152_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPushJavaScriptCallLoop153Result::MlSource1TemporaryPushJavaScriptCallLoop153Done(slots, argument_slots)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryBeginJavaScriptCallMachineLoop155Result {
    MlSource1TemporaryBeginJavaScriptCallMachineLoop155Done,
    MlSource1TemporaryBeginJavaScriptCallMachineLoop155Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_begin_java_script_call_machine_loop155(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values154: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index154: self::ml::Big) -> self::MlSource1TemporaryBeginJavaScriptCallMachineLoop155Result {
    loop {
        return if (source_ml_source1_temporary_index154 < (self::ml::Big::from_u128(source_ml_source1_temporary_values154.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values154, source_ml_source1_temporary_index154.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryBeginJavaScriptCallMachineLoop155Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop155Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))))
                } else {
                    {
                        let source_ml_source1_temporary_index154_2 = source_ml_source1_temporary_index154.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index154 = source_ml_source1_temporary_index154_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryBeginJavaScriptCallMachineLoop155Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop155Done
        };
    }
}
