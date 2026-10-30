// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop107Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_branch;
use super::lower_java_script_call_source_exception;
use super::ml;
use super::ml_array;
use super::read_java_script_call_constant_error;
use super::read_java_script_call_primitive_arity_error;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop103;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop104;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop105;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop106;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop107;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_expression(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value431) => {
            source_ml_source2_data_property_value431.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((depth > (128f64)) || ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value433, _, _, _) => {
                source_ml_source2_data_property_value433
            }
        }) >= (65536f64))) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its structural budget")
        } else {
            {
                let state = {
                    let source_ml_source2_nominal_spread435 = builder.clone();
                    {
                        let source_ml_source2_nominal_field436 = match source_ml_source2_nominal_spread435.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value437, _, _, _, _, _) => {
                                source_ml_source2_data_property_value437
                            }
                        };
                        {
                            let source_ml_source2_nominal_field438 = match source_ml_source2_nominal_spread435.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value439, _, _, _, _) => {
                                    source_ml_source2_data_property_value439
                                }
                            };
                            {
                                let source_ml_source2_nominal_field440 = match source_ml_source2_nominal_spread435.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value441, _, _, _) => {
                                        source_ml_source2_data_property_value441
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field442 = match source_ml_source2_nominal_spread435.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value443, _, _) => {
                                            source_ml_source2_data_property_value443
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field444 = match source_ml_source2_nominal_spread435.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value445, _) => {
                                                source_ml_source2_data_property_value445
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field446 = match source_ml_source2_nominal_spread435.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value447) => {
                                                    source_ml_source2_data_property_value447.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override448 = (match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value434, _, _, _) => {
                                                        source_ml_source2_data_property_value434
                                                    }
                                                } + 1f64);
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field436, source_ml_source2_nominal_field438, source_ml_source2_nominal_override448, source_ml_source2_nominal_field442, source_ml_source2_nominal_field444, source_ml_source2_nominal_field446.clone())
                                            }
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
                                        let source_ml_source2_nominal_spread458 = result.clone();
                                        {
                                            let source_ml_source2_nominal_field459 = match source_ml_source2_nominal_spread458.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value460, _) => {
                                                    let source_ml_source2_data_property_value460 = *source_ml_source2_data_property_value460;
                                                    source_ml_source2_data_property_value460.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field461 = match source_ml_source2_nominal_spread458.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value462) => {
                                                        source_ml_source2_data_property_value462
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override463 = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value456, _) => {
                                                            let source_ml_source2_data_property_value456 = *source_ml_source2_data_property_value456;
                                                            source_ml_source2_data_property_value456.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Constant(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value457) => {
                                                            source_ml_source2_data_property_value457
                                                        }
                                                    }, Box::new(value.clone())));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override463.clone()), source_ml_source2_nominal_field461)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceReceiver => {
                        self::JavaScriptCallSourceResult::SourceResult(Box::new(state.clone()), 0f64)
                    }
                    self::JavaScriptCallSource::SourceArgument(index) => {
                        if !(self::test_java_script_call_code_coordinate(index, 1048576f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source argument reads require an actual bounded nonnegative coordinate")
                        } else {
                            {
                                let result_2 = self::allocate_java_script_call_source_register(state.clone());
                                {
                                    let source_ml_source2_nominal_spread466 = result_2.clone();
                                    {
                                        let source_ml_source2_nominal_field467 = match source_ml_source2_nominal_spread466.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value468, _) => {
                                                let source_ml_source2_data_property_value468 = *source_ml_source2_data_property_value468;
                                                source_ml_source2_data_property_value468.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field469 = match source_ml_source2_nominal_spread466.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value470) => {
                                                    source_ml_source2_data_property_value470
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override471 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value464, _) => {
                                                        let source_ml_source2_data_property_value464 = *source_ml_source2_data_property_value464;
                                                        source_ml_source2_data_property_value464.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Argument(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value465) => {
                                                        source_ml_source2_data_property_value465
                                                    }
                                                }, index));
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override471.clone()), source_ml_source2_nominal_field469)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceThrow(source_value) => {
                        let source_value = *source_value;
                        {
                            let value_2 = self::lower_java_script_call_source_expression(source_value.clone(), bindings, state.clone(), (depth + 1f64));
                            {
                                let source_ml_source2_nominal_spread474 = value_2.clone();
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
                                            let source_ml_source2_nominal_override479 = self::append_java_script_call_source_instruction(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value472, _) => {
                                                    let source_ml_source2_data_property_value472 = *source_ml_source2_data_property_value472;
                                                    source_ml_source2_data_property_value472.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Throw(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value473) => {
                                                    source_ml_source2_data_property_value473
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override479.clone()), source_ml_source2_nominal_field477)
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceTry(source_ml_source2_f449, source_ml_source2_f450, source_ml_source2_f451) => {
                        let source_ml_source2_f450 = *source_ml_source2_f450;
                        let source_ml_source2_f451 = *source_ml_source2_f451;
                        self::lower_java_script_call_source_exception(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceBinding(identity) => {
                        if !(self::test_java_script_call_code_coordinate(identity, 65536f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source binding identities require exact nonnegative coordinates")
                        } else {
                            {
                                let index_2 = ((bindings.len() as f64) - 1f64);
                                {
                                    let source_ml_source2_s480 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop103(source.clone(), bindings, state.clone(), index_2);
                                    match source_ml_source2_s480.clone() {
                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Done => {
                                            self::refuse_java_script_call_source(state.clone(), "a source binding reference has no actual lexical owner")
                                        }
                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Return(source_ml_source1_temporary_result103) => {
                                            let source_ml_source1_temporary_result103 = *source_ml_source1_temporary_result103;
                                            source_ml_source1_temporary_result103.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceLet(identity_2, source_value_2, body) => {
                        let source_value_2 = *source_value_2;
                        let body = *body;
                        if (!(self::test_java_script_call_code_coordinate(identity_2, 65536f64)) || { let ml_values1 = bindings.clone(); let ml_callback1 = { let identity_2 = identity_2; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value482, _) => {
                                source_ml_source2_data_property_value482
                            }
                        }) == identity_2) }) }; ml_values1.iter().cloned().enumerate().any(|(ml_index1, ml_value1)| ml_callback1(ml_value1)) }) {
                            self::refuse_java_script_call_source(state.clone(), "a source binding owner must be unique in its lexical scope")
                        } else {
                            {
                                let value_3 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                self::lower_java_script_call_source_expression(body.clone(), {
                                    let source_ml_source2_concat_value484 = bindings;
                                    {
                                        let source_ml_source2_concat_value485 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity_2, match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value483) => {
                                                source_ml_source2_data_property_value483
                                            }
                                        })];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source2_concat_value484), source_ml_source2_concat_value485)
                                    }
                                }, match value_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value486, _) => {
                                        let source_ml_source2_data_property_value486 = *source_ml_source2_data_property_value486;
                                        source_ml_source2_data_property_value486.clone()
                                    }
                                }, (depth + 1f64))
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceIf(source_ml_source2_f452, source_ml_source2_f453, source_ml_source2_f454) => {
                        let source_ml_source2_f452 = *source_ml_source2_f452;
                        let source_ml_source2_f453 = *source_ml_source2_f453;
                        let source_ml_source2_f454 = *source_ml_source2_f454;
                        self::lower_java_script_call_source_branch(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f455, source_left, source_right) => {
                        let source_left = *source_left;
                        let source_right = *source_right;
                        {
                            let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings, match left.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value487, _) => {
                                        let source_ml_source2_data_property_value487 = *source_ml_source2_data_property_value487;
                                        source_ml_source2_data_property_value487.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_3 = self::allocate_java_script_call_source_register(match right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value488, _) => {
                                            let source_ml_source2_data_property_value488 = *source_ml_source2_data_property_value488;
                                            source_ml_source2_data_property_value488.clone()
                                        }
                                    });
                                    {
                                        let operator = source_ml_source2_f455.clone();
                                        {
                                            let source_ml_source2_f502 = source_left.clone();
                                            {
                                                let source_ml_source2_f503 = source_right.clone();
                                                if !(self::ml_array::includes(&vec![String::from("add"), String::from("subtract"), String::from("multiply"), String::from("divide"), String::from("remainder"), String::from("power"), String::from("lessThan"), String::from("equal")], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::refuse_java_script_call_source(match result_3.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value519, _) => {
                                                            let source_ml_source2_data_property_value519 = *source_ml_source2_data_property_value519;
                                                            source_ml_source2_data_property_value519.clone()
                                                        }
                                                    }, "source Number lowering requires its exact registered operator")
                                                } else {
                                                    {
                                                        let source_ml_source2_nominal_spread524 = result_3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field525 = match source_ml_source2_nominal_spread524.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value526, _) => {
                                                                    let source_ml_source2_data_property_value526 = *source_ml_source2_data_property_value526;
                                                                    source_ml_source2_data_property_value526.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field527 = match source_ml_source2_nominal_spread524.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value528) => {
                                                                        source_ml_source2_data_property_value528
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override529 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value520, _) => {
                                                                            let source_ml_source2_data_property_value520 = *source_ml_source2_data_property_value520;
                                                                            source_ml_source2_data_property_value520.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::NumberBinary(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value521) => {
                                                                            source_ml_source2_data_property_value521
                                                                        }
                                                                    }, match left.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value522) => {
                                                                            source_ml_source2_data_property_value522
                                                                        }
                                                                    }, match right.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value523) => {
                                                                            source_ml_source2_data_property_value523
                                                                        }
                                                                    }, operator.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override529.clone()), source_ml_source2_nominal_field527)
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value530, _) => {
                                        let source_ml_source2_data_property_value530 = *source_ml_source2_data_property_value530;
                                        source_ml_source2_data_property_value530.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value531, _) => {
                                            let source_ml_source2_data_property_value531 = *source_ml_source2_data_property_value531;
                                            source_ml_source2_data_property_value531.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f547 = source_left_2.clone();
                                        {
                                            let source_ml_source2_f548 = source_right_2.clone();
                                            {
                                                let source_ml_source2_nominal_spread566 = result_4.clone();
                                                {
                                                    let source_ml_source2_nominal_field567 = match source_ml_source2_nominal_spread566.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value568, _) => {
                                                            let source_ml_source2_data_property_value568 = *source_ml_source2_data_property_value568;
                                                            source_ml_source2_data_property_value568.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field569 = match source_ml_source2_nominal_spread566.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value570) => {
                                                                source_ml_source2_data_property_value570
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override571 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value562, _) => {
                                                                    let source_ml_source2_data_property_value562 = *source_ml_source2_data_property_value562;
                                                                    source_ml_source2_data_property_value562.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::SameValue(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value563) => {
                                                                    source_ml_source2_data_property_value563
                                                                }
                                                            }, match left_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value564) => {
                                                                    source_ml_source2_data_property_value564
                                                                }
                                                            }, match right_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value565) => {
                                                                    source_ml_source2_data_property_value565
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override571.clone()), source_ml_source2_nominal_field569)
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value572, _) => {
                                        let source_ml_source2_data_property_value572 = *source_ml_source2_data_property_value572;
                                        source_ml_source2_data_property_value572.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_5 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value573, _) => {
                                            let source_ml_source2_data_property_value573 = *source_ml_source2_data_property_value573;
                                            source_ml_source2_data_property_value573.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f591 = source_left_3.clone();
                                        {
                                            let source_ml_source2_f592 = source_right_3.clone();
                                            {
                                                let source_ml_source2_nominal_spread608 = result_5.clone();
                                                {
                                                    let source_ml_source2_nominal_field609 = match source_ml_source2_nominal_spread608.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value610, _) => {
                                                            let source_ml_source2_data_property_value610 = *source_ml_source2_data_property_value610;
                                                            source_ml_source2_data_property_value610.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field611 = match source_ml_source2_nominal_spread608.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value612) => {
                                                                source_ml_source2_data_property_value612
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override613 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value604, _) => {
                                                                    let source_ml_source2_data_property_value604 = *source_ml_source2_data_property_value604;
                                                                    source_ml_source2_data_property_value604.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::StrictEqual(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value605) => {
                                                                    source_ml_source2_data_property_value605
                                                                }
                                                            }, match left_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value606) => {
                                                                    source_ml_source2_data_property_value606
                                                                }
                                                            }, match right_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value607) => {
                                                                    source_ml_source2_data_property_value607
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override613.clone()), source_ml_source2_nominal_field611)
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
                                            let source_ml_source1_temporary_values100 = arguments;
                                            {
                                                let source_ml_source1_temporary_index100 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s614 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop104(bindings, depth, current.clone(), arguments_, source_ml_source1_temporary_values100, source_ml_source1_temporary_index100.clone());
                                                    match source_ml_source2_s614.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Done(current_2, arguments__2) => {
                                                            let current_2 = *current_2;
                                                            {
                                                                let result_6 = self::allocate_java_script_call_source_register(current_2.clone());
                                                                {
                                                                    let source_ml_source2_nominal_spread617 = result_6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field618 = match source_ml_source2_nominal_spread617.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value619, _) => {
                                                                                let source_ml_source2_data_property_value619 = *source_ml_source2_data_property_value619;
                                                                                source_ml_source2_data_property_value619.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field620 = match source_ml_source2_nominal_spread617.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value621) => {
                                                                                    source_ml_source2_data_property_value621
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override622 = self::append_java_script_call_source_instruction(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value615, _) => {
                                                                                        let source_ml_source2_data_property_value615 = *source_ml_source2_data_property_value615;
                                                                                        source_ml_source2_data_property_value615.clone()
                                                                                    }
                                                                                }, self::JavaScriptCallInstruction::Primitive(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value616) => {
                                                                                        source_ml_source2_data_property_value616
                                                                                    }
                                                                                }, operation.clone(), arguments__2));
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override622.clone()), source_ml_source2_nominal_field620)
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
                        if ((match state.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value624, _) => {
                                source_ml_source2_data_property_value624
                            }
                        }) != (0f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source recurrence through an active exception handler requires explicit unwinding")
                        } else {
                            if (((arguments_2.len() as f64)) != (match state.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value626, _, _) => {
                                    source_ml_source2_data_property_value626
                                }
                            })) {
                                self::refuse_java_script_call_source(state.clone(), "loop updates require every actual parameter operand")
                            } else {
                                {
                                    let current_3 = state.clone();
                                    {
                                        let arguments__3 = Vec::<f64>::new();
                                        {
                                            let source_ml_source1_temporary_values101 = arguments_2;
                                            {
                                                let source_ml_source1_temporary_index101 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s627 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop105(bindings, depth, current_3.clone(), arguments__3, source_ml_source1_temporary_values101, source_ml_source1_temporary_index101.clone());
                                                    match source_ml_source2_s627.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Done(current_4, arguments__4) => {
                                                            let current_4 = *current_4;
                                                            {
                                                                let index_3 = 0f64;
                                                                {
                                                                    let current_5 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop106(current_4.clone(), &(arguments__4), index_3);
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
                    }
                    self::JavaScriptCallSource::SourceSet(source_object, source_key, source_value_3) => {
                        let source_object = *source_object;
                        let source_key = *source_key;
                        let source_value_3 = *source_value_3;
                        {
                            let object = self::lower_java_script_call_source_expression(source_object.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let key = self::lower_java_script_call_source_expression(source_key.clone(), bindings.clone(), match object.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value628, _) => {
                                        let source_ml_source2_data_property_value628 = *source_ml_source2_data_property_value628;
                                        source_ml_source2_data_property_value628.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let value_4 = self::lower_java_script_call_source_expression(source_value_3.clone(), bindings, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value629, _) => {
                                            let source_ml_source2_data_property_value629 = *source_ml_source2_data_property_value629;
                                            source_ml_source2_data_property_value629.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let result_7 = self::allocate_java_script_call_source_register(match value_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value630, _) => {
                                                let source_ml_source2_data_property_value630 = *source_ml_source2_data_property_value630;
                                                source_ml_source2_data_property_value630.clone()
                                            }
                                        });
                                        {
                                            let write = self::append_java_script_call_source_instruction(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value631, _) => {
                                                    let source_ml_source2_data_property_value631 = *source_ml_source2_data_property_value631;
                                                    source_ml_source2_data_property_value631.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinarySet(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value632) => {
                                                    source_ml_source2_data_property_value632
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value633) => {
                                                    source_ml_source2_data_property_value633
                                                }
                                            }, match key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value634) => {
                                                    source_ml_source2_data_property_value634
                                                }
                                            }, match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value635) => {
                                                    source_ml_source2_data_property_value635
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value636) => {
                                                    source_ml_source2_data_property_value636
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value637) => {
                                                    source_ml_source2_data_property_value637
                                                }
                                            }))), match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value638) => {
                                                    source_ml_source2_data_property_value638
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value639, _) => {
                                        let source_ml_source2_data_property_value639 = *source_ml_source2_data_property_value639;
                                        source_ml_source2_data_property_value639.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_8 = self::allocate_java_script_call_source_register(match key_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value640, _) => {
                                            let source_ml_source2_data_property_value640 = *source_ml_source2_data_property_value640;
                                            source_ml_source2_data_property_value640.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_nominal_spread646 = result_8.clone();
                                        {
                                            let source_ml_source2_nominal_field647 = match source_ml_source2_nominal_spread646.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value648, _) => {
                                                    let source_ml_source2_data_property_value648 = *source_ml_source2_data_property_value648;
                                                    source_ml_source2_data_property_value648.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field649 = match source_ml_source2_nominal_spread646.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value650) => {
                                                        source_ml_source2_data_property_value650
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override651 = self::append_java_script_call_source_instruction(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value641, _) => {
                                                            let source_ml_source2_data_property_value641 = *source_ml_source2_data_property_value641;
                                                            source_ml_source2_data_property_value641.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryGet(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value642) => {
                                                            source_ml_source2_data_property_value642
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value643) => {
                                                            source_ml_source2_data_property_value643
                                                        }
                                                    }, match key_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value644) => {
                                                            source_ml_source2_data_property_value644
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value645) => {
                                                            source_ml_source2_data_property_value645
                                                        }
                                                    }));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override651.clone()), source_ml_source2_nominal_field649)
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
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value652, _) => {
                                            let source_ml_source2_data_property_value652 = *source_ml_source2_data_property_value652;
                                            source_ml_source2_data_property_value652.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let current_6 = match receiver.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value653, _) => {
                                                let source_ml_source2_data_property_value653 = *source_ml_source2_data_property_value653;
                                                source_ml_source2_data_property_value653.clone()
                                            }
                                        };
                                        {
                                            let arguments__5 = Vec::<f64>::new();
                                            {
                                                let source_ml_source1_temporary_values102 = arguments_3;
                                                {
                                                    let source_ml_source1_temporary_index102 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s654 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop107(bindings, depth, current_6.clone(), arguments__5, source_ml_source1_temporary_values102, source_ml_source1_temporary_index102.clone());
                                                        match source_ml_source2_s654.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop107Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop107Done(current_7, arguments__6) => {
                                                                let current_7 = *current_7;
                                                                {
                                                                    let result_9 = self::allocate_java_script_call_source_register(current_7.clone());
                                                                    {
                                                                        let source_ml_source2_nominal_spread659 = result_9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field660 = match source_ml_source2_nominal_spread659.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value661, _) => {
                                                                                    let source_ml_source2_data_property_value661 = *source_ml_source2_data_property_value661;
                                                                                    source_ml_source2_data_property_value661.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field662 = match source_ml_source2_nominal_spread659.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value663) => {
                                                                                        source_ml_source2_data_property_value663
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override664 = self::append_java_script_call_source_instruction(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value655, _) => {
                                                                                            let source_ml_source2_data_property_value655 = *source_ml_source2_data_property_value655;
                                                                                            source_ml_source2_data_property_value655.clone()
                                                                                        }
                                                                                    }, self::JavaScriptCallInstruction::Call(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value656) => {
                                                                                            source_ml_source2_data_property_value656
                                                                                        }
                                                                                    }, match callee.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value657) => {
                                                                                            source_ml_source2_data_property_value657
                                                                                        }
                                                                                    }, match receiver.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value658) => {
                                                                                            source_ml_source2_data_property_value658
                                                                                        }
                                                                                    }, arguments__6));
                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override664.clone()), source_ml_source2_nominal_field662)
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
