// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceResult;
use super::MlTemporaryCompileJavaScriptCallSourceLoop10Result;
use super::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Result;
use super::MlTemporaryLowerJavaScriptCallSourceExpressionLoop7Result;
use super::MlTemporaryLowerJavaScriptCallSourceExpressionLoop9Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_java_script_call_constant_error;
use super::refuse_java_script_call_source;
use super::source_ml_temporary_compile_java_script_call_source_loop10;
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
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value251, _) => {
                                                            let source_ml_source1_data_property_value251 = *source_ml_source1_data_property_value251;
                                                            source_ml_source1_data_property_value251.clone()
                                                        }
                                                    }, "source Number lowering requires its exact registered operator")
                                                } else {
                                                    {
                                                        let source_ml_source1_nominal_spread256 = result_2.clone();
                                                        {
                                                            let source_ml_source1_nominal_field257 = match source_ml_source1_nominal_spread256.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value258, _) => {
                                                                    let source_ml_source1_data_property_value258 = *source_ml_source1_data_property_value258;
                                                                    source_ml_source1_data_property_value258.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field259 = match source_ml_source1_nominal_spread256.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value260) => {
                                                                        source_ml_source1_data_property_value260
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_override261 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value252, _) => {
                                                                            let source_ml_source1_data_property_value252 = *source_ml_source1_data_property_value252;
                                                                            source_ml_source1_data_property_value252.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::NumberBinary(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value253) => {
                                                                            source_ml_source1_data_property_value253
                                                                        }
                                                                    }, match left.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value254) => {
                                                                            source_ml_source1_data_property_value254
                                                                        }
                                                                    }, match right.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value255) => {
                                                                            source_ml_source1_data_property_value255
                                                                        }
                                                                    }, operator.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override261.clone()), source_ml_source1_nominal_field259)
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value262, _) => {
                                        let source_ml_source1_data_property_value262 = *source_ml_source1_data_property_value262;
                                        source_ml_source1_data_property_value262.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_3 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value263, _) => {
                                            let source_ml_source1_data_property_value263 = *source_ml_source1_data_property_value263;
                                            source_ml_source1_data_property_value263.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source1_f274 = source_left_2.clone();
                                        {
                                            let source_ml_source1_f275 = source_right_2.clone();
                                            {
                                                let source_ml_source1_nominal_spread288 = result_3.clone();
                                                {
                                                    let source_ml_source1_nominal_field289 = match source_ml_source1_nominal_spread288.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value290, _) => {
                                                            let source_ml_source1_data_property_value290 = *source_ml_source1_data_property_value290;
                                                            source_ml_source1_data_property_value290.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field291 = match source_ml_source1_nominal_spread288.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value292) => {
                                                                source_ml_source1_data_property_value292
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_override293 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value284, _) => {
                                                                    let source_ml_source1_data_property_value284 = *source_ml_source1_data_property_value284;
                                                                    source_ml_source1_data_property_value284.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::SameValue(match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value285) => {
                                                                    source_ml_source1_data_property_value285
                                                                }
                                                            }, match left_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value286) => {
                                                                    source_ml_source1_data_property_value286
                                                                }
                                                            }, match right_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value287) => {
                                                                    source_ml_source1_data_property_value287
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override293.clone()), source_ml_source1_nominal_field291)
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value294, _) => {
                                        let source_ml_source1_data_property_value294 = *source_ml_source1_data_property_value294;
                                        source_ml_source1_data_property_value294.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value295, _) => {
                                            let source_ml_source1_data_property_value295 = *source_ml_source1_data_property_value295;
                                            source_ml_source1_data_property_value295.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source1_f308 = source_left_3.clone();
                                        {
                                            let source_ml_source1_f309 = source_right_3.clone();
                                            {
                                                let source_ml_source1_nominal_spread320 = result_4.clone();
                                                {
                                                    let source_ml_source1_nominal_field321 = match source_ml_source1_nominal_spread320.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value322, _) => {
                                                            let source_ml_source1_data_property_value322 = *source_ml_source1_data_property_value322;
                                                            source_ml_source1_data_property_value322.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field323 = match source_ml_source1_nominal_spread320.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value324) => {
                                                                source_ml_source1_data_property_value324
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_override325 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value316, _) => {
                                                                    let source_ml_source1_data_property_value316 = *source_ml_source1_data_property_value316;
                                                                    source_ml_source1_data_property_value316.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::StrictEqual(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value317) => {
                                                                    source_ml_source1_data_property_value317
                                                                }
                                                            }, match left_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value318) => {
                                                                    source_ml_source1_data_property_value318
                                                                }
                                                            }, match right_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value319) => {
                                                                    source_ml_source1_data_property_value319
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override325.clone()), source_ml_source1_nominal_field323)
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
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value327, _) => {
                                source_ml_source1_data_property_value327
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
                                                let source_ml_source1_s328 = self::source_ml_temporary_lower_java_script_call_source_expression_loop7(bindings, depth, current.clone(), arguments_, source_ml_temporary_values4, source_ml_temporary_index4.clone());
                                                match source_ml_source1_s328.clone() {
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
                    self::JavaScriptCallSource::SourceGet(source_object, source_key) => {
                        let source_object = *source_object;
                        let source_key = *source_key;
                        {
                            let object = self::lower_java_script_call_source_expression(source_object.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let key = self::lower_java_script_call_source_expression(source_key.clone(), bindings, match object.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value329, _) => {
                                        let source_ml_source1_data_property_value329 = *source_ml_source1_data_property_value329;
                                        source_ml_source1_data_property_value329.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_5 = self::allocate_java_script_call_source_register(match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value330, _) => {
                                            let source_ml_source1_data_property_value330 = *source_ml_source1_data_property_value330;
                                            source_ml_source1_data_property_value330.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source1_nominal_spread336 = result_5.clone();
                                        {
                                            let source_ml_source1_nominal_field337 = match source_ml_source1_nominal_spread336.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value338, _) => {
                                                    let source_ml_source1_data_property_value338 = *source_ml_source1_data_property_value338;
                                                    source_ml_source1_data_property_value338.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field339 = match source_ml_source1_nominal_spread336.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value340) => {
                                                        source_ml_source1_data_property_value340
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override341 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value331, _) => {
                                                            let source_ml_source1_data_property_value331 = *source_ml_source1_data_property_value331;
                                                            source_ml_source1_data_property_value331.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryGet(match result_5.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value332) => {
                                                            source_ml_source1_data_property_value332
                                                        }
                                                    }, match object.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value333) => {
                                                            source_ml_source1_data_property_value333
                                                        }
                                                    }, match key.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value334) => {
                                                            source_ml_source1_data_property_value334
                                                        }
                                                    }, match object.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value335) => {
                                                            source_ml_source1_data_property_value335
                                                        }
                                                    }));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override341.clone()), source_ml_source1_nominal_field339)
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
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value342, _) => {
                                            let source_ml_source1_data_property_value342 = *source_ml_source1_data_property_value342;
                                            source_ml_source1_data_property_value342.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let current_4 = match receiver.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value343, _) => {
                                                let source_ml_source1_data_property_value343 = *source_ml_source1_data_property_value343;
                                                source_ml_source1_data_property_value343.clone()
                                            }
                                        };
                                        {
                                            let arguments__3 = Vec::<f64>::new();
                                            {
                                                let source_ml_temporary_values5 = arguments_2;
                                                {
                                                    let source_ml_temporary_index5 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s344 = self::source_ml_temporary_lower_java_script_call_source_expression_loop9(bindings, depth, current_4.clone(), arguments__3, source_ml_temporary_values5, source_ml_temporary_index5.clone());
                                                        match source_ml_source1_s344.clone() {
                                                            self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop9Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop9Done(current_5, arguments__4) => {
                                                                let current_5 = *current_5;
                                                                {
                                                                    let result_6 = self::allocate_java_script_call_source_register(current_5.clone());
                                                                    {
                                                                        let source_ml_source1_nominal_spread349 = result_6.clone();
                                                                        {
                                                                            let source_ml_source1_nominal_field350 = match source_ml_source1_nominal_spread349.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value351, _) => {
                                                                                    let source_ml_source1_data_property_value351 = *source_ml_source1_data_property_value351;
                                                                                    source_ml_source1_data_property_value351.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field352 = match source_ml_source1_nominal_spread349.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value353) => {
                                                                                        source_ml_source1_data_property_value353
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_override354 = self::append_java_script_call_source_instruction(match result_6.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value345, _) => {
                                                                                            let source_ml_source1_data_property_value345 = *source_ml_source1_data_property_value345;
                                                                                            source_ml_source1_data_property_value345.clone()
                                                                                        }
                                                                                    }, self::JavaScriptCallInstruction::Call(match result_6.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value346) => {
                                                                                            source_ml_source1_data_property_value346
                                                                                        }
                                                                                    }, match callee.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value347) => {
                                                                                            source_ml_source1_data_property_value347
                                                                                        }
                                                                                    }, match receiver.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value348) => {
                                                                                            source_ml_source1_data_property_value348
                                                                                        }
                                                                                    }, arguments__4));
                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override354.clone()), source_ml_source1_nominal_field352)
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
        self::JavaScriptCallSource::SourceConstant(source_ml_source1_f355) => {
            let source_ml_source1_f355 = *source_ml_source1_f355;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceBinding(source_ml_source1_f356) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceLet(source_ml_source1_f357, source_ml_source1_f358, source_ml_source1_f359) => {
            let source_ml_source1_f358 = *source_ml_source1_f358;
            let source_ml_source1_f359 = *source_ml_source1_f359;
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
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value373, _) => {
                            let source_ml_source1_data_property_value373 = *source_ml_source1_data_property_value373;
                            source_ml_source1_data_property_value373.clone()
                        }
                    });
                    {
                        let branch = ((match match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value374, _) => {
                                let source_ml_source1_data_property_value374 = *source_ml_source1_data_property_value374;
                                source_ml_source1_data_property_value374.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value375, _, _, _, _) => {
                                source_ml_source1_data_property_value375
                            }
                        }).len() as f64);
                        {
                            let start = self::append_java_script_call_source_instruction(match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value376, _) => {
                                    let source_ml_source1_data_property_value376 = *source_ml_source1_data_property_value376;
                                    source_ml_source1_data_property_value376.clone()
                                }
                            }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value377) => {
                                    source_ml_source1_data_property_value377
                                }
                            }, 0f64, 0f64));
                            {
                                let yes_position = ((match &start { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value378, _, _, _, _) => source_ml_source1_data_property_value378.as_slice() }).len() as f64);
                                {
                                    let yes = self::lower_java_script_call_source_expression(source_yes.clone(), bindings.clone(), start.clone(), (depth + 1f64));
                                    {
                                        let copied_yes = self::append_java_script_call_source_instruction(match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value379, _) => {
                                                let source_ml_source1_data_property_value379 = *source_ml_source1_data_property_value379;
                                                source_ml_source1_data_property_value379.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value380) => {
                                                source_ml_source1_data_property_value380
                                            }
                                        }, match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value381) => {
                                                source_ml_source1_data_property_value381
                                            }
                                        }));
                                        {
                                            let jump = ((match &copied_yes { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value382, _, _, _, _) => source_ml_source1_data_property_value382.as_slice() }).len() as f64);
                                            {
                                                let alternative = self::append_java_script_call_source_instruction(copied_yes.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                {
                                                    let no_position = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value383, _, _, _, _) => source_ml_source1_data_property_value383.as_slice() }).len() as f64);
                                                    {
                                                        let no = self::lower_java_script_call_source_expression(source_no.clone(), bindings, alternative.clone(), (depth + 1f64));
                                                        {
                                                            let copied_no = self::append_java_script_call_source_instruction(match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value384, _) => {
                                                                    let source_ml_source1_data_property_value384 = *source_ml_source1_data_property_value384;
                                                                    source_ml_source1_data_property_value384.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value385) => {
                                                                    source_ml_source1_data_property_value385
                                                                }
                                                            }, match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value386) => {
                                                                    source_ml_source1_data_property_value386
                                                                }
                                                            }));
                                                            {
                                                                let end = ((match &copied_no { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value387, _, _, _, _) => source_ml_source1_data_property_value387.as_slice() }).len() as f64);
                                                                if ((match copied_no.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value389) => {
                                                                        source_ml_source1_data_property_value389.clone()
                                                                    }
                                                                }) != "") {
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(copied_no.clone()), 0f64)
                                                                } else {
                                                                    {
                                                                        let instructions = { let ml_values2 = match copied_no.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value390, _, _, _, _) => {
                                                                                source_ml_source1_data_property_value390
                                                                            }
                                                                        }; let ml_callback2 = { let branch = branch; let condition = condition.clone(); let yes_position = yes_position; let no_position = no_position; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                            self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value391) => {
                                                                                    source_ml_source1_data_property_value391
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
                                                                            let source_ml_source1_nominal_spread392 = copied_no.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field393 = match source_ml_source1_nominal_spread392.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value394, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value394
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field395 = match source_ml_source1_nominal_spread392.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value396, _, _, _) => {
                                                                                            source_ml_source1_data_property_value396
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field397 = match source_ml_source1_nominal_spread392.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value398, _, _) => {
                                                                                                source_ml_source1_data_property_value398
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field399 = match source_ml_source1_nominal_spread392.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value400, _) => {
                                                                                                    source_ml_source1_data_property_value400
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field401 = match source_ml_source1_nominal_spread392.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value402) => {
                                                                                                        source_ml_source1_data_property_value402.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_override403 = instructions;
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_override403, source_ml_source1_nominal_field395, source_ml_source1_nominal_field397, source_ml_source1_nominal_field399, source_ml_source1_nominal_field401.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }), match result.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value404) => {
                                                                                source_ml_source1_data_property_value404
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
        self::JavaScriptCallSource::SourceNumberBinary(source_ml_source1_f360, source_ml_source1_f361, source_ml_source1_f362) => {
            let source_ml_source1_f361 = *source_ml_source1_f361;
            let source_ml_source1_f362 = *source_ml_source1_f362;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSameValue(source_ml_source1_f363, source_ml_source1_f364) => {
            let source_ml_source1_f363 = *source_ml_source1_f363;
            let source_ml_source1_f364 = *source_ml_source1_f364;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceStrictEqual(source_ml_source1_f365, source_ml_source1_f366) => {
            let source_ml_source1_f365 = *source_ml_source1_f365;
            let source_ml_source1_f366 = *source_ml_source1_f366;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceRecur(source_ml_source1_f367) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceGet(source_ml_source1_f368, source_ml_source1_f369) => {
            let source_ml_source1_f368 = *source_ml_source1_f368;
            let source_ml_source1_f369 = *source_ml_source1_f369;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceCall(source_ml_source1_f370, source_ml_source1_f371, source_ml_source1_f372) => {
            let source_ml_source1_f370 = *source_ml_source1_f370;
            let source_ml_source1_f371 = *source_ml_source1_f371;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_call_source(source: self::JavaScriptCallSource, parameters: &[f64]) -> self::JavaScriptCallSourceCompilation {
    if (((parameters.len() as f64)) >= (4096f64)) {
        self::JavaScriptCallSourceCompilation::SourceCompiled(Vec::<self::JavaScriptCallInstruction>::new(), 0f64, String::from("source parameters exceed their explicit register budget"))
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s405 = self::source_ml_temporary_compile_java_script_call_source_loop10(parameters, index);
                match source_ml_source1_s405.clone() {
                    self::MlTemporaryCompileJavaScriptCallSourceLoop10Result::MlTemporaryCompileJavaScriptCallSourceLoop10Done => {
                        {
                            let bindings = { let ml_values3 = parameters.to_vec(); let ml_callback3 = std::rc::Rc::new(move |identity: f64, index_2: f64| -> self::JavaScriptCallSourceBinding { self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, (index_2 + 1f64)) }); ml_values3.iter().cloned().enumerate().map(|(ml_index3, ml_value3)| ml_callback3(ml_value3, (ml_index3 as f64))).collect::<Vec<_>>() };
                            {
                                let result = self::lower_java_script_call_source_expression(source.clone(), bindings, self::JavaScriptCallSourceBuilder::SourceBuilder(Vec::<self::JavaScriptCallInstruction>::new(), ((parameters.len() as f64) + 1f64), 0f64, (parameters.len() as f64), String::from("")), 0f64);
                                {
                                    let builder = self::append_java_script_call_source_instruction(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value406, _) => {
                                            let source_ml_source1_data_property_value406 = *source_ml_source1_data_property_value406;
                                            source_ml_source1_data_property_value406.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Return(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value407) => {
                                            source_ml_source1_data_property_value407
                                        }
                                    }));
                                    self::JavaScriptCallSourceCompilation::SourceCompiled(if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value409) => {
                                            source_ml_source1_data_property_value409.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value410, _, _, _, _) => {
                                                source_ml_source1_data_property_value410
                                            }
                                        }
                                    } else {
                                        Vec::<self::JavaScriptCallInstruction>::new()
                                    }, if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value412) => {
                                            source_ml_source1_data_property_value412.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value413, _, _, _) => {
                                                source_ml_source1_data_property_value413
                                            }
                                        }
                                    } else {
                                        0f64
                                    }, match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value414) => {
                                            source_ml_source1_data_property_value414.clone()
                                        }
                                    })
                                }
                            }
                        }
                    }
                    self::MlTemporaryCompileJavaScriptCallSourceLoop10Result::MlTemporaryCompileJavaScriptCallSourceLoop10Return(source_ml_temporary_result10) => {
                        let source_ml_temporary_result10 = *source_ml_temporary_result10;
                        source_ml_temporary_result10.clone()
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(units: &[f64], mut at: f64) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String::from("semantic text requires complete ordered UTF16 code units"))
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Done,
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(bool),
}
