// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_call_frame;
use super::test_java_script_call_coordinate;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_frame_error_loop148(mut fn_: self::JavaScriptCallFunction, mut source_ml_source1_temporary_values145: Vec<self::JavaScriptCallHandler>, mut source_ml_source1_temporary_index145: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Result {
    loop {
        return match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, shared_constructor_field3, _, shared_constructor_field4) => {
                if (source_ml_source1_temporary_index145 < (self::ml::Big::from_u128(source_ml_source1_temporary_values145.len() as u128))) {
                    {
                        let handler = self::ml_array::at(&source_ml_source1_temporary_values145, source_ml_source1_temporary_index145.to_index());
                        match handler.clone() {
                            self::JavaScriptCallHandler::CallHandler(shared_constructor_field1, shared_constructor_field2) => {
                                if (!(self::test_java_script_call_coordinate(shared_constructor_field2, shared_constructor_field3)) || !(self::test_java_script_call_coordinate(shared_constructor_field1, (shared_constructor_field4.len() as f64)))) {
                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                } else {
                                    {
                                        let source_ml_source1_temporary_index145_2 = source_ml_source1_temporary_index145.add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_source1_temporary_index145 = source_ml_source1_temporary_index145_2.clone();
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameErrorLoop147Result {
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop147Done,
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop147Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_frame_error_loop147(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values144: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index144: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop147Result {
    loop {
        return if (source_ml_source1_temporary_index144 < (self::ml::Big::from_u128(source_ml_source1_temporary_values144.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values144, source_ml_source1_temporary_index144.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop147Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop147Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_source1_temporary_index144_2 = source_ml_source1_temporary_index144.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index144 = source_ml_source1_temporary_index144_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop147Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop147Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameErrorLoop146Result {
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop146Done,
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop146Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_frame_error_loop146(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values143: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index143: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop146Result {
    loop {
        return if (source_ml_source1_temporary_index143 < (self::ml::Big::from_u128(source_ml_source1_temporary_values143.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values143, source_ml_source1_temporary_index143.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop146Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop146Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_source1_temporary_index143_2 = source_ml_source1_temporary_index143.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index143 = source_ml_source1_temporary_index143_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop146Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop146Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryThrowJavaScriptCallValueLoop152Result {
    MlSource1TemporaryThrowJavaScriptCallValueLoop152Done,
    MlSource1TemporaryThrowJavaScriptCallValueLoop152Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_throw_java_script_call_value_loop152(mut machine: self::JavaScriptCallMachine, mut value: self::JavaScriptSemanticValue, mut frames: Vec<self::JavaScriptCallFrame>) -> self::MlSource1TemporaryThrowJavaScriptCallValueLoop152Result {
    loop {
        return match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(shared_constructor_field7, _) => {
                let shared_constructor_field7 = *shared_constructor_field7;
                if (((frames.len() as f64)) > (0f64)) {
                    {
                        let frame = self::ml_array::optional_at(&frames, self::ml_array::number_index(((frames.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlSource1TemporaryThrowJavaScriptCallValueLoop152Result::MlSource1TemporaryThrowJavaScriptCallValueLoop152Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                        } else {
                            {
                                let handler = self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2904, _) => {
                                        source_ml_source2_data_property_value2904
                                    }
                                }, self::ml_array::number_index((((match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2905, _) => {
                                        source_ml_source2_data_property_value2905
                                    }
                                }).len() as f64) - 1f64)));
                                if !(match (handler.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    {
                                        let restored = {
                                            let source_ml_source2_nominal_spread2910 = self::write_java_script_call_register(frame.clone().expect("a proven present value"), match handler.clone().expect("a proven present value") {
                                                self::JavaScriptCallHandler::CallHandler(_, source_ml_source2_data_property_value2906) => {
                                                    source_ml_source2_data_property_value2906
                                                }
                                            }, value.clone());
                                            match source_ml_source2_nominal_spread2910.clone() {
                                                self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                                    let shared_constructor_field6 = *shared_constructor_field6;
                                                    {
                                                        let source_ml_source2_nominal_field2911 = shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source2_nominal_field2913 = shared_constructor_field2;
                                                            {
                                                                let source_ml_source2_nominal_field2915 = shared_constructor_field3;
                                                                {
                                                                    let source_ml_source2_nominal_field2917 = shared_constructor_field4;
                                                                    {
                                                                        let source_ml_source2_nominal_field2919 = shared_constructor_field5;
                                                                        {
                                                                            let source_ml_source2_nominal_field2921 = shared_constructor_field6.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_override2923 = match handler.clone().expect("a proven present value") {
                                                                                    self::JavaScriptCallHandler::CallHandler(source_ml_source2_data_property_value2907, _) => {
                                                                                        source_ml_source2_data_property_value2907
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override2924 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2909, _) => {
                                                                                            source_ml_source2_data_property_value2909
                                                                                        }
                                                                                    }, 0f64, (-1f64));
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2911.clone(), source_ml_source2_nominal_override2923, source_ml_source2_nominal_field2915, source_ml_source2_nominal_field2917, source_ml_source2_nominal_override2924, Box::new(source_ml_source2_nominal_field2921.clone()))
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
                                        };
                                        self::MlSource1TemporaryThrowJavaScriptCallValueLoop152Result::MlSource1TemporaryThrowJavaScriptCallValueLoop152Return(Box::new(self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field7.clone(), frames, restored.clone())))))
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
                    self::MlSource1TemporaryThrowJavaScriptCallValueLoop152Result::MlSource1TemporaryThrowJavaScriptCallValueLoop152Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPushJavaScriptCallLoop154Result {
    MlSource1TemporaryPushJavaScriptCallLoop154Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_call_loop154(mut slots: f64, mut argument_slots: f64, mut source_ml_source1_temporary_values153: Vec<self::JavaScriptCallFrame>, mut source_ml_source1_temporary_index153: self::ml::Big) -> self::MlSource1TemporaryPushJavaScriptCallLoop154Result {
    loop {
        return if (source_ml_source1_temporary_index153 < (self::ml::Big::from_u128(source_ml_source1_temporary_values153.len() as u128))) {
            {
                let active = self::ml_array::at(&source_ml_source1_temporary_values153, source_ml_source1_temporary_index153.to_index());
                match active.clone() {
                    self::JavaScriptCallFrame::CallFrame(_, _, shared_constructor_field1, shared_constructor_field2, _, _) => {
                        {
                            let slots_2 = (slots + (shared_constructor_field1.len() as f64));
                            {
                                let argument_slots_2 = (argument_slots + (shared_constructor_field2.len() as f64));
                                {
                                    let source_ml_source1_temporary_index153_2 = source_ml_source1_temporary_index153.add(&self::ml::Big::from_i128(1));
                                    {
                                        (slots, argument_slots, source_ml_source1_temporary_index153) = (slots_2, argument_slots_2, source_ml_source1_temporary_index153_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPushJavaScriptCallLoop154Result::MlSource1TemporaryPushJavaScriptCallLoop154Done(slots, argument_slots)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryBeginJavaScriptCallMachineLoop156Result {
    MlSource1TemporaryBeginJavaScriptCallMachineLoop156Done,
    MlSource1TemporaryBeginJavaScriptCallMachineLoop156Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_begin_java_script_call_machine_loop156(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values155: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index155: self::ml::Big) -> self::MlSource1TemporaryBeginJavaScriptCallMachineLoop156Result {
    loop {
        return if (source_ml_source1_temporary_index155 < (self::ml::Big::from_u128(source_ml_source1_temporary_values155.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values155, source_ml_source1_temporary_index155.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryBeginJavaScriptCallMachineLoop156Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop156Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))))
                } else {
                    {
                        let source_ml_source1_temporary_index155_2 = source_ml_source1_temporary_index155.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index155 = source_ml_source1_temporary_index155_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryBeginJavaScriptCallMachineLoop156Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop156Done
        };
    }
}
