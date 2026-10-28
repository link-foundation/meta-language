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
use super::MlTemporaryReadJavaScriptCallMachineErrorJoin85;
use super::MlTemporaryReadJavaScriptCallMachineErrorJoin86;
use super::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result;
use super::MlTemporaryReadJavaScriptCallMachineErrorLoop84Result;
use super::find_java_script_call_function;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_call_frame;
use super::test_java_script_call_coordinate;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_machine_error_loop84(mut fn_: Option<self::JavaScriptCallFunction>, mut source_ml_temporary_values80: Vec<self::JavaScriptCallHandler>, mut source_ml_temporary_index80: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallMachineErrorLoop84Result {
    loop {
        return if (source_ml_temporary_index80 < (self::ml::Big::from_u128(source_ml_temporary_values80.len() as u128))) {
            {
                let handler = self::ml_array::at(&source_ml_temporary_values80, source_ml_temporary_index80.to_index());
                if (!(self::test_java_script_call_coordinate(match handler.clone() {
                    self::JavaScriptCallHandler::CallHandler(_, source_ml_source1_data_property_value2173) => {
                        source_ml_source1_data_property_value2173
                    }
                }, {
                    let source_ml_source1_nullish_field_receiver2174 = (fn_.clone(),);
                    if match (source_ml_source1_nullish_field_receiver2174.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present2175 = ((source_ml_source1_nullish_field_receiver2174.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present2175.clone() {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2176, _, _) => {
                                    source_ml_source1_data_property_value2176
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 115, 39, 41])).0, message_present: true }).raise()
                    }
                })) || !(self::test_java_script_call_coordinate(match handler.clone() {
                    self::JavaScriptCallHandler::CallHandler(source_ml_source1_data_property_value2177, _) => {
                        source_ml_source1_data_property_value2177
                    }
                }, (({
                    let source_ml_source1_nullish_field_receiver2178 = (fn_.clone(),);
                    if match (source_ml_source1_nullish_field_receiver2178.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present2179 = ((source_ml_source1_nullish_field_receiver2178.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present2179.clone() {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2180) => {
                                    source_ml_source1_data_property_value2180
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 39, 41])).0, message_present: true }).raise()
                    }
                }).len() as f64)))) {
                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop84Result::MlTemporaryReadJavaScriptCallMachineErrorLoop84Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                } else {
                    {
                        let source_ml_temporary_index80_2 = source_ml_temporary_index80.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index80 = source_ml_temporary_index80_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallMachineErrorLoop84Result::MlTemporaryReadJavaScriptCallMachineErrorLoop84Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallMachineErrorLoop83Result {
    MlTemporaryReadJavaScriptCallMachineErrorLoop83Done,
    MlTemporaryReadJavaScriptCallMachineErrorLoop83Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_machine_error_loop83(mut machine: self::JavaScriptCallMachine, mut source_ml_temporary_values79: Vec<self::JavaScriptSemanticValue>, mut source_ml_temporary_index79: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallMachineErrorLoop83Result {
    loop {
        return if (source_ml_temporary_index79 < (self::ml::Big::from_u128(source_ml_temporary_values79.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_temporary_values79, source_ml_temporary_index79.to_index());
                if ((self::read_java_script_ordinary_value_error(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value2182, _) => {
                        let source_ml_source1_data_property_value2182 = *source_ml_source1_data_property_value2182;
                        source_ml_source1_data_property_value2182.clone()
                    }
                }, value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop83Result::MlTemporaryReadJavaScriptCallMachineErrorLoop83Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_temporary_index79_2 = source_ml_temporary_index79.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index79 = source_ml_temporary_index79_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallMachineErrorLoop83Result::MlTemporaryReadJavaScriptCallMachineErrorLoop83Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallMachineErrorLoop82Result {
    MlTemporaryReadJavaScriptCallMachineErrorLoop82Done,
    MlTemporaryReadJavaScriptCallMachineErrorLoop82Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_machine_error_loop82(mut machine: self::JavaScriptCallMachine, mut source_ml_temporary_values78: Vec<self::JavaScriptSemanticValue>, mut source_ml_temporary_index78: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallMachineErrorLoop82Result {
    loop {
        return if (source_ml_temporary_index78 < (self::ml::Big::from_u128(source_ml_temporary_values78.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_temporary_values78, source_ml_temporary_index78.to_index());
                if ((self::read_java_script_ordinary_value_error(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value2184, _) => {
                        let source_ml_source1_data_property_value2184 = *source_ml_source1_data_property_value2184;
                        source_ml_source1_data_property_value2184.clone()
                    }
                }, value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop82Result::MlTemporaryReadJavaScriptCallMachineErrorLoop82Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_temporary_index78_2 = source_ml_temporary_index78.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index78 = source_ml_temporary_index78_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallMachineErrorLoop82Result::MlTemporaryReadJavaScriptCallMachineErrorLoop82Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_machine_error_loop81(mut program: self::JavaScriptCallProgram, mut machine: self::JavaScriptCallMachine, mut slots: f64, mut argument_slots: f64, mut index: f64) -> self::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result {
    loop {
        return if (index < (((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value2186) => source_ml_source1_data_property_value2186.as_slice() }).len() as f64))) {
            {
                let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value2187) => source_ml_source1_data_property_value2187.as_slice() }), self::ml_array::number_index(index));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result::MlTemporaryReadJavaScriptCallMachineErrorLoop81Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                } else {
                    {
                        let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value2188, _, _) => source_ml_source1_data_property_value2188.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value2189, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2189.clone()
                            }
                        })));
                        if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result::MlTemporaryReadJavaScriptCallMachineErrorLoop81Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                        } else {
                            if (((((match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2192, _, _, _) => {
                                    source_ml_source1_data_property_value2192
                                }
                            }).len() as f64)) != (match fn_.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2193, _, _) => {
                                    source_ml_source1_data_property_value2193
                                }
                            })) || !(self::test_java_script_call_coordinate(match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value2194, _, _, _, _) => {
                                    source_ml_source1_data_property_value2194
                                }
                            }, (((match fn_.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2195) => {
                                    source_ml_source1_data_property_value2195
                                }
                            }).len() as f64) + 1f64)))) {
                                self::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result::MlTemporaryReadJavaScriptCallMachineErrorLoop81Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                            } else {
                                {
                                    let slots_2 = (slots + ((match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2196, _, _, _) => {
                                            source_ml_source1_data_property_value2196
                                        }
                                    }).len() as f64));
                                    {
                                        let argument_slots_2 = (argument_slots + ((match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value2197, _, _) => {
                                                source_ml_source1_data_property_value2197
                                            }
                                        }).len() as f64));
                                        if (((slots_2 > (1048576f64)) || (argument_slots_2 > (1048576f64))) || ((((match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2199, _) => {
                                                source_ml_source1_data_property_value2199
                                            }
                                        }).len() as f64)) > (1024f64))) {
                                            self::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result::MlTemporaryReadJavaScriptCallMachineErrorLoop81Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                        } else {
                                            {
                                                let source_ml_temporary_values78 = match frame.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2200, _, _, _) => {
                                                        source_ml_source1_data_property_value2200
                                                    }
                                                };
                                                {
                                                    let source_ml_temporary_index78 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s2201 = self::source_ml_temporary_read_java_script_call_machine_error_loop82(machine.clone(), source_ml_temporary_values78, source_ml_temporary_index78.clone());
                                                        match source_ml_source1_s2201.clone() {
                                                            self::MlTemporaryReadJavaScriptCallMachineErrorLoop82Result::MlTemporaryReadJavaScriptCallMachineErrorLoop82Done => {
                                                                {
                                                                    let source_ml_temporary_values79 = match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value2202, _, _) => {
                                                                            source_ml_source1_data_property_value2202
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_temporary_index79 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let source_ml_source1_s2203 = self::source_ml_temporary_read_java_script_call_machine_error_loop83(machine.clone(), source_ml_temporary_values79, source_ml_temporary_index79.clone());
                                                                            match source_ml_source1_s2203.clone() {
                                                                                self::MlTemporaryReadJavaScriptCallMachineErrorLoop83Result::MlTemporaryReadJavaScriptCallMachineErrorLoop83Done => {
                                                                                    {
                                                                                        let source_ml_temporary_values80 = match frame.clone().expect("a proven present value") {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2204, _) => {
                                                                                                source_ml_source1_data_property_value2204
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_temporary_index80 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source1_s2205 = self::source_ml_temporary_read_java_script_call_machine_error_loop84(Some(fn_.clone().expect("a proven present value")), source_ml_temporary_values80, source_ml_temporary_index80.clone());
                                                                                                match source_ml_source1_s2205.clone() {
                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop84Result::MlTemporaryReadJavaScriptCallMachineErrorLoop84Done => {
                                                                                                        {
                                                                                                            let pending = match frame.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value2206) => {
                                                                                                                    let source_ml_source1_data_property_value2206 = *source_ml_source1_data_property_value2206;
                                                                                                                    source_ml_source1_data_property_value2206.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_s2219 = match pending.clone() {
                                                                                                                    self::JavaScriptCallPending::CallIdle => {
                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin85::MlTemporaryReadJavaScriptCallMachineErrorJoin85Next
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitCall(destination) => {
                                                                                                                        if !(self::test_java_script_call_coordinate(destination, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2227, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2227
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin85::MlTemporaryReadJavaScriptCallMachineErrorJoin85Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin85::MlTemporaryReadJavaScriptCallMachineErrorJoin85Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitGet(destination_2) => {
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_2, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2228, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2228
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin85::MlTemporaryReadJavaScriptCallMachineErrorJoin85Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin85::MlTemporaryReadJavaScriptCallMachineErrorJoin85Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitSet(destination_3) => {
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_3, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2229, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2229
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin85::MlTemporaryReadJavaScriptCallMachineErrorJoin85Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin85::MlTemporaryReadJavaScriptCallMachineErrorJoin85Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitKey(destination_4, source_ml_source1_f2220, source_ml_source1_f2221) => {
                                                                                                                        let source_ml_source1_f2221 = *source_ml_source1_f2221;
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_4, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2230, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2230
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin85::MlTemporaryReadJavaScriptCallMachineErrorJoin85Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin85::MlTemporaryReadJavaScriptCallMachineErrorJoin85Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitDefinition(destination_5, source_ml_source1_f2222, source_ml_source1_f2223) => {
                                                                                                                        let source_ml_source1_f2223 = *source_ml_source1_f2223;
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_5, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2231, _, _) => {
                                                                                                                                source_ml_source1_data_property_value2231
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin85::MlTemporaryReadJavaScriptCallMachineErrorJoin85Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlTemporaryReadJavaScriptCallMachineErrorJoin85::MlTemporaryReadJavaScriptCallMachineErrorJoin85Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitError(source_ml_source1_f2224, source_ml_source1_f2225, source_ml_source1_f2226) => {
                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin85::MlTemporaryReadJavaScriptCallMachineErrorJoin85Next
                                                                                                                    }
                                                                                                                };
                                                                                                                match source_ml_source1_s2219.clone() {
                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin85::MlTemporaryReadJavaScriptCallMachineErrorJoin85Next => {
                                                                                                                        {
                                                                                                                            let source_ml_source1_s2247 = match pending.clone() {
                                                                                                                                self::JavaScriptCallPending::CallIdle => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin86::MlTemporaryReadJavaScriptCallMachineErrorJoin86Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitCall(source_ml_source1_f2248) => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin86::MlTemporaryReadJavaScriptCallMachineErrorJoin86Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitGet(source_ml_source1_f2249) => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin86::MlTemporaryReadJavaScriptCallMachineErrorJoin86Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitSet(source_ml_source1_f2250) => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin86::MlTemporaryReadJavaScriptCallMachineErrorJoin86Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitKey(source_ml_source1_f2251, realm, source_ml_source1_f2252) => {
                                                                                                                                    let source_ml_source1_f2252 = *source_ml_source1_f2252;
                                                                                                                                    if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value2258, _) => source_ml_source1_data_property_value2258.as_slice() }), &realm.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin86::MlTemporaryReadJavaScriptCallMachineErrorJoin86Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                    } else {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin86::MlTemporaryReadJavaScriptCallMachineErrorJoin86Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitDefinition(source_ml_source1_f2253, realm_2, source_ml_source1_f2254) => {
                                                                                                                                    let source_ml_source1_f2254 = *source_ml_source1_f2254;
                                                                                                                                    if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value2260, _) => source_ml_source1_data_property_value2260.as_slice() }), &realm_2.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin86::MlTemporaryReadJavaScriptCallMachineErrorJoin86Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                    } else {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin86::MlTemporaryReadJavaScriptCallMachineErrorJoin86Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitError(realm_3, source_ml_source1_f2255, source_ml_source1_f2256) => {
                                                                                                                                    if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value2262, _) => source_ml_source1_data_property_value2262.as_slice() }), &realm_3.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin86::MlTemporaryReadJavaScriptCallMachineErrorJoin86Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                    } else {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorJoin86::MlTemporaryReadJavaScriptCallMachineErrorJoin86Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            };
                                                                                                                            match source_ml_source1_s2247.clone() {
                                                                                                                                self::MlTemporaryReadJavaScriptCallMachineErrorJoin86::MlTemporaryReadJavaScriptCallMachineErrorJoin86Next => {
                                                                                                                                    if ((index < ((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value2264) => source_ml_source1_data_property_value2264.as_slice() }).len() as f64) - 1f64))) && (match pending.clone() {
                                                                                                                                        self::JavaScriptCallPending::CallIdle => {
                                                                                                                                            true
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitCall(source_ml_source1_f2265) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitGet(source_ml_source1_f2266) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitSet(source_ml_source1_f2267) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitKey(source_ml_source1_f2268, source_ml_source1_f2269, source_ml_source1_f2270) => {
                                                                                                                                            let source_ml_source1_f2270 = *source_ml_source1_f2270;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitDefinition(source_ml_source1_f2271, source_ml_source1_f2272, source_ml_source1_f2273) => {
                                                                                                                                            let source_ml_source1_f2273 = *source_ml_source1_f2273;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitError(source_ml_source1_f2274, source_ml_source1_f2275, source_ml_source1_f2276) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                    } || match pending.clone() {
                                                                                                                                        self::JavaScriptCallPending::CallIdle => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitCall(source_ml_source1_f2277) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitGet(source_ml_source1_f2278) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitSet(source_ml_source1_f2279) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitKey(source_ml_source1_f2280, source_ml_source1_f2281, source_ml_source1_f2282) => {
                                                                                                                                            let source_ml_source1_f2282 = *source_ml_source1_f2282;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitDefinition(source_ml_source1_f2283, source_ml_source1_f2284, source_ml_source1_f2285) => {
                                                                                                                                            let source_ml_source1_f2285 = *source_ml_source1_f2285;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitError(source_ml_source1_f2286, source_ml_source1_f2287, source_ml_source1_f2288) => {
                                                                                                                                            true
                                                                                                                                        }
                                                                                                                                    })) {
                                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result::MlTemporaryReadJavaScriptCallMachineErrorLoop81Return(self::ml_string::Text::from_units(vec![97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108]))
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
                                                                                                                                self::MlTemporaryReadJavaScriptCallMachineErrorJoin86::MlTemporaryReadJavaScriptCallMachineErrorJoin86Return(source_ml_temporary_result86) => {
                                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result::MlTemporaryReadJavaScriptCallMachineErrorLoop81Return(source_ml_temporary_result86.clone())
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorJoin85::MlTemporaryReadJavaScriptCallMachineErrorJoin85Return(source_ml_temporary_result86_2) => {
                                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result::MlTemporaryReadJavaScriptCallMachineErrorLoop81Return(source_ml_temporary_result86_2.clone())
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop84Result::MlTemporaryReadJavaScriptCallMachineErrorLoop84Return(source_ml_temporary_result86_3) => {
                                                                                                        self::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result::MlTemporaryReadJavaScriptCallMachineErrorLoop81Return(source_ml_temporary_result86_3.clone())
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlTemporaryReadJavaScriptCallMachineErrorLoop83Result::MlTemporaryReadJavaScriptCallMachineErrorLoop83Return(source_ml_temporary_result86_4) => {
                                                                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result::MlTemporaryReadJavaScriptCallMachineErrorLoop81Return(source_ml_temporary_result86_4.clone())
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporaryReadJavaScriptCallMachineErrorLoop82Result::MlTemporaryReadJavaScriptCallMachineErrorLoop82Return(source_ml_temporary_result86_5) => {
                                                                self::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result::MlTemporaryReadJavaScriptCallMachineErrorLoop81Return(source_ml_temporary_result86_5.clone())
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
            self::MlTemporaryReadJavaScriptCallMachineErrorLoop81Result::MlTemporaryReadJavaScriptCallMachineErrorLoop81Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryThrowJavaScriptCallValueLoop87Result {
    MlTemporaryThrowJavaScriptCallValueLoop87Done,
    MlTemporaryThrowJavaScriptCallValueLoop87Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_throw_java_script_call_value_loop87(mut machine: self::JavaScriptCallMachine, mut value: self::JavaScriptSemanticValue, mut frames: Vec<self::JavaScriptCallFrame>) -> self::MlTemporaryThrowJavaScriptCallValueLoop87Result {
    loop {
        return if (((frames.len() as f64)) > (0f64)) {
            {
                let frame = self::ml_array::optional_at(&frames, self::ml_array::number_index(((frames.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlTemporaryThrowJavaScriptCallValueLoop87Result::MlTemporaryThrowJavaScriptCallValueLoop87Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value2289, _) => {
                            let source_ml_source1_data_property_value2289 = *source_ml_source1_data_property_value2289;
                            source_ml_source1_data_property_value2289.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                } else {
                    {
                        let handler = self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2290, _) => {
                                source_ml_source1_data_property_value2290
                            }
                        }, self::ml_array::number_index((((match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2291, _) => {
                                source_ml_source1_data_property_value2291
                            }
                        }).len() as f64) - 1f64)));
                        if !(match (handler.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                            {
                                let restored = {
                                    let source_ml_source1_nominal_spread2296 = self::write_java_script_call_register(frame.clone().expect("a proven present value"), match handler.clone().expect("a proven present value") {
                                        self::JavaScriptCallHandler::CallHandler(_, source_ml_source1_data_property_value2292) => {
                                            source_ml_source1_data_property_value2292
                                        }
                                    }, value.clone());
                                    {
                                        let source_ml_source1_nominal_field2297 = match source_ml_source1_nominal_spread2296.clone() {
                                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value2298, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2298.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field2299 = match source_ml_source1_nominal_spread2296.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value2300, _, _, _, _) => {
                                                    source_ml_source1_data_property_value2300
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field2301 = match source_ml_source1_nominal_spread2296.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2302, _, _, _) => {
                                                        source_ml_source1_data_property_value2302
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_field2303 = match source_ml_source1_nominal_spread2296.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value2304, _, _) => {
                                                            source_ml_source1_data_property_value2304
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field2305 = match source_ml_source1_nominal_spread2296.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2306, _) => {
                                                                source_ml_source1_data_property_value2306
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field2307 = match source_ml_source1_nominal_spread2296.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value2308) => {
                                                                    let source_ml_source1_data_property_value2308 = *source_ml_source1_data_property_value2308;
                                                                    source_ml_source1_data_property_value2308.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_override2309 = match handler.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallHandler::CallHandler(source_ml_source1_data_property_value2293, _) => {
                                                                        source_ml_source1_data_property_value2293
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_override2310 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value2295, _) => {
                                                                            source_ml_source1_data_property_value2295
                                                                        }
                                                                    }, 0f64, (-1f64));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field2297.clone(), source_ml_source1_nominal_override2309, source_ml_source1_nominal_field2301, source_ml_source1_nominal_field2303, source_ml_source1_nominal_override2310, Box::new(source_ml_source1_nominal_field2307.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                                self::MlTemporaryThrowJavaScriptCallValueLoop87Result::MlTemporaryThrowJavaScriptCallValueLoop87Return(Box::new(self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value2311, _) => {
                                        let source_ml_source1_data_property_value2311 = *source_ml_source1_data_property_value2311;
                                        source_ml_source1_data_property_value2311.clone()
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
            self::MlTemporaryThrowJavaScriptCallValueLoop87Result::MlTemporaryThrowJavaScriptCallValueLoop87Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPushJavaScriptCallLoop89Result {
    MlTemporaryPushJavaScriptCallLoop89Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_push_java_script_call_loop89(mut slots: f64, mut argument_slots: f64, mut source_ml_temporary_values88: Vec<self::JavaScriptCallFrame>, mut source_ml_temporary_index88: self::ml::Big) -> self::MlTemporaryPushJavaScriptCallLoop89Result {
    loop {
        return if (source_ml_temporary_index88 < (self::ml::Big::from_u128(source_ml_temporary_values88.len() as u128))) {
            {
                let active = self::ml_array::at(&source_ml_temporary_values88, source_ml_temporary_index88.to_index());
                {
                    let slots_2 = (slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value2312, _, _, _) => source_ml_source1_data_property_value2312.as_slice() }).len() as f64));
                    {
                        let argument_slots_2 = (argument_slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value2313, _, _) => source_ml_source1_data_property_value2313.as_slice() }).len() as f64));
                        {
                            let source_ml_temporary_index88_2 = source_ml_temporary_index88.add(&self::ml::Big::from_i128(1));
                            {
                                (slots, argument_slots, source_ml_temporary_index88) = (slots_2, argument_slots_2, source_ml_temporary_index88_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPushJavaScriptCallLoop89Result::MlTemporaryPushJavaScriptCallLoop89Done(slots, argument_slots)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryBeginJavaScriptCallMachineLoop91Result {
    MlTemporaryBeginJavaScriptCallMachineLoop91Done,
    MlTemporaryBeginJavaScriptCallMachineLoop91Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_begin_java_script_call_machine_loop91(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_temporary_values90: Vec<self::JavaScriptSemanticValue>, mut source_ml_temporary_index90: self::ml::Big) -> self::MlTemporaryBeginJavaScriptCallMachineLoop91Result {
    loop {
        return if (source_ml_temporary_index90 < (self::ml::Big::from_u128(source_ml_temporary_values90.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_temporary_values90, source_ml_temporary_index90.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlTemporaryBeginJavaScriptCallMachineLoop91Result::MlTemporaryBeginJavaScriptCallMachineLoop91Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))))
                } else {
                    {
                        let source_ml_temporary_index90_2 = source_ml_temporary_index90.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index90 = source_ml_temporary_index90_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryBeginJavaScriptCallMachineLoop91Result::MlTemporaryBeginJavaScriptCallMachineLoop91Done
        };
    }
}
