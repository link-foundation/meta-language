// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop99Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_branch;
use super::ml;
use super::ml_array;
use super::read_java_script_call_constant_error;
use super::read_java_script_call_primitive_arity_error;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop97;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop98;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop99;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop100;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop101;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_expression(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value324) => {
            source_ml_source2_data_property_value324.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((depth > (128f64)) || ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value326, _, _) => {
                source_ml_source2_data_property_value326
            }
        }) >= (65536f64))) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its structural budget")
        } else {
            {
                let state = {
                    let source_ml_source2_nominal_spread328 = builder.clone();
                    {
                        let source_ml_source2_nominal_field329 = match source_ml_source2_nominal_spread328.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value330, _, _, _, _) => {
                                source_ml_source2_data_property_value330
                            }
                        };
                        {
                            let source_ml_source2_nominal_field331 = match source_ml_source2_nominal_spread328.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value332, _, _, _) => {
                                    source_ml_source2_data_property_value332
                                }
                            };
                            {
                                let source_ml_source2_nominal_field333 = match source_ml_source2_nominal_spread328.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value334, _, _) => {
                                        source_ml_source2_data_property_value334
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field335 = match source_ml_source2_nominal_spread328.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value336, _) => {
                                            source_ml_source2_data_property_value336
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field337 = match source_ml_source2_nominal_spread328.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value338) => {
                                                source_ml_source2_data_property_value338.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override339 = (match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value327, _, _) => {
                                                    source_ml_source2_data_property_value327
                                                }
                                            } + 1f64);
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field329, source_ml_source2_nominal_field331, source_ml_source2_nominal_override339, source_ml_source2_nominal_field335, source_ml_source2_nominal_field337.clone())
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
                                        let source_ml_source2_nominal_spread346 = result.clone();
                                        {
                                            let source_ml_source2_nominal_field347 = match source_ml_source2_nominal_spread346.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value348, _) => {
                                                    let source_ml_source2_data_property_value348 = *source_ml_source2_data_property_value348;
                                                    source_ml_source2_data_property_value348.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field349 = match source_ml_source2_nominal_spread346.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value350) => {
                                                        source_ml_source2_data_property_value350
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override351 = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value344, _) => {
                                                            let source_ml_source2_data_property_value344 = *source_ml_source2_data_property_value344;
                                                            source_ml_source2_data_property_value344.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Constant(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value345) => {
                                                            source_ml_source2_data_property_value345
                                                        }
                                                    }, Box::new(value.clone())));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override351.clone()), source_ml_source2_nominal_field349)
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
                                    let source_ml_source2_s352 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop97(source.clone(), bindings, state.clone(), index);
                                    match source_ml_source2_s352.clone() {
                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Done => {
                                            self::refuse_java_script_call_source(state.clone(), "a source binding reference has no actual lexical owner")
                                        }
                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Return(source_ml_source1_temporary_result97) => {
                                            let source_ml_source1_temporary_result97 = *source_ml_source1_temporary_result97;
                                            source_ml_source1_temporary_result97.clone()
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
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value354, _) => {
                                source_ml_source2_data_property_value354
                            }
                        }) == identity_2) }) }; ml_values1.iter().cloned().enumerate().any(|(ml_index1, ml_value1)| ml_callback1(ml_value1)) }) {
                            self::refuse_java_script_call_source(state.clone(), "a source binding owner must be unique in its lexical scope")
                        } else {
                            {
                                let value_2 = self::lower_java_script_call_source_expression(source_value.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                self::lower_java_script_call_source_expression(body.clone(), {
                                    let source_ml_source2_concat_value356 = bindings;
                                    {
                                        let source_ml_source2_concat_value357 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity_2, match value_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value355) => {
                                                source_ml_source2_data_property_value355
                                            }
                                        })];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source2_concat_value356), source_ml_source2_concat_value357)
                                    }
                                }, match value_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value358, _) => {
                                        let source_ml_source2_data_property_value358 = *source_ml_source2_data_property_value358;
                                        source_ml_source2_data_property_value358.clone()
                                    }
                                }, (depth + 1f64))
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceIf(source_ml_source2_f340, source_ml_source2_f341, source_ml_source2_f342) => {
                        let source_ml_source2_f340 = *source_ml_source2_f340;
                        let source_ml_source2_f341 = *source_ml_source2_f341;
                        let source_ml_source2_f342 = *source_ml_source2_f342;
                        self::lower_java_script_call_source_branch(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f343, source_left, source_right) => {
                        let source_left = *source_left;
                        let source_right = *source_right;
                        {
                            let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings, match left.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value359, _) => {
                                        let source_ml_source2_data_property_value359 = *source_ml_source2_data_property_value359;
                                        source_ml_source2_data_property_value359.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_2 = self::allocate_java_script_call_source_register(match right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value360, _) => {
                                            let source_ml_source2_data_property_value360 = *source_ml_source2_data_property_value360;
                                            source_ml_source2_data_property_value360.clone()
                                        }
                                    });
                                    {
                                        let operator = source_ml_source2_f343.clone();
                                        {
                                            let source_ml_source2_f369 = source_left.clone();
                                            {
                                                let source_ml_source2_f370 = source_right.clone();
                                                if !(self::ml_array::includes(&vec![String::from("add"), String::from("subtract"), String::from("multiply"), String::from("divide"), String::from("remainder"), String::from("power"), String::from("lessThan"), String::from("equal")], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::refuse_java_script_call_source(match result_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value386, _) => {
                                                            let source_ml_source2_data_property_value386 = *source_ml_source2_data_property_value386;
                                                            source_ml_source2_data_property_value386.clone()
                                                        }
                                                    }, "source Number lowering requires its exact registered operator")
                                                } else {
                                                    {
                                                        let source_ml_source2_nominal_spread391 = result_2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field392 = match source_ml_source2_nominal_spread391.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value393, _) => {
                                                                    let source_ml_source2_data_property_value393 = *source_ml_source2_data_property_value393;
                                                                    source_ml_source2_data_property_value393.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field394 = match source_ml_source2_nominal_spread391.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value395) => {
                                                                        source_ml_source2_data_property_value395
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override396 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value387, _) => {
                                                                            let source_ml_source2_data_property_value387 = *source_ml_source2_data_property_value387;
                                                                            source_ml_source2_data_property_value387.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::NumberBinary(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value388) => {
                                                                            source_ml_source2_data_property_value388
                                                                        }
                                                                    }, match left.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value389) => {
                                                                            source_ml_source2_data_property_value389
                                                                        }
                                                                    }, match right.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value390) => {
                                                                            source_ml_source2_data_property_value390
                                                                        }
                                                                    }, operator.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override396.clone()), source_ml_source2_nominal_field394)
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value397, _) => {
                                        let source_ml_source2_data_property_value397 = *source_ml_source2_data_property_value397;
                                        source_ml_source2_data_property_value397.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_3 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value398, _) => {
                                            let source_ml_source2_data_property_value398 = *source_ml_source2_data_property_value398;
                                            source_ml_source2_data_property_value398.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f409 = source_left_2.clone();
                                        {
                                            let source_ml_source2_f410 = source_right_2.clone();
                                            {
                                                let source_ml_source2_nominal_spread428 = result_3.clone();
                                                {
                                                    let source_ml_source2_nominal_field429 = match source_ml_source2_nominal_spread428.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value430, _) => {
                                                            let source_ml_source2_data_property_value430 = *source_ml_source2_data_property_value430;
                                                            source_ml_source2_data_property_value430.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field431 = match source_ml_source2_nominal_spread428.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value432) => {
                                                                source_ml_source2_data_property_value432
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override433 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value424, _) => {
                                                                    let source_ml_source2_data_property_value424 = *source_ml_source2_data_property_value424;
                                                                    source_ml_source2_data_property_value424.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::SameValue(match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value425) => {
                                                                    source_ml_source2_data_property_value425
                                                                }
                                                            }, match left_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value426) => {
                                                                    source_ml_source2_data_property_value426
                                                                }
                                                            }, match right_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value427) => {
                                                                    source_ml_source2_data_property_value427
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override433.clone()), source_ml_source2_nominal_field431)
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value434, _) => {
                                        let source_ml_source2_data_property_value434 = *source_ml_source2_data_property_value434;
                                        source_ml_source2_data_property_value434.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value435, _) => {
                                            let source_ml_source2_data_property_value435 = *source_ml_source2_data_property_value435;
                                            source_ml_source2_data_property_value435.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f448 = source_left_3.clone();
                                        {
                                            let source_ml_source2_f449 = source_right_3.clone();
                                            {
                                                let source_ml_source2_nominal_spread465 = result_4.clone();
                                                {
                                                    let source_ml_source2_nominal_field466 = match source_ml_source2_nominal_spread465.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value467, _) => {
                                                            let source_ml_source2_data_property_value467 = *source_ml_source2_data_property_value467;
                                                            source_ml_source2_data_property_value467.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field468 = match source_ml_source2_nominal_spread465.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value469) => {
                                                                source_ml_source2_data_property_value469
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override470 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value461, _) => {
                                                                    let source_ml_source2_data_property_value461 = *source_ml_source2_data_property_value461;
                                                                    source_ml_source2_data_property_value461.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::StrictEqual(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value462) => {
                                                                    source_ml_source2_data_property_value462
                                                                }
                                                            }, match left_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value463) => {
                                                                    source_ml_source2_data_property_value463
                                                                }
                                                            }, match right_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value464) => {
                                                                    source_ml_source2_data_property_value464
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override470.clone()), source_ml_source2_nominal_field468)
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
                                            let source_ml_source1_temporary_values94 = arguments;
                                            {
                                                let source_ml_source1_temporary_index94 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s471 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop98(bindings, depth, current.clone(), arguments_, source_ml_source1_temporary_values94, source_ml_source1_temporary_index94.clone());
                                                    match source_ml_source2_s471.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Done(current_2, arguments__2) => {
                                                            let current_2 = *current_2;
                                                            {
                                                                let result_5 = self::allocate_java_script_call_source_register(current_2.clone());
                                                                {
                                                                    let source_ml_source2_nominal_spread474 = result_5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field475 = match source_ml_source2_nominal_spread474.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value476, _) => {
                                                                                let source_ml_source2_data_property_value476 = *source_ml_source2_data_property_value476;
                                                                                source_ml_source2_data_property_value476.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field477 = match source_ml_source2_nominal_spread474.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value478) => {
                                                                                    source_ml_source2_data_property_value478
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override479 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value472, _) => {
                                                                                        let source_ml_source2_data_property_value472 = *source_ml_source2_data_property_value472;
                                                                                        source_ml_source2_data_property_value472.clone()
                                                                                    }
                                                                                }, self::JavaScriptCallInstruction::Primitive(match result_5.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value473) => {
                                                                                        source_ml_source2_data_property_value473
                                                                                    }
                                                                                }, operation.clone(), arguments__2));
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override479.clone()), source_ml_source2_nominal_field477)
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
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value481, _) => {
                                source_ml_source2_data_property_value481
                            }
                        })) {
                            self::refuse_java_script_call_source(state.clone(), "loop updates require every actual parameter operand")
                        } else {
                            {
                                let current_3 = state.clone();
                                {
                                    let arguments__3 = Vec::<f64>::new();
                                    {
                                        let source_ml_source1_temporary_values95 = arguments_2;
                                        {
                                            let source_ml_source1_temporary_index95 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s482 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop99(bindings, depth, current_3.clone(), arguments__3, source_ml_source1_temporary_values95, source_ml_source1_temporary_index95.clone());
                                                match source_ml_source2_s482.clone() {
                                                    self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop99Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop99Done(current_4, arguments__4) => {
                                                        let current_4 = *current_4;
                                                        {
                                                            let index_2 = 0f64;
                                                            {
                                                                let current_5 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop100(current_4.clone(), &(arguments__4), index_2);
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value483, _) => {
                                        let source_ml_source2_data_property_value483 = *source_ml_source2_data_property_value483;
                                        source_ml_source2_data_property_value483.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let value_3 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value484, _) => {
                                            let source_ml_source2_data_property_value484 = *source_ml_source2_data_property_value484;
                                            source_ml_source2_data_property_value484.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let result_6 = self::allocate_java_script_call_source_register(match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value485, _) => {
                                                let source_ml_source2_data_property_value485 = *source_ml_source2_data_property_value485;
                                                source_ml_source2_data_property_value485.clone()
                                            }
                                        });
                                        {
                                            let write = self::append_java_script_call_source_instruction(match result_6.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value486, _) => {
                                                    let source_ml_source2_data_property_value486 = *source_ml_source2_data_property_value486;
                                                    source_ml_source2_data_property_value486.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinarySet(match result_6.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value487) => {
                                                    source_ml_source2_data_property_value487
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value488) => {
                                                    source_ml_source2_data_property_value488
                                                }
                                            }, match key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value489) => {
                                                    source_ml_source2_data_property_value489
                                                }
                                            }, match value_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value490) => {
                                                    source_ml_source2_data_property_value490
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value491) => {
                                                    source_ml_source2_data_property_value491
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_6.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value492) => {
                                                    source_ml_source2_data_property_value492
                                                }
                                            }))), match value_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value493) => {
                                                    source_ml_source2_data_property_value493
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value494, _) => {
                                        let source_ml_source2_data_property_value494 = *source_ml_source2_data_property_value494;
                                        source_ml_source2_data_property_value494.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_7 = self::allocate_java_script_call_source_register(match key_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value495, _) => {
                                            let source_ml_source2_data_property_value495 = *source_ml_source2_data_property_value495;
                                            source_ml_source2_data_property_value495.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_nominal_spread501 = result_7.clone();
                                        {
                                            let source_ml_source2_nominal_field502 = match source_ml_source2_nominal_spread501.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value503, _) => {
                                                    let source_ml_source2_data_property_value503 = *source_ml_source2_data_property_value503;
                                                    source_ml_source2_data_property_value503.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field504 = match source_ml_source2_nominal_spread501.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value505) => {
                                                        source_ml_source2_data_property_value505
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override506 = self::append_java_script_call_source_instruction(match result_7.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value496, _) => {
                                                            let source_ml_source2_data_property_value496 = *source_ml_source2_data_property_value496;
                                                            source_ml_source2_data_property_value496.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryGet(match result_7.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value497) => {
                                                            source_ml_source2_data_property_value497
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value498) => {
                                                            source_ml_source2_data_property_value498
                                                        }
                                                    }, match key_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value499) => {
                                                            source_ml_source2_data_property_value499
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value500) => {
                                                            source_ml_source2_data_property_value500
                                                        }
                                                    }));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override506.clone()), source_ml_source2_nominal_field504)
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
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value507, _) => {
                                            let source_ml_source2_data_property_value507 = *source_ml_source2_data_property_value507;
                                            source_ml_source2_data_property_value507.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let current_6 = match receiver.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value508, _) => {
                                                let source_ml_source2_data_property_value508 = *source_ml_source2_data_property_value508;
                                                source_ml_source2_data_property_value508.clone()
                                            }
                                        };
                                        {
                                            let arguments__5 = Vec::<f64>::new();
                                            {
                                                let source_ml_source1_temporary_values96 = arguments_3;
                                                {
                                                    let source_ml_source1_temporary_index96 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s509 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop101(bindings, depth, current_6.clone(), arguments__5, source_ml_source1_temporary_values96, source_ml_source1_temporary_index96.clone());
                                                        match source_ml_source2_s509.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Done(current_7, arguments__6) => {
                                                                let current_7 = *current_7;
                                                                {
                                                                    let result_8 = self::allocate_java_script_call_source_register(current_7.clone());
                                                                    {
                                                                        let source_ml_source2_nominal_spread514 = result_8.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field515 = match source_ml_source2_nominal_spread514.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value516, _) => {
                                                                                    let source_ml_source2_data_property_value516 = *source_ml_source2_data_property_value516;
                                                                                    source_ml_source2_data_property_value516.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field517 = match source_ml_source2_nominal_spread514.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value518) => {
                                                                                        source_ml_source2_data_property_value518
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override519 = self::append_java_script_call_source_instruction(match result_8.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value510, _) => {
                                                                                            let source_ml_source2_data_property_value510 = *source_ml_source2_data_property_value510;
                                                                                            source_ml_source2_data_property_value510.clone()
                                                                                        }
                                                                                    }, self::JavaScriptCallInstruction::Call(match result_8.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value511) => {
                                                                                            source_ml_source2_data_property_value511
                                                                                        }
                                                                                    }, match callee.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value512) => {
                                                                                            source_ml_source2_data_property_value512
                                                                                        }
                                                                                    }, match receiver.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value513) => {
                                                                                            source_ml_source2_data_property_value513
                                                                                        }
                                                                                    }, arguments__6));
                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override519.clone()), source_ml_source2_nominal_field517)
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
