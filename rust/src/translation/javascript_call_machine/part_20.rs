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
use super::find_java_script_call_function;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_number_call_source_operation;
use super::read_java_script_number_call_state_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_text_call_source_operation;
use super::read_java_script_text_call_state_error;
use super::replace_java_script_call_frame;
use super::test_java_script_call_coordinate;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_create_java_script_call_frame_loop135(mut fn_: self::JavaScriptCallFunction, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (index < (match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2619, _, _) => {
                source_ml_source2_data_property_value2619
            }
        })) {
            {
                let registers_2 = { let (ml_append_right, ml_append_left) = (vec![self::JavaScriptSemanticValue::Undefined], registers); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (registers, index) = (registers_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            registers
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_create_java_script_call_frame_loop136(mut fn_: self::JavaScriptCallFunction, mut arguments_: Vec<self::JavaScriptSemanticValue>, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (index < (match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source2_data_property_value2621, _, _, _) => {
                source_ml_source2_data_property_value2621
            }
        })) {
            {
                let value = (self::ml_array::optional_at(&arguments_, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                {
                    let slot = (index + 1f64);
                    {
                        let registers_2 = { let ml_values8 = registers; let ml_callback8 = { let slot = slot; let value = value.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, position: f64| -> self::JavaScriptSemanticValue { if (position == slot) {
                            value.clone()
                        } else {
                            entry.clone()
                        } }) }; ml_values8.iter().cloned().enumerate().map(|(ml_index8, ml_value8)| ml_callback8(ml_value8, (ml_index8 as f64))).collect::<Vec<_>>() };
                        {
                            let index_2 = (index + 1f64);
                            {
                                (registers, index) = (registers_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            registers
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_create_java_script_call_frame_loop137(mut fn_: self::JavaScriptCallFunction, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, shared_constructor_field1, _, shared_constructor_field2, _) => {
                if (index < ((shared_constructor_field2.len() as f64))) {
                    {
                        let value = (self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        {
                            let slot = ((shared_constructor_field1 + 1f64) + index);
                            {
                                let registers_2 = { let ml_values9 = registers; let ml_callback9 = { let slot = slot; let value = value.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, position: f64| -> self::JavaScriptSemanticValue { if (position == slot) {
                                    value.clone()
                                } else {
                                    entry.clone()
                                } }) }; ml_values9.iter().cloned().enumerate().map(|(ml_index9, ml_value9)| ml_callback9(ml_value9, (ml_index9 as f64))).collect::<Vec<_>>() };
                                {
                                    let index_2 = (index + 1f64);
                                    {
                                        (registers, index) = (registers_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    registers
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin138 {
    MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin138Next,
    MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin138Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin139 {
    MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin139Next,
    MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin139Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result {
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Done,
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148 {
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Next,
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin148Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorJoin149 {
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin149Next,
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin149Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorJoin150 {
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin150Next,
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin150Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147 {
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next,
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorLoop146Result {
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop146Done,
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop146Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_machine_error_loop146(mut fn_: Option<self::JavaScriptCallFunction>, mut source_ml_source1_temporary_values142: Vec<self::JavaScriptCallHandler>, mut source_ml_source1_temporary_index142: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop146Result {
    loop {
        return if (source_ml_source1_temporary_index142 < (self::ml::Big::from_u128(source_ml_source1_temporary_values142.len() as u128))) {
            {
                let handler = self::ml_array::at(&source_ml_source1_temporary_values142, source_ml_source1_temporary_index142.to_index());
                match handler.clone() {
                    self::JavaScriptCallHandler::CallHandler(shared_constructor_field1, shared_constructor_field2) => {
                        if (!(self::test_java_script_call_coordinate(shared_constructor_field2, {
                            let source_ml_source2_nullish_field_receiver2627 = (fn_.clone(),);
                            if match (source_ml_source2_nullish_field_receiver2627.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                {
                                    let source_ml_source2_nullish_field_present2628 = ((source_ml_source2_nullish_field_receiver2627.clone()).0).clone().expect("a proven present value");
                                    match source_ml_source2_nullish_field_present2628.clone() {
                                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2629, _, _) => {
                                            source_ml_source2_data_property_value2629
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 115, 39, 41])).0, message_present: true }).raise()
                            }
                        })) || !(self::test_java_script_call_coordinate(shared_constructor_field1, (({
                            let source_ml_source2_nullish_field_receiver2631 = (fn_.clone(),);
                            if match (source_ml_source2_nullish_field_receiver2631.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                {
                                    let source_ml_source2_nullish_field_present2632 = ((source_ml_source2_nullish_field_receiver2631.clone()).0).clone().expect("a proven present value");
                                    match source_ml_source2_nullish_field_present2632.clone() {
                                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2633) => {
                                            source_ml_source2_data_property_value2633
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 39, 41])).0, message_present: true }).raise()
                            }
                        }).len() as f64)))) {
                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop146Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop146Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                        } else {
                            {
                                let source_ml_source1_temporary_index142_2 = source_ml_source1_temporary_index142.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index142 = source_ml_source1_temporary_index142_2.clone();
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop146Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop146Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Result {
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Done,
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_machine_error_loop145(mut machine: self::JavaScriptCallMachine, mut source_ml_source1_temporary_values141: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index141: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Result {
    loop {
        return if (source_ml_source1_temporary_index141 < (self::ml::Big::from_u128(source_ml_source1_temporary_values141.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values141, source_ml_source1_temporary_index141.to_index());
                if ((self::read_java_script_ordinary_value_error(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value2635, _) => {
                        let source_ml_source2_data_property_value2635 = *source_ml_source2_data_property_value2635;
                        source_ml_source2_data_property_value2635.clone()
                    }
                }, value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_source1_temporary_index141_2 = source_ml_source1_temporary_index141.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index141 = source_ml_source1_temporary_index141_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Result {
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Done,
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_machine_error_loop144(mut machine: self::JavaScriptCallMachine, mut source_ml_source1_temporary_values140: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index140: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Result {
    loop {
        return if (source_ml_source1_temporary_index140 < (self::ml::Big::from_u128(source_ml_source1_temporary_values140.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values140, source_ml_source1_temporary_index140.to_index());
                if ((self::read_java_script_ordinary_value_error(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value2637, _) => {
                        let source_ml_source2_data_property_value2637 = *source_ml_source2_data_property_value2637;
                        source_ml_source2_data_property_value2637.clone()
                    }
                }, value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_source1_temporary_index140_2 = source_ml_source1_temporary_index140.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index140 = source_ml_source1_temporary_index140_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Done
        };
    }
}

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
                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2642, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value2642.clone()
                                            }
                                        })));
                                        if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                        } else {
                                            if (((((match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2645, _, _, _) => {
                                                    source_ml_source2_data_property_value2645
                                                }
                                            }).len() as f64)) != (match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2646, _, _) => {
                                                    source_ml_source2_data_property_value2646
                                                }
                                            })) || !(self::test_java_script_call_coordinate(match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2647, _, _, _, _) => {
                                                    source_ml_source2_data_property_value2647
                                                }
                                            }, (((match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2648) => {
                                                    source_ml_source2_data_property_value2648
                                                }
                                            }).len() as f64) + 1f64)))) {
                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                            } else {
                                                {
                                                    let slots_2 = (slots + ((match frame.clone().expect("a proven present value") {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2649, _, _, _) => {
                                                            source_ml_source2_data_property_value2649
                                                        }
                                                    }).len() as f64));
                                                    {
                                                        let argument_slots_2 = (argument_slots + ((match frame.clone().expect("a proven present value") {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2650, _, _) => {
                                                                source_ml_source2_data_property_value2650
                                                            }
                                                        }).len() as f64));
                                                        if (((slots_2 > (1048576f64)) || (argument_slots_2 > (1048576f64))) || ((((match frame.clone().expect("a proven present value") {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2652, _) => {
                                                                source_ml_source2_data_property_value2652
                                                            }
                                                        }).len() as f64)) > (1024f64))) {
                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop143Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                        } else {
                                                            {
                                                                let source_ml_source1_temporary_values140 = match frame.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2653, _, _, _) => {
                                                                        source_ml_source2_data_property_value2653
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_temporary_index140 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let source_ml_source2_s2654 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop144(machine.clone(), source_ml_source1_temporary_values140, source_ml_source1_temporary_index140.clone());
                                                                        match source_ml_source2_s2654.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Done => {
                                                                                {
                                                                                    let source_ml_source1_temporary_values141 = match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2655, _, _) => {
                                                                                            source_ml_source2_data_property_value2655
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_temporary_index141 = self::ml::Big::from_i128(0);
                                                                                        {
                                                                                            let source_ml_source2_s2656 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop145(machine.clone(), source_ml_source1_temporary_values141, source_ml_source1_temporary_index141.clone());
                                                                                            match source_ml_source2_s2656.clone() {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop145Done => {
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_values142 = match frame.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2657, _) => {
                                                                                                                source_ml_source2_data_property_value2657
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_index142 = self::ml::Big::from_i128(0);
                                                                                                            {
                                                                                                                let source_ml_source2_s2658 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop146(Some(fn_.clone().expect("a proven present value")), source_ml_source1_temporary_values142, source_ml_source1_temporary_index142.clone());
                                                                                                                match source_ml_source2_s2658.clone() {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop146Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop146Done => {
                                                                                                                        {
                                                                                                                            let pending = match frame.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2659) => {
                                                                                                                                    let source_ml_source2_data_property_value2659 = *source_ml_source2_data_property_value2659;
                                                                                                                                    source_ml_source2_data_property_value2659.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_s2678 = match pending.clone() {
                                                                                                                                    self::JavaScriptCallPending::CallIdle => {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitCall(destination) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2690, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2690
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitGet(destination_2) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_2, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2691, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2691
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitSet(destination_3) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_3, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2692, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2692
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitKey(destination_4, _, _) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_4, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2693, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2693
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitDefinition(destination_5, _, _) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_5, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2694, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2694
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitNumber(destination_6, _, _) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_6, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2695, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2695
                                                                                                                                            }
                                                                                                                                        })) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::JavaScriptCallPending::WaitText(destination_7, _, _) => {
                                                                                                                                        if !(self::test_java_script_call_coordinate(destination_7, match fn_.clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2696, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value2696
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
                                                                                                                                match source_ml_source2_s2678.clone() {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin147Next => {
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_s2740 = match pending.clone() {
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
                                                                                                                                                                        let source_ml_source2_s2763 = if (index < (((shared_constructor_field5.len() as f64) - 1f64))) {
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
                                                                                                                                                                        match source_ml_source2_s2763.clone() {
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
                                                                                                                                                                        let source_ml_source2_s2777 = if (index < (((shared_constructor_field5.len() as f64) - 1f64))) {
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
                                                                                                                                                                        match source_ml_source2_s2777.clone() {
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
                                                                                                                                            match source_ml_source2_s2740.clone() {
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
            self::JavaScriptCallMachine::CallMachine(shared_constructor_field7, _) => {
                let shared_constructor_field7 = *shared_constructor_field7;
                if (((frames.len() as f64)) > (0f64)) {
                    {
                        let frame = self::ml_array::optional_at(&frames, self::ml_array::number_index(((frames.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlSource1TemporaryThrowJavaScriptCallValueLoop151Result::MlSource1TemporaryThrowJavaScriptCallValueLoop151Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                        } else {
                            {
                                let handler = self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2823, _) => {
                                        source_ml_source2_data_property_value2823
                                    }
                                }, self::ml_array::number_index((((match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2824, _) => {
                                        source_ml_source2_data_property_value2824
                                    }
                                }).len() as f64) - 1f64)));
                                if !(match (handler.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    {
                                        let restored = {
                                            let source_ml_source2_nominal_spread2829 = self::write_java_script_call_register(frame.clone().expect("a proven present value"), match handler.clone().expect("a proven present value") {
                                                self::JavaScriptCallHandler::CallHandler(_, source_ml_source2_data_property_value2825) => {
                                                    source_ml_source2_data_property_value2825
                                                }
                                            }, value.clone());
                                            match source_ml_source2_nominal_spread2829.clone() {
                                                self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                                    let shared_constructor_field6 = *shared_constructor_field6;
                                                    {
                                                        let source_ml_source2_nominal_field2830 = shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source2_nominal_field2832 = shared_constructor_field2;
                                                            {
                                                                let source_ml_source2_nominal_field2834 = shared_constructor_field3;
                                                                {
                                                                    let source_ml_source2_nominal_field2836 = shared_constructor_field4;
                                                                    {
                                                                        let source_ml_source2_nominal_field2838 = shared_constructor_field5;
                                                                        {
                                                                            let source_ml_source2_nominal_field2840 = shared_constructor_field6.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_override2842 = match handler.clone().expect("a proven present value") {
                                                                                    self::JavaScriptCallHandler::CallHandler(source_ml_source2_data_property_value2826, _) => {
                                                                                        source_ml_source2_data_property_value2826
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override2843 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2828, _) => {
                                                                                            source_ml_source2_data_property_value2828
                                                                                        }
                                                                                    }, 0f64, (-1f64));
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2830.clone(), source_ml_source2_nominal_override2842, source_ml_source2_nominal_field2834, source_ml_source2_nominal_field2836, source_ml_source2_nominal_override2843, Box::new(source_ml_source2_nominal_field2840.clone()))
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
                                        self::MlSource1TemporaryThrowJavaScriptCallValueLoop151Result::MlSource1TemporaryThrowJavaScriptCallValueLoop151Return(Box::new(self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field7.clone(), frames, restored.clone())))))
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
                match active.clone() {
                    self::JavaScriptCallFrame::CallFrame(_, _, shared_constructor_field1, shared_constructor_field2, _, _) => {
                        {
                            let slots_2 = (slots + (shared_constructor_field1.len() as f64));
                            {
                                let argument_slots_2 = (argument_slots + (shared_constructor_field2.len() as f64));
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
