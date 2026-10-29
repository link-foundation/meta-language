// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop102Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_branch;
use super::lower_java_script_call_source_exception;
use super::ml;
use super::ml_array;
use super::read_java_script_call_constant_error;
use super::read_java_script_call_primitive_arity_error;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop101;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop102;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop103;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop104;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop105;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_expression(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value412) => {
            source_ml_source2_data_property_value412.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((depth > (128f64)) || ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value414, _, _, _) => {
                source_ml_source2_data_property_value414
            }
        }) >= (65536f64))) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its structural budget")
        } else {
            {
                let state = {
                    let source_ml_source2_nominal_spread416 = builder.clone();
                    {
                        let source_ml_source2_nominal_field417 = match source_ml_source2_nominal_spread416.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value418, _, _, _, _, _) => {
                                source_ml_source2_data_property_value418
                            }
                        };
                        {
                            let source_ml_source2_nominal_field419 = match source_ml_source2_nominal_spread416.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value420, _, _, _, _) => {
                                    source_ml_source2_data_property_value420
                                }
                            };
                            {
                                let source_ml_source2_nominal_field421 = match source_ml_source2_nominal_spread416.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value422, _, _, _) => {
                                        source_ml_source2_data_property_value422
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field423 = match source_ml_source2_nominal_spread416.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value424, _, _) => {
                                            source_ml_source2_data_property_value424
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field425 = match source_ml_source2_nominal_spread416.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value426, _) => {
                                                source_ml_source2_data_property_value426
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field427 = match source_ml_source2_nominal_spread416.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value428) => {
                                                    source_ml_source2_data_property_value428.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override429 = (match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value415, _, _, _) => {
                                                        source_ml_source2_data_property_value415
                                                    }
                                                } + 1f64);
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field417, source_ml_source2_nominal_field419, source_ml_source2_nominal_override429, source_ml_source2_nominal_field423, source_ml_source2_nominal_field425, source_ml_source2_nominal_field427.clone())
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
                                        let source_ml_source2_nominal_spread439 = result.clone();
                                        {
                                            let source_ml_source2_nominal_field440 = match source_ml_source2_nominal_spread439.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value441, _) => {
                                                    let source_ml_source2_data_property_value441 = *source_ml_source2_data_property_value441;
                                                    source_ml_source2_data_property_value441.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field442 = match source_ml_source2_nominal_spread439.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value443) => {
                                                        source_ml_source2_data_property_value443
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override444 = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value437, _) => {
                                                            let source_ml_source2_data_property_value437 = *source_ml_source2_data_property_value437;
                                                            source_ml_source2_data_property_value437.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Constant(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value438) => {
                                                            source_ml_source2_data_property_value438
                                                        }
                                                    }, Box::new(value.clone())));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override444.clone()), source_ml_source2_nominal_field442)
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
                                    let source_ml_source2_nominal_spread447 = result_2.clone();
                                    {
                                        let source_ml_source2_nominal_field448 = match source_ml_source2_nominal_spread447.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value449, _) => {
                                                let source_ml_source2_data_property_value449 = *source_ml_source2_data_property_value449;
                                                source_ml_source2_data_property_value449.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field450 = match source_ml_source2_nominal_spread447.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value451) => {
                                                    source_ml_source2_data_property_value451
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override452 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value445, _) => {
                                                        let source_ml_source2_data_property_value445 = *source_ml_source2_data_property_value445;
                                                        source_ml_source2_data_property_value445.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Argument(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value446) => {
                                                        source_ml_source2_data_property_value446
                                                    }
                                                }, index));
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override452.clone()), source_ml_source2_nominal_field450)
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
                                let source_ml_source2_nominal_spread455 = value_2.clone();
                                {
                                    let source_ml_source2_nominal_field456 = match source_ml_source2_nominal_spread455.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value457, _) => {
                                            let source_ml_source2_data_property_value457 = *source_ml_source2_data_property_value457;
                                            source_ml_source2_data_property_value457.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field458 = match source_ml_source2_nominal_spread455.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value459) => {
                                                source_ml_source2_data_property_value459
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override460 = self::append_java_script_call_source_instruction(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value453, _) => {
                                                    let source_ml_source2_data_property_value453 = *source_ml_source2_data_property_value453;
                                                    source_ml_source2_data_property_value453.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Throw(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value454) => {
                                                    source_ml_source2_data_property_value454
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override460.clone()), source_ml_source2_nominal_field458)
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceTry(source_ml_source2_f430, source_ml_source2_f431, source_ml_source2_f432) => {
                        let source_ml_source2_f431 = *source_ml_source2_f431;
                        let source_ml_source2_f432 = *source_ml_source2_f432;
                        self::lower_java_script_call_source_exception(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceBinding(identity) => {
                        if !(self::test_java_script_call_code_coordinate(identity, 65536f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source binding identities require exact nonnegative coordinates")
                        } else {
                            {
                                let index_2 = ((bindings.len() as f64) - 1f64);
                                {
                                    let source_ml_source2_s461 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop101(source.clone(), bindings, state.clone(), index_2);
                                    match source_ml_source2_s461.clone() {
                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Done => {
                                            self::refuse_java_script_call_source(state.clone(), "a source binding reference has no actual lexical owner")
                                        }
                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Return(source_ml_source1_temporary_result101) => {
                                            let source_ml_source1_temporary_result101 = *source_ml_source1_temporary_result101;
                                            source_ml_source1_temporary_result101.clone()
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
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value463, _) => {
                                source_ml_source2_data_property_value463
                            }
                        }) == identity_2) }) }; ml_values1.iter().cloned().enumerate().any(|(ml_index1, ml_value1)| ml_callback1(ml_value1)) }) {
                            self::refuse_java_script_call_source(state.clone(), "a source binding owner must be unique in its lexical scope")
                        } else {
                            {
                                let value_3 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                self::lower_java_script_call_source_expression(body.clone(), {
                                    let source_ml_source2_concat_value465 = bindings;
                                    {
                                        let source_ml_source2_concat_value466 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity_2, match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value464) => {
                                                source_ml_source2_data_property_value464
                                            }
                                        })];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source2_concat_value465), source_ml_source2_concat_value466)
                                    }
                                }, match value_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value467, _) => {
                                        let source_ml_source2_data_property_value467 = *source_ml_source2_data_property_value467;
                                        source_ml_source2_data_property_value467.clone()
                                    }
                                }, (depth + 1f64))
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceIf(source_ml_source2_f433, source_ml_source2_f434, source_ml_source2_f435) => {
                        let source_ml_source2_f433 = *source_ml_source2_f433;
                        let source_ml_source2_f434 = *source_ml_source2_f434;
                        let source_ml_source2_f435 = *source_ml_source2_f435;
                        self::lower_java_script_call_source_branch(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f436, source_left, source_right) => {
                        let source_left = *source_left;
                        let source_right = *source_right;
                        {
                            let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings, match left.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value468, _) => {
                                        let source_ml_source2_data_property_value468 = *source_ml_source2_data_property_value468;
                                        source_ml_source2_data_property_value468.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_3 = self::allocate_java_script_call_source_register(match right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value469, _) => {
                                            let source_ml_source2_data_property_value469 = *source_ml_source2_data_property_value469;
                                            source_ml_source2_data_property_value469.clone()
                                        }
                                    });
                                    {
                                        let operator = source_ml_source2_f436.clone();
                                        {
                                            let source_ml_source2_f483 = source_left.clone();
                                            {
                                                let source_ml_source2_f484 = source_right.clone();
                                                if !(self::ml_array::includes(&vec![String::from("add"), String::from("subtract"), String::from("multiply"), String::from("divide"), String::from("remainder"), String::from("power"), String::from("lessThan"), String::from("equal")], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::refuse_java_script_call_source(match result_3.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value500, _) => {
                                                            let source_ml_source2_data_property_value500 = *source_ml_source2_data_property_value500;
                                                            source_ml_source2_data_property_value500.clone()
                                                        }
                                                    }, "source Number lowering requires its exact registered operator")
                                                } else {
                                                    {
                                                        let source_ml_source2_nominal_spread505 = result_3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field506 = match source_ml_source2_nominal_spread505.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value507, _) => {
                                                                    let source_ml_source2_data_property_value507 = *source_ml_source2_data_property_value507;
                                                                    source_ml_source2_data_property_value507.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field508 = match source_ml_source2_nominal_spread505.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value509) => {
                                                                        source_ml_source2_data_property_value509
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override510 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value501, _) => {
                                                                            let source_ml_source2_data_property_value501 = *source_ml_source2_data_property_value501;
                                                                            source_ml_source2_data_property_value501.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::NumberBinary(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value502) => {
                                                                            source_ml_source2_data_property_value502
                                                                        }
                                                                    }, match left.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value503) => {
                                                                            source_ml_source2_data_property_value503
                                                                        }
                                                                    }, match right.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value504) => {
                                                                            source_ml_source2_data_property_value504
                                                                        }
                                                                    }, operator.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override510.clone()), source_ml_source2_nominal_field508)
                                                                }
                                                            }
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value511, _) => {
                                        let source_ml_source2_data_property_value511 = *source_ml_source2_data_property_value511;
                                        source_ml_source2_data_property_value511.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value512, _) => {
                                            let source_ml_source2_data_property_value512 = *source_ml_source2_data_property_value512;
                                            source_ml_source2_data_property_value512.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f528 = source_left_2.clone();
                                        {
                                            let source_ml_source2_f529 = source_right_2.clone();
                                            {
                                                let source_ml_source2_nominal_spread547 = result_4.clone();
                                                {
                                                    let source_ml_source2_nominal_field548 = match source_ml_source2_nominal_spread547.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value549, _) => {
                                                            let source_ml_source2_data_property_value549 = *source_ml_source2_data_property_value549;
                                                            source_ml_source2_data_property_value549.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field550 = match source_ml_source2_nominal_spread547.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value551) => {
                                                                source_ml_source2_data_property_value551
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override552 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value543, _) => {
                                                                    let source_ml_source2_data_property_value543 = *source_ml_source2_data_property_value543;
                                                                    source_ml_source2_data_property_value543.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::SameValue(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value544) => {
                                                                    source_ml_source2_data_property_value544
                                                                }
                                                            }, match left_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value545) => {
                                                                    source_ml_source2_data_property_value545
                                                                }
                                                            }, match right_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value546) => {
                                                                    source_ml_source2_data_property_value546
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override552.clone()), source_ml_source2_nominal_field550)
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value553, _) => {
                                        let source_ml_source2_data_property_value553 = *source_ml_source2_data_property_value553;
                                        source_ml_source2_data_property_value553.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_5 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value554, _) => {
                                            let source_ml_source2_data_property_value554 = *source_ml_source2_data_property_value554;
                                            source_ml_source2_data_property_value554.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f572 = source_left_3.clone();
                                        {
                                            let source_ml_source2_f573 = source_right_3.clone();
                                            {
                                                let source_ml_source2_nominal_spread589 = result_5.clone();
                                                {
                                                    let source_ml_source2_nominal_field590 = match source_ml_source2_nominal_spread589.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value591, _) => {
                                                            let source_ml_source2_data_property_value591 = *source_ml_source2_data_property_value591;
                                                            source_ml_source2_data_property_value591.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field592 = match source_ml_source2_nominal_spread589.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value593) => {
                                                                source_ml_source2_data_property_value593
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override594 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value585, _) => {
                                                                    let source_ml_source2_data_property_value585 = *source_ml_source2_data_property_value585;
                                                                    source_ml_source2_data_property_value585.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::StrictEqual(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value586) => {
                                                                    source_ml_source2_data_property_value586
                                                                }
                                                            }, match left_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value587) => {
                                                                    source_ml_source2_data_property_value587
                                                                }
                                                            }, match right_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value588) => {
                                                                    source_ml_source2_data_property_value588
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override594.clone()), source_ml_source2_nominal_field592)
                                                        }
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
                                            let source_ml_source1_temporary_values98 = arguments;
                                            {
                                                let source_ml_source1_temporary_index98 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s595 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop102(bindings, depth, current.clone(), arguments_, source_ml_source1_temporary_values98, source_ml_source1_temporary_index98.clone());
                                                    match source_ml_source2_s595.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop102Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop102Done(current_2, arguments__2) => {
                                                            let current_2 = *current_2;
                                                            {
                                                                let result_6 = self::allocate_java_script_call_source_register(current_2.clone());
                                                                {
                                                                    let source_ml_source2_nominal_spread598 = result_6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field599 = match source_ml_source2_nominal_spread598.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value600, _) => {
                                                                                let source_ml_source2_data_property_value600 = *source_ml_source2_data_property_value600;
                                                                                source_ml_source2_data_property_value600.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field601 = match source_ml_source2_nominal_spread598.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value602) => {
                                                                                    source_ml_source2_data_property_value602
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override603 = self::append_java_script_call_source_instruction(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value596, _) => {
                                                                                        let source_ml_source2_data_property_value596 = *source_ml_source2_data_property_value596;
                                                                                        source_ml_source2_data_property_value596.clone()
                                                                                    }
                                                                                }, self::JavaScriptCallInstruction::Primitive(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value597) => {
                                                                                        source_ml_source2_data_property_value597
                                                                                    }
                                                                                }, operation.clone(), arguments__2));
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override603.clone()), source_ml_source2_nominal_field601)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value605, _) => {
                                source_ml_source2_data_property_value605
                            }
                        }) != (0f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source recurrence through an active exception handler requires explicit unwinding")
                        } else {
                            if (((arguments_2.len() as f64)) != (match state.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value607, _, _) => {
                                    source_ml_source2_data_property_value607
                                }
                            })) {
                                self::refuse_java_script_call_source(state.clone(), "loop updates require every actual parameter operand")
                            } else {
                                {
                                    let current_3 = state.clone();
                                    {
                                        let arguments__3 = Vec::<f64>::new();
                                        {
                                            let source_ml_source1_temporary_values99 = arguments_2;
                                            {
                                                let source_ml_source1_temporary_index99 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s608 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop103(bindings, depth, current_3.clone(), arguments__3, source_ml_source1_temporary_values99, source_ml_source1_temporary_index99.clone());
                                                    match source_ml_source2_s608.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Done(current_4, arguments__4) => {
                                                            let current_4 = *current_4;
                                                            {
                                                                let index_3 = 0f64;
                                                                {
                                                                    let current_5 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop104(current_4.clone(), &(arguments__4), index_3);
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value609, _) => {
                                        let source_ml_source2_data_property_value609 = *source_ml_source2_data_property_value609;
                                        source_ml_source2_data_property_value609.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let value_4 = self::lower_java_script_call_source_expression(source_value_3.clone(), bindings, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value610, _) => {
                                            let source_ml_source2_data_property_value610 = *source_ml_source2_data_property_value610;
                                            source_ml_source2_data_property_value610.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let result_7 = self::allocate_java_script_call_source_register(match value_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value611, _) => {
                                                let source_ml_source2_data_property_value611 = *source_ml_source2_data_property_value611;
                                                source_ml_source2_data_property_value611.clone()
                                            }
                                        });
                                        {
                                            let write = self::append_java_script_call_source_instruction(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value612, _) => {
                                                    let source_ml_source2_data_property_value612 = *source_ml_source2_data_property_value612;
                                                    source_ml_source2_data_property_value612.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinarySet(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value613) => {
                                                    source_ml_source2_data_property_value613
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value614) => {
                                                    source_ml_source2_data_property_value614
                                                }
                                            }, match key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value615) => {
                                                    source_ml_source2_data_property_value615
                                                }
                                            }, match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value616) => {
                                                    source_ml_source2_data_property_value616
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value617) => {
                                                    source_ml_source2_data_property_value617
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value618) => {
                                                    source_ml_source2_data_property_value618
                                                }
                                            }))), match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value619) => {
                                                    source_ml_source2_data_property_value619
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value620, _) => {
                                        let source_ml_source2_data_property_value620 = *source_ml_source2_data_property_value620;
                                        source_ml_source2_data_property_value620.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_8 = self::allocate_java_script_call_source_register(match key_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value621, _) => {
                                            let source_ml_source2_data_property_value621 = *source_ml_source2_data_property_value621;
                                            source_ml_source2_data_property_value621.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_nominal_spread627 = result_8.clone();
                                        {
                                            let source_ml_source2_nominal_field628 = match source_ml_source2_nominal_spread627.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value629, _) => {
                                                    let source_ml_source2_data_property_value629 = *source_ml_source2_data_property_value629;
                                                    source_ml_source2_data_property_value629.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field630 = match source_ml_source2_nominal_spread627.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value631) => {
                                                        source_ml_source2_data_property_value631
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override632 = self::append_java_script_call_source_instruction(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value622, _) => {
                                                            let source_ml_source2_data_property_value622 = *source_ml_source2_data_property_value622;
                                                            source_ml_source2_data_property_value622.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryGet(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value623) => {
                                                            source_ml_source2_data_property_value623
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value624) => {
                                                            source_ml_source2_data_property_value624
                                                        }
                                                    }, match key_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value625) => {
                                                            source_ml_source2_data_property_value625
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value626) => {
                                                            source_ml_source2_data_property_value626
                                                        }
                                                    }));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override632.clone()), source_ml_source2_nominal_field630)
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
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value633, _) => {
                                            let source_ml_source2_data_property_value633 = *source_ml_source2_data_property_value633;
                                            source_ml_source2_data_property_value633.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let current_6 = match receiver.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value634, _) => {
                                                let source_ml_source2_data_property_value634 = *source_ml_source2_data_property_value634;
                                                source_ml_source2_data_property_value634.clone()
                                            }
                                        };
                                        {
                                            let arguments__5 = Vec::<f64>::new();
                                            {
                                                let source_ml_source1_temporary_values100 = arguments_3;
                                                {
                                                    let source_ml_source1_temporary_index100 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s635 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop105(bindings, depth, current_6.clone(), arguments__5, source_ml_source1_temporary_values100, source_ml_source1_temporary_index100.clone());
                                                        match source_ml_source2_s635.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Done(current_7, arguments__6) => {
                                                                let current_7 = *current_7;
                                                                {
                                                                    let result_9 = self::allocate_java_script_call_source_register(current_7.clone());
                                                                    {
                                                                        let source_ml_source2_nominal_spread640 = result_9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field641 = match source_ml_source2_nominal_spread640.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value642, _) => {
                                                                                    let source_ml_source2_data_property_value642 = *source_ml_source2_data_property_value642;
                                                                                    source_ml_source2_data_property_value642.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field643 = match source_ml_source2_nominal_spread640.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value644) => {
                                                                                        source_ml_source2_data_property_value644
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override645 = self::append_java_script_call_source_instruction(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value636, _) => {
                                                                                            let source_ml_source2_data_property_value636 = *source_ml_source2_data_property_value636;
                                                                                            source_ml_source2_data_property_value636.clone()
                                                                                        }
                                                                                    }, self::JavaScriptCallInstruction::Call(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value637) => {
                                                                                            source_ml_source2_data_property_value637
                                                                                        }
                                                                                    }, match callee.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value638) => {
                                                                                            source_ml_source2_data_property_value638
                                                                                        }
                                                                                    }, match receiver.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value639) => {
                                                                                            source_ml_source2_data_property_value639
                                                                                        }
                                                                                    }, arguments__6));
                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override645.clone()), source_ml_source2_nominal_field643)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
