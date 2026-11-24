// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::refuse_java_script_call_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_binding_write_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceAssignBinding(identity, source_value) => {
            let source_value = *source_value;
            if {
                let source_ml_source2_comparison_operand4103 = match state.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4102, _, _, _) => {
                        ((source_ml_source2_data_property_value4102.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                };
                !(match (source_ml_source2_comparison_operand4103.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
            } {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![109, 117, 116, 97, 98, 108, 101, 32, 115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 99, 111, 109, 112, 105, 108, 97, 116, 105, 111, 110, 32, 112, 97, 115, 115])))
            } else {
                {
                    let binding = { let ml_values40 = { let ml_reference_value41 = bindings.borrow().clone(); ml_reference_value41 }; let ml_callback40 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4106, _, _, _) => {
                            source_ml_source2_data_property_value4106
                        }
                    }) == identity) }) }; ml_values40.iter().cloned().enumerate().find_map(|(ml_index40, ml_value40)| if ml_callback40(ml_value40.clone()) { Some(ml_value40) } else { None }) };
                    if (((match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match binding.clone().expect("a proven present value") {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4108, _, _) => {
                            source_ml_source2_data_property_value4108
                        }
                    }) == (0f64))) || !(match binding.clone().expect("a proven present value") {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, source_ml_source2_data_property_value4109, _) => {
                            source_ml_source2_data_property_value4109
                        }
                    })) || ({
                        let source_ml_source2_comparison_operand4111 = match binding.clone().expect("a proven present value") {
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, _, source_ml_source2_data_property_value4110) => {
                                ((source_ml_source2_data_property_value4110.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                            }
                        };
                        match (source_ml_source2_comparison_operand4111.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                    } && !(self::ml_array::includes(&{ let ml_reference_value42 = ((match state.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4114, _, _) => {
                            ((source_ml_source2_data_property_value4114.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                        }
                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value42 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))))) {
                        self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 109, 117, 116, 97, 98, 108, 101, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 108, 111, 99, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114])))
                    } else {
                        {
                            let value = self::lower_java_script_call_source_expression(source_value.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let source_ml_source2_nominal_field4119 = match value.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4120, _) => {
                                        let source_ml_source2_data_property_value4120 = *source_ml_source2_data_property_value4120;
                                        source_ml_source2_data_property_value4120.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field4121 = match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4122) => {
                                            source_ml_source2_data_property_value4122
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override4123 = self::append_java_script_call_source_instruction(match value.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4115, _) => {
                                                let source_ml_source2_data_property_value4115 = *source_ml_source2_data_property_value4115;
                                                source_ml_source2_data_property_value4115.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match binding.clone().expect("a proven present value") {
                                            self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4116, _, _) => {
                                                source_ml_source2_data_property_value4116
                                            }
                                        }, match value.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4117) => {
                                                source_ml_source2_data_property_value4117
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override4123.clone()), source_ml_source2_nominal_field4121)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(source_value_2) => {
            let source_value_2 = *source_value_2;
            {
                let value_2 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let message = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![65f64, 115f64, 115f64, 105f64, 103f64, 110f64, 109f64, 101f64, 110f64, 116f64, 32f64, 116f64, 111f64, 32f64, 99f64, 111f64, 110f64, 115f64, 116f64, 97f64, 110f64, 116f64, 32f64, 118f64, 97f64, 114f64, 105f64, 97f64, 98f64, 108f64, 101f64, 46f64]))))), bindings.clone(), match value_2.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4124, _) => {
                            let source_ml_source2_data_property_value4124 = *source_ml_source2_data_property_value4124;
                            source_ml_source2_data_property_value4124.clone()
                        }
                    }, (depth + 1f64));
                    {
                        let present = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Boolean(true))), bindings.clone(), match message.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4125, _) => {
                                let source_ml_source2_data_property_value4125 = *source_ml_source2_data_property_value4125;
                                source_ml_source2_data_property_value4125.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let result = self::allocate_java_script_call_source_register(match present.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4126, _) => {
                                    let source_ml_source2_data_property_value4126 = *source_ml_source2_data_property_value4126;
                                    source_ml_source2_data_property_value4126.clone()
                                }
                            });
                            {
                                let source_ml_source2_nominal_field4132 = match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4133, _) => {
                                        let source_ml_source2_data_property_value4133 = *source_ml_source2_data_property_value4133;
                                        source_ml_source2_data_property_value4133.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field4134 = match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4135) => {
                                            source_ml_source2_data_property_value4135
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override4136 = self::append_java_script_call_source_instruction(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4127, _) => {
                                                let source_ml_source2_data_property_value4127 = *source_ml_source2_data_property_value4127;
                                                source_ml_source2_data_property_value4127.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Primitive(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4128) => {
                                                source_ml_source2_data_property_value4128
                                            }
                                        }, self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112, 101, 69, 114, 114, 111, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![match message.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4129) => {
                                                source_ml_source2_data_property_value4129
                                            }
                                        }, match present.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4130) => {
                                                source_ml_source2_data_property_value4130
                                            }
                                        }]))));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override4136.clone()), source_ml_source2_nominal_field4134)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(identity_2, realm, lexical) => {
            {
                let binding_2 = { let ml_values43 = { let ml_reference_value44 = bindings.borrow().clone(); ml_reference_value44 }; let ml_callback43 = { let identity_2 = identity_2; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4138, _, _, _) => {
                        source_ml_source2_data_property_value4138
                    }
                }) == identity_2) }) }; ml_values43.iter().cloned().enumerate().find_map(|(ml_index43, ml_value43)| if ml_callback43(ml_value43.clone()) { Some(ml_value43) } else { None }) };
                if ((((({
                    let source_ml_source2_comparison_operand4140 = match state.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4139, _, _, _) => {
                            ((source_ml_source2_data_property_value4139.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                        }
                    };
                    !(match (source_ml_source2_comparison_operand4140.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                } || match (binding_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || ((match binding_2.clone().expect("a proven present value") {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4143, _, _) => {
                        source_ml_source2_data_property_value4143
                    }
                }) == (0f64))) || {
                    let source_ml_source2_comparison_operand4145 = match binding_2.clone().expect("a proven present value") {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, _, source_ml_source2_data_property_value4144) => {
                            ((source_ml_source2_data_property_value4144.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                        }
                    };
                    match (source_ml_source2_comparison_operand4145.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                }) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 99, 108, 111, 110, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 108, 101, 120, 105, 99, 97, 108, 32, 99, 101, 108, 108, 32, 97, 110, 100, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109])))
                } else {
                    {
                        let cell = {
                            let source_ml_source2_constructor_field4147 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Null));
                            {
                                let source_ml_source2_constructor_field4157 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![{
                                    let source_ml_source2_constructor_field4149 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![118f64, 97f64, 108f64, 117f64, 101f64])))));
                                    {
                                        let source_ml_source2_constructor_field4152 = {
                                            let source_ml_source2_constructor_field4150 = self::JavaScriptCallSource::SourceBinding(identity_2);
                                            {
                                                let source_ml_source2_constructor_field4151 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![118f64, 97f64, 108f64, 117f64, 101f64])))));
                                                self::JavaScriptCallSource::SourceGet(Box::new(source_ml_source2_constructor_field4150.clone()), Box::new(source_ml_source2_constructor_field4151.clone()), None::<(Option<self::JavaScriptCallSource>,)>)
                                            }
                                        };
                                        self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field4149.clone()), Box::new(source_ml_source2_constructor_field4152.clone()), None::<(Option<bool>,)>)
                                    }
                                }], { let ml_reference_value45 = (if match ((lexical.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                        let source_ml_source2_constructor_field4153 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![105f64, 110f64, 105f64, 116f64, 105f64, 97f64, 108f64, 105f64, 122f64, 101f64, 100f64])))));
                                        {
                                            let source_ml_source2_constructor_field4156 = {
                                                let source_ml_source2_constructor_field4154 = self::JavaScriptCallSource::SourceBinding(identity_2);
                                                {
                                                    let source_ml_source2_constructor_field4155 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![105f64, 110f64, 105f64, 116f64, 105f64, 97f64, 108f64, 105f64, 122f64, 101f64, 100f64])))));
                                                    self::JavaScriptCallSource::SourceGet(Box::new(source_ml_source2_constructor_field4154.clone()), Box::new(source_ml_source2_constructor_field4155.clone()), None::<(Option<self::JavaScriptCallSource>,)>)
                                                }
                                            };
                                            self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field4153.clone()), Box::new(source_ml_source2_constructor_field4156.clone()), None::<(Option<bool>,)>)
                                        }
                                    }]))
                                } else {
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceProperty>::new()))
                                }).borrow().clone(); ml_reference_value45 })));
                                self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field4147.clone()), source_ml_source2_constructor_field4157.clone(), realm.clone())
                            }
                        };
                        {
                            let copied = self::lower_java_script_call_source_expression(cell.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let source_ml_source2_nominal_field4162 = match copied.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4163, _) => {
                                        let source_ml_source2_data_property_value4163 = *source_ml_source2_data_property_value4163;
                                        source_ml_source2_data_property_value4163.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field4164 = match copied.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4165) => {
                                            source_ml_source2_data_property_value4165
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override4166 = self::append_java_script_call_source_instruction(match copied.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4158, _) => {
                                                let source_ml_source2_data_property_value4158 = *source_ml_source2_data_property_value4158;
                                                source_ml_source2_data_property_value4158.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match binding_2.clone().expect("a proven present value") {
                                            self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4159, _, _) => {
                                                source_ml_source2_data_property_value4159
                                            }
                                        }, match copied.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4160) => {
                                                source_ml_source2_data_property_value4160
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override4166.clone()), source_ml_source2_nominal_field4164)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceWriteExisting(identity_3, cell_2, source_value_3, body) => {
            let source_value_3 = *source_value_3;
            let body = *body;
            {
                let binding_3 = { let ml_values46 = { let ml_reference_value47 = bindings.borrow().clone(); ml_reference_value47 }; let ml_callback46 = { let identity_3 = identity_3; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4168, _, _, _) => {
                        source_ml_source2_data_property_value4168
                    }
                }) == identity_3) }) }; ml_values46.iter().cloned().enumerate().find_map(|(ml_index46, ml_value46)| if ml_callback46(ml_value46.clone()) { Some(ml_value46) } else { None }) };
                if ((({
                    let source_ml_source2_comparison_operand4170 = match state.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4169, _, _, _) => {
                            ((source_ml_source2_data_property_value4169.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                        }
                    };
                    !(match (source_ml_source2_comparison_operand4170.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                } || match (binding_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || ((match binding_3.clone().expect("a proven present value") {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4173, _, _) => {
                        source_ml_source2_data_property_value4173
                    }
                }) == (0f64))) || (!cell_2 && !(match binding_3.clone().expect("a proven present value") {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, source_ml_source2_data_property_value4174, _) => {
                        source_ml_source2_data_property_value4174
                    }
                }))) {
                    self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![101, 120, 105, 115, 116, 105, 110, 103, 32, 98, 105, 110, 100, 105, 110, 103, 32, 119, 114, 105, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 111, 114, 32, 112, 114, 105, 118, 97, 116, 101, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 99, 101, 108, 108])))
                } else {
                    {
                        let value_3 = self::lower_java_script_call_source_expression(source_value_3.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                        {
                            let held = self::allocate_java_script_call_source_register(match value_3.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4175, _) => {
                                    let source_ml_source2_data_property_value4175 = *source_ml_source2_data_property_value4175;
                                    source_ml_source2_data_property_value4175.clone()
                                }
                            });
                            {
                                let current = self::append_java_script_call_source_instruction(match held.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4176, _) => {
                                        let source_ml_source2_data_property_value4176 = *source_ml_source2_data_property_value4176;
                                        source_ml_source2_data_property_value4176.clone()
                                    }
                                }, self::JavaScriptCallInstruction::Copy(match held.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4177) => {
                                        source_ml_source2_data_property_value4177
                                    }
                                }, match value_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4178) => {
                                        source_ml_source2_data_property_value4178
                                    }
                                }));
                                {
                                    let current_6 = if cell_2 {
                                        {
                                            let key = self::allocate_java_script_call_source_register(current.clone());
                                            {
                                                let written = self::allocate_java_script_call_source_register(match key.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4179, _) => {
                                                        let source_ml_source2_data_property_value4179 = *source_ml_source2_data_property_value4179;
                                                        source_ml_source2_data_property_value4179.clone()
                                                    }
                                                });
                                                {
                                                    let current_2 = self::append_java_script_call_source_instruction(match written.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4180, _) => {
                                                            let source_ml_source2_data_property_value4180 = *source_ml_source2_data_property_value4180;
                                                            source_ml_source2_data_property_value4180.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Constant(match key.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4181) => {
                                                            source_ml_source2_data_property_value4181
                                                        }
                                                    }, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![118f64, 97f64, 108f64, 117f64, 101f64]))))));
                                                    {
                                                        let current_3 = self::append_java_script_call_source_instruction(current_2.clone(), self::JavaScriptCallInstruction::OrdinarySet(match written.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4182) => {
                                                                source_ml_source2_data_property_value4182
                                                            }
                                                        }, match binding_3.clone().expect("a proven present value") {
                                                            self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4183, _, _) => {
                                                                source_ml_source2_data_property_value4183
                                                            }
                                                        }, match key.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4184) => {
                                                                source_ml_source2_data_property_value4184
                                                            }
                                                        }, match held.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4185) => {
                                                                source_ml_source2_data_property_value4185
                                                            }
                                                        }, match binding_3.clone().expect("a proven present value") {
                                                            self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4186, _, _) => {
                                                                source_ml_source2_data_property_value4186
                                                            }
                                                        }));
                                                        {
                                                            let current_4 = self::append_java_script_call_source_instruction(current_3.clone(), self::JavaScriptCallInstruction::RequireWrite(match written.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4187) => {
                                                                    source_ml_source2_data_property_value4187
                                                                }
                                                            }));
                                                            current_4.clone()
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let current_5 = self::append_java_script_call_source_instruction(current.clone(), self::JavaScriptCallInstruction::Copy(match binding_3.clone().expect("a proven present value") {
                                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4188, _, _) => {
                                                    source_ml_source2_data_property_value4188
                                                }
                                            }, match held.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4189) => {
                                                    source_ml_source2_data_property_value4189
                                                }
                                            }));
                                            current_5.clone()
                                        }
                                    };
                                    self::lower_java_script_call_source_expression(body.clone(), bindings.clone(), current_6.clone(), (depth + 1f64))
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInitializeBinding(identity_4, source_value_4, body_2) => {
            let source_value_4 = *source_value_4;
            let body_2 = *body_2;
            {
                let binding_4 = { let ml_values48 = { let ml_reference_value49 = bindings.borrow().clone(); ml_reference_value49 }; let ml_callback48 = { let identity_4 = identity_4; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4191, _, _, _) => {
                        source_ml_source2_data_property_value4191
                    }
                }) == identity_4) }) }; ml_values48.iter().cloned().enumerate().find_map(|(ml_index48, ml_value48)| if ml_callback48(ml_value48.clone()) { Some(ml_value48) } else { None }) };
                if ((match (binding_4.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || {
                    let source_ml_source2_comparison_operand4193 = match binding_4.clone().expect("a proven present value") {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, _, source_ml_source2_data_property_value4192) => {
                            ((source_ml_source2_data_property_value4192.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                        }
                    };
                    !(match (source_ml_source2_comparison_operand4193.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }) || self::ml_array::includes(&{ let ml_reference_value50 = ((match state.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4196, _, _) => {
                        ((source_ml_source2_data_property_value4196.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value50 }, &identity_4, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                    self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 111, 119, 110, 101, 100, 32, 114, 101, 103, 105, 115, 116, 101, 114])))
                } else {
                    {
                        let value_4 = self::lower_java_script_call_source_expression(source_value_4.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                        {
                            let copied_2 = self::append_java_script_call_source_instruction(match value_4.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4197, _) => {
                                    let source_ml_source2_data_property_value4197 = *source_ml_source2_data_property_value4197;
                                    source_ml_source2_data_property_value4197.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match binding_4.clone().expect("a proven present value") {
                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4198, _, _) => {
                                    source_ml_source2_data_property_value4198
                                }
                            }, match value_4.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4199) => {
                                    source_ml_source2_data_property_value4199
                                }
                            }));
                            self::lower_java_script_call_source_expression(body_2.clone(), bindings.clone(), {
                                let source_ml_source2_nominal_field4204 = match copied_2.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value4205, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value4205.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field4206 = match copied_2.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value4207, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value4207
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field4208 = match copied_2.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value4209, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value4209
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field4210 = match copied_2.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value4211, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value4211
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field4212 = match copied_2.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value4213, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value4213
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field4214 = match copied_2.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value4215, _, _, _, _) => {
                                                            source_ml_source2_data_property_value4215.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field4216 = match copied_2.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4217, _, _, _) => {
                                                                source_ml_source2_data_property_value4217.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field4218 = match copied_2.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4219, _, _) => {
                                                                    source_ml_source2_data_property_value4219.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field4220 = match copied_2.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value4221, _) => {
                                                                        source_ml_source2_data_property_value4221.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field4222 = match copied_2.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4223) => {
                                                                            source_ml_source2_data_property_value4223.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_override4224 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                            let source_ml_source2_concat_value4201 = (match copied_2.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4200, _, _) => {
                                                                                    ((source_ml_source2_data_property_value4200.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                }
                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                                                            {
                                                                                let source_ml_source2_concat_value4202 = std::rc::Rc::new(std::cell::RefCell::new(vec![identity_4]));
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value51 = source_ml_source2_concat_value4201.borrow().clone(); ml_reference_value51 }), { let ml_reference_value52 = source_ml_source2_concat_value4202.borrow().clone(); ml_reference_value52 })
                                                                            }
                                                                        }));
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field4204.clone(), source_ml_source2_nominal_field4206, source_ml_source2_nominal_field4208, source_ml_source2_nominal_field4210, source_ml_source2_nominal_field4212, source_ml_source2_nominal_field4214.clone(), source_ml_source2_nominal_field4216.clone(), Some((Some(source_ml_source2_nominal_override4224.clone()),)), source_ml_source2_nominal_field4220.clone(), source_ml_source2_nominal_field4222.clone())
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
                            }, (depth + 1f64))
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}
