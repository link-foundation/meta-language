// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_argument_list;
use super::lower_java_script_call_source_expression;
use super::lower_java_script_call_source_operand;
use super::ml;
use super::ml_string;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_constructor_expression_loop15;
use super::source_ml_source1_temporary_lower_java_script_call_source_constructor_expression_loop16;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_constructor_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(source_callee, arguments) => {
            let source_callee = *source_callee;
            {
                let snapshot = false;
                if { let ml_values14 = { let ml_reference_value15 = arguments.borrow().clone(); ml_reference_value15 }; let ml_callback14 = std::rc::Rc::new(move |argument: self::JavaScriptCallSource| -> bool { match argument.clone() {
                    self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                        true
                    }
                    _ => {
                        false
                    }
                } }); ml_values14.iter().cloned().enumerate().any(|(ml_index14, ml_value14)| ml_callback14(ml_value14)) } {
                    self::lower_java_script_call_source_argument_list(self::JavaScriptCallSource::SourceVariadicCall(Box::new(source_callee.clone()), Box::new(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Undefined))), arguments.clone(), self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114, 32, 99, 108, 97, 115, 115]), false), bindings.clone(), state.clone(), (depth + 1f64))
                } else {
                    if (({ let ml_reference_array16 = arguments.clone(); let ml_reference_projection16 = ml_reference_array16.borrow().len() as f64; ml_reference_projection16 }) > (4096f64)) {
                        self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 112, 101, 114, 97, 110, 100, 32, 98, 117, 100, 103, 101, 116])))
                    } else {
                        {
                            let callee = self::lower_java_script_call_source_operand(source_callee.clone(), bindings.clone(), state.clone(), (depth + 1f64), snapshot);
                            {
                                let current = match callee.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value2543, _) => {
                                        let source_ml_source2_data_property_value2543 = *source_ml_source2_data_property_value2543;
                                        source_ml_source2_data_property_value2543.clone()
                                    }
                                };
                                {
                                    let arguments_ = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                    {
                                        let source_ml_source1_temporary_index13 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s2544 = self::source_ml_source1_temporary_lower_java_script_call_source_constructor_expression_loop15(bindings.clone(), depth, snapshot, current.clone(), arguments_.clone(), arguments.clone(), source_ml_source1_temporary_index13.clone());
                                            match source_ml_source2_s2544.clone() {
                                                self::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Result::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Done(current_2, arguments__2) => {
                                                    let current_2 = *current_2;
                                                    {
                                                        let result = self::allocate_java_script_call_source_register(current_2.clone());
                                                        {
                                                            let source_ml_source2_nominal_field2735 = match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value2736, _) => {
                                                                    let source_ml_source2_data_property_value2736 = *source_ml_source2_data_property_value2736;
                                                                    source_ml_source2_data_property_value2736.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field2737 = match result.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value2738) => {
                                                                        source_ml_source2_data_property_value2738
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override2739 = self::append_java_script_call_source_instruction(match result.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value2545, _) => {
                                                                            let source_ml_source2_data_property_value2545 = *source_ml_source2_data_property_value2545;
                                                                            source_ml_source2_data_property_value2545.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::ConstructErrorClass(match result.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value2732) => {
                                                                            source_ml_source2_data_property_value2732
                                                                        }
                                                                    }, match callee.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value2733) => {
                                                                            source_ml_source2_data_property_value2733
                                                                        }
                                                                    }, arguments__2.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override2739.clone()), source_ml_source2_nominal_field2737)
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
        self::JavaScriptCallSource::SourceClassConstructorCall(source_callee_2, source_receiver, arguments_2) => {
            let source_callee_2 = *source_callee_2;
            let source_receiver = *source_receiver;
            if { let ml_values17 = { let ml_reference_value18 = arguments_2.borrow().clone(); ml_reference_value18 }; let ml_callback17 = std::rc::Rc::new(move |argument: self::JavaScriptCallSource| -> bool { match argument.clone() {
                self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                    true
                }
                _ => {
                    false
                }
            } }); ml_values17.iter().cloned().enumerate().any(|(ml_index17, ml_value17)| ml_callback17(ml_value17)) } {
                self::lower_java_script_call_source_argument_list(self::JavaScriptCallSource::SourceVariadicCall(Box::new(source_callee_2.clone()), Box::new(source_receiver.clone()), arguments_2.clone(), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), false), bindings.clone(), state.clone(), (depth + 1f64))
            } else {
                if (({ let ml_reference_array19 = arguments_2.clone(); let ml_reference_projection19 = ml_reference_array19.borrow().len() as f64; ml_reference_projection19 }) > (4096f64)) {
                    self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 112, 101, 114, 97, 110, 100, 32, 98, 117, 100, 103, 101, 116])))
                } else {
                    {
                        let selected = self::lower_java_script_call_source_expression(source_callee_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                        {
                            let callee_2 = self::allocate_java_script_call_source_register(match selected.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value2924, _) => {
                                    let source_ml_source2_data_property_value2924 = *source_ml_source2_data_property_value2924;
                                    source_ml_source2_data_property_value2924.clone()
                                }
                            });
                            {
                                let held_callee = self::append_java_script_call_source_instruction(match callee_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value2925, _) => {
                                        let source_ml_source2_data_property_value2925 = *source_ml_source2_data_property_value2925;
                                        source_ml_source2_data_property_value2925.clone()
                                    }
                                }, self::JavaScriptCallInstruction::Copy(match callee_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value2926) => {
                                        source_ml_source2_data_property_value2926
                                    }
                                }, match selected.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value2927) => {
                                        source_ml_source2_data_property_value2927
                                    }
                                }));
                                {
                                    let raw_receiver = self::lower_java_script_call_source_expression(source_receiver.clone(), bindings.clone(), held_callee.clone(), (depth + 1f64));
                                    {
                                        let receiver = self::allocate_java_script_call_source_register(match raw_receiver.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value2928, _) => {
                                                let source_ml_source2_data_property_value2928 = *source_ml_source2_data_property_value2928;
                                                source_ml_source2_data_property_value2928.clone()
                                            }
                                        });
                                        {
                                            let current_3 = self::append_java_script_call_source_instruction(match receiver.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value2929, _) => {
                                                    let source_ml_source2_data_property_value2929 = *source_ml_source2_data_property_value2929;
                                                    source_ml_source2_data_property_value2929.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Copy(match receiver.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value2930) => {
                                                    source_ml_source2_data_property_value2930
                                                }
                                            }, match raw_receiver.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value2931) => {
                                                    source_ml_source2_data_property_value2931
                                                }
                                            }));
                                            {
                                                let arguments__3 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                {
                                                    let source_ml_source1_temporary_index14 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s2932 = self::source_ml_source1_temporary_lower_java_script_call_source_constructor_expression_loop16(bindings.clone(), depth, current_3.clone(), arguments__3.clone(), arguments_2.clone(), source_ml_source1_temporary_index14.clone());
                                                        match source_ml_source2_s2932.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Result::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Done(current_4, arguments__4) => {
                                                                let current_4 = *current_4;
                                                                {
                                                                    let result_2 = self::allocate_java_script_call_source_register(current_4.clone());
                                                                    {
                                                                        let source_ml_source2_nominal_field2938 = match result_2.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value2939, _) => {
                                                                                let source_ml_source2_data_property_value2939 = *source_ml_source2_data_property_value2939;
                                                                                source_ml_source2_data_property_value2939.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field2940 = match result_2.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value2941) => {
                                                                                    source_ml_source2_data_property_value2941
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override2942 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value2933, _) => {
                                                                                        let source_ml_source2_data_property_value2933 = *source_ml_source2_data_property_value2933;
                                                                                        source_ml_source2_data_property_value2933.clone()
                                                                                    }
                                                                                }, self::JavaScriptCallInstruction::CallClassConstructor(match result_2.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value2934) => {
                                                                                        source_ml_source2_data_property_value2934
                                                                                    }
                                                                                }, match callee_2.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value2935) => {
                                                                                        source_ml_source2_data_property_value2935
                                                                                    }
                                                                                }, match receiver.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value2936) => {
                                                                                        source_ml_source2_data_property_value2936
                                                                                    }
                                                                                }, arguments__4.clone()));
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override2942.clone()), source_ml_source2_nominal_field2940)
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
        self::JavaScriptCallSource::SourceConstructCollection(source_callee_3, arguments_3) => {
            let source_callee_3 = *source_callee_3;
            {
                let snapshot_2 = true;
                if { let ml_values20 = { let ml_reference_value21 = arguments_3.borrow().clone(); ml_reference_value21 }; let ml_callback20 = std::rc::Rc::new(move |argument: self::JavaScriptCallSource| -> bool { match argument.clone() {
                    self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                        true
                    }
                    _ => {
                        false
                    }
                } }); ml_values20.iter().cloned().enumerate().any(|(ml_index20, ml_value20)| ml_callback20(ml_value20)) } {
                    self::lower_java_script_call_source_argument_list(self::JavaScriptCallSource::SourceVariadicCall(Box::new(source_callee_3.clone()), Box::new(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Undefined))), arguments_3.clone(), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110]), false), bindings.clone(), state.clone(), (depth + 1f64))
                } else {
                    if (({ let ml_reference_array22 = arguments_3.clone(); let ml_reference_projection22 = ml_reference_array22.borrow().len() as f64; ml_reference_projection22 }) > (4096f64)) {
                        self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 112, 101, 114, 97, 110, 100, 32, 98, 117, 100, 103, 101, 116])))
                    } else {
                        {
                            let callee_3 = self::lower_java_script_call_source_operand(source_callee_3.clone(), bindings.clone(), state.clone(), (depth + 1f64), snapshot_2);
                            {
                                let current_5 = match callee_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3495, _) => {
                                        let source_ml_source2_data_property_value3495 = *source_ml_source2_data_property_value3495;
                                        source_ml_source2_data_property_value3495.clone()
                                    }
                                };
                                {
                                    let arguments__5 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                    {
                                        let source_ml_source1_temporary_index13_2 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s3496 = self::source_ml_source1_temporary_lower_java_script_call_source_constructor_expression_loop15(bindings.clone(), depth, snapshot_2, current_5.clone(), arguments__5.clone(), arguments_3.clone(), source_ml_source1_temporary_index13_2.clone());
                                            match source_ml_source2_s3496.clone() {
                                                self::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Result::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Done(current_6, arguments__6) => {
                                                    let current_6 = *current_6;
                                                    {
                                                        let result_3 = self::allocate_java_script_call_source_register(current_6.clone());
                                                        {
                                                            let source_ml_source2_nominal_field3687 = match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3688, _) => {
                                                                    let source_ml_source2_data_property_value3688 = *source_ml_source2_data_property_value3688;
                                                                    source_ml_source2_data_property_value3688.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field3689 = match result_3.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3690) => {
                                                                        source_ml_source2_data_property_value3690
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override3691 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3497, _) => {
                                                                            let source_ml_source2_data_property_value3497 = *source_ml_source2_data_property_value3497;
                                                                            source_ml_source2_data_property_value3497.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::ConstructCollection(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3682) => {
                                                                            source_ml_source2_data_property_value3682
                                                                        }
                                                                    }, match callee_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3683) => {
                                                                            source_ml_source2_data_property_value3683
                                                                        }
                                                                    }, arguments__6.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override3691.clone()), source_ml_source2_nominal_field3689)
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
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(source_constructor, source_new_target, source_prototype, source_message, source_present, realm, error_class, bind_receiver) => {
            let source_constructor = *source_constructor;
            let source_new_target = *source_new_target;
            let source_prototype = *source_prototype;
            let source_message = *source_message;
            let source_present = *source_present;
            if ((realm <= (self::ml::Big::from_i128(0))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109])))
            } else {
                {
                    let constructor = self::lower_java_script_call_source_expression(source_constructor.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let new_target = self::lower_java_script_call_source_expression(source_new_target.clone(), bindings.clone(), match constructor.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3692, _) => {
                                let source_ml_source2_data_property_value3692 = *source_ml_source2_data_property_value3692;
                                source_ml_source2_data_property_value3692.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let prototype = self::lower_java_script_call_source_expression(source_prototype.clone(), bindings.clone(), match new_target.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3693, _) => {
                                    let source_ml_source2_data_property_value3693 = *source_ml_source2_data_property_value3693;
                                    source_ml_source2_data_property_value3693.clone()
                                }
                            }, (depth + 1f64));
                            {
                                let message = self::lower_java_script_call_source_expression(source_message.clone(), bindings.clone(), match prototype.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3694, _) => {
                                        let source_ml_source2_data_property_value3694 = *source_ml_source2_data_property_value3694;
                                        source_ml_source2_data_property_value3694.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let present = self::lower_java_script_call_source_expression(source_present.clone(), bindings.clone(), match message.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3695, _) => {
                                            let source_ml_source2_data_property_value3695 = *source_ml_source2_data_property_value3695;
                                            source_ml_source2_data_property_value3695.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let result_4 = self::allocate_java_script_call_source_register(match present.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3696, _) => {
                                                let source_ml_source2_data_property_value3696 = *source_ml_source2_data_property_value3696;
                                                source_ml_source2_data_property_value3696.clone()
                                            }
                                        });
                                        {
                                            let source_ml_source2_nominal_field3705 = match result_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3706, _) => {
                                                    let source_ml_source2_data_property_value3706 = *source_ml_source2_data_property_value3706;
                                                    source_ml_source2_data_property_value3706.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field3707 = match result_4.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3708) => {
                                                        source_ml_source2_data_property_value3708
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override3709 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3697, _) => {
                                                            let source_ml_source2_data_property_value3697 = *source_ml_source2_data_property_value3697;
                                                            source_ml_source2_data_property_value3697.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::ConstructErrorValue(match result_4.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3698) => {
                                                            source_ml_source2_data_property_value3698
                                                        }
                                                    }, match constructor.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3699) => {
                                                            source_ml_source2_data_property_value3699
                                                        }
                                                    }, match new_target.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3700) => {
                                                            source_ml_source2_data_property_value3700
                                                        }
                                                    }, match prototype.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3701) => {
                                                            source_ml_source2_data_property_value3701
                                                        }
                                                    }, match message.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3702) => {
                                                            source_ml_source2_data_property_value3702
                                                        }
                                                    }, match present.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3703) => {
                                                            source_ml_source2_data_property_value3703
                                                        }
                                                    }, realm.clone(), error_class.clone(), bind_receiver));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override3709.clone()), source_ml_source2_nominal_field3707)
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
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}
