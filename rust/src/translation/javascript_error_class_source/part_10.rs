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
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10560, _) => {
                                let source_ml_source2_data_property_value10560 = *source_ml_source2_data_property_value10560;
                                source_ml_source2_data_property_value10560.clone()
                            }
                        });
                        {
                            let current = self::append_java_script_call_source_instruction(match saved_receiver.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10561, _) => {
                                    let source_ml_source2_data_property_value10561 = *source_ml_source2_data_property_value10561;
                                    source_ml_source2_data_property_value10561.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match saved_receiver.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10562) => {
                                    source_ml_source2_data_property_value10562
                                }
                            }, match receiver.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10563) => {
                                    source_ml_source2_data_property_value10563
                                }
                            }));
                            {
                                let captures = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                {
                                    let source_ml_source1_temporary_index43 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s10564 = self::source_ml_source1_temporary_lower_java_script_call_source_call_expression_loop45(bindings.clone(), depth, current.clone(), captures.clone(), source_captures.clone(), source_ml_source1_temporary_index43.clone());
                                        match source_ml_source2_s10564.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop45Result::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop45Done(current_2, captures_2) => {
                                                let current_2 = *current_2;
                                                {
                                                    let result = self::allocate_java_script_call_source_register(current_2.clone());
                                                    {
                                                        let source_ml_source2_nominal_field10569 = match result.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10570, _) => {
                                                                let source_ml_source2_data_property_value10570 = *source_ml_source2_data_property_value10570;
                                                                source_ml_source2_data_property_value10570.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field10571 = match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10572) => {
                                                                    source_ml_source2_data_property_value10572
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_override10573 = self::append_java_script_call_source_instruction(match result.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10565, _) => {
                                                                        let source_ml_source2_data_property_value10565 = *source_ml_source2_data_property_value10565;
                                                                        source_ml_source2_data_property_value10565.clone()
                                                                    }
                                                                }, self::JavaScriptCallInstruction::ClosureAllocate(match result.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10566) => {
                                                                        source_ml_source2_data_property_value10566
                                                                    }
                                                                }, owner.clone(), ordinal, captures_2.clone(), match saved_receiver.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10567) => {
                                                                        source_ml_source2_data_property_value10567
                                                                    }
                                                                }));
                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override10573.clone()), source_ml_source2_nominal_field10571)
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
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10942, _) => {
                                    let source_ml_source2_data_property_value10942 = *source_ml_source2_data_property_value10942;
                                    source_ml_source2_data_property_value10942.clone()
                                }
                            }, (depth + 1f64), false);
                            {
                                let current_3 = match receiver_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10943, _) => {
                                        let source_ml_source2_data_property_value10943 = *source_ml_source2_data_property_value10943;
                                        source_ml_source2_data_property_value10943.clone()
                                    }
                                };
                                {
                                    let arguments_ = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                    {
                                        let source_ml_source1_temporary_index44 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s10944 = self::source_ml_source1_temporary_lower_java_script_call_source_call_expression_loop46(bindings.clone(), depth, false, current_3.clone(), arguments_.clone(), arguments.clone(), source_ml_source1_temporary_index44.clone());
                                            match source_ml_source2_s10944.clone() {
                                                self::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Result::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Done(current_4, arguments__2) => {
                                                    let current_4 = *current_4;
                                                    {
                                                        let result_2 = self::allocate_java_script_call_source_register(current_4.clone());
                                                        {
                                                            let source_ml_source2_nominal_field10950 = match result_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10951, _) => {
                                                                    let source_ml_source2_data_property_value10951 = *source_ml_source2_data_property_value10951;
                                                                    source_ml_source2_data_property_value10951.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field10952 = match result_2.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10953) => {
                                                                        source_ml_source2_data_property_value10953
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override10954 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10945, _) => {
                                                                            let source_ml_source2_data_property_value10945 = *source_ml_source2_data_property_value10945;
                                                                            source_ml_source2_data_property_value10945.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Call(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10946) => {
                                                                            source_ml_source2_data_property_value10946
                                                                        }
                                                                    }, match callee.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10947) => {
                                                                            source_ml_source2_data_property_value10947
                                                                        }
                                                                    }, match receiver_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10948) => {
                                                                            source_ml_source2_data_property_value10948
                                                                        }
                                                                    }, arguments__2.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override10954.clone()), source_ml_source2_nominal_field10952)
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
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11323, _) => {
                                    let source_ml_source2_data_property_value11323 = *source_ml_source2_data_property_value11323;
                                    source_ml_source2_data_property_value11323.clone()
                                }
                            }, (depth + 1f64), true);
                            {
                                let current_5 = match receiver_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11324, _) => {
                                        let source_ml_source2_data_property_value11324 = *source_ml_source2_data_property_value11324;
                                        source_ml_source2_data_property_value11324.clone()
                                    }
                                };
                                {
                                    let arguments__3 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                    {
                                        let source_ml_source1_temporary_index44_2 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s11325 = self::source_ml_source1_temporary_lower_java_script_call_source_call_expression_loop46(bindings.clone(), depth, true, current_5.clone(), arguments__3.clone(), arguments_2.clone(), source_ml_source1_temporary_index44_2.clone());
                                            match source_ml_source2_s11325.clone() {
                                                self::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Result::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Done(current_6, arguments__4) => {
                                                    let current_6 = *current_6;
                                                    {
                                                        let result_3 = self::allocate_java_script_call_source_register(current_6.clone());
                                                        {
                                                            let source_ml_source2_nominal_field11331 = match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11332, _) => {
                                                                    let source_ml_source2_data_property_value11332 = *source_ml_source2_data_property_value11332;
                                                                    source_ml_source2_data_property_value11332.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field11333 = match result_3.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11334) => {
                                                                        source_ml_source2_data_property_value11334
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override11335 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11326, _) => {
                                                                            let source_ml_source2_data_property_value11326 = *source_ml_source2_data_property_value11326;
                                                                            source_ml_source2_data_property_value11326.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Call(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11327) => {
                                                                            source_ml_source2_data_property_value11327
                                                                        }
                                                                    }, match callee_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11328) => {
                                                                            source_ml_source2_data_property_value11328
                                                                        }
                                                                    }, match receiver_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11329) => {
                                                                            source_ml_source2_data_property_value11329
                                                                        }
                                                                    }, arguments__4.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override11335.clone()), source_ml_source2_nominal_field11333)
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
                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11336, _) => {
                        let source_ml_source2_data_property_value11336 = *source_ml_source2_data_property_value11336;
                        source_ml_source2_data_property_value11336.clone()
                    }
                });
                {
                    let source_ml_source2_nominal_field11341 = match saved.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11342, _) => {
                            let source_ml_source2_data_property_value11342 = *source_ml_source2_data_property_value11342;
                            source_ml_source2_data_property_value11342.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field11343 = match saved.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11344) => {
                                source_ml_source2_data_property_value11344
                            }
                        };
                        {
                            let source_ml_source2_nominal_override11345 = self::append_java_script_call_source_instruction(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11337, _) => {
                                    let source_ml_source2_data_property_value11337 = *source_ml_source2_data_property_value11337;
                                    source_ml_source2_data_property_value11337.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11338) => {
                                    source_ml_source2_data_property_value11338
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11339) => {
                                    source_ml_source2_data_property_value11339
                                }
                            }));
                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override11345.clone()), source_ml_source2_nominal_field11343)
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
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11525, _) => {
                                    let source_ml_source2_data_property_value11525 = *source_ml_source2_data_property_value11525;
                                    source_ml_source2_data_property_value11525.clone()
                                }
                            });
                            {
                                let current = self::append_java_script_call_source_instruction(match callee.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11526, _) => {
                                        let source_ml_source2_data_property_value11526 = *source_ml_source2_data_property_value11526;
                                        source_ml_source2_data_property_value11526.clone()
                                    }
                                }, self::JavaScriptCallInstruction::Copy(match callee.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11527) => {
                                        source_ml_source2_data_property_value11527
                                    }
                                }, match selected.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11528) => {
                                        source_ml_source2_data_property_value11528
                                    }
                                }));
                                {
                                    let receiver_register = 0f64;
                                    {
                                        let source_ml_source2_s11535 = if (mode == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]))) {
                                            {
                                                let receiver_register_2 = match callee.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11536) => {
                                                        source_ml_source2_data_property_value11536
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
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11537, _) => {
                                                                let source_ml_source2_data_property_value11537 = *source_ml_source2_data_property_value11537;
                                                                source_ml_source2_data_property_value11537.clone()
                                                            }
                                                        });
                                                        {
                                                            let current_2 = self::append_java_script_call_source_instruction(match held.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11538, _) => {
                                                                    let source_ml_source2_data_property_value11538 = *source_ml_source2_data_property_value11538;
                                                                    source_ml_source2_data_property_value11538.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Copy(match held.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11539) => {
                                                                    source_ml_source2_data_property_value11539
                                                                }
                                                            }, match raw_.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11540) => {
                                                                    source_ml_source2_data_property_value11540
                                                                }
                                                            }));
                                                            {
                                                                let receiver_register_3 = match held.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11541) => {
                                                                        source_ml_source2_data_property_value11541
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
                                        match source_ml_source2_s11535.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48Next(current_3, receiver_register_4) => {
                                                let current_3 = *current_3;
                                                {
                                                    let slot = { let ml_reference_array92 = match current_3.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11542, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value11542.clone()
                                                        }
                                                    }; let ml_reference_projection92 = ml_reference_array92.borrow().len() as f64; ml_reference_projection92 };
                                                    {
                                                        let current_4 = self::append_java_script_call_source_instruction(current_3.clone(), self::JavaScriptCallInstruction::ArgumentListOpen(slot));
                                                        {
                                                            let source_ml_source1_temporary_index47 = self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s11543 = self::source_ml_source1_temporary_lower_java_script_call_source_argument_list_loop49(bindings.clone(), builder.clone(), depth, current_4.clone(), slot, arguments.clone(), source_ml_source1_temporary_index47.clone());
                                                                match source_ml_source2_s11543.clone() {
                                                                    self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Result::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Done(current_5) => {
                                                                        let current_5 = *current_5;
                                                                        {
                                                                            let source_ml_source2_s11549 = if receiver_after_arguments {
                                                                                {
                                                                                    let raw_2 = self::lower_java_script_call_source_expression(receiver.clone(), bindings.clone(), current_5.clone(), (depth + 1f64));
                                                                                    {
                                                                                        let held_2 = self::allocate_java_script_call_source_register(match raw_2.clone() {
                                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11550, _) => {
                                                                                                let source_ml_source2_data_property_value11550 = *source_ml_source2_data_property_value11550;
                                                                                                source_ml_source2_data_property_value11550.clone()
                                                                                            }
                                                                                        });
                                                                                        {
                                                                                            let current_6 = self::append_java_script_call_source_instruction(match held_2.clone() {
                                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11551, _) => {
                                                                                                    let source_ml_source2_data_property_value11551 = *source_ml_source2_data_property_value11551;
                                                                                                    source_ml_source2_data_property_value11551.clone()
                                                                                                }
                                                                                            }, self::JavaScriptCallInstruction::Copy(match held_2.clone() {
                                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11552) => {
                                                                                                    source_ml_source2_data_property_value11552
                                                                                                }
                                                                                            }, match raw_2.clone() {
                                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11553) => {
                                                                                                    source_ml_source2_data_property_value11553
                                                                                                }
                                                                                            }));
                                                                                            {
                                                                                                let receiver_register_5 = match held_2.clone() {
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11554) => {
                                                                                                        source_ml_source2_data_property_value11554
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
                                                                            match source_ml_source2_s11549.clone() {
                                                                                self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50Next(current_7, receiver_register_6) => {
                                                                                    let current_7 = *current_7;
                                                                                    {
                                                                                        let result = self::allocate_java_script_call_source_register(current_7.clone());
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11559 = match result.clone() {
                                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11560, _) => {
                                                                                                    let source_ml_source2_data_property_value11560 = *source_ml_source2_data_property_value11560;
                                                                                                    source_ml_source2_data_property_value11560.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11561 = match result.clone() {
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11562) => {
                                                                                                        source_ml_source2_data_property_value11562
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override11563 = self::append_java_script_call_source_instruction(match result.clone() {
                                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11555, _) => {
                                                                                                            let source_ml_source2_data_property_value11555 = *source_ml_source2_data_property_value11555;
                                                                                                            source_ml_source2_data_property_value11555.clone()
                                                                                                        }
                                                                                                    }, self::JavaScriptCallInstruction::ArgumentListCall(match result.clone() {
                                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11556) => {
                                                                                                            source_ml_source2_data_property_value11556
                                                                                                        }
                                                                                                    }, slot, match callee.clone() {
                                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11557) => {
                                                                                                            source_ml_source2_data_property_value11557
                                                                                                        }
                                                                                                    }, receiver_register_6, mode.clone()));
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override11563.clone()), source_ml_source2_nominal_field11561)
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
            let source_ml_source2_field11747 = self::append_java_script_call_source_instruction(match value.clone() {
                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11745, _) => {
                    let source_ml_source2_data_property_value11745 = *source_ml_source2_data_property_value11745;
                    source_ml_source2_data_property_value11745.clone()
                }
            }, self::JavaScriptCallInstruction::ArgumentListAppend(slot, match value.clone() {
                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11746) => {
                    source_ml_source2_data_property_value11746
                }
            }));
            (source_ml_source2_field11747.clone(), false)
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
                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11749, _) => {
                    let source_ml_source2_data_property_value11749 = *source_ml_source2_data_property_value11749;
                    source_ml_source2_data_property_value11749.clone()
                }
            }, depth, true);
            {
                let value = self::allocate_java_script_call_source_register(match key.clone() {
                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11750, _) => {
                        let source_ml_source2_data_property_value11750 = *source_ml_source2_data_property_value11750;
                        source_ml_source2_data_property_value11750.clone()
                    }
                });
                {
                    let done = self::allocate_java_script_call_source_register(match value.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11751, _) => {
                            let source_ml_source2_data_property_value11751 = *source_ml_source2_data_property_value11751;
                            source_ml_source2_data_property_value11751.clone()
                        }
                    });
                    {
                        let iterator = self::find_java_script_call_source_spread_iterator_slot(match match done.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11752, _) => {
                                let source_ml_source2_data_property_value11752 = *source_ml_source2_data_property_value11752;
                                source_ml_source2_data_property_value11752.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11753, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value11753.clone()
                            }
                        });
                        if ((iterator >= (65536f64)) || ((match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value11755, _, _, _, _, _) => {
                                source_ml_source2_data_property_value11755
                            }
                        }) >= (1024f64))) {
                            std::rc::Rc::new(std::cell::RefCell::new((builder.clone(), true)))
                        } else {
                            {
                                let acquired = self::append_java_script_call_source_instruction(match done.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11758, _) => {
                                        let source_ml_source2_data_property_value11758 = *source_ml_source2_data_property_value11758;
                                        source_ml_source2_data_property_value11758.clone()
                                    }
                                }, {
                                    let source_ml_source2_constructor_field11760 = match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11759) => {
                                            source_ml_source2_data_property_value11759
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field11762 = match done.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11761) => {
                                                source_ml_source2_data_property_value11761
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field11764 = match input.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11763) => {
                                                    source_ml_source2_data_property_value11763
                                                }
                                            };
                                            {
                                                let source_ml_source2_constructor_field11766 = match key.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11765) => {
                                                        source_ml_source2_data_property_value11765
                                                    }
                                                };
                                                self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_constructor_field11760, source_ml_source2_constructor_field11762, source_ml_source2_constructor_field11764, source_ml_source2_constructor_field11766, iterator, self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]), realm.clone(), None::<(Option<bool>,)>)
                                            }
                                        }
                                    }
                                });
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_field11772 = self::append_java_script_call_source_argument_spread(acquired.clone(), slot, iterator, realm.clone(), match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11770) => {
                                            source_ml_source2_data_property_value11770
                                        }
                                    }, match done.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11771) => {
                                            source_ml_source2_data_property_value11771
                                        }
                                    }, handlers);
                                    (source_ml_source2_field11772.clone(), false)
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
            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11774, _, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value11774.clone()
            }
        }; let ml_reference_projection93 = ml_reference_array93.borrow().len() as f64; ml_reference_projection93 };
        {
            let current = self::append_java_script_call_source_instruction(builder.clone(), self::JavaScriptCallInstruction::PushFinally(0f64));
            {
                let start = { let ml_reference_array94 = match current.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11775, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value11775.clone()
                    }
                }; let ml_reference_projection94 = ml_reference_array94.borrow().len() as f64; ml_reference_projection94 };
                {
                    let current_2 = self::append_java_script_call_source_instruction(current.clone(), {
                        let source_ml_source2_constructor_field11778 = 0f64;
                        {
                            let source_ml_source2_constructor_field11779 = 0f64;
                            self::JavaScriptCallInstruction::IteratorProtocol(value, done, source_ml_source2_constructor_field11778, source_ml_source2_constructor_field11779, iterator, self::ml_string::Text::from_units(vec![115, 116, 101, 112]), realm.clone(), None::<(Option<bool>,)>)
                        }
                    });
                    {
                        let branch = { let ml_reference_array95 = match current_2.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11783, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value11783.clone()
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
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11784, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value11784.clone()
                                            }
                                        }; let ml_reference_projection96 = ml_reference_array96.borrow().len() as f64; ml_reference_projection96 };
                                        {
                                            let current_6 = self::append_java_script_call_source_instruction(current_5.clone(), self::JavaScriptCallInstruction::CompleteJump(0f64, handlers));
                                            {
                                                let cleanup = { let ml_reference_array97 = match current_6.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11785, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value11785.clone()
                                                    }
                                                }; let ml_reference_projection97 = ml_reference_array97.borrow().len() as f64; ml_reference_projection97 };
                                                {
                                                    let current_7 = self::append_java_script_call_source_instruction(current_6.clone(), {
                                                        let source_ml_source2_constructor_field11788 = 0f64;
                                                        {
                                                            let source_ml_source2_constructor_field11789 = 0f64;
                                                            self::JavaScriptCallInstruction::IteratorProtocol(value, done, source_ml_source2_constructor_field11788, source_ml_source2_constructor_field11789, iterator, self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]), realm.clone(), None::<(Option<bool>,)>)
                                                        }
                                                    });
                                                    {
                                                        let current_8 = self::append_java_script_call_source_instruction(current_7.clone(), self::JavaScriptCallInstruction::FinishFinally);
                                                        {
                                                            let end = { let ml_reference_array98 = match current_8.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11793, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value11793.clone()
                                                                }
                                                            }; let ml_reference_projection98 = ml_reference_array98.borrow().len() as f64; ml_reference_projection98 };
                                                            {
                                                                let source_ml_source2_nominal_spread11795 = current_8.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field11796 = match source_ml_source2_nominal_spread11795.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11797, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value11797.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field11798 = match source_ml_source2_nominal_spread11795.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value11799, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value11799
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field11800 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value11801, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value11801
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field11802 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value11803, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value11803
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field11804 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value11805, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value11805
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11806 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value11807, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value11807.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11808 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value11809, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value11809.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11810 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value11811, _, _) => {
                                                                                                        source_ml_source2_data_property_value11811.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11812 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value11813, _) => {
                                                                                                            source_ml_source2_data_property_value11813.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11814 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11815) => {
                                                                                                                source_ml_source2_data_property_value11815.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override11816 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values99 = { let ml_reference_value100 = (match current_8.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11794, _, _, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value11794.clone()
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
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override11816.clone(), source_ml_source2_nominal_field11798, source_ml_source2_nominal_field11800, source_ml_source2_nominal_field11802, source_ml_source2_nominal_field11804, source_ml_source2_nominal_field11806.clone(), source_ml_source2_nominal_field11808.clone(), source_ml_source2_nominal_field11810.clone(), source_ml_source2_nominal_field11812.clone(), source_ml_source2_nominal_field11814.clone())
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
