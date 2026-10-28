// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Result;
use super::MlTemporaryLowerJavaScriptCallSourceExpressionLoop7Result;
use super::MlTemporaryLowerJavaScriptCallSourceExpressionLoop9Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::ml;
use super::ml_array;
use super::read_java_script_call_constant_error;
use super::refuse_java_script_call_source;
use super::source_ml_temporary_lower_java_script_call_source_expression_loop6;
use super::source_ml_temporary_lower_java_script_call_source_expression_loop7;
use super::source_ml_temporary_lower_java_script_call_source_expression_loop8;
use super::source_ml_temporary_lower_java_script_call_source_expression_loop9;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_expression(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value194) => {
            source_ml_source1_data_property_value194.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((depth > (128f64)) || ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value196, _, _) => {
                source_ml_source1_data_property_value196
            }
        }) >= (65536f64))) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its structural budget")
        } else {
            {
                let state = {
                    let source_ml_source1_nominal_spread198 = builder.clone();
                    {
                        let source_ml_source1_nominal_field199 = match source_ml_source1_nominal_spread198.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value200, _, _, _, _) => {
                                source_ml_source1_data_property_value200
                            }
                        };
                        {
                            let source_ml_source1_nominal_field201 = match source_ml_source1_nominal_spread198.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value202, _, _, _) => {
                                    source_ml_source1_data_property_value202
                                }
                            };
                            {
                                let source_ml_source1_nominal_field203 = match source_ml_source1_nominal_spread198.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value204, _, _) => {
                                        source_ml_source1_data_property_value204
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field205 = match source_ml_source1_nominal_spread198.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value206, _) => {
                                            source_ml_source1_data_property_value206
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field207 = match source_ml_source1_nominal_spread198.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value208) => {
                                                source_ml_source1_data_property_value208.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_override209 = (match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value197, _, _) => {
                                                    source_ml_source1_data_property_value197
                                                }
                                            } + 1f64);
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_field199, source_ml_source1_nominal_field201, source_ml_source1_nominal_override209, source_ml_source1_nominal_field205, source_ml_source1_nominal_field207.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                };
                match source.clone() {
                    self::JavaScriptCallSource::SourceConstant(value) => {
                        let value = *value;
                        {
                            let error = self::read_java_script_call_constant_error(value.clone());
                            if (error != "") {
                                self::refuse_java_script_call_source(state.clone(), &(error))
                            } else {
                                {
                                    let result = self::allocate_java_script_call_source_register(state.clone());
                                    {
                                        let source_ml_source1_nominal_spread216 = result.clone();
                                        {
                                            let source_ml_source1_nominal_field217 = match source_ml_source1_nominal_spread216.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value218, _) => {
                                                    let source_ml_source1_data_property_value218 = *source_ml_source1_data_property_value218;
                                                    source_ml_source1_data_property_value218.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field219 = match source_ml_source1_nominal_spread216.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value220) => {
                                                        source_ml_source1_data_property_value220
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override221 = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value214, _) => {
                                                            let source_ml_source1_data_property_value214 = *source_ml_source1_data_property_value214;
                                                            source_ml_source1_data_property_value214.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Constant(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value215) => {
                                                            source_ml_source1_data_property_value215
                                                        }
                                                    }, Box::new(value.clone())));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override221.clone()), source_ml_source1_nominal_field219)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceBinding(identity) => {
                        if !(self::test_java_script_call_code_coordinate(identity, 65536f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source binding identities require exact nonnegative coordinates")
                        } else {
                            {
                                let index = ((bindings.len() as f64) - 1f64);
                                {
                                    let source_ml_source1_s222 = self::source_ml_temporary_lower_java_script_call_source_expression_loop6(source.clone(), bindings, state.clone(), index);
                                    match source_ml_source1_s222.clone() {
                                        self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Done => {
                                            self::refuse_java_script_call_source(state.clone(), "a source binding reference has no actual lexical owner")
                                        }
                                        self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Return(source_ml_temporary_result6) => {
                                            let source_ml_temporary_result6 = *source_ml_temporary_result6;
                                            source_ml_temporary_result6.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceLet(identity_2, source_value, body) => {
                        let source_value = *source_value;
                        let body = *body;
                        if (!(self::test_java_script_call_code_coordinate(identity_2, 65536f64)) || { let ml_values1 = bindings.clone(); let ml_callback1 = { let identity_2 = identity_2; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source1_data_property_value224, _) => {
                                source_ml_source1_data_property_value224
                            }
                        }) == identity_2) }) }; ml_values1.iter().cloned().enumerate().any(|(ml_index1, ml_value1)| ml_callback1(ml_value1)) }) {
                            self::refuse_java_script_call_source(state.clone(), "a source binding owner must be unique in its lexical scope")
                        } else {
                            {
                                let value_2 = self::lower_java_script_call_source_expression(source_value.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                self::lower_java_script_call_source_expression(body.clone(), {
                                    let source_ml_source1_concat_value226 = bindings;
                                    {
                                        let source_ml_source1_concat_value227 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity_2, match value_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value225) => {
                                                source_ml_source1_data_property_value225
                                            }
                                        })];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source1_concat_value226), source_ml_source1_concat_value227)
                                    }
                                }, match value_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value228, _) => {
                                        let source_ml_source1_data_property_value228 = *source_ml_source1_data_property_value228;
                                        source_ml_source1_data_property_value228.clone()
                                    }
                                }, (depth + 1f64))
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceIf(source_ml_source1_f210, source_ml_source1_f211, source_ml_source1_f212) => {
                        let source_ml_source1_f210 = *source_ml_source1_f210;
                        let source_ml_source1_f211 = *source_ml_source1_f211;
                        let source_ml_source1_f212 = *source_ml_source1_f212;
                        self::lower_java_script_call_source_branch(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(source_ml_source1_f213, source_left, source_right) => {
                        let source_left = *source_left;
                        let source_right = *source_right;
                        {
                            let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings, match left.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value229, _) => {
                                        let source_ml_source1_data_property_value229 = *source_ml_source1_data_property_value229;
                                        source_ml_source1_data_property_value229.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_2 = self::allocate_java_script_call_source_register(match right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value230, _) => {
                                            let source_ml_source1_data_property_value230 = *source_ml_source1_data_property_value230;
                                            source_ml_source1_data_property_value230.clone()
                                        }
                                    });
                                    {
                                        let operator = source_ml_source1_f213.clone();
                                        {
                                            let source_ml_source1_f239 = source_left.clone();
                                            {
                                                let source_ml_source1_f240 = source_right.clone();
                                                if !(self::ml_array::includes(&vec![String::from("add"), String::from("subtract"), String::from("multiply"), String::from("divide"), String::from("remainder"), String::from("power"), String::from("lessThan"), String::from("equal")], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::refuse_java_script_call_source(match result_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value254, _) => {
                                                            let source_ml_source1_data_property_value254 = *source_ml_source1_data_property_value254;
                                                            source_ml_source1_data_property_value254.clone()
                                                        }
                                                    }, "source Number lowering requires its exact registered operator")
                                                } else {
                                                    {
                                                        let source_ml_source1_nominal_spread259 = result_2.clone();
                                                        {
                                                            let source_ml_source1_nominal_field260 = match source_ml_source1_nominal_spread259.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value261, _) => {
                                                                    let source_ml_source1_data_property_value261 = *source_ml_source1_data_property_value261;
                                                                    source_ml_source1_data_property_value261.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field262 = match source_ml_source1_nominal_spread259.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value263) => {
                                                                        source_ml_source1_data_property_value263
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_override264 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value255, _) => {
                                                                            let source_ml_source1_data_property_value255 = *source_ml_source1_data_property_value255;
                                                                            source_ml_source1_data_property_value255.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::NumberBinary(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value256) => {
                                                                            source_ml_source1_data_property_value256
                                                                        }
                                                                    }, match left.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value257) => {
                                                                            source_ml_source1_data_property_value257
                                                                        }
                                                                    }, match right.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value258) => {
                                                                            source_ml_source1_data_property_value258
                                                                        }
                                                                    }, operator.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override264.clone()), source_ml_source1_nominal_field262)
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
                    self::JavaScriptCallSource::SourceSameValue(source_left_2, source_right_2) => {
                        let source_left_2 = *source_left_2;
                        let source_right_2 = *source_right_2;
                        {
                            let left_2 = self::lower_java_script_call_source_expression(source_left_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right_2 = self::lower_java_script_call_source_expression(source_right_2.clone(), bindings, match left_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value265, _) => {
                                        let source_ml_source1_data_property_value265 = *source_ml_source1_data_property_value265;
                                        source_ml_source1_data_property_value265.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_3 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value266, _) => {
                                            let source_ml_source1_data_property_value266 = *source_ml_source1_data_property_value266;
                                            source_ml_source1_data_property_value266.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source1_f277 = source_left_2.clone();
                                        {
                                            let source_ml_source1_f278 = source_right_2.clone();
                                            {
                                                let source_ml_source1_nominal_spread294 = result_3.clone();
                                                {
                                                    let source_ml_source1_nominal_field295 = match source_ml_source1_nominal_spread294.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value296, _) => {
                                                            let source_ml_source1_data_property_value296 = *source_ml_source1_data_property_value296;
                                                            source_ml_source1_data_property_value296.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field297 = match source_ml_source1_nominal_spread294.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value298) => {
                                                                source_ml_source1_data_property_value298
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_override299 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value290, _) => {
                                                                    let source_ml_source1_data_property_value290 = *source_ml_source1_data_property_value290;
                                                                    source_ml_source1_data_property_value290.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::SameValue(match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value291) => {
                                                                    source_ml_source1_data_property_value291
                                                                }
                                                            }, match left_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value292) => {
                                                                    source_ml_source1_data_property_value292
                                                                }
                                                            }, match right_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value293) => {
                                                                    source_ml_source1_data_property_value293
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override299.clone()), source_ml_source1_nominal_field297)
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
                    self::JavaScriptCallSource::SourceStrictEqual(source_left_3, source_right_3) => {
                        let source_left_3 = *source_left_3;
                        let source_right_3 = *source_right_3;
                        {
                            let left_3 = self::lower_java_script_call_source_expression(source_left_3.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right_3 = self::lower_java_script_call_source_expression(source_right_3.clone(), bindings, match left_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value300, _) => {
                                        let source_ml_source1_data_property_value300 = *source_ml_source1_data_property_value300;
                                        source_ml_source1_data_property_value300.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value301, _) => {
                                            let source_ml_source1_data_property_value301 = *source_ml_source1_data_property_value301;
                                            source_ml_source1_data_property_value301.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source1_f314 = source_left_3.clone();
                                        {
                                            let source_ml_source1_f315 = source_right_3.clone();
                                            {
                                                let source_ml_source1_nominal_spread329 = result_4.clone();
                                                {
                                                    let source_ml_source1_nominal_field330 = match source_ml_source1_nominal_spread329.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value331, _) => {
                                                            let source_ml_source1_data_property_value331 = *source_ml_source1_data_property_value331;
                                                            source_ml_source1_data_property_value331.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field332 = match source_ml_source1_nominal_spread329.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value333) => {
                                                                source_ml_source1_data_property_value333
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_override334 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value325, _) => {
                                                                    let source_ml_source1_data_property_value325 = *source_ml_source1_data_property_value325;
                                                                    source_ml_source1_data_property_value325.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::StrictEqual(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value326) => {
                                                                    source_ml_source1_data_property_value326
                                                                }
                                                            }, match left_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value327) => {
                                                                    source_ml_source1_data_property_value327
                                                                }
                                                            }, match right_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value328) => {
                                                                    source_ml_source1_data_property_value328
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override334.clone()), source_ml_source1_nominal_field332)
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
                    self::JavaScriptCallSource::SourceRecur(arguments) => {
                        if (((arguments.len() as f64)) != (match state.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value336, _) => {
                                source_ml_source1_data_property_value336
                            }
                        })) {
                            self::refuse_java_script_call_source(state.clone(), "loop updates require every actual parameter operand")
                        } else {
                            {
                                let current = state.clone();
                                {
                                    let arguments_ = Vec::<f64>::new();
                                    {
                                        let source_ml_temporary_values4 = arguments;
                                        {
                                            let source_ml_temporary_index4 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_s337 = self::source_ml_temporary_lower_java_script_call_source_expression_loop7(bindings, depth, current.clone(), arguments_, source_ml_temporary_values4, source_ml_temporary_index4.clone());
                                                match source_ml_source1_s337.clone() {
                                                    self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop7Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop7Done(current_2, arguments__2) => {
                                                        let current_2 = *current_2;
                                                        {
                                                            let index_2 = 0f64;
                                                            {
                                                                let current_3 = self::source_ml_temporary_lower_java_script_call_source_expression_loop8(current_2.clone(), &(arguments__2), index_2);
                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(current_3.clone(), self::JavaScriptCallInstruction::Jump(0f64))), 0f64)
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
                    self::JavaScriptCallSource::SourceSet(source_object, source_key, source_value_2) => {
                        let source_object = *source_object;
                        let source_key = *source_key;
                        let source_value_2 = *source_value_2;
                        {
                            let object = self::lower_java_script_call_source_expression(source_object.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let key = self::lower_java_script_call_source_expression(source_key.clone(), bindings.clone(), match object.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value338, _) => {
                                        let source_ml_source1_data_property_value338 = *source_ml_source1_data_property_value338;
                                        source_ml_source1_data_property_value338.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let value_3 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value339, _) => {
                                            let source_ml_source1_data_property_value339 = *source_ml_source1_data_property_value339;
                                            source_ml_source1_data_property_value339.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let result_5 = self::allocate_java_script_call_source_register(match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value340, _) => {
                                                let source_ml_source1_data_property_value340 = *source_ml_source1_data_property_value340;
                                                source_ml_source1_data_property_value340.clone()
                                            }
                                        });
                                        {
                                            let write = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value341, _) => {
                                                    let source_ml_source1_data_property_value341 = *source_ml_source1_data_property_value341;
                                                    source_ml_source1_data_property_value341.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinarySet(match result_5.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value342) => {
                                                    source_ml_source1_data_property_value342
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value343) => {
                                                    source_ml_source1_data_property_value343
                                                }
                                            }, match key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value344) => {
                                                    source_ml_source1_data_property_value344
                                                }
                                            }, match value_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value345) => {
                                                    source_ml_source1_data_property_value345
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value346) => {
                                                    source_ml_source1_data_property_value346
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_5.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value347) => {
                                                    source_ml_source1_data_property_value347
                                                }
                                            }))), match value_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value348) => {
                                                    source_ml_source1_data_property_value348
                                                }
                                            })
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceGet(source_object_2, source_key_2) => {
                        let source_object_2 = *source_object_2;
                        let source_key_2 = *source_key_2;
                        {
                            let object_2 = self::lower_java_script_call_source_expression(source_object_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let key_2 = self::lower_java_script_call_source_expression(source_key_2.clone(), bindings, match object_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value349, _) => {
                                        let source_ml_source1_data_property_value349 = *source_ml_source1_data_property_value349;
                                        source_ml_source1_data_property_value349.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_6 = self::allocate_java_script_call_source_register(match key_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value350, _) => {
                                            let source_ml_source1_data_property_value350 = *source_ml_source1_data_property_value350;
                                            source_ml_source1_data_property_value350.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source1_nominal_spread356 = result_6.clone();
                                        {
                                            let source_ml_source1_nominal_field357 = match source_ml_source1_nominal_spread356.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value358, _) => {
                                                    let source_ml_source1_data_property_value358 = *source_ml_source1_data_property_value358;
                                                    source_ml_source1_data_property_value358.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field359 = match source_ml_source1_nominal_spread356.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value360) => {
                                                        source_ml_source1_data_property_value360
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override361 = self::append_java_script_call_source_instruction(match result_6.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value351, _) => {
                                                            let source_ml_source1_data_property_value351 = *source_ml_source1_data_property_value351;
                                                            source_ml_source1_data_property_value351.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryGet(match result_6.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value352) => {
                                                            source_ml_source1_data_property_value352
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value353) => {
                                                            source_ml_source1_data_property_value353
                                                        }
                                                    }, match key_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value354) => {
                                                            source_ml_source1_data_property_value354
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value355) => {
                                                            source_ml_source1_data_property_value355
                                                        }
                                                    }));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override361.clone()), source_ml_source1_nominal_field359)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceCall(source_callee, source_receiver, arguments_2) => {
                        let source_callee = *source_callee;
                        let source_receiver = *source_receiver;
                        if (((arguments_2.len() as f64)) > (4096f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source call operands exceed their explicit budget")
                        } else {
                            {
                                let callee = self::lower_java_script_call_source_expression(source_callee.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                {
                                    let receiver = self::lower_java_script_call_source_expression(source_receiver.clone(), bindings.clone(), match callee.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value362, _) => {
                                            let source_ml_source1_data_property_value362 = *source_ml_source1_data_property_value362;
                                            source_ml_source1_data_property_value362.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let current_4 = match receiver.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value363, _) => {
                                                let source_ml_source1_data_property_value363 = *source_ml_source1_data_property_value363;
                                                source_ml_source1_data_property_value363.clone()
                                            }
                                        };
                                        {
                                            let arguments__3 = Vec::<f64>::new();
                                            {
                                                let source_ml_temporary_values5 = arguments_2;
                                                {
                                                    let source_ml_temporary_index5 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s364 = self::source_ml_temporary_lower_java_script_call_source_expression_loop9(bindings, depth, current_4.clone(), arguments__3, source_ml_temporary_values5, source_ml_temporary_index5.clone());
                                                        match source_ml_source1_s364.clone() {
                                                            self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop9Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop9Done(current_5, arguments__4) => {
                                                                let current_5 = *current_5;
                                                                {
                                                                    let result_7 = self::allocate_java_script_call_source_register(current_5.clone());
                                                                    {
                                                                        let source_ml_source1_nominal_spread369 = result_7.clone();
                                                                        {
                                                                            let source_ml_source1_nominal_field370 = match source_ml_source1_nominal_spread369.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value371, _) => {
                                                                                    let source_ml_source1_data_property_value371 = *source_ml_source1_data_property_value371;
                                                                                    source_ml_source1_data_property_value371.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field372 = match source_ml_source1_nominal_spread369.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value373) => {
                                                                                        source_ml_source1_data_property_value373
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_override374 = self::append_java_script_call_source_instruction(match result_7.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value365, _) => {
                                                                                            let source_ml_source1_data_property_value365 = *source_ml_source1_data_property_value365;
                                                                                            source_ml_source1_data_property_value365.clone()
                                                                                        }
                                                                                    }, self::JavaScriptCallInstruction::Call(match result_7.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value366) => {
                                                                                            source_ml_source1_data_property_value366
                                                                                        }
                                                                                    }, match callee.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value367) => {
                                                                                            source_ml_source1_data_property_value367
                                                                                        }
                                                                                    }, match receiver.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value368) => {
                                                                                            source_ml_source1_data_property_value368
                                                                                        }
                                                                                    }, arguments__4));
                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override374.clone()), source_ml_source1_nominal_field372)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_branch(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(source_ml_source1_f375) => {
            let source_ml_source1_f375 = *source_ml_source1_f375;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceBinding(source_ml_source1_f376) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceLet(source_ml_source1_f377, source_ml_source1_f378, source_ml_source1_f379) => {
            let source_ml_source1_f378 = *source_ml_source1_f378;
            let source_ml_source1_f379 = *source_ml_source1_f379;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceIf(source_condition, source_yes, source_no) => {
            let source_condition = *source_condition;
            let source_yes = *source_yes;
            let source_no = *source_no;
            {
                let condition = self::lower_java_script_call_source_expression(source_condition.clone(), bindings.clone(), builder.clone(), (depth + 1f64));
                {
                    let result = self::allocate_java_script_call_source_register(match condition.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value396, _) => {
                            let source_ml_source1_data_property_value396 = *source_ml_source1_data_property_value396;
                            source_ml_source1_data_property_value396.clone()
                        }
                    });
                    {
                        let branch = ((match match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value397, _) => {
                                let source_ml_source1_data_property_value397 = *source_ml_source1_data_property_value397;
                                source_ml_source1_data_property_value397.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value398, _, _, _, _) => {
                                source_ml_source1_data_property_value398
                            }
                        }).len() as f64);
                        {
                            let start = self::append_java_script_call_source_instruction(match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value399, _) => {
                                    let source_ml_source1_data_property_value399 = *source_ml_source1_data_property_value399;
                                    source_ml_source1_data_property_value399.clone()
                                }
                            }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value400) => {
                                    source_ml_source1_data_property_value400
                                }
                            }, 0f64, 0f64));
                            {
                                let yes_position = ((match &start { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value401, _, _, _, _) => source_ml_source1_data_property_value401.as_slice() }).len() as f64);
                                {
                                    let yes = self::lower_java_script_call_source_expression(source_yes.clone(), bindings.clone(), start.clone(), (depth + 1f64));
                                    {
                                        let copied_yes = self::append_java_script_call_source_instruction(match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value402, _) => {
                                                let source_ml_source1_data_property_value402 = *source_ml_source1_data_property_value402;
                                                source_ml_source1_data_property_value402.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value403) => {
                                                source_ml_source1_data_property_value403
                                            }
                                        }, match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value404) => {
                                                source_ml_source1_data_property_value404
                                            }
                                        }));
                                        {
                                            let jump = ((match &copied_yes { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value405, _, _, _, _) => source_ml_source1_data_property_value405.as_slice() }).len() as f64);
                                            {
                                                let alternative = self::append_java_script_call_source_instruction(copied_yes.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                {
                                                    let no_position = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value406, _, _, _, _) => source_ml_source1_data_property_value406.as_slice() }).len() as f64);
                                                    {
                                                        let no = self::lower_java_script_call_source_expression(source_no.clone(), bindings, alternative.clone(), (depth + 1f64));
                                                        {
                                                            let copied_no = self::append_java_script_call_source_instruction(match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value407, _) => {
                                                                    let source_ml_source1_data_property_value407 = *source_ml_source1_data_property_value407;
                                                                    source_ml_source1_data_property_value407.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value408) => {
                                                                    source_ml_source1_data_property_value408
                                                                }
                                                            }, match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value409) => {
                                                                    source_ml_source1_data_property_value409
                                                                }
                                                            }));
                                                            {
                                                                let end = ((match &copied_no { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value410, _, _, _, _) => source_ml_source1_data_property_value410.as_slice() }).len() as f64);
                                                                if ((match copied_no.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value412) => {
                                                                        source_ml_source1_data_property_value412.clone()
                                                                    }
                                                                }) != "") {
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(copied_no.clone()), 0f64)
                                                                } else {
                                                                    {
                                                                        let instructions = { let ml_values2 = match copied_no.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value413, _, _, _, _) => {
                                                                                source_ml_source1_data_property_value413
                                                                            }
                                                                        }; let ml_callback2 = { let branch = branch; let condition = condition.clone(); let yes_position = yes_position; let no_position = no_position; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                            self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value414) => {
                                                                                    source_ml_source1_data_property_value414
                                                                                }
                                                                            }, yes_position, no_position)
                                                                        } else {
                                                                            if (index == jump) {
                                                                                self::JavaScriptCallInstruction::Jump(end)
                                                                            } else {
                                                                                entry.clone()
                                                                            }
                                                                        } }) }; ml_values2.iter().cloned().enumerate().map(|(ml_index2, ml_value2)| ml_callback2(ml_value2, (ml_index2 as f64))).collect::<Vec<_>>() };
                                                                        self::JavaScriptCallSourceResult::SourceResult(Box::new({
                                                                            let source_ml_source1_nominal_spread415 = copied_no.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field416 = match source_ml_source1_nominal_spread415.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value417, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value417
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field418 = match source_ml_source1_nominal_spread415.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value419, _, _, _) => {
                                                                                            source_ml_source1_data_property_value419
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field420 = match source_ml_source1_nominal_spread415.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value421, _, _) => {
                                                                                                source_ml_source1_data_property_value421
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field422 = match source_ml_source1_nominal_spread415.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value423, _) => {
                                                                                                    source_ml_source1_data_property_value423
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field424 = match source_ml_source1_nominal_spread415.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value425) => {
                                                                                                        source_ml_source1_data_property_value425.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_override426 = instructions;
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_override426, source_ml_source1_nominal_field418, source_ml_source1_nominal_field420, source_ml_source1_nominal_field422, source_ml_source1_nominal_field424.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }), match result.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value427) => {
                                                                                source_ml_source1_data_property_value427
                                                                            }
                                                                        })
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
        self::JavaScriptCallSource::SourceNumberBinary(source_ml_source1_f380, source_ml_source1_f381, source_ml_source1_f382) => {
            let source_ml_source1_f381 = *source_ml_source1_f381;
            let source_ml_source1_f382 = *source_ml_source1_f382;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSameValue(source_ml_source1_f383, source_ml_source1_f384) => {
            let source_ml_source1_f383 = *source_ml_source1_f383;
            let source_ml_source1_f384 = *source_ml_source1_f384;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceStrictEqual(source_ml_source1_f385, source_ml_source1_f386) => {
            let source_ml_source1_f385 = *source_ml_source1_f385;
            let source_ml_source1_f386 = *source_ml_source1_f386;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceRecur(source_ml_source1_f387) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSet(source_ml_source1_f388, source_ml_source1_f389, source_ml_source1_f390) => {
            let source_ml_source1_f388 = *source_ml_source1_f388;
            let source_ml_source1_f389 = *source_ml_source1_f389;
            let source_ml_source1_f390 = *source_ml_source1_f390;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceGet(source_ml_source1_f391, source_ml_source1_f392) => {
            let source_ml_source1_f391 = *source_ml_source1_f391;
            let source_ml_source1_f392 = *source_ml_source1_f392;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceCall(source_ml_source1_f393, source_ml_source1_f394, source_ml_source1_f395) => {
            let source_ml_source1_f393 = *source_ml_source1_f393;
            let source_ml_source1_f394 = *source_ml_source1_f394;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
    }
}
