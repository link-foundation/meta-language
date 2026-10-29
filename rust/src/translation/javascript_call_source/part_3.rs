// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop102Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_branch;
use super::lower_java_script_call_source_exception;
use super::ml;
use super::ml_array;
use super::read_java_script_call_constant_error;
use super::read_java_script_call_primitive_arity_error;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop100;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop101;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop102;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop103;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop104;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_expression(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value394) => {
            source_ml_source2_data_property_value394.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((depth > (128f64)) || ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value396, _, _, _) => {
                source_ml_source2_data_property_value396
            }
        }) >= (65536f64))) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its structural budget")
        } else {
            {
                let state = {
                    let source_ml_source2_nominal_spread398 = builder.clone();
                    {
                        let source_ml_source2_nominal_field399 = match source_ml_source2_nominal_spread398.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value400, _, _, _, _, _) => {
                                source_ml_source2_data_property_value400
                            }
                        };
                        {
                            let source_ml_source2_nominal_field401 = match source_ml_source2_nominal_spread398.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value402, _, _, _, _) => {
                                    source_ml_source2_data_property_value402
                                }
                            };
                            {
                                let source_ml_source2_nominal_field403 = match source_ml_source2_nominal_spread398.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value404, _, _, _) => {
                                        source_ml_source2_data_property_value404
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field405 = match source_ml_source2_nominal_spread398.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value406, _, _) => {
                                            source_ml_source2_data_property_value406
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field407 = match source_ml_source2_nominal_spread398.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value408, _) => {
                                                source_ml_source2_data_property_value408
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field409 = match source_ml_source2_nominal_spread398.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value410) => {
                                                    source_ml_source2_data_property_value410.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override411 = (match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value397, _, _, _) => {
                                                        source_ml_source2_data_property_value397
                                                    }
                                                } + 1f64);
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field399, source_ml_source2_nominal_field401, source_ml_source2_nominal_override411, source_ml_source2_nominal_field405, source_ml_source2_nominal_field407, source_ml_source2_nominal_field409.clone())
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
                                        let source_ml_source2_nominal_spread421 = result.clone();
                                        {
                                            let source_ml_source2_nominal_field422 = match source_ml_source2_nominal_spread421.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value423, _) => {
                                                    let source_ml_source2_data_property_value423 = *source_ml_source2_data_property_value423;
                                                    source_ml_source2_data_property_value423.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field424 = match source_ml_source2_nominal_spread421.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value425) => {
                                                        source_ml_source2_data_property_value425
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override426 = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value419, _) => {
                                                            let source_ml_source2_data_property_value419 = *source_ml_source2_data_property_value419;
                                                            source_ml_source2_data_property_value419.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Constant(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value420) => {
                                                            source_ml_source2_data_property_value420
                                                        }
                                                    }, Box::new(value.clone())));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override426.clone()), source_ml_source2_nominal_field424)
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
                                    let source_ml_source2_nominal_spread429 = result_2.clone();
                                    {
                                        let source_ml_source2_nominal_field430 = match source_ml_source2_nominal_spread429.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value431, _) => {
                                                let source_ml_source2_data_property_value431 = *source_ml_source2_data_property_value431;
                                                source_ml_source2_data_property_value431.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field432 = match source_ml_source2_nominal_spread429.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value433) => {
                                                    source_ml_source2_data_property_value433
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override434 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value427, _) => {
                                                        let source_ml_source2_data_property_value427 = *source_ml_source2_data_property_value427;
                                                        source_ml_source2_data_property_value427.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Argument(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value428) => {
                                                        source_ml_source2_data_property_value428
                                                    }
                                                }, index));
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override434.clone()), source_ml_source2_nominal_field432)
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
                                let source_ml_source2_nominal_spread437 = value_2.clone();
                                {
                                    let source_ml_source2_nominal_field438 = match source_ml_source2_nominal_spread437.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value439, _) => {
                                            let source_ml_source2_data_property_value439 = *source_ml_source2_data_property_value439;
                                            source_ml_source2_data_property_value439.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field440 = match source_ml_source2_nominal_spread437.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value441) => {
                                                source_ml_source2_data_property_value441
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override442 = self::append_java_script_call_source_instruction(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value435, _) => {
                                                    let source_ml_source2_data_property_value435 = *source_ml_source2_data_property_value435;
                                                    source_ml_source2_data_property_value435.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Throw(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value436) => {
                                                    source_ml_source2_data_property_value436
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override442.clone()), source_ml_source2_nominal_field440)
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceTry(source_ml_source2_f412, source_ml_source2_f413, source_ml_source2_f414) => {
                        let source_ml_source2_f413 = *source_ml_source2_f413;
                        let source_ml_source2_f414 = *source_ml_source2_f414;
                        self::lower_java_script_call_source_exception(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceBinding(identity) => {
                        if !(self::test_java_script_call_code_coordinate(identity, 65536f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source binding identities require exact nonnegative coordinates")
                        } else {
                            {
                                let index_2 = ((bindings.len() as f64) - 1f64);
                                {
                                    let source_ml_source2_s443 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop100(source.clone(), bindings, state.clone(), index_2);
                                    match source_ml_source2_s443.clone() {
                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Done => {
                                            self::refuse_java_script_call_source(state.clone(), "a source binding reference has no actual lexical owner")
                                        }
                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Return(source_ml_source1_temporary_result100) => {
                                            let source_ml_source1_temporary_result100 = *source_ml_source1_temporary_result100;
                                            source_ml_source1_temporary_result100.clone()
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
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value445, _) => {
                                source_ml_source2_data_property_value445
                            }
                        }) == identity_2) }) }; ml_values1.iter().cloned().enumerate().any(|(ml_index1, ml_value1)| ml_callback1(ml_value1)) }) {
                            self::refuse_java_script_call_source(state.clone(), "a source binding owner must be unique in its lexical scope")
                        } else {
                            {
                                let value_3 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                self::lower_java_script_call_source_expression(body.clone(), {
                                    let source_ml_source2_concat_value447 = bindings;
                                    {
                                        let source_ml_source2_concat_value448 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity_2, match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value446) => {
                                                source_ml_source2_data_property_value446
                                            }
                                        })];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source2_concat_value447), source_ml_source2_concat_value448)
                                    }
                                }, match value_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value449, _) => {
                                        let source_ml_source2_data_property_value449 = *source_ml_source2_data_property_value449;
                                        source_ml_source2_data_property_value449.clone()
                                    }
                                }, (depth + 1f64))
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceIf(source_ml_source2_f415, source_ml_source2_f416, source_ml_source2_f417) => {
                        let source_ml_source2_f415 = *source_ml_source2_f415;
                        let source_ml_source2_f416 = *source_ml_source2_f416;
                        let source_ml_source2_f417 = *source_ml_source2_f417;
                        self::lower_java_script_call_source_branch(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f418, source_left, source_right) => {
                        let source_left = *source_left;
                        let source_right = *source_right;
                        {
                            let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings, match left.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value450, _) => {
                                        let source_ml_source2_data_property_value450 = *source_ml_source2_data_property_value450;
                                        source_ml_source2_data_property_value450.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_3 = self::allocate_java_script_call_source_register(match right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value451, _) => {
                                            let source_ml_source2_data_property_value451 = *source_ml_source2_data_property_value451;
                                            source_ml_source2_data_property_value451.clone()
                                        }
                                    });
                                    {
                                        let operator = source_ml_source2_f418.clone();
                                        {
                                            let source_ml_source2_f465 = source_left.clone();
                                            {
                                                let source_ml_source2_f466 = source_right.clone();
                                                if !(self::ml_array::includes(&vec![String::from("add"), String::from("subtract"), String::from("multiply"), String::from("divide"), String::from("remainder"), String::from("power"), String::from("lessThan"), String::from("equal")], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::refuse_java_script_call_source(match result_3.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value482, _) => {
                                                            let source_ml_source2_data_property_value482 = *source_ml_source2_data_property_value482;
                                                            source_ml_source2_data_property_value482.clone()
                                                        }
                                                    }, "source Number lowering requires its exact registered operator")
                                                } else {
                                                    {
                                                        let source_ml_source2_nominal_spread487 = result_3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field488 = match source_ml_source2_nominal_spread487.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value489, _) => {
                                                                    let source_ml_source2_data_property_value489 = *source_ml_source2_data_property_value489;
                                                                    source_ml_source2_data_property_value489.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field490 = match source_ml_source2_nominal_spread487.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value491) => {
                                                                        source_ml_source2_data_property_value491
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override492 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value483, _) => {
                                                                            let source_ml_source2_data_property_value483 = *source_ml_source2_data_property_value483;
                                                                            source_ml_source2_data_property_value483.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::NumberBinary(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value484) => {
                                                                            source_ml_source2_data_property_value484
                                                                        }
                                                                    }, match left.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value485) => {
                                                                            source_ml_source2_data_property_value485
                                                                        }
                                                                    }, match right.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value486) => {
                                                                            source_ml_source2_data_property_value486
                                                                        }
                                                                    }, operator.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override492.clone()), source_ml_source2_nominal_field490)
                                                                }
                                                            }
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value493, _) => {
                                        let source_ml_source2_data_property_value493 = *source_ml_source2_data_property_value493;
                                        source_ml_source2_data_property_value493.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value494, _) => {
                                            let source_ml_source2_data_property_value494 = *source_ml_source2_data_property_value494;
                                            source_ml_source2_data_property_value494.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f510 = source_left_2.clone();
                                        {
                                            let source_ml_source2_f511 = source_right_2.clone();
                                            {
                                                let source_ml_source2_nominal_spread529 = result_4.clone();
                                                {
                                                    let source_ml_source2_nominal_field530 = match source_ml_source2_nominal_spread529.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value531, _) => {
                                                            let source_ml_source2_data_property_value531 = *source_ml_source2_data_property_value531;
                                                            source_ml_source2_data_property_value531.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field532 = match source_ml_source2_nominal_spread529.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value533) => {
                                                                source_ml_source2_data_property_value533
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override534 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value525, _) => {
                                                                    let source_ml_source2_data_property_value525 = *source_ml_source2_data_property_value525;
                                                                    source_ml_source2_data_property_value525.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::SameValue(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value526) => {
                                                                    source_ml_source2_data_property_value526
                                                                }
                                                            }, match left_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value527) => {
                                                                    source_ml_source2_data_property_value527
                                                                }
                                                            }, match right_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value528) => {
                                                                    source_ml_source2_data_property_value528
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override534.clone()), source_ml_source2_nominal_field532)
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value535, _) => {
                                        let source_ml_source2_data_property_value535 = *source_ml_source2_data_property_value535;
                                        source_ml_source2_data_property_value535.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_5 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value536, _) => {
                                            let source_ml_source2_data_property_value536 = *source_ml_source2_data_property_value536;
                                            source_ml_source2_data_property_value536.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f554 = source_left_3.clone();
                                        {
                                            let source_ml_source2_f555 = source_right_3.clone();
                                            {
                                                let source_ml_source2_nominal_spread571 = result_5.clone();
                                                {
                                                    let source_ml_source2_nominal_field572 = match source_ml_source2_nominal_spread571.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value573, _) => {
                                                            let source_ml_source2_data_property_value573 = *source_ml_source2_data_property_value573;
                                                            source_ml_source2_data_property_value573.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field574 = match source_ml_source2_nominal_spread571.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value575) => {
                                                                source_ml_source2_data_property_value575
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override576 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value567, _) => {
                                                                    let source_ml_source2_data_property_value567 = *source_ml_source2_data_property_value567;
                                                                    source_ml_source2_data_property_value567.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::StrictEqual(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value568) => {
                                                                    source_ml_source2_data_property_value568
                                                                }
                                                            }, match left_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value569) => {
                                                                    source_ml_source2_data_property_value569
                                                                }
                                                            }, match right_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value570) => {
                                                                    source_ml_source2_data_property_value570
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override576.clone()), source_ml_source2_nominal_field574)
                                                        }
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
                                            let source_ml_source1_temporary_values97 = arguments;
                                            {
                                                let source_ml_source1_temporary_index97 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s577 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop101(bindings, depth, current.clone(), arguments_, source_ml_source1_temporary_values97, source_ml_source1_temporary_index97.clone());
                                                    match source_ml_source2_s577.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Done(current_2, arguments__2) => {
                                                            let current_2 = *current_2;
                                                            {
                                                                let result_6 = self::allocate_java_script_call_source_register(current_2.clone());
                                                                {
                                                                    let source_ml_source2_nominal_spread580 = result_6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field581 = match source_ml_source2_nominal_spread580.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value582, _) => {
                                                                                let source_ml_source2_data_property_value582 = *source_ml_source2_data_property_value582;
                                                                                source_ml_source2_data_property_value582.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field583 = match source_ml_source2_nominal_spread580.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value584) => {
                                                                                    source_ml_source2_data_property_value584
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override585 = self::append_java_script_call_source_instruction(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value578, _) => {
                                                                                        let source_ml_source2_data_property_value578 = *source_ml_source2_data_property_value578;
                                                                                        source_ml_source2_data_property_value578.clone()
                                                                                    }
                                                                                }, self::JavaScriptCallInstruction::Primitive(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value579) => {
                                                                                        source_ml_source2_data_property_value579
                                                                                    }
                                                                                }, operation.clone(), arguments__2));
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override585.clone()), source_ml_source2_nominal_field583)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value587, _) => {
                                source_ml_source2_data_property_value587
                            }
                        }) != (0f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source recurrence through an active exception handler requires explicit unwinding")
                        } else {
                            if (((arguments_2.len() as f64)) != (match state.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value589, _, _) => {
                                    source_ml_source2_data_property_value589
                                }
                            })) {
                                self::refuse_java_script_call_source(state.clone(), "loop updates require every actual parameter operand")
                            } else {
                                {
                                    let current_3 = state.clone();
                                    {
                                        let arguments__3 = Vec::<f64>::new();
                                        {
                                            let source_ml_source1_temporary_values98 = arguments_2;
                                            {
                                                let source_ml_source1_temporary_index98 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s590 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop102(bindings, depth, current_3.clone(), arguments__3, source_ml_source1_temporary_values98, source_ml_source1_temporary_index98.clone());
                                                    match source_ml_source2_s590.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop102Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop102Done(current_4, arguments__4) => {
                                                            let current_4 = *current_4;
                                                            {
                                                                let index_3 = 0f64;
                                                                {
                                                                    let current_5 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop103(current_4.clone(), &(arguments__4), index_3);
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value591, _) => {
                                        let source_ml_source2_data_property_value591 = *source_ml_source2_data_property_value591;
                                        source_ml_source2_data_property_value591.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let value_4 = self::lower_java_script_call_source_expression(source_value_3.clone(), bindings, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value592, _) => {
                                            let source_ml_source2_data_property_value592 = *source_ml_source2_data_property_value592;
                                            source_ml_source2_data_property_value592.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let result_7 = self::allocate_java_script_call_source_register(match value_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value593, _) => {
                                                let source_ml_source2_data_property_value593 = *source_ml_source2_data_property_value593;
                                                source_ml_source2_data_property_value593.clone()
                                            }
                                        });
                                        {
                                            let write = self::append_java_script_call_source_instruction(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value594, _) => {
                                                    let source_ml_source2_data_property_value594 = *source_ml_source2_data_property_value594;
                                                    source_ml_source2_data_property_value594.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinarySet(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value595) => {
                                                    source_ml_source2_data_property_value595
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value596) => {
                                                    source_ml_source2_data_property_value596
                                                }
                                            }, match key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value597) => {
                                                    source_ml_source2_data_property_value597
                                                }
                                            }, match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value598) => {
                                                    source_ml_source2_data_property_value598
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value599) => {
                                                    source_ml_source2_data_property_value599
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value600) => {
                                                    source_ml_source2_data_property_value600
                                                }
                                            }))), match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value601) => {
                                                    source_ml_source2_data_property_value601
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value602, _) => {
                                        let source_ml_source2_data_property_value602 = *source_ml_source2_data_property_value602;
                                        source_ml_source2_data_property_value602.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_8 = self::allocate_java_script_call_source_register(match key_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value603, _) => {
                                            let source_ml_source2_data_property_value603 = *source_ml_source2_data_property_value603;
                                            source_ml_source2_data_property_value603.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_nominal_spread609 = result_8.clone();
                                        {
                                            let source_ml_source2_nominal_field610 = match source_ml_source2_nominal_spread609.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value611, _) => {
                                                    let source_ml_source2_data_property_value611 = *source_ml_source2_data_property_value611;
                                                    source_ml_source2_data_property_value611.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field612 = match source_ml_source2_nominal_spread609.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value613) => {
                                                        source_ml_source2_data_property_value613
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override614 = self::append_java_script_call_source_instruction(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value604, _) => {
                                                            let source_ml_source2_data_property_value604 = *source_ml_source2_data_property_value604;
                                                            source_ml_source2_data_property_value604.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryGet(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value605) => {
                                                            source_ml_source2_data_property_value605
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value606) => {
                                                            source_ml_source2_data_property_value606
                                                        }
                                                    }, match key_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value607) => {
                                                            source_ml_source2_data_property_value607
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value608) => {
                                                            source_ml_source2_data_property_value608
                                                        }
                                                    }));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override614.clone()), source_ml_source2_nominal_field612)
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
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value615, _) => {
                                            let source_ml_source2_data_property_value615 = *source_ml_source2_data_property_value615;
                                            source_ml_source2_data_property_value615.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let current_6 = match receiver.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value616, _) => {
                                                let source_ml_source2_data_property_value616 = *source_ml_source2_data_property_value616;
                                                source_ml_source2_data_property_value616.clone()
                                            }
                                        };
                                        {
                                            let arguments__5 = Vec::<f64>::new();
                                            {
                                                let source_ml_source1_temporary_values99 = arguments_3;
                                                {
                                                    let source_ml_source1_temporary_index99 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s617 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop104(bindings, depth, current_6.clone(), arguments__5, source_ml_source1_temporary_values99, source_ml_source1_temporary_index99.clone());
                                                        match source_ml_source2_s617.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Done(current_7, arguments__6) => {
                                                                let current_7 = *current_7;
                                                                {
                                                                    let result_9 = self::allocate_java_script_call_source_register(current_7.clone());
                                                                    {
                                                                        let source_ml_source2_nominal_spread622 = result_9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field623 = match source_ml_source2_nominal_spread622.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value624, _) => {
                                                                                    let source_ml_source2_data_property_value624 = *source_ml_source2_data_property_value624;
                                                                                    source_ml_source2_data_property_value624.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field625 = match source_ml_source2_nominal_spread622.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value626) => {
                                                                                        source_ml_source2_data_property_value626
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override627 = self::append_java_script_call_source_instruction(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value618, _) => {
                                                                                            let source_ml_source2_data_property_value618 = *source_ml_source2_data_property_value618;
                                                                                            source_ml_source2_data_property_value618.clone()
                                                                                        }
                                                                                    }, self::JavaScriptCallInstruction::Call(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value619) => {
                                                                                            source_ml_source2_data_property_value619
                                                                                        }
                                                                                    }, match callee.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value620) => {
                                                                                            source_ml_source2_data_property_value620
                                                                                        }
                                                                                    }, match receiver.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value621) => {
                                                                                            source_ml_source2_data_property_value621
                                                                                        }
                                                                                    }, arguments__6));
                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override627.clone()), source_ml_source2_nominal_field625)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
