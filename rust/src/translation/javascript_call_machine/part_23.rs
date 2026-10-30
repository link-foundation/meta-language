// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_call_frame;
use super::test_java_script_call_coordinate;
use super::write_java_script_call_register;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameErrorLoop150Result {
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop150Done,
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop150Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_frame_error_loop150(mut fn_: self::JavaScriptCallFunction, mut source_ml_source1_temporary_values147: Vec<self::JavaScriptCallHandler>, mut source_ml_source1_temporary_index147: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop150Result {
    loop {
        return match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, shared_constructor_field3, _, shared_constructor_field4) => {
                if (source_ml_source1_temporary_index147 < (self::ml::Big::from_u128(source_ml_source1_temporary_values147.len() as u128))) {
                    {
                        let handler = self::ml_array::at(&source_ml_source1_temporary_values147, source_ml_source1_temporary_index147.to_index());
                        match handler.clone() {
                            self::JavaScriptCallHandler::CallHandler(shared_constructor_field1, shared_constructor_field2) => {
                                if (!(self::test_java_script_call_coordinate(shared_constructor_field2, shared_constructor_field3)) || !(self::test_java_script_call_coordinate(shared_constructor_field1, (shared_constructor_field4.len() as f64)))) {
                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop150Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop150Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                } else {
                                    {
                                        let source_ml_source1_temporary_index147_2 = source_ml_source1_temporary_index147.add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_source1_temporary_index147 = source_ml_source1_temporary_index147_2.clone();
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop150Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop150Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameErrorLoop149Result {
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop149Done,
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop149Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_frame_error_loop149(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values146: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index146: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop149Result {
    loop {
        return if (source_ml_source1_temporary_index146 < (self::ml::Big::from_u128(source_ml_source1_temporary_values146.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values146, source_ml_source1_temporary_index146.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop149Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop149Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_source1_temporary_index146_2 = source_ml_source1_temporary_index146.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index146 = source_ml_source1_temporary_index146_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop149Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop149Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Result {
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Done,
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_frame_error_loop148(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values145: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index145: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Result {
    loop {
        return if (source_ml_source1_temporary_index145 < (self::ml::Big::from_u128(source_ml_source1_temporary_values145.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values145, source_ml_source1_temporary_index145.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
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
        } else {
            self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryThrowJavaScriptCallValueLoop154Result {
    MlSource1TemporaryThrowJavaScriptCallValueLoop154Done,
    MlSource1TemporaryThrowJavaScriptCallValueLoop154Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_throw_java_script_call_value_loop154(mut machine: self::JavaScriptCallMachine, mut value: self::JavaScriptSemanticValue, mut frames: Vec<self::JavaScriptCallFrame>) -> self::MlSource1TemporaryThrowJavaScriptCallValueLoop154Result {
    loop {
        return match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(shared_constructor_field7, _) => {
                let shared_constructor_field7 = *shared_constructor_field7;
                if (((frames.len() as f64)) > (0f64)) {
                    {
                        let frame = self::ml_array::optional_at(&frames, self::ml_array::number_index(((frames.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlSource1TemporaryThrowJavaScriptCallValueLoop154Result::MlSource1TemporaryThrowJavaScriptCallValueLoop154Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                        } else {
                            {
                                let handler = self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2923, _) => {
                                        source_ml_source2_data_property_value2923
                                    }
                                }, self::ml_array::number_index((((match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2924, _) => {
                                        source_ml_source2_data_property_value2924
                                    }
                                }).len() as f64) - 1f64)));
                                if !(match (handler.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    {
                                        let restored = {
                                            let source_ml_source2_nominal_spread2929 = self::write_java_script_call_register(frame.clone().expect("a proven present value"), match handler.clone().expect("a proven present value") {
                                                self::JavaScriptCallHandler::CallHandler(_, source_ml_source2_data_property_value2925) => {
                                                    source_ml_source2_data_property_value2925
                                                }
                                            }, value.clone());
                                            match source_ml_source2_nominal_spread2929.clone() {
                                                self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                                    let shared_constructor_field6 = *shared_constructor_field6;
                                                    {
                                                        let source_ml_source2_nominal_field2930 = shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source2_nominal_field2932 = shared_constructor_field2;
                                                            {
                                                                let source_ml_source2_nominal_field2934 = shared_constructor_field3;
                                                                {
                                                                    let source_ml_source2_nominal_field2936 = shared_constructor_field4;
                                                                    {
                                                                        let source_ml_source2_nominal_field2938 = shared_constructor_field5;
                                                                        {
                                                                            let source_ml_source2_nominal_field2940 = shared_constructor_field6.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_override2942 = match handler.clone().expect("a proven present value") {
                                                                                    self::JavaScriptCallHandler::CallHandler(source_ml_source2_data_property_value2926, _) => {
                                                                                        source_ml_source2_data_property_value2926
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override2943 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2928, _) => {
                                                                                            source_ml_source2_data_property_value2928
                                                                                        }
                                                                                    }, 0f64, (-1f64));
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2930.clone(), source_ml_source2_nominal_override2942, source_ml_source2_nominal_field2934, source_ml_source2_nominal_field2936, source_ml_source2_nominal_override2943, Box::new(source_ml_source2_nominal_field2940.clone()))
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
                                        self::MlSource1TemporaryThrowJavaScriptCallValueLoop154Result::MlSource1TemporaryThrowJavaScriptCallValueLoop154Return(Box::new(self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field7.clone(), frames, restored.clone())))))
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
                    self::MlSource1TemporaryThrowJavaScriptCallValueLoop154Result::MlSource1TemporaryThrowJavaScriptCallValueLoop154Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPushJavaScriptCallLoop156Result {
    MlSource1TemporaryPushJavaScriptCallLoop156Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_call_loop156(mut slots: f64, mut argument_slots: f64, mut source_ml_source1_temporary_values155: Vec<self::JavaScriptCallFrame>, mut source_ml_source1_temporary_index155: self::ml::Big) -> self::MlSource1TemporaryPushJavaScriptCallLoop156Result {
    loop {
        return if (source_ml_source1_temporary_index155 < (self::ml::Big::from_u128(source_ml_source1_temporary_values155.len() as u128))) {
            {
                let active = self::ml_array::at(&source_ml_source1_temporary_values155, source_ml_source1_temporary_index155.to_index());
                match active.clone() {
                    self::JavaScriptCallFrame::CallFrame(_, _, shared_constructor_field1, shared_constructor_field2, _, _) => {
                        {
                            let slots_2 = (slots + (shared_constructor_field1.len() as f64));
                            {
                                let argument_slots_2 = (argument_slots + (shared_constructor_field2.len() as f64));
                                {
                                    let source_ml_source1_temporary_index155_2 = source_ml_source1_temporary_index155.add(&self::ml::Big::from_i128(1));
                                    {
                                        (slots, argument_slots, source_ml_source1_temporary_index155) = (slots_2, argument_slots_2, source_ml_source1_temporary_index155_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPushJavaScriptCallLoop156Result::MlSource1TemporaryPushJavaScriptCallLoop156Done(slots, argument_slots)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryBeginJavaScriptCallMachineLoop158Result {
    MlSource1TemporaryBeginJavaScriptCallMachineLoop158Done,
    MlSource1TemporaryBeginJavaScriptCallMachineLoop158Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_begin_java_script_call_machine_loop158(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values157: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index157: self::ml::Big) -> self::MlSource1TemporaryBeginJavaScriptCallMachineLoop158Result {
    loop {
        return if (source_ml_source1_temporary_index157 < (self::ml::Big::from_u128(source_ml_source1_temporary_values157.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values157, source_ml_source1_temporary_index157.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryBeginJavaScriptCallMachineLoop158Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop158Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))))
                } else {
                    {
                        let source_ml_source1_temporary_index157_2 = source_ml_source1_temporary_index157.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index157 = source_ml_source1_temporary_index157_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryBeginJavaScriptCallMachineLoop158Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop158Done
        };
    }
}
