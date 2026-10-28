// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlTemporaryPushJavaScriptCallLoop57Result;
use super::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result;
use super::MlTemporaryThrowJavaScriptCallValueLoop55Result;
use super::find_java_script_call_function;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_call_program_error;
use super::replace_java_script_call_frame;
use super::resume_java_script_property_definition;
use super::resume_java_script_property_key_conversion;
use super::source_ml_temporary_create_java_script_call_frame_loop43;
use super::source_ml_temporary_create_java_script_call_frame_loop44;
use super::source_ml_temporary_create_java_script_call_frame_loop45;
use super::source_ml_temporary_push_java_script_call_loop57;
use super::source_ml_temporary_read_java_script_call_machine_error_loop49;
use super::source_ml_temporary_throw_java_script_call_value_loop55;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_call_frame(fn_: self::JavaScriptCallFunction, receiver: self::JavaScriptSemanticValue, arguments_: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallFrame {
    {
        let registers = Vec::<self::JavaScriptSemanticValue>::new();
        {
            let index = 0f64;
            {
                let registers_2 = self::source_ml_temporary_create_java_script_call_frame_loop43(fn_.clone(), registers, index);
                {
                    let bound = if ((match fn_.clone() {
                        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source1_data_property_value1151, _, _, _, _, _) => {
                            source_ml_source1_data_property_value1151.clone()
                        }
                    }) == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108]))) {
                        match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(_, _, _, source_ml_source1_data_property_value1152, _, _, _, _) => {
                                let source_ml_source1_data_property_value1152 = *source_ml_source1_data_property_value1152;
                                source_ml_source1_data_property_value1152.clone()
                            }
                        }
                    } else {
                        receiver.clone()
                    };
                    {
                        let registers_3 = { let ml_values6 = registers_2; let ml_callback6 = { let bound = bound.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, index_2: f64| -> self::JavaScriptSemanticValue { if (index_2 == (0f64)) {
                            bound.clone()
                        } else {
                            entry.clone()
                        } }) }; ml_values6.iter().cloned().enumerate().map(|(ml_index6, ml_value6)| ml_callback6(ml_value6, (ml_index6 as f64))).collect::<Vec<_>>() };
                        {
                            let index_2 = 0f64;
                            {
                                let registers_4 = self::source_ml_temporary_create_java_script_call_frame_loop44(fn_.clone(), arguments_.clone(), registers_3, index_2);
                                {
                                    let index_3 = 0f64;
                                    {
                                        let registers_5 = self::source_ml_temporary_create_java_script_call_frame_loop45(fn_.clone(), registers_4, index_3);
                                        self::JavaScriptCallFrame::CallFrame(match fn_.clone() {
                                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value1153, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value1153.clone()
                                            }
                                        }, 0f64, registers_5, arguments_, Vec::<self::JavaScriptCallHandler>::new(), Box::new(self::JavaScriptCallPending::CallIdle))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_machine_error(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine) -> self::ml_string::Text {
    {
        let error = self::read_java_script_call_program_error(program.clone(), match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1154, _) => {
                let source_ml_source1_data_property_value1154 = *source_ml_source1_data_property_value1154;
                source_ml_source1_data_property_value1154.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            if (((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1156) => source_ml_source1_data_property_value1156.as_slice() }).len() as f64)) == (0f64)) || ((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1158) => source_ml_source1_data_property_value1158.as_slice() }).len() as f64)) > (1024f64))) {
                self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 109, 97, 99, 104, 105, 110, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 97, 99, 116, 105, 118, 101, 32, 115, 116, 97, 99, 107])
            } else {
                {
                    let slots = 0f64;
                    {
                        let argument_slots = 0f64;
                        {
                            let index = 0f64;
                            {
                                let source_ml_source1_s1159 = self::source_ml_temporary_read_java_script_call_machine_error_loop49(program.clone(), machine.clone(), slots, argument_slots, index);
                                match source_ml_source1_s1159.clone() {
                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Done => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(source_ml_temporary_result49) => {
                                        source_ml_temporary_result49.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn throw_java_script_call_value(machine: self::JavaScriptCallMachine, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    {
        let frames = match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1160) => {
                source_ml_source1_data_property_value1160
            }
        };
        {
            let source_ml_source1_s1161 = self::source_ml_temporary_throw_java_script_call_value_loop55(machine.clone(), value.clone(), frames);
            match source_ml_source1_s1161.clone() {
                self::MlTemporaryThrowJavaScriptCallValueLoop55Result::MlTemporaryThrowJavaScriptCallValueLoop55Done => {
                    self::JavaScriptCallMachineTransition::MachineThrow(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1162, _) => {
                            let source_ml_source1_data_property_value1162 = *source_ml_source1_data_property_value1162;
                            source_ml_source1_data_property_value1162.clone()
                        }
                    }), Box::new(value.clone()))
                }
                self::MlTemporaryThrowJavaScriptCallValueLoop55Result::MlTemporaryThrowJavaScriptCallValueLoop55Return(source_ml_temporary_result55) => {
                    let source_ml_temporary_result55 = *source_ml_temporary_result55;
                    source_ml_temporary_result55.clone()
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn request_java_script_call_error(machine: self::JavaScriptCallMachine, realm: self::ml::Big, error_class: &self::ml_string::Text, reason: &self::ml_string::Text) -> self::JavaScriptCallMachineTransition {
    {
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1163) => source_ml_source1_data_property_value1163.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1164) => source_ml_source1_data_property_value1164.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1165, _) => {
                    let source_ml_source1_data_property_value1165 = *source_ml_source1_data_property_value1165;
                    source_ml_source1_data_property_value1165.clone()
                }
            }), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
        } else {
            {
                let next = self::replace_java_script_call_frame(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1166, _) => {
                        let source_ml_source1_data_property_value1166 = *source_ml_source1_data_property_value1166;
                        source_ml_source1_data_property_value1166.clone()
                    }
                }, match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1167) => {
                        source_ml_source1_data_property_value1167
                    }
                }, {
                    let source_ml_source1_nominal_spread1168 = frame.clone().expect("a proven present value");
                    {
                        let source_ml_source1_nominal_field1169 = match source_ml_source1_nominal_spread1168.clone() {
                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1170, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1170.clone()
                            }
                        };
                        {
                            let source_ml_source1_nominal_field1171 = match source_ml_source1_nominal_spread1168.clone() {
                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1172, _, _, _, _) => {
                                    source_ml_source1_data_property_value1172
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1173 = match source_ml_source1_nominal_spread1168.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1174, _, _, _) => {
                                        source_ml_source1_data_property_value1174
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1175 = match source_ml_source1_nominal_spread1168.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1176, _, _) => {
                                            source_ml_source1_data_property_value1176
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1177 = match source_ml_source1_nominal_spread1168.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1178, _) => {
                                                source_ml_source1_data_property_value1178
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1179 = match source_ml_source1_nominal_spread1168.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1180) => {
                                                    let source_ml_source1_data_property_value1180 = *source_ml_source1_data_property_value1180;
                                                    source_ml_source1_data_property_value1180.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_override1181 = self::JavaScriptCallPending::WaitError(realm.clone(), error_class.clone(), reason.clone());
                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1169.clone(), source_ml_source1_nominal_field1171, source_ml_source1_nominal_field1173, source_ml_source1_nominal_field1175, source_ml_source1_nominal_field1177, Box::new(source_ml_source1_nominal_override1181.clone()))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                });
                self::JavaScriptCallMachineTransition::MachineAllocateError(Box::new(next.clone()), realm.clone(), error_class.clone(), reason.clone())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn push_java_script_call(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, callee: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue, arguments_: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallMachineTransition {
    {
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1182) => source_ml_source1_data_property_value1182.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1183) => source_ml_source1_data_property_value1183.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1184, _) => {
                    let source_ml_source1_data_property_value1184 = *source_ml_source1_data_property_value1184;
                    source_ml_source1_data_property_value1184.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 110, 101, 115, 116, 101, 100, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116]))
        } else {
            {
                let owner = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1185, _, _) => source_ml_source1_data_property_value1185.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1186, _, _, _, _, _) => {
                        source_ml_source1_data_property_value1186.clone()
                    }
                })));
                if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1187, _) => {
                            let source_ml_source1_data_property_value1187 = *source_ml_source1_data_property_value1187;
                            source_ml_source1_data_property_value1187.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                } else {
                    match callee.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1193, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1193.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1194, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1194.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1188) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1195, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1195.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1189) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1196, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1196.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1190) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1197, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1197.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1191) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1198, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1198.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1192) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1199, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1199.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Object(identity, callable) => {
                            if !callable {
                                self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1200, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value1200.clone()
                                    }
                                }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                            } else {
                                {
                                    let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1201, _, _) => source_ml_source1_data_property_value1201.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), identity.clone())));
                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1202, _) => {
                                                let source_ml_source1_data_property_value1202 = *source_ml_source1_data_property_value1202;
                                                source_ml_source1_data_property_value1202.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))
                                    } else {
                                        {
                                            let slots = match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value1203, _, _) => {
                                                    source_ml_source1_data_property_value1203
                                                }
                                            };
                                            {
                                                let argument_slots = (arguments_.len() as f64);
                                                {
                                                    let source_ml_temporary_values56 = match machine.clone() {
                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1204) => {
                                                            source_ml_source1_data_property_value1204
                                                        }
                                                    };
                                                    {
                                                        let source_ml_temporary_index56 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source1_s1205 = self::source_ml_temporary_push_java_script_call_loop57(slots, argument_slots, source_ml_temporary_values56, source_ml_temporary_index56.clone());
                                                            match source_ml_source1_s1205.clone() {
                                                                self::MlTemporaryPushJavaScriptCallLoop57Result::MlTemporaryPushJavaScriptCallLoop57Done(slots_2, argument_slots_2) => {
                                                                    if ((((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1207) => source_ml_source1_data_property_value1207.as_slice() }).len() as f64)) >= (1024f64)) || (slots_2 > (1048576f64))) || (argument_slots_2 > (1048576f64))) {
                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1208, _) => {
                                                                                let source_ml_source1_data_property_value1208 = *source_ml_source1_data_property_value1208;
                                                                                source_ml_source1_data_property_value1208.clone()
                                                                            }
                                                                        }), {
                                                                            let source_ml_source1_concat_value1210 = match machine.clone() {
                                                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1209) => {
                                                                                    source_ml_source1_data_property_value1209
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_concat_value1211 = vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_)];
                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), source_ml_source1_concat_value1210), source_ml_source1_concat_value1211)
                                                                            }
                                                                        })))
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
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_key(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptPropertyKeyConversionTransition) -> self::JavaScriptCallMachineTransition {
    {
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1212) => source_ml_source1_data_property_value1212.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1213) => source_ml_source1_data_property_value1213.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1214, _) => {
                    let source_ml_source1_data_property_value1214 = *source_ml_source1_data_property_value1214;
                    source_ml_source1_data_property_value1214.clone()
                }
            }), self::ml_string::Text::from_units(vec![107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
        } else {
            match transition.clone() {
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(state) => {
                    let state = *state;
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1215, _) => {
                            let source_ml_source1_data_property_value1215 = *source_ml_source1_data_property_value1215;
                            source_ml_source1_data_property_value1215.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1216) => {
                            source_ml_source1_data_property_value1216
                        }
                    }, {
                        let source_ml_source1_nominal_spread1217 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source1_nominal_field1218 = match source_ml_source1_nominal_spread1217.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1219, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1219.clone()
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1220 = match source_ml_source1_nominal_spread1217.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1221, _, _, _, _) => {
                                        source_ml_source1_data_property_value1221
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1222 = match source_ml_source1_nominal_spread1217.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1223, _, _, _) => {
                                            source_ml_source1_data_property_value1223
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1224 = match source_ml_source1_nominal_spread1217.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1225, _, _) => {
                                                source_ml_source1_data_property_value1225
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1226 = match source_ml_source1_nominal_spread1217.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1227, _) => {
                                                    source_ml_source1_data_property_value1227
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field1228 = match source_ml_source1_nominal_spread1217.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1229) => {
                                                        let source_ml_source1_data_property_value1229 = *source_ml_source1_data_property_value1229;
                                                        source_ml_source1_data_property_value1229.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override1230 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1218.clone(), source_ml_source1_nominal_field1220, source_ml_source1_nominal_field1222, source_ml_source1_nominal_field1224, source_ml_source1_nominal_field1226, Box::new(source_ml_source1_nominal_override1230.clone()))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    })))
                }
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCall(state_2, callee, receiver, arguments) => {
                    let state_2 = *state_2;
                    let callee = *callee;
                    let receiver = *receiver;
                    self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1231, _) => {
                            let source_ml_source1_data_property_value1231 = *source_ml_source1_data_property_value1231;
                            source_ml_source1_data_property_value1231.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1232) => {
                            source_ml_source1_data_property_value1232
                        }
                    }, {
                        let source_ml_source1_nominal_spread1233 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source1_nominal_field1234 = match source_ml_source1_nominal_spread1233.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1235, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1235.clone()
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1236 = match source_ml_source1_nominal_spread1233.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1237, _, _, _, _) => {
                                        source_ml_source1_data_property_value1237
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1238 = match source_ml_source1_nominal_spread1233.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1239, _, _, _) => {
                                            source_ml_source1_data_property_value1239
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1240 = match source_ml_source1_nominal_spread1233.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1241, _, _) => {
                                                source_ml_source1_data_property_value1241
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1242 = match source_ml_source1_nominal_spread1233.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1243, _) => {
                                                    source_ml_source1_data_property_value1243
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field1244 = match source_ml_source1_nominal_spread1233.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1245) => {
                                                        let source_ml_source1_data_property_value1245 = *source_ml_source1_data_property_value1245;
                                                        source_ml_source1_data_property_value1245.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override1246 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state_2.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1234.clone(), source_ml_source1_nominal_field1236, source_ml_source1_nominal_field1238, source_ml_source1_nominal_field1240, source_ml_source1_nominal_field1242, Box::new(source_ml_source1_nominal_override1246.clone()))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }), callee.clone(), receiver.clone(), arguments)
                }
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(key) => {
                    let key = *key;
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1247, _) => {
                            let source_ml_source1_data_property_value1247 = *source_ml_source1_data_property_value1247;
                            source_ml_source1_data_property_value1247.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1248) => {
                            source_ml_source1_data_property_value1248
                        }
                    }, self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, key.clone()))))
                }
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyThrow(value) => {
                    let value = *value;
                    self::throw_java_script_call_value(machine.clone(), value.clone())
                }
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyIntrinsicError(error_class, reason) => {
                    self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                }
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(reason_2) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1249, _) => {
                            let source_ml_source1_data_property_value1249 = *source_ml_source1_data_property_value1249;
                            source_ml_source1_data_property_value1249.clone()
                        }
                    }), reason_2.clone())
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_definition(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptPropertyDefinitionTransition) -> self::JavaScriptCallMachineTransition {
    {
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1250) => source_ml_source1_data_property_value1250.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1251) => source_ml_source1_data_property_value1251.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1252, _) => {
                    let source_ml_source1_data_property_value1252 = *source_ml_source1_data_property_value1252;
                    source_ml_source1_data_property_value1252.clone()
                }
            }), self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
        } else {
            match transition.clone() {
                self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(state) => {
                    let state = *state;
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1254, _) => {
                            let source_ml_source1_data_property_value1254 = *source_ml_source1_data_property_value1254;
                            source_ml_source1_data_property_value1254.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1255) => {
                            source_ml_source1_data_property_value1255
                        }
                    }, {
                        let source_ml_source1_nominal_spread1256 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source1_nominal_field1257 = match source_ml_source1_nominal_spread1256.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1258, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1258.clone()
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1259 = match source_ml_source1_nominal_spread1256.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1260, _, _, _, _) => {
                                        source_ml_source1_data_property_value1260
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1261 = match source_ml_source1_nominal_spread1256.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1262, _, _, _) => {
                                            source_ml_source1_data_property_value1262
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1263 = match source_ml_source1_nominal_spread1256.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1264, _, _) => {
                                                source_ml_source1_data_property_value1264
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1265 = match source_ml_source1_nominal_spread1256.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1266, _) => {
                                                    source_ml_source1_data_property_value1266
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field1267 = match source_ml_source1_nominal_spread1256.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1268) => {
                                                        let source_ml_source1_data_property_value1268 = *source_ml_source1_data_property_value1268;
                                                        source_ml_source1_data_property_value1268.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override1269 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1257.clone(), source_ml_source1_nominal_field1259, source_ml_source1_nominal_field1261, source_ml_source1_nominal_field1263, source_ml_source1_nominal_field1265, Box::new(source_ml_source1_nominal_override1269.clone()))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    })))
                }
                self::JavaScriptPropertyDefinitionTransition::DefinitionCall(state_2, callee, receiver, arguments) => {
                    let state_2 = *state_2;
                    let callee = *callee;
                    let receiver = *receiver;
                    self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1270, _) => {
                            let source_ml_source1_data_property_value1270 = *source_ml_source1_data_property_value1270;
                            source_ml_source1_data_property_value1270.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1271) => {
                            source_ml_source1_data_property_value1271
                        }
                    }, {
                        let source_ml_source1_nominal_spread1272 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source1_nominal_field1273 = match source_ml_source1_nominal_spread1272.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1274, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1274.clone()
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1275 = match source_ml_source1_nominal_spread1272.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1276, _, _, _, _) => {
                                        source_ml_source1_data_property_value1276
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1277 = match source_ml_source1_nominal_spread1272.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1278, _, _, _) => {
                                            source_ml_source1_data_property_value1278
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1279 = match source_ml_source1_nominal_spread1272.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1280, _, _) => {
                                                source_ml_source1_data_property_value1280
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1281 = match source_ml_source1_nominal_spread1272.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1282, _) => {
                                                    source_ml_source1_data_property_value1282
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field1283 = match source_ml_source1_nominal_spread1272.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1284) => {
                                                        let source_ml_source1_data_property_value1284 = *source_ml_source1_data_property_value1284;
                                                        source_ml_source1_data_property_value1284.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override1285 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state_2.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1273.clone(), source_ml_source1_nominal_field1275, source_ml_source1_nominal_field1277, source_ml_source1_nominal_field1279, source_ml_source1_nominal_field1281, Box::new(source_ml_source1_nominal_override1285.clone()))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }), callee.clone(), receiver.clone(), arguments)
                }
                self::JavaScriptPropertyDefinitionTransition::DefinitionCompleted(heap, value, source_ml_source1_f1253) => {
                    let heap = *heap;
                    let value = *value;
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1286) => {
                            source_ml_source1_data_property_value1286
                        }
                    }, self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                }
                self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(value_2) => {
                    let value_2 = *value_2;
                    self::throw_java_script_call_value(machine.clone(), value_2.clone())
                }
                self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(error_class, reason) => {
                    self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                }
                self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(reason_2) => {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1287, _) => {
                            let source_ml_source1_data_property_value1287 = *source_ml_source1_data_property_value1287;
                            source_ml_source1_data_property_value1287.clone()
                        }
                    }), reason_2.clone())
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn return_java_script_call_value(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    {
        let frames = self::ml_array::slice((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1289) => source_ml_source1_data_property_value1289.as_slice() }), 0f64, (-1f64));
        {
            let parent = self::ml_array::optional_at(&frames, self::ml_array::number_index(((frames.len() as f64) - 1f64)));
            if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                self::JavaScriptCallMachineTransition::MachineCompleted(Box::new(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1290, _) => {
                        let source_ml_source1_data_property_value1290 = *source_ml_source1_data_property_value1290;
                        source_ml_source1_data_property_value1290.clone()
                    }
                }), Box::new(value.clone()))
            } else {
                {
                    let current = self::JavaScriptCallMachine::CallMachine(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1291, _) => {
                            let source_ml_source1_data_property_value1291 = *source_ml_source1_data_property_value1291;
                            source_ml_source1_data_property_value1291.clone()
                        }
                    }), frames.clone());
                    {
                        let pending = match parent.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1292) => {
                                let source_ml_source1_data_property_value1292 = *source_ml_source1_data_property_value1292;
                                source_ml_source1_data_property_value1292.clone()
                            }
                        };
                        match pending.clone() {
                            self::JavaScriptCallPending::CallIdle => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1296, _) => {
                                        let source_ml_source1_data_property_value1296 = *source_ml_source1_data_property_value1296;
                                        source_ml_source1_data_property_value1296.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                            }
                            self::JavaScriptCallPending::WaitCall(destination) => {
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1297, _) => {
                                        let source_ml_source1_data_property_value1297 = *source_ml_source1_data_property_value1297;
                                        source_ml_source1_data_property_value1297.clone()
                                    }
                                }, frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination, value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitGet(destination_2) => {
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1298, _) => {
                                        let source_ml_source1_data_property_value1298 = *source_ml_source1_data_property_value1298;
                                        source_ml_source1_data_property_value1298.clone()
                                    }
                                }, frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination_2, value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitSet(destination_3) => {
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1299, _) => {
                                        let source_ml_source1_data_property_value1299 = *source_ml_source1_data_property_value1299;
                                        source_ml_source1_data_property_value1299.clone()
                                    }
                                }, frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination_3, self::JavaScriptSemanticValue::Boolean(true)))))
                            }
                            self::JavaScriptCallPending::WaitKey(destination_4, realm, state) => {
                                let state = *state;
                                self::accept_java_script_call_key(program.clone(), current.clone(), destination_4, realm.clone(), self::resume_java_script_property_key_conversion(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1300, _) => {
                                        let source_ml_source1_data_property_value1300 = *source_ml_source1_data_property_value1300;
                                        source_ml_source1_data_property_value1300.clone()
                                    }
                                }, state.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitDefinition(destination_5, realm_2, state_2) => {
                                let state_2 = *state_2;
                                self::accept_java_script_call_definition(program.clone(), current.clone(), destination_5, realm_2.clone(), self::resume_java_script_property_definition(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1301, _) => {
                                        let source_ml_source1_data_property_value1301 = *source_ml_source1_data_property_value1301;
                                        source_ml_source1_data_property_value1301.clone()
                                    }
                                }, state_2.clone(), self::JavaScriptSemanticCompletion::NormalCompletion(Box::new(value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitError(source_ml_source1_f1293, source_ml_source1_f1294, source_ml_source1_f1295) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1302, _) => {
                                        let source_ml_source1_data_property_value1302 = *source_ml_source1_data_property_value1302;
                                        source_ml_source1_data_property_value1302.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                            }
                        }
                    }
                }
            }
        }
    }
}
