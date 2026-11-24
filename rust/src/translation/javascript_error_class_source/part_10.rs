// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48;
use super::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50;
use super::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop45Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_find_java_script_call_source_spread_iterator_slot_loop51;
use super::source_ml_source1_temporary_lower_java_script_call_source_argument_list_loop49;
use super::source_ml_source1_temporary_lower_java_script_call_source_call_expression_loop45;
use super::source_ml_source1_temporary_lower_java_script_call_source_call_expression_loop46;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_call_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::lower_java_script_call_source_argument_list(source.clone(), bindings.clone(), state.clone(), (depth + 1f64))
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 32, 115, 112, 114, 101, 97, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 114, 114, 111, 117, 110, 100, 105, 110, 103, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116])))
        }
        self::JavaScriptCallSource::SourceClosure(owner, ordinal, source_captures, source_receiver) => {
            let source_receiver = *source_receiver;
            if ((((owner <= (self::ml::Big::from_i128(0))) || (owner > (self::ml::Big::from_i128(18446744073709551615)))) || !(self::test_java_script_call_code_coordinate(ordinal, 4096f64))) || (({ let ml_reference_array83 = source_captures.clone(); let ml_reference_projection83 = ml_reference_array83.borrow().len() as f64; ml_reference_projection83 }) > (4096f64))) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 108, 111, 115, 117, 114, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 116, 105, 99, 32, 111, 119, 110, 101, 114, 44, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 105, 116, 101, 32, 97, 110, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 97, 112, 116, 117, 114, 101, 32, 118, 101, 99, 116, 111, 114])))
            } else {
                {
                    let receiver = self::lower_java_script_call_source_expression(source_receiver.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let saved_receiver = self::allocate_java_script_call_source_register(match receiver.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10506, _) => {
                                let source_ml_source2_data_property_value10506 = *source_ml_source2_data_property_value10506;
                                source_ml_source2_data_property_value10506.clone()
                            }
                        });
                        {
                            let current = self::append_java_script_call_source_instruction(match saved_receiver.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10507, _) => {
                                    let source_ml_source2_data_property_value10507 = *source_ml_source2_data_property_value10507;
                                    source_ml_source2_data_property_value10507.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match saved_receiver.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10508) => {
                                    source_ml_source2_data_property_value10508
                                }
                            }, match receiver.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10509) => {
                                    source_ml_source2_data_property_value10509
                                }
                            }));
                            {
                                let captures = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                {
                                    let source_ml_source1_temporary_index43 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s10510 = self::source_ml_source1_temporary_lower_java_script_call_source_call_expression_loop45(bindings.clone(), depth, current.clone(), captures.clone(), source_captures.clone(), source_ml_source1_temporary_index43.clone());
                                        match source_ml_source2_s10510.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop45Result::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop45Done(current_2, captures_2) => {
                                                let current_2 = *current_2;
                                                {
                                                    let result = self::allocate_java_script_call_source_register(current_2.clone());
                                                    {
                                                        let source_ml_source2_nominal_field10515 = match result.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10516, _) => {
                                                                let source_ml_source2_data_property_value10516 = *source_ml_source2_data_property_value10516;
                                                                source_ml_source2_data_property_value10516.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field10517 = match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10518) => {
                                                                    source_ml_source2_data_property_value10518
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_override10519 = self::append_java_script_call_source_instruction(match result.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10511, _) => {
                                                                        let source_ml_source2_data_property_value10511 = *source_ml_source2_data_property_value10511;
                                                                        source_ml_source2_data_property_value10511.clone()
                                                                    }
                                                                }, self::JavaScriptCallInstruction::ClosureAllocate(match result.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10512) => {
                                                                        source_ml_source2_data_property_value10512
                                                                    }
                                                                }, owner.clone(), ordinal, captures_2.clone(), match saved_receiver.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10513) => {
                                                                        source_ml_source2_data_property_value10513
                                                                    }
                                                                }));
                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override10519.clone()), source_ml_source2_nominal_field10517)
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
        self::JavaScriptCallSource::SourceCall(source_callee, source_receiver_2, arguments) => {
            let source_callee = *source_callee;
            let source_receiver_2 = *source_receiver_2;
            if { let ml_values84 = { let ml_reference_value85 = arguments.borrow().clone(); ml_reference_value85 }; let ml_callback84 = std::rc::Rc::new(move |argument: self::JavaScriptCallSource| -> bool { match argument.clone() {
                self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                    true
                }
                _ => {
                    false
                }
            } }); ml_values84.iter().cloned().enumerate().any(|(ml_index84, ml_value84)| ml_callback84(ml_value84)) } {
                self::lower_java_script_call_source_argument_list(self::JavaScriptCallSource::SourceVariadicCall(Box::new(source_callee.clone()), Box::new(source_receiver_2.clone()), arguments.clone(), self::ml_string::Text::from_units(vec![99, 97, 108, 108]), false), bindings.clone(), state.clone(), (depth + 1f64))
            } else {
                if (({ let ml_reference_array86 = arguments.clone(); let ml_reference_projection86 = ml_reference_array86.borrow().len() as f64; ml_reference_projection86 }) > (4096f64)) {
                    self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 108, 108, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 98, 117, 100, 103, 101, 116])))
                } else {
                    {
                        let callee = self::lower_java_script_call_source_operand(source_callee.clone(), bindings.clone(), state.clone(), (depth + 1f64), false);
                        {
                            let receiver_2 = self::lower_java_script_call_source_operand(source_receiver_2.clone(), bindings.clone(), match callee.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10888, _) => {
                                    let source_ml_source2_data_property_value10888 = *source_ml_source2_data_property_value10888;
                                    source_ml_source2_data_property_value10888.clone()
                                }
                            }, (depth + 1f64), false);
                            {
                                let current_3 = match receiver_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10889, _) => {
                                        let source_ml_source2_data_property_value10889 = *source_ml_source2_data_property_value10889;
                                        source_ml_source2_data_property_value10889.clone()
                                    }
                                };
                                {
                                    let arguments_ = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                    {
                                        let source_ml_source1_temporary_index44 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s10890 = self::source_ml_source1_temporary_lower_java_script_call_source_call_expression_loop46(bindings.clone(), depth, false, current_3.clone(), arguments_.clone(), arguments.clone(), source_ml_source1_temporary_index44.clone());
                                            match source_ml_source2_s10890.clone() {
                                                self::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Result::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Done(current_4, arguments__2) => {
                                                    let current_4 = *current_4;
                                                    {
                                                        let result_2 = self::allocate_java_script_call_source_register(current_4.clone());
                                                        {
                                                            let source_ml_source2_nominal_field10896 = match result_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10897, _) => {
                                                                    let source_ml_source2_data_property_value10897 = *source_ml_source2_data_property_value10897;
                                                                    source_ml_source2_data_property_value10897.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field10898 = match result_2.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10899) => {
                                                                        source_ml_source2_data_property_value10899
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override10900 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10891, _) => {
                                                                            let source_ml_source2_data_property_value10891 = *source_ml_source2_data_property_value10891;
                                                                            source_ml_source2_data_property_value10891.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Call(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10892) => {
                                                                            source_ml_source2_data_property_value10892
                                                                        }
                                                                    }, match callee.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10893) => {
                                                                            source_ml_source2_data_property_value10893
                                                                        }
                                                                    }, match receiver_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10894) => {
                                                                            source_ml_source2_data_property_value10894
                                                                        }
                                                                    }, arguments__2.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override10900.clone()), source_ml_source2_nominal_field10898)
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
        self::JavaScriptCallSource::SourceSnapshotCall(source_callee_2, source_receiver_3, arguments_2) => {
            let source_callee_2 = *source_callee_2;
            let source_receiver_3 = *source_receiver_3;
            if { let ml_values87 = { let ml_reference_value88 = arguments_2.borrow().clone(); ml_reference_value88 }; let ml_callback87 = std::rc::Rc::new(move |argument: self::JavaScriptCallSource| -> bool { match argument.clone() {
                self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                    true
                }
                _ => {
                    false
                }
            } }); ml_values87.iter().cloned().enumerate().any(|(ml_index87, ml_value87)| ml_callback87(ml_value87)) } {
                self::lower_java_script_call_source_argument_list(self::JavaScriptCallSource::SourceVariadicCall(Box::new(source_callee_2.clone()), Box::new(source_receiver_3.clone()), arguments_2.clone(), self::ml_string::Text::from_units(vec![99, 97, 108, 108]), false), bindings.clone(), state.clone(), (depth + 1f64))
            } else {
                if (({ let ml_reference_array89 = arguments_2.clone(); let ml_reference_projection89 = ml_reference_array89.borrow().len() as f64; ml_reference_projection89 }) > (4096f64)) {
                    self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 108, 108, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 98, 117, 100, 103, 101, 116])))
                } else {
                    {
                        let callee_2 = self::lower_java_script_call_source_operand(source_callee_2.clone(), bindings.clone(), state.clone(), (depth + 1f64), true);
                        {
                            let receiver_3 = self::lower_java_script_call_source_operand(source_receiver_3.clone(), bindings.clone(), match callee_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11269, _) => {
                                    let source_ml_source2_data_property_value11269 = *source_ml_source2_data_property_value11269;
                                    source_ml_source2_data_property_value11269.clone()
                                }
                            }, (depth + 1f64), true);
                            {
                                let current_5 = match receiver_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11270, _) => {
                                        let source_ml_source2_data_property_value11270 = *source_ml_source2_data_property_value11270;
                                        source_ml_source2_data_property_value11270.clone()
                                    }
                                };
                                {
                                    let arguments__3 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                    {
                                        let source_ml_source1_temporary_index44_2 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s11271 = self::source_ml_source1_temporary_lower_java_script_call_source_call_expression_loop46(bindings.clone(), depth, true, current_5.clone(), arguments__3.clone(), arguments_2.clone(), source_ml_source1_temporary_index44_2.clone());
                                            match source_ml_source2_s11271.clone() {
                                                self::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Result::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Done(current_6, arguments__4) => {
                                                    let current_6 = *current_6;
                                                    {
                                                        let result_3 = self::allocate_java_script_call_source_register(current_6.clone());
                                                        {
                                                            let source_ml_source2_nominal_field11277 = match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11278, _) => {
                                                                    let source_ml_source2_data_property_value11278 = *source_ml_source2_data_property_value11278;
                                                                    source_ml_source2_data_property_value11278.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field11279 = match result_3.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11280) => {
                                                                        source_ml_source2_data_property_value11280
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override11281 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11272, _) => {
                                                                            let source_ml_source2_data_property_value11272 = *source_ml_source2_data_property_value11272;
                                                                            source_ml_source2_data_property_value11272.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Call(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11273) => {
                                                                            source_ml_source2_data_property_value11273
                                                                        }
                                                                    }, match callee_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11274) => {
                                                                            source_ml_source2_data_property_value11274
                                                                        }
                                                                    }, match receiver_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11275) => {
                                                                            source_ml_source2_data_property_value11275
                                                                        }
                                                                    }, arguments__4.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override11281.clone()), source_ml_source2_nominal_field11279)
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
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 108, 108, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_operand(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64, snapshot: bool) -> self::JavaScriptCallSourceResult {
    {
        let value = self::lower_java_script_call_source_expression(source.clone(), bindings.clone(), builder.clone(), depth);
        if !snapshot {
            value.clone()
        } else {
            {
                let saved = self::allocate_java_script_call_source_register(match value.clone() {
                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11282, _) => {
                        let source_ml_source2_data_property_value11282 = *source_ml_source2_data_property_value11282;
                        source_ml_source2_data_property_value11282.clone()
                    }
                });
                {
                    let source_ml_source2_nominal_field11287 = match saved.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11288, _) => {
                            let source_ml_source2_data_property_value11288 = *source_ml_source2_data_property_value11288;
                            source_ml_source2_data_property_value11288.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field11289 = match saved.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11290) => {
                                source_ml_source2_data_property_value11290
                            }
                        };
                        {
                            let source_ml_source2_nominal_override11291 = self::append_java_script_call_source_instruction(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11283, _) => {
                                    let source_ml_source2_data_property_value11283 = *source_ml_source2_data_property_value11283;
                                    source_ml_source2_data_property_value11283.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11284) => {
                                    source_ml_source2_data_property_value11284
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11285) => {
                                    source_ml_source2_data_property_value11285
                                }
                            }));
                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override11291.clone()), source_ml_source2_nominal_field11289)
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_argument_list(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceVariadicCall(source_callee, receiver, arguments, mode, receiver_after_arguments) => {
            let source_callee = *source_callee;
            let receiver = *receiver;
            if (!(self::ml_array::includes(&{ let ml_reference_value90 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 97, 108, 108]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116])]))).borrow().clone(); ml_reference_value90 }, &mode.clone(), 0f64, false, |a, b| a == b)) || (receiver_after_arguments && (mode != (self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]))))) {
                self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 100, 105, 99, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 118, 111, 99, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 112, 111, 108, 105, 99, 121])))
            } else {
                if (({ let ml_reference_array91 = arguments.clone(); let ml_reference_projection91 = ml_reference_array91.borrow().len() as f64; ml_reference_projection91 }) > (4096f64)) {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 100, 105, 99, 32, 115, 111, 117, 114, 99, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 117, 116, 104, 111, 114, 101, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116])))
                } else {
                    {
                        let selected = self::lower_java_script_call_source_expression(source_callee.clone(), bindings.clone(), builder.clone(), (depth + 1f64));
                        {
                            let callee = self::allocate_java_script_call_source_register(match selected.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11471, _) => {
                                    let source_ml_source2_data_property_value11471 = *source_ml_source2_data_property_value11471;
                                    source_ml_source2_data_property_value11471.clone()
                                }
                            });
                            {
                                let current = self::append_java_script_call_source_instruction(match callee.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11472, _) => {
                                        let source_ml_source2_data_property_value11472 = *source_ml_source2_data_property_value11472;
                                        source_ml_source2_data_property_value11472.clone()
                                    }
                                }, self::JavaScriptCallInstruction::Copy(match callee.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11473) => {
                                        source_ml_source2_data_property_value11473
                                    }
                                }, match selected.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11474) => {
                                        source_ml_source2_data_property_value11474
                                    }
                                }));
                                {
                                    let receiver_register = 0f64;
                                    {
                                        let source_ml_source2_s11481 = if (mode == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]))) {
                                            {
                                                let receiver_register_2 = match callee.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11482) => {
                                                        source_ml_source2_data_property_value11482
                                                    }
                                                };
                                                self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48Next(Box::new(current.clone()), receiver_register_2)
                                            }
                                        } else {
                                            if !receiver_after_arguments {
                                                {
                                                    let raw_ = self::lower_java_script_call_source_expression(receiver.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                                                    {
                                                        let held = self::allocate_java_script_call_source_register(match raw_.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11483, _) => {
                                                                let source_ml_source2_data_property_value11483 = *source_ml_source2_data_property_value11483;
                                                                source_ml_source2_data_property_value11483.clone()
                                                            }
                                                        });
                                                        {
                                                            let current_2 = self::append_java_script_call_source_instruction(match held.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11484, _) => {
                                                                    let source_ml_source2_data_property_value11484 = *source_ml_source2_data_property_value11484;
                                                                    source_ml_source2_data_property_value11484.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Copy(match held.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11485) => {
                                                                    source_ml_source2_data_property_value11485
                                                                }
                                                            }, match raw_.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11486) => {
                                                                    source_ml_source2_data_property_value11486
                                                                }
                                                            }));
                                                            {
                                                                let receiver_register_3 = match held.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11487) => {
                                                                        source_ml_source2_data_property_value11487
                                                                    }
                                                                };
                                                                self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48Next(Box::new(current_2.clone()), receiver_register_3)
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48Next(Box::new(current.clone()), receiver_register)
                                            }
                                        };
                                        match source_ml_source2_s11481.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48Next(current_3, receiver_register_4) => {
                                                let current_3 = *current_3;
                                                {
                                                    let slot = { let ml_reference_array92 = match current_3.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11488, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value11488.clone()
                                                        }
                                                    }; let ml_reference_projection92 = ml_reference_array92.borrow().len() as f64; ml_reference_projection92 };
                                                    {
                                                        let current_4 = self::append_java_script_call_source_instruction(current_3.clone(), self::JavaScriptCallInstruction::ArgumentListOpen(slot));
                                                        {
                                                            let source_ml_source1_temporary_index47 = self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s11489 = self::source_ml_source1_temporary_lower_java_script_call_source_argument_list_loop49(bindings.clone(), builder.clone(), depth, current_4.clone(), slot, arguments.clone(), source_ml_source1_temporary_index47.clone());
                                                                match source_ml_source2_s11489.clone() {
                                                                    self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Result::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Done(current_5) => {
                                                                        let current_5 = *current_5;
                                                                        {
                                                                            let source_ml_source2_s11495 = if receiver_after_arguments {
                                                                                {
                                                                                    let raw_2 = self::lower_java_script_call_source_expression(receiver.clone(), bindings.clone(), current_5.clone(), (depth + 1f64));
                                                                                    {
                                                                                        let held_2 = self::allocate_java_script_call_source_register(match raw_2.clone() {
                                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11496, _) => {
                                                                                                let source_ml_source2_data_property_value11496 = *source_ml_source2_data_property_value11496;
                                                                                                source_ml_source2_data_property_value11496.clone()
                                                                                            }
                                                                                        });
                                                                                        {
                                                                                            let current_6 = self::append_java_script_call_source_instruction(match held_2.clone() {
                                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11497, _) => {
                                                                                                    let source_ml_source2_data_property_value11497 = *source_ml_source2_data_property_value11497;
                                                                                                    source_ml_source2_data_property_value11497.clone()
                                                                                                }
                                                                                            }, self::JavaScriptCallInstruction::Copy(match held_2.clone() {
                                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11498) => {
                                                                                                    source_ml_source2_data_property_value11498
                                                                                                }
                                                                                            }, match raw_2.clone() {
                                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11499) => {
                                                                                                    source_ml_source2_data_property_value11499
                                                                                                }
                                                                                            }));
                                                                                            {
                                                                                                let receiver_register_5 = match held_2.clone() {
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11500) => {
                                                                                                        source_ml_source2_data_property_value11500
                                                                                                    }
                                                                                                };
                                                                                                self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50Next(Box::new(current_6.clone()), receiver_register_5)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50Next(Box::new(current_5.clone()), receiver_register_4)
                                                                            };
                                                                            match source_ml_source2_s11495.clone() {
                                                                                self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50Next(current_7, receiver_register_6) => {
                                                                                    let current_7 = *current_7;
                                                                                    {
                                                                                        let result = self::allocate_java_script_call_source_register(current_7.clone());
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11505 = match result.clone() {
                                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11506, _) => {
                                                                                                    let source_ml_source2_data_property_value11506 = *source_ml_source2_data_property_value11506;
                                                                                                    source_ml_source2_data_property_value11506.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11507 = match result.clone() {
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11508) => {
                                                                                                        source_ml_source2_data_property_value11508
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override11509 = self::append_java_script_call_source_instruction(match result.clone() {
                                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11501, _) => {
                                                                                                            let source_ml_source2_data_property_value11501 = *source_ml_source2_data_property_value11501;
                                                                                                            source_ml_source2_data_property_value11501.clone()
                                                                                                        }
                                                                                                    }, self::JavaScriptCallInstruction::ArgumentListCall(match result.clone() {
                                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11502) => {
                                                                                                            source_ml_source2_data_property_value11502
                                                                                                        }
                                                                                                    }, slot, match callee.clone() {
                                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11503) => {
                                                                                                            source_ml_source2_data_property_value11503
                                                                                                        }
                                                                                                    }, receiver_register_6, mode.clone()));
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override11509.clone()), source_ml_source2_nominal_field11507)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Result::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Return(source_ml_source1_temporary_result50) => {
                                                                        let source_ml_source1_temporary_result50 = *source_ml_source1_temporary_result50;
                                                                        source_ml_source1_temporary_result50.clone()
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
        _ => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 100, 105, 99, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 118, 111, 99, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 112, 111, 108, 105, 99, 121])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_argument(argument: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64, slot: f64, handlers: f64) -> std::rc::Rc<std::cell::RefCell<(self::JavaScriptCallSourceBuilder, bool)>> {
    match argument.clone() {
        self::JavaScriptCallSource::SourceArgumentSpread(value, iterator_key, realm) => {
            let value = *value;
            let iterator_key = *iterator_key;
            self::lower_java_script_call_source_spread_argument(value.clone(), iterator_key.clone(), realm.clone(), bindings.clone(), builder.clone(), depth, slot, handlers)
        }
        _ => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_plain_argument(argument: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64, slot: f64) -> std::rc::Rc<std::cell::RefCell<(self::JavaScriptCallSourceBuilder, bool)>> {
    {
        let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), builder.clone(), depth);
        std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source2_field11693 = self::append_java_script_call_source_instruction(match value.clone() {
                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11691, _) => {
                    let source_ml_source2_data_property_value11691 = *source_ml_source2_data_property_value11691;
                    source_ml_source2_data_property_value11691.clone()
                }
            }, self::JavaScriptCallInstruction::ArgumentListAppend(slot, match value.clone() {
                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11692) => {
                    source_ml_source2_data_property_value11692
                }
            }));
            (source_ml_source2_field11693.clone(), false)
        }))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_spread_argument(argument: self::JavaScriptCallSource, iterator_key: self::JavaScriptCallSource, realm: self::ml::Big, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64, slot: f64, handlers: f64) -> std::rc::Rc<std::cell::RefCell<(self::JavaScriptCallSourceBuilder, bool)>> {
    {
        let input = self::lower_java_script_call_source_operand(argument.clone(), bindings.clone(), builder.clone(), depth, true);
        {
            let key = self::lower_java_script_call_source_operand(iterator_key.clone(), bindings.clone(), match input.clone() {
                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11695, _) => {
                    let source_ml_source2_data_property_value11695 = *source_ml_source2_data_property_value11695;
                    source_ml_source2_data_property_value11695.clone()
                }
            }, depth, true);
            {
                let value = self::allocate_java_script_call_source_register(match key.clone() {
                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11696, _) => {
                        let source_ml_source2_data_property_value11696 = *source_ml_source2_data_property_value11696;
                        source_ml_source2_data_property_value11696.clone()
                    }
                });
                {
                    let done = self::allocate_java_script_call_source_register(match value.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11697, _) => {
                            let source_ml_source2_data_property_value11697 = *source_ml_source2_data_property_value11697;
                            source_ml_source2_data_property_value11697.clone()
                        }
                    });
                    {
                        let iterator = self::find_java_script_call_source_spread_iterator_slot(match match done.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11698, _) => {
                                let source_ml_source2_data_property_value11698 = *source_ml_source2_data_property_value11698;
                                source_ml_source2_data_property_value11698.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11699, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value11699.clone()
                            }
                        });
                        if ((iterator >= (65536f64)) || ((match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value11701, _, _, _, _, _) => {
                                source_ml_source2_data_property_value11701
                            }
                        }) >= (1024f64))) {
                            std::rc::Rc::new(std::cell::RefCell::new((builder.clone(), true)))
                        } else {
                            {
                                let acquired = self::append_java_script_call_source_instruction(match done.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11704, _) => {
                                        let source_ml_source2_data_property_value11704 = *source_ml_source2_data_property_value11704;
                                        source_ml_source2_data_property_value11704.clone()
                                    }
                                }, {
                                    let source_ml_source2_constructor_field11706 = match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11705) => {
                                            source_ml_source2_data_property_value11705
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field11708 = match done.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11707) => {
                                                source_ml_source2_data_property_value11707
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field11710 = match input.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11709) => {
                                                    source_ml_source2_data_property_value11709
                                                }
                                            };
                                            {
                                                let source_ml_source2_constructor_field11712 = match key.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11711) => {
                                                        source_ml_source2_data_property_value11711
                                                    }
                                                };
                                                self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_constructor_field11706, source_ml_source2_constructor_field11708, source_ml_source2_constructor_field11710, source_ml_source2_constructor_field11712, iterator, self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]), realm.clone(), None::<(Option<bool>,)>)
                                            }
                                        }
                                    }
                                });
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_field11718 = self::append_java_script_call_source_argument_spread(acquired.clone(), slot, iterator, realm.clone(), match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11716) => {
                                            source_ml_source2_data_property_value11716
                                        }
                                    }, match done.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11717) => {
                                            source_ml_source2_data_property_value11717
                                        }
                                    }, handlers);
                                    (source_ml_source2_field11718.clone(), false)
                                }))
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
pub fn find_java_script_call_source_spread_iterator_slot(instructions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>) -> f64 {
    {
        let iterator = 0f64;
        {
            let iterator_2 = self::source_ml_source1_temporary_find_java_script_call_source_spread_iterator_slot_loop51(instructions.clone(), iterator);
            iterator_2
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn append_java_script_call_source_argument_spread(builder: self::JavaScriptCallSourceBuilder, slot: f64, iterator: f64, realm: self::ml::Big, value: f64, done: f64, handlers: f64) -> self::JavaScriptCallSourceBuilder {
    {
        let protect = { let ml_reference_array93 = match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11720, _, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value11720.clone()
            }
        }; let ml_reference_projection93 = ml_reference_array93.borrow().len() as f64; ml_reference_projection93 };
        {
            let current = self::append_java_script_call_source_instruction(builder.clone(), self::JavaScriptCallInstruction::PushFinally(0f64));
            {
                let start = { let ml_reference_array94 = match current.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11721, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value11721.clone()
                    }
                }; let ml_reference_projection94 = ml_reference_array94.borrow().len() as f64; ml_reference_projection94 };
                {
                    let current_2 = self::append_java_script_call_source_instruction(current.clone(), {
                        let source_ml_source2_constructor_field11724 = 0f64;
                        {
                            let source_ml_source2_constructor_field11725 = 0f64;
                            self::JavaScriptCallInstruction::IteratorProtocol(value, done, source_ml_source2_constructor_field11724, source_ml_source2_constructor_field11725, iterator, self::ml_string::Text::from_units(vec![115, 116, 101, 112]), realm.clone(), None::<(Option<bool>,)>)
                        }
                    });
                    {
                        let branch = { let ml_reference_array95 = match current_2.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11729, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value11729.clone()
                            }
                        }; let ml_reference_projection95 = ml_reference_array95.borrow().len() as f64; ml_reference_projection95 };
                        {
                            let current_3 = self::append_java_script_call_source_instruction(current_2.clone(), self::JavaScriptCallInstruction::Branch(done, 0f64, (branch + 1f64)));
                            {
                                let current_4 = self::append_java_script_call_source_instruction(current_3.clone(), self::JavaScriptCallInstruction::ArgumentListAppend(slot, value));
                                {
                                    let current_5 = self::append_java_script_call_source_instruction(current_4.clone(), self::JavaScriptCallInstruction::Jump(start));
                                    {
                                        let leave = { let ml_reference_array96 = match current_5.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11730, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value11730.clone()
                                            }
                                        }; let ml_reference_projection96 = ml_reference_array96.borrow().len() as f64; ml_reference_projection96 };
                                        {
                                            let current_6 = self::append_java_script_call_source_instruction(current_5.clone(), self::JavaScriptCallInstruction::CompleteJump(0f64, handlers));
                                            {
                                                let cleanup = { let ml_reference_array97 = match current_6.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11731, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value11731.clone()
                                                    }
                                                }; let ml_reference_projection97 = ml_reference_array97.borrow().len() as f64; ml_reference_projection97 };
                                                {
                                                    let current_7 = self::append_java_script_call_source_instruction(current_6.clone(), {
                                                        let source_ml_source2_constructor_field11734 = 0f64;
                                                        {
                                                            let source_ml_source2_constructor_field11735 = 0f64;
                                                            self::JavaScriptCallInstruction::IteratorProtocol(value, done, source_ml_source2_constructor_field11734, source_ml_source2_constructor_field11735, iterator, self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]), realm.clone(), None::<(Option<bool>,)>)
                                                        }
                                                    });
                                                    {
                                                        let current_8 = self::append_java_script_call_source_instruction(current_7.clone(), self::JavaScriptCallInstruction::FinishFinally);
                                                        {
                                                            let end = { let ml_reference_array98 = match current_8.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11739, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value11739.clone()
                                                                }
                                                            }; let ml_reference_projection98 = ml_reference_array98.borrow().len() as f64; ml_reference_projection98 };
                                                            {
                                                                let source_ml_source2_nominal_spread11741 = current_8.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field11742 = match source_ml_source2_nominal_spread11741.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11743, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value11743.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field11744 = match source_ml_source2_nominal_spread11741.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value11745, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value11745
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field11746 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value11747, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value11747
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field11748 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value11749, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value11749
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field11750 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value11751, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value11751
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11752 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value11753, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value11753.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11754 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value11755, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value11755.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11756 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value11757, _, _) => {
                                                                                                        source_ml_source2_data_property_value11757.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11758 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value11759, _) => {
                                                                                                            source_ml_source2_data_property_value11759.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11760 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11761) => {
                                                                                                                source_ml_source2_data_property_value11761.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override11762 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values99 = { let ml_reference_value100 = (match current_8.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11740, _, _, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value11740.clone()
                                                                                                                }
                                                                                                            }).borrow().clone(); ml_reference_value100 }; let ml_callback99 = { let protect = protect; let cleanup = cleanup; let branch = branch; let done = done; let leave = leave; let end = end; let handlers = handlers; std::rc::Rc::new(move |row: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == protect) {
                                                                                                                self::JavaScriptCallInstruction::PushFinally(cleanup)
                                                                                                            } else {
                                                                                                                if (index == branch) {
                                                                                                                    self::JavaScriptCallInstruction::Branch(done, leave, (branch + 1f64))
                                                                                                                } else {
                                                                                                                    if (index == leave) {
                                                                                                                        self::JavaScriptCallInstruction::CompleteJump(end, handlers)
                                                                                                                    } else {
                                                                                                                        row.clone()
                                                                                                                    }
                                                                                                                }
                                                                                                            } }) }; ml_values99.iter().cloned().enumerate().map(|(ml_index99, ml_value99)| ml_callback99(ml_value99, (ml_index99 as f64))).collect::<Vec<_>>() }));
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override11762.clone(), source_ml_source2_nominal_field11744, source_ml_source2_nominal_field11746, source_ml_source2_nominal_field11748, source_ml_source2_nominal_field11750, source_ml_source2_nominal_field11752.clone(), source_ml_source2_nominal_field11754.clone(), source_ml_source2_nominal_field11756.clone(), source_ml_source2_nominal_field11758.clone(), source_ml_source2_nominal_field11760.clone())
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
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
