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
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Result;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result;
use super::find_java_script_call_function;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_call_frame;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop140;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop141;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop142;
use super::test_java_script_call_coordinate;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_machine_error_loop139(mut program: self::JavaScriptCallProgram, mut machine: self::JavaScriptCallMachine, mut slots: f64, mut argument_slots: f64, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result {
    loop {
        return if (index < (((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value2283) => source_ml_source2_data_property_value2283.as_slice() }).len() as f64))) {
            {
                let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value2284) => source_ml_source2_data_property_value2284.as_slice() }), self::ml_array::number_index(index));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                } else {
                    {
                        let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value2285, _, _) => source_ml_source2_data_property_value2285.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2286, _, _, _, _, _) => {
                                source_ml_source2_data_property_value2286.clone()
                            }
                        })));
                        if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                        } else {
                            if (((((match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2289, _, _, _) => {
                                    source_ml_source2_data_property_value2289
                                }
                            }).len() as f64)) != (match fn_.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2290, _, _) => {
                                    source_ml_source2_data_property_value2290
                                }
                            })) || !(self::test_java_script_call_coordinate(match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2291, _, _, _, _) => {
                                    source_ml_source2_data_property_value2291
                                }
                            }, (((match fn_.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2292) => {
                                    source_ml_source2_data_property_value2292
                                }
                            }).len() as f64) + 1f64)))) {
                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                            } else {
                                {
                                    let slots_2 = (slots + ((match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2293, _, _, _) => {
                                            source_ml_source2_data_property_value2293
                                        }
                                    }).len() as f64));
                                    {
                                        let argument_slots_2 = (argument_slots + ((match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2294, _, _) => {
                                                source_ml_source2_data_property_value2294
                                            }
                                        }).len() as f64));
                                        if (((slots_2 > (1048576f64)) || (argument_slots_2 > (1048576f64))) || ((((match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2296, _) => {
                                                source_ml_source2_data_property_value2296
                                            }
                                        }).len() as f64)) > (1024f64))) {
                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                        } else {
                                            {
                                                let source_ml_source1_temporary_values136 = match frame.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2297, _, _, _) => {
                                                        source_ml_source2_data_property_value2297
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_temporary_index136 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s2298 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop140(machine.clone(), source_ml_source1_temporary_values136, source_ml_source1_temporary_index136.clone());
                                                        match source_ml_source2_s2298.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Done => {
                                                                {
                                                                    let source_ml_source1_temporary_values137 = match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2299, _, _) => {
                                                                            source_ml_source2_data_property_value2299
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_temporary_index137 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let source_ml_source2_s2300 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop141(machine.clone(), source_ml_source1_temporary_values137, source_ml_source1_temporary_index137.clone());
                                                                            match source_ml_source2_s2300.clone() {
                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Done => {
                                                                                    {
                                                                                        let source_ml_source1_temporary_values138 = match frame.clone().expect("a proven present value") {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2301, _) => {
                                                                                                source_ml_source2_data_property_value2301
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_temporary_index138 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source2_s2302 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop142(Some(fn_.clone().expect("a proven present value")), source_ml_source1_temporary_values138, source_ml_source1_temporary_index138.clone());
                                                                                                match source_ml_source2_s2302.clone() {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Done => {
                                                                                                        {
                                                                                                            let pending = match frame.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2303) => {
                                                                                                                    let source_ml_source2_data_property_value2303 = *source_ml_source2_data_property_value2303;
                                                                                                                    source_ml_source2_data_property_value2303.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_s2316 = match pending.clone() {
                                                                                                                    self::JavaScriptCallPending::CallIdle => {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Next
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitCall(destination) => {
                                                                                                                        if !(self::test_java_script_call_coordinate(destination, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2324, _, _) => {
                                                                                                                                source_ml_source2_data_property_value2324
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitGet(destination_2) => {
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_2, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2325, _, _) => {
                                                                                                                                source_ml_source2_data_property_value2325
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitSet(destination_3) => {
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_3, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2326, _, _) => {
                                                                                                                                source_ml_source2_data_property_value2326
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitKey(destination_4, source_ml_source2_f2317, source_ml_source2_f2318) => {
                                                                                                                        let source_ml_source2_f2318 = *source_ml_source2_f2318;
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_4, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2327, _, _) => {
                                                                                                                                source_ml_source2_data_property_value2327
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitDefinition(destination_5, source_ml_source2_f2319, source_ml_source2_f2320) => {
                                                                                                                        let source_ml_source2_f2320 = *source_ml_source2_f2320;
                                                                                                                        if !(self::test_java_script_call_coordinate(destination_5, match fn_.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2328, _, _) => {
                                                                                                                                source_ml_source2_data_property_value2328
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                                                                                                        } else {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::JavaScriptCallPending::WaitError(source_ml_source2_f2321, source_ml_source2_f2322, source_ml_source2_f2323) => {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Next
                                                                                                                    }
                                                                                                                };
                                                                                                                match source_ml_source2_s2316.clone() {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Next => {
                                                                                                                        {
                                                                                                                            let source_ml_source2_s2344 = match pending.clone() {
                                                                                                                                self::JavaScriptCallPending::CallIdle => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitCall(source_ml_source2_f2345) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitGet(source_ml_source2_f2346) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitSet(source_ml_source2_f2347) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitKey(source_ml_source2_f2348, realm, source_ml_source2_f2349) => {
                                                                                                                                    let source_ml_source2_f2349 = *source_ml_source2_f2349;
                                                                                                                                    if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source2_data_property_value2355, _) => source_ml_source2_data_property_value2355.as_slice() }), &realm.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                    } else {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitDefinition(source_ml_source2_f2350, realm_2, source_ml_source2_f2351) => {
                                                                                                                                    let source_ml_source2_f2351 = *source_ml_source2_f2351;
                                                                                                                                    if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source2_data_property_value2357, _) => source_ml_source2_data_property_value2357.as_slice() }), &realm_2.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                    } else {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                self::JavaScriptCallPending::WaitError(realm_3, source_ml_source2_f2352, source_ml_source2_f2353) => {
                                                                                                                                    if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source2_data_property_value2359, _) => source_ml_source2_data_property_value2359.as_slice() }), &realm_3.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                                                    } else {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            };
                                                                                                                            match source_ml_source2_s2344.clone() {
                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next => {
                                                                                                                                    if ((index < ((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value2361) => source_ml_source2_data_property_value2361.as_slice() }).len() as f64) - 1f64))) && (match pending.clone() {
                                                                                                                                        self::JavaScriptCallPending::CallIdle => {
                                                                                                                                            true
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitCall(source_ml_source2_f2362) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitGet(source_ml_source2_f2363) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitSet(source_ml_source2_f2364) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitKey(source_ml_source2_f2365, source_ml_source2_f2366, source_ml_source2_f2367) => {
                                                                                                                                            let source_ml_source2_f2367 = *source_ml_source2_f2367;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitDefinition(source_ml_source2_f2368, source_ml_source2_f2369, source_ml_source2_f2370) => {
                                                                                                                                            let source_ml_source2_f2370 = *source_ml_source2_f2370;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitError(source_ml_source2_f2371, source_ml_source2_f2372, source_ml_source2_f2373) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                    } || match pending.clone() {
                                                                                                                                        self::JavaScriptCallPending::CallIdle => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitCall(source_ml_source2_f2374) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitGet(source_ml_source2_f2375) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitSet(source_ml_source2_f2376) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitKey(source_ml_source2_f2377, source_ml_source2_f2378, source_ml_source2_f2379) => {
                                                                                                                                            let source_ml_source2_f2379 = *source_ml_source2_f2379;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitDefinition(source_ml_source2_f2380, source_ml_source2_f2381, source_ml_source2_f2382) => {
                                                                                                                                            let source_ml_source2_f2382 = *source_ml_source2_f2382;
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        self::JavaScriptCallPending::WaitError(source_ml_source2_f2383, source_ml_source2_f2384, source_ml_source2_f2385) => {
                                                                                                                                            true
                                                                                                                                        }
                                                                                                                                    })) {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Return(self::ml_string::Text::from_units(vec![97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108]))
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
                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Return(source_ml_source1_temporary_result144) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Return(source_ml_source1_temporary_result144.clone())
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Return(source_ml_source1_temporary_result144_2) => {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Return(source_ml_source1_temporary_result144_2.clone())
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(source_ml_source1_temporary_result144_3) => {
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Return(source_ml_source1_temporary_result144_3.clone())
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop141Return(source_ml_source1_temporary_result144_4) => {
                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Return(source_ml_source1_temporary_result144_4.clone())
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop140Return(source_ml_source1_temporary_result144_5) => {
                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Return(source_ml_source1_temporary_result144_5.clone())
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
            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryThrowJavaScriptCallValueLoop145Result {
    MlSource1TemporaryThrowJavaScriptCallValueLoop145Done,
    MlSource1TemporaryThrowJavaScriptCallValueLoop145Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_throw_java_script_call_value_loop145(mut machine: self::JavaScriptCallMachine, mut value: self::JavaScriptSemanticValue, mut frames: Vec<self::JavaScriptCallFrame>) -> self::MlSource1TemporaryThrowJavaScriptCallValueLoop145Result {
    loop {
        return if (((frames.len() as f64)) > (0f64)) {
            {
                let frame = self::ml_array::optional_at(&frames, self::ml_array::number_index(((frames.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlSource1TemporaryThrowJavaScriptCallValueLoop145Result::MlSource1TemporaryThrowJavaScriptCallValueLoop145Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value2386, _) => {
                            let source_ml_source2_data_property_value2386 = *source_ml_source2_data_property_value2386;
                            source_ml_source2_data_property_value2386.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                } else {
                    {
                        let handler = self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2387, _) => {
                                source_ml_source2_data_property_value2387
                            }
                        }, self::ml_array::number_index((((match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2388, _) => {
                                source_ml_source2_data_property_value2388
                            }
                        }).len() as f64) - 1f64)));
                        if !(match (handler.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                            {
                                let restored = {
                                    let source_ml_source2_nominal_spread2393 = self::write_java_script_call_register(frame.clone().expect("a proven present value"), match handler.clone().expect("a proven present value") {
                                        self::JavaScriptCallHandler::CallHandler(_, source_ml_source2_data_property_value2389) => {
                                            source_ml_source2_data_property_value2389
                                        }
                                    }, value.clone());
                                    {
                                        let source_ml_source2_nominal_field2394 = match source_ml_source2_nominal_spread2393.clone() {
                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2395, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value2395.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field2396 = match source_ml_source2_nominal_spread2393.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2397, _, _, _, _) => {
                                                    source_ml_source2_data_property_value2397
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field2398 = match source_ml_source2_nominal_spread2393.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2399, _, _, _) => {
                                                        source_ml_source2_data_property_value2399
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field2400 = match source_ml_source2_nominal_spread2393.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2401, _, _) => {
                                                            source_ml_source2_data_property_value2401
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field2402 = match source_ml_source2_nominal_spread2393.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2403, _) => {
                                                                source_ml_source2_data_property_value2403
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field2404 = match source_ml_source2_nominal_spread2393.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2405) => {
                                                                    let source_ml_source2_data_property_value2405 = *source_ml_source2_data_property_value2405;
                                                                    source_ml_source2_data_property_value2405.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_override2406 = match handler.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallHandler::CallHandler(source_ml_source2_data_property_value2390, _) => {
                                                                        source_ml_source2_data_property_value2390
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override2407 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2392, _) => {
                                                                            source_ml_source2_data_property_value2392
                                                                        }
                                                                    }, 0f64, (-1f64));
                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2394.clone(), source_ml_source2_nominal_override2406, source_ml_source2_nominal_field2398, source_ml_source2_nominal_field2400, source_ml_source2_nominal_override2407, Box::new(source_ml_source2_nominal_field2404.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                                self::MlSource1TemporaryThrowJavaScriptCallValueLoop145Result::MlSource1TemporaryThrowJavaScriptCallValueLoop145Return(Box::new(self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value2408, _) => {
                                        let source_ml_source2_data_property_value2408 = *source_ml_source2_data_property_value2408;
                                        source_ml_source2_data_property_value2408.clone()
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
            self::MlSource1TemporaryThrowJavaScriptCallValueLoop145Result::MlSource1TemporaryThrowJavaScriptCallValueLoop145Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPushJavaScriptCallLoop147Result {
    MlSource1TemporaryPushJavaScriptCallLoop147Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_call_loop147(mut slots: f64, mut argument_slots: f64, mut source_ml_source1_temporary_values146: Vec<self::JavaScriptCallFrame>, mut source_ml_source1_temporary_index146: self::ml::Big) -> self::MlSource1TemporaryPushJavaScriptCallLoop147Result {
    loop {
        return if (source_ml_source1_temporary_index146 < (self::ml::Big::from_u128(source_ml_source1_temporary_values146.len() as u128))) {
            {
                let active = self::ml_array::at(&source_ml_source1_temporary_values146, source_ml_source1_temporary_index146.to_index());
                {
                    let slots_2 = (slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2409, _, _, _) => source_ml_source2_data_property_value2409.as_slice() }).len() as f64));
                    {
                        let argument_slots_2 = (argument_slots + ((match &active { self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2410, _, _) => source_ml_source2_data_property_value2410.as_slice() }).len() as f64));
                        {
                            let source_ml_source1_temporary_index146_2 = source_ml_source1_temporary_index146.add(&self::ml::Big::from_i128(1));
                            {
                                (slots, argument_slots, source_ml_source1_temporary_index146) = (slots_2, argument_slots_2, source_ml_source1_temporary_index146_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPushJavaScriptCallLoop147Result::MlSource1TemporaryPushJavaScriptCallLoop147Done(slots, argument_slots)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryBeginJavaScriptCallMachineLoop149Result {
    MlSource1TemporaryBeginJavaScriptCallMachineLoop149Done,
    MlSource1TemporaryBeginJavaScriptCallMachineLoop149Return(Box<self::JavaScriptCallMachineTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_begin_java_script_call_machine_loop149(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values148: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index148: self::ml::Big) -> self::MlSource1TemporaryBeginJavaScriptCallMachineLoop149Result {
    loop {
        return if (source_ml_source1_temporary_index148 < (self::ml::Big::from_u128(source_ml_source1_temporary_values148.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values148, source_ml_source1_temporary_index148.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryBeginJavaScriptCallMachineLoop149Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop149Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))))
                } else {
                    {
                        let source_ml_source1_temporary_index148_2 = source_ml_source1_temporary_index148.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index148 = source_ml_source1_temporary_index148_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryBeginJavaScriptCallMachineLoop149Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop149Done
        };
    }
}
