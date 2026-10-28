// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
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
use super::MlTemporaryThrowJavaScriptCallValueLoop55Result;
use super::create_java_script_call_frame;
use super::find_java_script_call_function;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::replace_java_script_call_frame;
use super::resume_java_script_property_definition;
use super::resume_java_script_property_key_conversion;
use super::source_ml_temporary_push_java_script_call_loop57;
use super::source_ml_temporary_throw_java_script_call_value_loop55;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn throw_java_script_call_value(machine: self::JavaScriptCallMachine, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    {
        let frames = match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1172) => {
                source_ml_source1_data_property_value1172
            }
        };
        {
            let source_ml_source1_s1173 = self::source_ml_temporary_throw_java_script_call_value_loop55(machine.clone(), value.clone(), frames);
            match source_ml_source1_s1173.clone() {
                self::MlTemporaryThrowJavaScriptCallValueLoop55Result::MlTemporaryThrowJavaScriptCallValueLoop55Done => {
                    self::JavaScriptCallMachineTransition::MachineThrow(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1174, _) => {
                            let source_ml_source1_data_property_value1174 = *source_ml_source1_data_property_value1174;
                            source_ml_source1_data_property_value1174.clone()
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
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1175) => source_ml_source1_data_property_value1175.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1176) => source_ml_source1_data_property_value1176.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1177, _) => {
                    let source_ml_source1_data_property_value1177 = *source_ml_source1_data_property_value1177;
                    source_ml_source1_data_property_value1177.clone()
                }
            }), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
        } else {
            {
                let next = self::replace_java_script_call_frame(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1178, _) => {
                        let source_ml_source1_data_property_value1178 = *source_ml_source1_data_property_value1178;
                        source_ml_source1_data_property_value1178.clone()
                    }
                }, match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1179) => {
                        source_ml_source1_data_property_value1179
                    }
                }, {
                    let source_ml_source1_nominal_spread1180 = frame.clone().expect("a proven present value");
                    {
                        let source_ml_source1_nominal_field1181 = match source_ml_source1_nominal_spread1180.clone() {
                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1182, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1182.clone()
                            }
                        };
                        {
                            let source_ml_source1_nominal_field1183 = match source_ml_source1_nominal_spread1180.clone() {
                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1184, _, _, _, _) => {
                                    source_ml_source1_data_property_value1184
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1185 = match source_ml_source1_nominal_spread1180.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1186, _, _, _) => {
                                        source_ml_source1_data_property_value1186
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1187 = match source_ml_source1_nominal_spread1180.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1188, _, _) => {
                                            source_ml_source1_data_property_value1188
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1189 = match source_ml_source1_nominal_spread1180.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1190, _) => {
                                                source_ml_source1_data_property_value1190
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1191 = match source_ml_source1_nominal_spread1180.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1192) => {
                                                    let source_ml_source1_data_property_value1192 = *source_ml_source1_data_property_value1192;
                                                    source_ml_source1_data_property_value1192.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_override1193 = self::JavaScriptCallPending::WaitError(realm.clone(), error_class.clone(), reason.clone());
                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1181.clone(), source_ml_source1_nominal_field1183, source_ml_source1_nominal_field1185, source_ml_source1_nominal_field1187, source_ml_source1_nominal_field1189, Box::new(source_ml_source1_nominal_override1193.clone()))
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
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1194) => source_ml_source1_data_property_value1194.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1195) => source_ml_source1_data_property_value1195.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1196, _) => {
                    let source_ml_source1_data_property_value1196 = *source_ml_source1_data_property_value1196;
                    source_ml_source1_data_property_value1196.clone()
                }
            }), self::ml_string::Text::from_units(vec![97, 32, 110, 101, 115, 116, 101, 100, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116]))
        } else {
            {
                let owner = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1197, _, _) => source_ml_source1_data_property_value1197.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1198, _, _, _, _, _) => {
                        source_ml_source1_data_property_value1198.clone()
                    }
                })));
                if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1199, _) => {
                            let source_ml_source1_data_property_value1199 = *source_ml_source1_data_property_value1199;
                            source_ml_source1_data_property_value1199.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                } else {
                    match callee.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1205, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1205.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1206, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1206.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1200) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1207, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1207.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1201) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1208, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1208.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1202) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1209, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1209.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1203) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1210, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1210.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1204) => {
                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1211, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1211.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                        }
                        self::JavaScriptSemanticValue::Object(identity, callable) => {
                            if !callable {
                                self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value1212, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value1212.clone()
                                    }
                                }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                            } else {
                                {
                                    let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1213, _, _) => source_ml_source1_data_property_value1213.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), identity.clone())));
                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1214, _) => {
                                                let source_ml_source1_data_property_value1214 = *source_ml_source1_data_property_value1214;
                                                source_ml_source1_data_property_value1214.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))
                                    } else {
                                        {
                                            let slots = match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value1215, _, _) => {
                                                    source_ml_source1_data_property_value1215
                                                }
                                            };
                                            {
                                                let argument_slots = (arguments_.len() as f64);
                                                {
                                                    let source_ml_temporary_values56 = match machine.clone() {
                                                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1216) => {
                                                            source_ml_source1_data_property_value1216
                                                        }
                                                    };
                                                    {
                                                        let source_ml_temporary_index56 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source1_s1217 = self::source_ml_temporary_push_java_script_call_loop57(slots, argument_slots, source_ml_temporary_values56, source_ml_temporary_index56.clone());
                                                            match source_ml_source1_s1217.clone() {
                                                                self::MlTemporaryPushJavaScriptCallLoop57Result::MlTemporaryPushJavaScriptCallLoop57Done(slots_2, argument_slots_2) => {
                                                                    if ((((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1219) => source_ml_source1_data_property_value1219.as_slice() }).len() as f64)) >= (1024f64)) || (slots_2 > (1048576f64))) || (argument_slots_2 > (1048576f64))) {
                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(match machine.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1220, _) => {
                                                                                let source_ml_source1_data_property_value1220 = *source_ml_source1_data_property_value1220;
                                                                                source_ml_source1_data_property_value1220.clone()
                                                                            }
                                                                        }), {
                                                                            let source_ml_source1_concat_value1222 = match machine.clone() {
                                                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1221) => {
                                                                                    source_ml_source1_data_property_value1221
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_concat_value1223 = vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_)];
                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), source_ml_source1_concat_value1222), source_ml_source1_concat_value1223)
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
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1224) => source_ml_source1_data_property_value1224.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1225) => source_ml_source1_data_property_value1225.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1226, _) => {
                    let source_ml_source1_data_property_value1226 = *source_ml_source1_data_property_value1226;
                    source_ml_source1_data_property_value1226.clone()
                }
            }), self::ml_string::Text::from_units(vec![107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
        } else {
            match transition.clone() {
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(state) => {
                    let state = *state;
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1227, _) => {
                            let source_ml_source1_data_property_value1227 = *source_ml_source1_data_property_value1227;
                            source_ml_source1_data_property_value1227.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1228) => {
                            source_ml_source1_data_property_value1228
                        }
                    }, {
                        let source_ml_source1_nominal_spread1229 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source1_nominal_field1230 = match source_ml_source1_nominal_spread1229.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1231, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1231.clone()
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1232 = match source_ml_source1_nominal_spread1229.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1233, _, _, _, _) => {
                                        source_ml_source1_data_property_value1233
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1234 = match source_ml_source1_nominal_spread1229.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1235, _, _, _) => {
                                            source_ml_source1_data_property_value1235
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1236 = match source_ml_source1_nominal_spread1229.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1237, _, _) => {
                                                source_ml_source1_data_property_value1237
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1238 = match source_ml_source1_nominal_spread1229.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1239, _) => {
                                                    source_ml_source1_data_property_value1239
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field1240 = match source_ml_source1_nominal_spread1229.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1241) => {
                                                        let source_ml_source1_data_property_value1241 = *source_ml_source1_data_property_value1241;
                                                        source_ml_source1_data_property_value1241.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override1242 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1230.clone(), source_ml_source1_nominal_field1232, source_ml_source1_nominal_field1234, source_ml_source1_nominal_field1236, source_ml_source1_nominal_field1238, Box::new(source_ml_source1_nominal_override1242.clone()))
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
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1243, _) => {
                            let source_ml_source1_data_property_value1243 = *source_ml_source1_data_property_value1243;
                            source_ml_source1_data_property_value1243.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1244) => {
                            source_ml_source1_data_property_value1244
                        }
                    }, {
                        let source_ml_source1_nominal_spread1245 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source1_nominal_field1246 = match source_ml_source1_nominal_spread1245.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1247, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1247.clone()
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1248 = match source_ml_source1_nominal_spread1245.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1249, _, _, _, _) => {
                                        source_ml_source1_data_property_value1249
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1250 = match source_ml_source1_nominal_spread1245.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1251, _, _, _) => {
                                            source_ml_source1_data_property_value1251
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1252 = match source_ml_source1_nominal_spread1245.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1253, _, _) => {
                                                source_ml_source1_data_property_value1253
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1254 = match source_ml_source1_nominal_spread1245.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1255, _) => {
                                                    source_ml_source1_data_property_value1255
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field1256 = match source_ml_source1_nominal_spread1245.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1257) => {
                                                        let source_ml_source1_data_property_value1257 = *source_ml_source1_data_property_value1257;
                                                        source_ml_source1_data_property_value1257.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override1258 = self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state_2.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1246.clone(), source_ml_source1_nominal_field1248, source_ml_source1_nominal_field1250, source_ml_source1_nominal_field1252, source_ml_source1_nominal_field1254, Box::new(source_ml_source1_nominal_override1258.clone()))
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
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1259, _) => {
                            let source_ml_source1_data_property_value1259 = *source_ml_source1_data_property_value1259;
                            source_ml_source1_data_property_value1259.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1260) => {
                            source_ml_source1_data_property_value1260
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
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1261, _) => {
                            let source_ml_source1_data_property_value1261 = *source_ml_source1_data_property_value1261;
                            source_ml_source1_data_property_value1261.clone()
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
        let frame = self::ml_array::optional_at((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1262) => source_ml_source1_data_property_value1262.as_slice() }), self::ml_array::number_index((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1263) => source_ml_source1_data_property_value1263.as_slice() }).len() as f64) - 1f64)));
        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1264, _) => {
                    let source_ml_source1_data_property_value1264 = *source_ml_source1_data_property_value1264;
                    source_ml_source1_data_property_value1264.clone()
                }
            }), self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
        } else {
            match transition.clone() {
                self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(state) => {
                    let state = *state;
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1266, _) => {
                            let source_ml_source1_data_property_value1266 = *source_ml_source1_data_property_value1266;
                            source_ml_source1_data_property_value1266.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1267) => {
                            source_ml_source1_data_property_value1267
                        }
                    }, {
                        let source_ml_source1_nominal_spread1268 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source1_nominal_field1269 = match source_ml_source1_nominal_spread1268.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1270, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1270.clone()
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1271 = match source_ml_source1_nominal_spread1268.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1272, _, _, _, _) => {
                                        source_ml_source1_data_property_value1272
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1273 = match source_ml_source1_nominal_spread1268.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1274, _, _, _) => {
                                            source_ml_source1_data_property_value1274
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1275 = match source_ml_source1_nominal_spread1268.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1276, _, _) => {
                                                source_ml_source1_data_property_value1276
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1277 = match source_ml_source1_nominal_spread1268.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1278, _) => {
                                                    source_ml_source1_data_property_value1278
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field1279 = match source_ml_source1_nominal_spread1268.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1280) => {
                                                        let source_ml_source1_data_property_value1280 = *source_ml_source1_data_property_value1280;
                                                        source_ml_source1_data_property_value1280.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override1281 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1269.clone(), source_ml_source1_nominal_field1271, source_ml_source1_nominal_field1273, source_ml_source1_nominal_field1275, source_ml_source1_nominal_field1277, Box::new(source_ml_source1_nominal_override1281.clone()))
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
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1282, _) => {
                            let source_ml_source1_data_property_value1282 = *source_ml_source1_data_property_value1282;
                            source_ml_source1_data_property_value1282.clone()
                        }
                    }, match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1283) => {
                            source_ml_source1_data_property_value1283
                        }
                    }, {
                        let source_ml_source1_nominal_spread1284 = frame.clone().expect("a proven present value");
                        {
                            let source_ml_source1_nominal_field1285 = match source_ml_source1_nominal_spread1284.clone() {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1286, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1286.clone()
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1287 = match source_ml_source1_nominal_spread1284.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1288, _, _, _, _) => {
                                        source_ml_source1_data_property_value1288
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1289 = match source_ml_source1_nominal_spread1284.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1290, _, _, _) => {
                                            source_ml_source1_data_property_value1290
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field1291 = match source_ml_source1_nominal_spread1284.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1292, _, _) => {
                                                source_ml_source1_data_property_value1292
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1293 = match source_ml_source1_nominal_spread1284.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1294, _) => {
                                                    source_ml_source1_data_property_value1294
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field1295 = match source_ml_source1_nominal_spread1284.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1296) => {
                                                        let source_ml_source1_data_property_value1296 = *source_ml_source1_data_property_value1296;
                                                        source_ml_source1_data_property_value1296.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override1297 = self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state_2.clone()));
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1285.clone(), source_ml_source1_nominal_field1287, source_ml_source1_nominal_field1289, source_ml_source1_nominal_field1291, source_ml_source1_nominal_field1293, Box::new(source_ml_source1_nominal_override1297.clone()))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }), callee.clone(), receiver.clone(), arguments)
                }
                self::JavaScriptPropertyDefinitionTransition::DefinitionCompleted(heap, value, source_ml_source1_f1265) => {
                    let heap = *heap;
                    let value = *value;
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1298) => {
                            source_ml_source1_data_property_value1298
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
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1299, _) => {
                            let source_ml_source1_data_property_value1299 = *source_ml_source1_data_property_value1299;
                            source_ml_source1_data_property_value1299.clone()
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
        let frames = self::ml_array::slice((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1301) => source_ml_source1_data_property_value1301.as_slice() }), 0f64, (-1f64));
        {
            let parent = self::ml_array::optional_at(&frames, self::ml_array::number_index(((frames.len() as f64) - 1f64)));
            if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                self::JavaScriptCallMachineTransition::MachineCompleted(Box::new(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1302, _) => {
                        let source_ml_source1_data_property_value1302 = *source_ml_source1_data_property_value1302;
                        source_ml_source1_data_property_value1302.clone()
                    }
                }), Box::new(value.clone()))
            } else {
                {
                    let current = self::JavaScriptCallMachine::CallMachine(Box::new(match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1303, _) => {
                            let source_ml_source1_data_property_value1303 = *source_ml_source1_data_property_value1303;
                            source_ml_source1_data_property_value1303.clone()
                        }
                    }), frames.clone());
                    {
                        let pending = match parent.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1304) => {
                                let source_ml_source1_data_property_value1304 = *source_ml_source1_data_property_value1304;
                                source_ml_source1_data_property_value1304.clone()
                            }
                        };
                        match pending.clone() {
                            self::JavaScriptCallPending::CallIdle => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1308, _) => {
                                        let source_ml_source1_data_property_value1308 = *source_ml_source1_data_property_value1308;
                                        source_ml_source1_data_property_value1308.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                            }
                            self::JavaScriptCallPending::WaitCall(destination) => {
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1309, _) => {
                                        let source_ml_source1_data_property_value1309 = *source_ml_source1_data_property_value1309;
                                        source_ml_source1_data_property_value1309.clone()
                                    }
                                }, frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination, value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitGet(destination_2) => {
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1310, _) => {
                                        let source_ml_source1_data_property_value1310 = *source_ml_source1_data_property_value1310;
                                        source_ml_source1_data_property_value1310.clone()
                                    }
                                }, frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination_2, value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitSet(destination_3) => {
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1311, _) => {
                                        let source_ml_source1_data_property_value1311 = *source_ml_source1_data_property_value1311;
                                        source_ml_source1_data_property_value1311.clone()
                                    }
                                }, frames, self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination_3, self::JavaScriptSemanticValue::Boolean(true)))))
                            }
                            self::JavaScriptCallPending::WaitKey(destination_4, realm, state) => {
                                let state = *state;
                                self::accept_java_script_call_key(program.clone(), current.clone(), destination_4, realm.clone(), self::resume_java_script_property_key_conversion(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1312, _) => {
                                        let source_ml_source1_data_property_value1312 = *source_ml_source1_data_property_value1312;
                                        source_ml_source1_data_property_value1312.clone()
                                    }
                                }, state.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitDefinition(destination_5, realm_2, state_2) => {
                                let state_2 = *state_2;
                                self::accept_java_script_call_definition(program.clone(), current.clone(), destination_5, realm_2.clone(), self::resume_java_script_property_definition(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1313, _) => {
                                        let source_ml_source1_data_property_value1313 = *source_ml_source1_data_property_value1313;
                                        source_ml_source1_data_property_value1313.clone()
                                    }
                                }, state_2.clone(), self::JavaScriptSemanticCompletion::NormalCompletion(Box::new(value.clone()))))
                            }
                            self::JavaScriptCallPending::WaitError(source_ml_source1_f1305, source_ml_source1_f1306, source_ml_source1_f1307) => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                    self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1314, _) => {
                                        let source_ml_source1_data_property_value1314 = *source_ml_source1_data_property_value1314;
                                        source_ml_source1_data_property_value1314.clone()
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
