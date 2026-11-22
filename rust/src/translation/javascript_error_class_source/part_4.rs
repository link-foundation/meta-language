// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallCellInitializer;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_argument_list;
use super::lower_java_script_call_source_expression;
use super::lower_java_script_call_source_operand;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_constructor_expression_loop15;
use super::source_ml_source1_temporary_lower_java_script_call_source_constructor_expression_loop16;
use super::source_ml_source1_temporary_lower_java_script_call_source_lexical_cell_expression_loop17;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_constructor_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstructErrorClass(source_callee, arguments) => {
            let source_callee = *source_callee;
            {
                let snapshot = false;
                if { let ml_values15 = { let ml_reference_value16 = arguments.borrow().clone(); ml_reference_value16 }; let ml_callback15 = std::rc::Rc::new(move |argument: self::JavaScriptCallSource| -> bool { match argument.clone() {
                    self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                        true
                    }
                    _ => {
                        false
                    }
                } }); ml_values15.iter().cloned().enumerate().any(|(ml_index15, ml_value15)| ml_callback15(ml_value15)) } {
                    self::lower_java_script_call_source_argument_list(self::JavaScriptCallSource::SourceVariadicCall(Box::new(source_callee.clone()), Box::new(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Undefined))), arguments.clone(), self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114, 32, 99, 108, 97, 115, 115]), false), bindings.clone(), state.clone(), (depth + 1f64))
                } else {
                    if (({ let ml_reference_array17 = arguments.clone(); let ml_reference_projection17 = ml_reference_array17.borrow().len() as f64; ml_reference_projection17 }) > (4096f64)) {
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
            if { let ml_values18 = { let ml_reference_value19 = arguments_2.borrow().clone(); ml_reference_value19 }; let ml_callback18 = std::rc::Rc::new(move |argument: self::JavaScriptCallSource| -> bool { match argument.clone() {
                self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                    true
                }
                _ => {
                    false
                }
            } }); ml_values18.iter().cloned().enumerate().any(|(ml_index18, ml_value18)| ml_callback18(ml_value18)) } {
                self::lower_java_script_call_source_argument_list(self::JavaScriptCallSource::SourceVariadicCall(Box::new(source_callee_2.clone()), Box::new(source_receiver.clone()), arguments_2.clone(), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), false), bindings.clone(), state.clone(), (depth + 1f64))
            } else {
                if (({ let ml_reference_array20 = arguments_2.clone(); let ml_reference_projection20 = ml_reference_array20.borrow().len() as f64; ml_reference_projection20 }) > (4096f64)) {
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
                if { let ml_values21 = { let ml_reference_value22 = arguments_3.borrow().clone(); ml_reference_value22 }; let ml_callback21 = std::rc::Rc::new(move |argument: self::JavaScriptCallSource| -> bool { match argument.clone() {
                    self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                        true
                    }
                    _ => {
                        false
                    }
                } }); ml_values21.iter().cloned().enumerate().any(|(ml_index21, ml_value21)| ml_callback21(ml_value21)) } {
                    self::lower_java_script_call_source_argument_list(self::JavaScriptCallSource::SourceVariadicCall(Box::new(source_callee_3.clone()), Box::new(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Undefined))), arguments_3.clone(), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110]), false), bindings.clone(), state.clone(), (depth + 1f64))
                } else {
                    if (({ let ml_reference_array23 = arguments_3.clone(); let ml_reference_projection23 = ml_reference_array23.borrow().len() as f64; ml_reference_projection23 }) > (4096f64)) {
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
        _ => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_lexical_cell_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceLexicalScope(identities, source_existing, source_initializers, realm, body) => {
            let body = *body;
            {
                let existing = (((source_existing.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                if ((((((({ let ml_reference_array24 = identities.clone(); let ml_reference_projection24 = ml_reference_array24.borrow().len() as f64; ml_reference_projection24 }) > (4096f64)) || (({ let ml_reference_array25 = existing.clone(); let ml_reference_projection25 = ml_reference_array25.borrow().len() as f64; ml_reference_projection25 }) > ({ let ml_reference_array26 = identities.clone(); let ml_reference_projection26 = ml_reference_array26.borrow().len() as f64; ml_reference_projection26 }))) || ((({ let ml_reference_array27 = existing.clone(); let ml_reference_projection27 = ml_reference_array27.borrow().len() as f64; ml_reference_projection27 }) > (0f64)) && {
                    let source_ml_source2_comparison_operand3887 = match state.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value3886, _, _, _) => {
                            ((source_ml_source2_data_property_value3886.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                        }
                    };
                    !(match (source_ml_source2_comparison_operand3887.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                })) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) || { let ml_values28 = { let ml_reference_value29 = existing.borrow().clone(); ml_reference_value29 }; let ml_callback28 = { let identities = identities.clone(); let existing = existing.clone(); std::rc::Rc::new(move |identity: f64, index: f64| -> bool { (!(self::ml_array::includes(&{ let ml_reference_value30 = identities.borrow().clone(); ml_reference_value30 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) || ((self::ml_array::index_of(&{ let ml_reference_value31 = existing.borrow().clone(); ml_reference_value31 }, &identity, 0f64, false, |a, b| a == b)) != index)) }) }; ml_values28.iter().cloned().enumerate().any(|(ml_index28, ml_value28)| ml_callback28(ml_value28, (ml_index28 as f64))) }) {
                    self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 115, 99, 111, 112, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 98, 111, 117, 110, 100, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 99, 101, 108, 108, 115, 32, 97, 110, 100, 32, 102, 111, 114, 109, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 115])))
                } else {
                    {
                        let initializers = (((source_initializers.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallCellInitializer>::new())));
                        if ((({ let ml_reference_array32 = initializers.clone(); let ml_reference_projection32 = ml_reference_array32.borrow().len() as f64; ml_reference_projection32 }) > ({ let ml_reference_array33 = identities.clone(); let ml_reference_projection33 = ml_reference_array33.borrow().len() as f64; ml_reference_projection33 })) || { let ml_values34 = { let ml_reference_value35 = initializers.borrow().clone(); ml_reference_value35 }; let ml_callback34 = { let identities = identities.clone(); let initializers = initializers.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallCellInitializer, index: f64| -> bool { (((match row.clone() {
                            self::JavaScriptCallCellInitializer::SourceCellInitializer(_, _, _) => {
                                false
                            }
                        } || !(self::ml_array::includes(&{ let ml_reference_value36 = identities.borrow().clone(); ml_reference_value36 }, &match row.clone() {
                            self::JavaScriptCallCellInitializer::SourceCellInitializer(source_ml_source2_data_property_value3892, _, _) => {
                                source_ml_source2_data_property_value3892
                            }
                        }, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) || (({ let ml_values37 = { let ml_reference_value38 = initializers.borrow().clone(); ml_reference_value38 }; let ml_callback37 = { let row = row.clone(); std::rc::Rc::new(move |prior: self::JavaScriptCallCellInitializer| -> bool { ((match prior.clone() {
                            self::JavaScriptCallCellInitializer::SourceCellInitializer(source_ml_source2_data_property_value3899, _, _) => {
                                source_ml_source2_data_property_value3899
                            }
                        }) == (match row.clone() {
                            self::JavaScriptCallCellInitializer::SourceCellInitializer(source_ml_source2_data_property_value3900, _, _) => {
                                source_ml_source2_data_property_value3900
                            }
                        })) }) }; ml_values37.iter().cloned().enumerate().position(|(ml_index37, ml_value37)| ml_callback37(ml_value37)).map_or(-1f64, |ml_position| ml_position as f64) }) != index)) || (({
                            let source_ml_source2_type_query_operand3904 = match row.clone() {
                                self::JavaScriptCallCellInitializer::SourceCellInitializer(_, _, source_ml_source2_data_property_value3903) => {
                                    source_ml_source2_data_property_value3903
                                }
                            };
                            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                        }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) }) }; ml_values34.iter().cloned().enumerate().any(|(ml_index34, ml_value34)| ml_callback34(ml_value34, (ml_index34 as f64))) }) {
                            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![99, 101, 108, 108, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 97, 110, 100, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 102, 108, 97, 103, 115])))
                        } else {
                            {
                                let current = state.clone();
                                {
                                    let entries = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceBinding>::new()));
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source2_s3905 = self::source_ml_source1_temporary_lower_java_script_call_source_lexical_cell_expression_loop17(bindings.clone(), depth, realm.clone(), identities.clone(), existing.clone(), initializers.clone(), current.clone(), entries.clone(), at);
                                            match source_ml_source2_s3905.clone() {
                                                self::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Result::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Done(current_2, entries_2) => {
                                                    let current_2 = *current_2;
                                                    self::lower_java_script_call_source_expression(body.clone(), std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value39 = bindings.borrow().clone(); ml_reference_value39 }), { let ml_reference_value40 = entries_2.borrow().clone(); ml_reference_value40 }))), current_2.clone(), (depth + 1f64))
                                                }
                                                self::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Result::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Return(source_ml_source1_temporary_result17) => {
                                                    let source_ml_source1_temporary_result17 = *source_ml_source1_temporary_result17;
                                                    source_ml_source1_temporary_result17.clone()
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
        self::JavaScriptCallSource::SourceInitializeCell(identity, source_value, body_2) => {
            let source_value = *source_value;
            let body_2 = *body_2;
            {
                let cell = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourceBinding(identity), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let value = self::lower_java_script_call_source_expression(source_value.clone(), bindings.clone(), match cell.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3908, _) => {
                            let source_ml_source2_data_property_value3908 = *source_ml_source2_data_property_value3908;
                            source_ml_source2_data_property_value3908.clone()
                        }
                    }, (depth + 1f64));
                    {
                        let held = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3909, _) => {
                                let source_ml_source2_data_property_value3909 = *source_ml_source2_data_property_value3909;
                                source_ml_source2_data_property_value3909.clone()
                            }
                        });
                        {
                            let saved = self::append_java_script_call_source_instruction(match held.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3910, _) => {
                                    let source_ml_source2_data_property_value3910 = *source_ml_source2_data_property_value3910;
                                    source_ml_source2_data_property_value3910.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match held.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3911) => {
                                    source_ml_source2_data_property_value3911
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3912) => {
                                    source_ml_source2_data_property_value3912
                                }
                            }));
                            {
                                let key = self::allocate_java_script_call_source_register(saved.clone());
                                {
                                    let written = self::allocate_java_script_call_source_register(match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3913, _) => {
                                            let source_ml_source2_data_property_value3913 = *source_ml_source2_data_property_value3913;
                                            source_ml_source2_data_property_value3913.clone()
                                        }
                                    });
                                    {
                                        let flag = self::allocate_java_script_call_source_register(match written.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3914, _) => {
                                                let source_ml_source2_data_property_value3914 = *source_ml_source2_data_property_value3914;
                                                source_ml_source2_data_property_value3914.clone()
                                            }
                                        });
                                        {
                                            let current_3 = self::append_java_script_call_source_instruction(match flag.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value3915, _) => {
                                                    let source_ml_source2_data_property_value3915 = *source_ml_source2_data_property_value3915;
                                                    source_ml_source2_data_property_value3915.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Constant(match key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3916) => {
                                                    source_ml_source2_data_property_value3916
                                                }
                                            }, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![118f64, 97f64, 108f64, 117f64, 101f64]))))));
                                            {
                                                let current_4 = self::append_java_script_call_source_instruction(current_3.clone(), self::JavaScriptCallInstruction::OrdinarySet(match written.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3917) => {
                                                        source_ml_source2_data_property_value3917
                                                    }
                                                }, match cell.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3918) => {
                                                        source_ml_source2_data_property_value3918
                                                    }
                                                }, match key.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3919) => {
                                                        source_ml_source2_data_property_value3919
                                                    }
                                                }, match held.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3920) => {
                                                        source_ml_source2_data_property_value3920
                                                    }
                                                }, match cell.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3921) => {
                                                        source_ml_source2_data_property_value3921
                                                    }
                                                }));
                                                {
                                                    let current_5 = self::append_java_script_call_source_instruction(current_4.clone(), self::JavaScriptCallInstruction::RequireWrite(match written.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3922) => {
                                                            source_ml_source2_data_property_value3922
                                                        }
                                                    }));
                                                    {
                                                        let current_6 = self::append_java_script_call_source_instruction(current_5.clone(), self::JavaScriptCallInstruction::Constant(match key.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3923) => {
                                                                source_ml_source2_data_property_value3923
                                                            }
                                                        }, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![105f64, 110f64, 105f64, 116f64, 105f64, 97f64, 108f64, 105f64, 122f64, 101f64, 100f64]))))));
                                                        {
                                                            let current_7 = self::append_java_script_call_source_instruction(current_6.clone(), self::JavaScriptCallInstruction::Constant(match flag.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3924) => {
                                                                    source_ml_source2_data_property_value3924
                                                                }
                                                            }, Box::new(self::JavaScriptSemanticValue::Boolean(true))));
                                                            {
                                                                let current_8 = self::append_java_script_call_source_instruction(current_7.clone(), self::JavaScriptCallInstruction::OrdinarySet(match written.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3925) => {
                                                                        source_ml_source2_data_property_value3925
                                                                    }
                                                                }, match cell.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3926) => {
                                                                        source_ml_source2_data_property_value3926
                                                                    }
                                                                }, match key.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3927) => {
                                                                        source_ml_source2_data_property_value3927
                                                                    }
                                                                }, match flag.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3928) => {
                                                                        source_ml_source2_data_property_value3928
                                                                    }
                                                                }, match cell.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3929) => {
                                                                        source_ml_source2_data_property_value3929
                                                                    }
                                                                }));
                                                                {
                                                                    let current_9 = self::append_java_script_call_source_instruction(current_8.clone(), self::JavaScriptCallInstruction::RequireWrite(match written.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value3930) => {
                                                                            source_ml_source2_data_property_value3930
                                                                        }
                                                                    }));
                                                                    self::lower_java_script_call_source_expression(body_2.clone(), bindings.clone(), current_9.clone(), (depth + 1f64))
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
        _ => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 108, 101, 120, 105, 99, 97, 108, 99, 101, 108, 108, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}
