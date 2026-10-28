// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::MlTemporaryLowerJavaScriptCallSourceExpressionLoop39Result;
use super::MlTemporaryLowerJavaScriptCallSourceExpressionLoop40Result;
use super::MlTemporaryLowerJavaScriptCallSourceExpressionLoop41Result;
use super::MlTemporaryLowerJavaScriptCallSourceExpressionLoop43Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_branch;
use super::ml;
use super::ml_array;
use super::read_java_script_call_constant_error;
use super::read_java_script_call_primitive_arity_error;
use super::refuse_java_script_call_source;
use super::source_ml_temporary_lower_java_script_call_source_expression_loop39;
use super::source_ml_temporary_lower_java_script_call_source_expression_loop40;
use super::source_ml_temporary_lower_java_script_call_source_expression_loop41;
use super::source_ml_temporary_lower_java_script_call_source_expression_loop42;
use super::source_ml_temporary_lower_java_script_call_source_expression_loop43;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_expression(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value249) => {
            source_ml_source1_data_property_value249.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((depth > (128f64)) || ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value251, _, _) => {
                source_ml_source1_data_property_value251
            }
        }) >= (65536f64))) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its structural budget")
        } else {
            {
                let state = {
                    let source_ml_source1_nominal_spread253 = builder.clone();
                    {
                        let source_ml_source1_nominal_field254 = match source_ml_source1_nominal_spread253.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value255, _, _, _, _) => {
                                source_ml_source1_data_property_value255
                            }
                        };
                        {
                            let source_ml_source1_nominal_field256 = match source_ml_source1_nominal_spread253.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value257, _, _, _) => {
                                    source_ml_source1_data_property_value257
                                }
                            };
                            {
                                let source_ml_source1_nominal_field258 = match source_ml_source1_nominal_spread253.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value259, _, _) => {
                                        source_ml_source1_data_property_value259
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field260 = match source_ml_source1_nominal_spread253.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value261, _) => {
                                            source_ml_source1_data_property_value261
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_field262 = match source_ml_source1_nominal_spread253.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value263) => {
                                                source_ml_source1_data_property_value263.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_override264 = (match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value252, _, _) => {
                                                    source_ml_source1_data_property_value252
                                                }
                                            } + 1f64);
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_field254, source_ml_source1_nominal_field256, source_ml_source1_nominal_override264, source_ml_source1_nominal_field260, source_ml_source1_nominal_field262.clone())
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
                                        let source_ml_source1_nominal_spread271 = result.clone();
                                        {
                                            let source_ml_source1_nominal_field272 = match source_ml_source1_nominal_spread271.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value273, _) => {
                                                    let source_ml_source1_data_property_value273 = *source_ml_source1_data_property_value273;
                                                    source_ml_source1_data_property_value273.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field274 = match source_ml_source1_nominal_spread271.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value275) => {
                                                        source_ml_source1_data_property_value275
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override276 = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value269, _) => {
                                                            let source_ml_source1_data_property_value269 = *source_ml_source1_data_property_value269;
                                                            source_ml_source1_data_property_value269.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Constant(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value270) => {
                                                            source_ml_source1_data_property_value270
                                                        }
                                                    }, Box::new(value.clone())));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override276.clone()), source_ml_source1_nominal_field274)
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
                                    let source_ml_source1_s277 = self::source_ml_temporary_lower_java_script_call_source_expression_loop39(source.clone(), bindings, state.clone(), index);
                                    match source_ml_source1_s277.clone() {
                                        self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop39Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop39Done => {
                                            self::refuse_java_script_call_source(state.clone(), "a source binding reference has no actual lexical owner")
                                        }
                                        self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop39Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop39Return(source_ml_temporary_result39) => {
                                            let source_ml_temporary_result39 = *source_ml_temporary_result39;
                                            source_ml_temporary_result39.clone()
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
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source1_data_property_value279, _) => {
                                source_ml_source1_data_property_value279
                            }
                        }) == identity_2) }) }; ml_values1.iter().cloned().enumerate().any(|(ml_index1, ml_value1)| ml_callback1(ml_value1)) }) {
                            self::refuse_java_script_call_source(state.clone(), "a source binding owner must be unique in its lexical scope")
                        } else {
                            {
                                let value_2 = self::lower_java_script_call_source_expression(source_value.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                self::lower_java_script_call_source_expression(body.clone(), {
                                    let source_ml_source1_concat_value281 = bindings;
                                    {
                                        let source_ml_source1_concat_value282 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity_2, match value_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value280) => {
                                                source_ml_source1_data_property_value280
                                            }
                                        })];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source1_concat_value281), source_ml_source1_concat_value282)
                                    }
                                }, match value_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value283, _) => {
                                        let source_ml_source1_data_property_value283 = *source_ml_source1_data_property_value283;
                                        source_ml_source1_data_property_value283.clone()
                                    }
                                }, (depth + 1f64))
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceIf(source_ml_source1_f265, source_ml_source1_f266, source_ml_source1_f267) => {
                        let source_ml_source1_f265 = *source_ml_source1_f265;
                        let source_ml_source1_f266 = *source_ml_source1_f266;
                        let source_ml_source1_f267 = *source_ml_source1_f267;
                        self::lower_java_script_call_source_branch(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(source_ml_source1_f268, source_left, source_right) => {
                        let source_left = *source_left;
                        let source_right = *source_right;
                        {
                            let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings, match left.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value284, _) => {
                                        let source_ml_source1_data_property_value284 = *source_ml_source1_data_property_value284;
                                        source_ml_source1_data_property_value284.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_2 = self::allocate_java_script_call_source_register(match right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value285, _) => {
                                            let source_ml_source1_data_property_value285 = *source_ml_source1_data_property_value285;
                                            source_ml_source1_data_property_value285.clone()
                                        }
                                    });
                                    {
                                        let operator = source_ml_source1_f268.clone();
                                        {
                                            let source_ml_source1_f294 = source_left.clone();
                                            {
                                                let source_ml_source1_f295 = source_right.clone();
                                                if !(self::ml_array::includes(&vec![String::from("add"), String::from("subtract"), String::from("multiply"), String::from("divide"), String::from("remainder"), String::from("power"), String::from("lessThan"), String::from("equal")], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::refuse_java_script_call_source(match result_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value311, _) => {
                                                            let source_ml_source1_data_property_value311 = *source_ml_source1_data_property_value311;
                                                            source_ml_source1_data_property_value311.clone()
                                                        }
                                                    }, "source Number lowering requires its exact registered operator")
                                                } else {
                                                    {
                                                        let source_ml_source1_nominal_spread316 = result_2.clone();
                                                        {
                                                            let source_ml_source1_nominal_field317 = match source_ml_source1_nominal_spread316.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value318, _) => {
                                                                    let source_ml_source1_data_property_value318 = *source_ml_source1_data_property_value318;
                                                                    source_ml_source1_data_property_value318.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field319 = match source_ml_source1_nominal_spread316.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value320) => {
                                                                        source_ml_source1_data_property_value320
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_override321 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value312, _) => {
                                                                            let source_ml_source1_data_property_value312 = *source_ml_source1_data_property_value312;
                                                                            source_ml_source1_data_property_value312.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::NumberBinary(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value313) => {
                                                                            source_ml_source1_data_property_value313
                                                                        }
                                                                    }, match left.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value314) => {
                                                                            source_ml_source1_data_property_value314
                                                                        }
                                                                    }, match right.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value315) => {
                                                                            source_ml_source1_data_property_value315
                                                                        }
                                                                    }, operator.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override321.clone()), source_ml_source1_nominal_field319)
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value322, _) => {
                                        let source_ml_source1_data_property_value322 = *source_ml_source1_data_property_value322;
                                        source_ml_source1_data_property_value322.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_3 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value323, _) => {
                                            let source_ml_source1_data_property_value323 = *source_ml_source1_data_property_value323;
                                            source_ml_source1_data_property_value323.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source1_f334 = source_left_2.clone();
                                        {
                                            let source_ml_source1_f335 = source_right_2.clone();
                                            {
                                                let source_ml_source1_nominal_spread353 = result_3.clone();
                                                {
                                                    let source_ml_source1_nominal_field354 = match source_ml_source1_nominal_spread353.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value355, _) => {
                                                            let source_ml_source1_data_property_value355 = *source_ml_source1_data_property_value355;
                                                            source_ml_source1_data_property_value355.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field356 = match source_ml_source1_nominal_spread353.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value357) => {
                                                                source_ml_source1_data_property_value357
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_override358 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value349, _) => {
                                                                    let source_ml_source1_data_property_value349 = *source_ml_source1_data_property_value349;
                                                                    source_ml_source1_data_property_value349.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::SameValue(match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value350) => {
                                                                    source_ml_source1_data_property_value350
                                                                }
                                                            }, match left_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value351) => {
                                                                    source_ml_source1_data_property_value351
                                                                }
                                                            }, match right_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value352) => {
                                                                    source_ml_source1_data_property_value352
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override358.clone()), source_ml_source1_nominal_field356)
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value359, _) => {
                                        let source_ml_source1_data_property_value359 = *source_ml_source1_data_property_value359;
                                        source_ml_source1_data_property_value359.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value360, _) => {
                                            let source_ml_source1_data_property_value360 = *source_ml_source1_data_property_value360;
                                            source_ml_source1_data_property_value360.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source1_f373 = source_left_3.clone();
                                        {
                                            let source_ml_source1_f374 = source_right_3.clone();
                                            {
                                                let source_ml_source1_nominal_spread390 = result_4.clone();
                                                {
                                                    let source_ml_source1_nominal_field391 = match source_ml_source1_nominal_spread390.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value392, _) => {
                                                            let source_ml_source1_data_property_value392 = *source_ml_source1_data_property_value392;
                                                            source_ml_source1_data_property_value392.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field393 = match source_ml_source1_nominal_spread390.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value394) => {
                                                                source_ml_source1_data_property_value394
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_override395 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value386, _) => {
                                                                    let source_ml_source1_data_property_value386 = *source_ml_source1_data_property_value386;
                                                                    source_ml_source1_data_property_value386.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::StrictEqual(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value387) => {
                                                                    source_ml_source1_data_property_value387
                                                                }
                                                            }, match left_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value388) => {
                                                                    source_ml_source1_data_property_value388
                                                                }
                                                            }, match right_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value389) => {
                                                                    source_ml_source1_data_property_value389
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override395.clone()), source_ml_source1_nominal_field393)
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
                    self::JavaScriptCallSource::SourcePrimitive(operation, arguments) => {
                        {
                            let error_2 = self::read_java_script_call_primitive_arity_error(&(operation), (arguments.len() as f64));
                            if (error_2 != "") {
                                self::refuse_java_script_call_source(state.clone(), &(error_2))
                            } else {
                                {
                                    let current = state.clone();
                                    {
                                        let arguments_ = Vec::<f64>::new();
                                        {
                                            let source_ml_temporary_values36 = arguments;
                                            {
                                                let source_ml_temporary_index36 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_s396 = self::source_ml_temporary_lower_java_script_call_source_expression_loop40(bindings, depth, current.clone(), arguments_, source_ml_temporary_values36, source_ml_temporary_index36.clone());
                                                    match source_ml_source1_s396.clone() {
                                                        self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop40Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop40Done(current_2, arguments__2) => {
                                                            let current_2 = *current_2;
                                                            {
                                                                let result_5 = self::allocate_java_script_call_source_register(current_2.clone());
                                                                {
                                                                    let source_ml_source1_nominal_spread399 = result_5.clone();
                                                                    {
                                                                        let source_ml_source1_nominal_field400 = match source_ml_source1_nominal_spread399.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value401, _) => {
                                                                                let source_ml_source1_data_property_value401 = *source_ml_source1_data_property_value401;
                                                                                source_ml_source1_data_property_value401.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_field402 = match source_ml_source1_nominal_spread399.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value403) => {
                                                                                    source_ml_source1_data_property_value403
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_override404 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value397, _) => {
                                                                                        let source_ml_source1_data_property_value397 = *source_ml_source1_data_property_value397;
                                                                                        source_ml_source1_data_property_value397.clone()
                                                                                    }
                                                                                }, self::JavaScriptCallInstruction::Primitive(match result_5.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value398) => {
                                                                                        source_ml_source1_data_property_value398
                                                                                    }
                                                                                }, operation.clone(), arguments__2));
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override404.clone()), source_ml_source1_nominal_field402)
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
                    self::JavaScriptCallSource::SourceRecur(arguments_2) => {
                        if (((arguments_2.len() as f64)) != (match state.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value406, _) => {
                                source_ml_source1_data_property_value406
                            }
                        })) {
                            self::refuse_java_script_call_source(state.clone(), "loop updates require every actual parameter operand")
                        } else {
                            {
                                let current_3 = state.clone();
                                {
                                    let arguments__3 = Vec::<f64>::new();
                                    {
                                        let source_ml_temporary_values37 = arguments_2;
                                        {
                                            let source_ml_temporary_index37 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_s407 = self::source_ml_temporary_lower_java_script_call_source_expression_loop41(bindings, depth, current_3.clone(), arguments__3, source_ml_temporary_values37, source_ml_temporary_index37.clone());
                                                match source_ml_source1_s407.clone() {
                                                    self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop41Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop41Done(current_4, arguments__4) => {
                                                        let current_4 = *current_4;
                                                        {
                                                            let index_2 = 0f64;
                                                            {
                                                                let current_5 = self::source_ml_temporary_lower_java_script_call_source_expression_loop42(current_4.clone(), &(arguments__4), index_2);
                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(current_5.clone(), self::JavaScriptCallInstruction::Jump(0f64))), 0f64)
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value408, _) => {
                                        let source_ml_source1_data_property_value408 = *source_ml_source1_data_property_value408;
                                        source_ml_source1_data_property_value408.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let value_3 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value409, _) => {
                                            let source_ml_source1_data_property_value409 = *source_ml_source1_data_property_value409;
                                            source_ml_source1_data_property_value409.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let result_6 = self::allocate_java_script_call_source_register(match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value410, _) => {
                                                let source_ml_source1_data_property_value410 = *source_ml_source1_data_property_value410;
                                                source_ml_source1_data_property_value410.clone()
                                            }
                                        });
                                        {
                                            let write = self::append_java_script_call_source_instruction(match result_6.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value411, _) => {
                                                    let source_ml_source1_data_property_value411 = *source_ml_source1_data_property_value411;
                                                    source_ml_source1_data_property_value411.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinarySet(match result_6.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value412) => {
                                                    source_ml_source1_data_property_value412
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value413) => {
                                                    source_ml_source1_data_property_value413
                                                }
                                            }, match key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value414) => {
                                                    source_ml_source1_data_property_value414
                                                }
                                            }, match value_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value415) => {
                                                    source_ml_source1_data_property_value415
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value416) => {
                                                    source_ml_source1_data_property_value416
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_6.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value417) => {
                                                    source_ml_source1_data_property_value417
                                                }
                                            }))), match value_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value418) => {
                                                    source_ml_source1_data_property_value418
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value419, _) => {
                                        let source_ml_source1_data_property_value419 = *source_ml_source1_data_property_value419;
                                        source_ml_source1_data_property_value419.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_7 = self::allocate_java_script_call_source_register(match key_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value420, _) => {
                                            let source_ml_source1_data_property_value420 = *source_ml_source1_data_property_value420;
                                            source_ml_source1_data_property_value420.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source1_nominal_spread426 = result_7.clone();
                                        {
                                            let source_ml_source1_nominal_field427 = match source_ml_source1_nominal_spread426.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value428, _) => {
                                                    let source_ml_source1_data_property_value428 = *source_ml_source1_data_property_value428;
                                                    source_ml_source1_data_property_value428.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field429 = match source_ml_source1_nominal_spread426.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value430) => {
                                                        source_ml_source1_data_property_value430
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override431 = self::append_java_script_call_source_instruction(match result_7.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value421, _) => {
                                                            let source_ml_source1_data_property_value421 = *source_ml_source1_data_property_value421;
                                                            source_ml_source1_data_property_value421.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryGet(match result_7.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value422) => {
                                                            source_ml_source1_data_property_value422
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value423) => {
                                                            source_ml_source1_data_property_value423
                                                        }
                                                    }, match key_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value424) => {
                                                            source_ml_source1_data_property_value424
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value425) => {
                                                            source_ml_source1_data_property_value425
                                                        }
                                                    }));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override431.clone()), source_ml_source1_nominal_field429)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceCall(source_callee, source_receiver, arguments_3) => {
                        let source_callee = *source_callee;
                        let source_receiver = *source_receiver;
                        if (((arguments_3.len() as f64)) > (4096f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source call operands exceed their explicit budget")
                        } else {
                            {
                                let callee = self::lower_java_script_call_source_expression(source_callee.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                {
                                    let receiver = self::lower_java_script_call_source_expression(source_receiver.clone(), bindings.clone(), match callee.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value432, _) => {
                                            let source_ml_source1_data_property_value432 = *source_ml_source1_data_property_value432;
                                            source_ml_source1_data_property_value432.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let current_6 = match receiver.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value433, _) => {
                                                let source_ml_source1_data_property_value433 = *source_ml_source1_data_property_value433;
                                                source_ml_source1_data_property_value433.clone()
                                            }
                                        };
                                        {
                                            let arguments__5 = Vec::<f64>::new();
                                            {
                                                let source_ml_temporary_values38 = arguments_3;
                                                {
                                                    let source_ml_temporary_index38 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s434 = self::source_ml_temporary_lower_java_script_call_source_expression_loop43(bindings, depth, current_6.clone(), arguments__5, source_ml_temporary_values38, source_ml_temporary_index38.clone());
                                                        match source_ml_source1_s434.clone() {
                                                            self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop43Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop43Done(current_7, arguments__6) => {
                                                                let current_7 = *current_7;
                                                                {
                                                                    let result_8 = self::allocate_java_script_call_source_register(current_7.clone());
                                                                    {
                                                                        let source_ml_source1_nominal_spread439 = result_8.clone();
                                                                        {
                                                                            let source_ml_source1_nominal_field440 = match source_ml_source1_nominal_spread439.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value441, _) => {
                                                                                    let source_ml_source1_data_property_value441 = *source_ml_source1_data_property_value441;
                                                                                    source_ml_source1_data_property_value441.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field442 = match source_ml_source1_nominal_spread439.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value443) => {
                                                                                        source_ml_source1_data_property_value443
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_override444 = self::append_java_script_call_source_instruction(match result_8.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value435, _) => {
                                                                                            let source_ml_source1_data_property_value435 = *source_ml_source1_data_property_value435;
                                                                                            source_ml_source1_data_property_value435.clone()
                                                                                        }
                                                                                    }, self::JavaScriptCallInstruction::Call(match result_8.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value436) => {
                                                                                            source_ml_source1_data_property_value436
                                                                                        }
                                                                                    }, match callee.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value437) => {
                                                                                            source_ml_source1_data_property_value437
                                                                                        }
                                                                                    }, match receiver.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value438) => {
                                                                                            source_ml_source1_data_property_value438
                                                                                        }
                                                                                    }, arguments__6));
                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override444.clone()), source_ml_source1_nominal_field442)
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
