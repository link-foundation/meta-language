// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop99Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_branch;
use super::lower_java_script_call_source_exception;
use super::ml;
use super::ml_array;
use super::read_java_script_call_constant_error;
use super::read_java_script_call_primitive_arity_error;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop99;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop100;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop101;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop102;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop103;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_expression(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value383) => {
            source_ml_source2_data_property_value383.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((depth > (128f64)) || ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value385, _, _, _) => {
                source_ml_source2_data_property_value385
            }
        }) >= (65536f64))) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its structural budget")
        } else {
            {
                let state = {
                    let source_ml_source2_nominal_spread387 = builder.clone();
                    {
                        let source_ml_source2_nominal_field388 = match source_ml_source2_nominal_spread387.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value389, _, _, _, _, _) => {
                                source_ml_source2_data_property_value389
                            }
                        };
                        {
                            let source_ml_source2_nominal_field390 = match source_ml_source2_nominal_spread387.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value391, _, _, _, _) => {
                                    source_ml_source2_data_property_value391
                                }
                            };
                            {
                                let source_ml_source2_nominal_field392 = match source_ml_source2_nominal_spread387.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value393, _, _, _) => {
                                        source_ml_source2_data_property_value393
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field394 = match source_ml_source2_nominal_spread387.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value395, _, _) => {
                                            source_ml_source2_data_property_value395
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field396 = match source_ml_source2_nominal_spread387.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value397, _) => {
                                                source_ml_source2_data_property_value397
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field398 = match source_ml_source2_nominal_spread387.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value399) => {
                                                    source_ml_source2_data_property_value399.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override400 = (match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value386, _, _, _) => {
                                                        source_ml_source2_data_property_value386
                                                    }
                                                } + 1f64);
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field388, source_ml_source2_nominal_field390, source_ml_source2_nominal_override400, source_ml_source2_nominal_field394, source_ml_source2_nominal_field396, source_ml_source2_nominal_field398.clone())
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
                                        let source_ml_source2_nominal_spread410 = result.clone();
                                        {
                                            let source_ml_source2_nominal_field411 = match source_ml_source2_nominal_spread410.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value412, _) => {
                                                    let source_ml_source2_data_property_value412 = *source_ml_source2_data_property_value412;
                                                    source_ml_source2_data_property_value412.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field413 = match source_ml_source2_nominal_spread410.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value414) => {
                                                        source_ml_source2_data_property_value414
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override415 = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value408, _) => {
                                                            let source_ml_source2_data_property_value408 = *source_ml_source2_data_property_value408;
                                                            source_ml_source2_data_property_value408.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Constant(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value409) => {
                                                            source_ml_source2_data_property_value409
                                                        }
                                                    }, Box::new(value.clone())));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override415.clone()), source_ml_source2_nominal_field413)
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
                                    let source_ml_source2_nominal_spread418 = result_2.clone();
                                    {
                                        let source_ml_source2_nominal_field419 = match source_ml_source2_nominal_spread418.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value420, _) => {
                                                let source_ml_source2_data_property_value420 = *source_ml_source2_data_property_value420;
                                                source_ml_source2_data_property_value420.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field421 = match source_ml_source2_nominal_spread418.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value422) => {
                                                    source_ml_source2_data_property_value422
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override423 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value416, _) => {
                                                        let source_ml_source2_data_property_value416 = *source_ml_source2_data_property_value416;
                                                        source_ml_source2_data_property_value416.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Argument(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value417) => {
                                                        source_ml_source2_data_property_value417
                                                    }
                                                }, index));
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override423.clone()), source_ml_source2_nominal_field421)
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
                                let source_ml_source2_nominal_spread426 = value_2.clone();
                                {
                                    let source_ml_source2_nominal_field427 = match source_ml_source2_nominal_spread426.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value428, _) => {
                                            let source_ml_source2_data_property_value428 = *source_ml_source2_data_property_value428;
                                            source_ml_source2_data_property_value428.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field429 = match source_ml_source2_nominal_spread426.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value430) => {
                                                source_ml_source2_data_property_value430
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override431 = self::append_java_script_call_source_instruction(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value424, _) => {
                                                    let source_ml_source2_data_property_value424 = *source_ml_source2_data_property_value424;
                                                    source_ml_source2_data_property_value424.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Throw(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value425) => {
                                                    source_ml_source2_data_property_value425
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override431.clone()), source_ml_source2_nominal_field429)
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceTry(source_ml_source2_f401, source_ml_source2_f402, source_ml_source2_f403) => {
                        let source_ml_source2_f402 = *source_ml_source2_f402;
                        let source_ml_source2_f403 = *source_ml_source2_f403;
                        self::lower_java_script_call_source_exception(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceBinding(identity) => {
                        if !(self::test_java_script_call_code_coordinate(identity, 65536f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source binding identities require exact nonnegative coordinates")
                        } else {
                            {
                                let index_2 = ((bindings.len() as f64) - 1f64);
                                {
                                    let source_ml_source2_s432 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop99(source.clone(), bindings, state.clone(), index_2);
                                    match source_ml_source2_s432.clone() {
                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop99Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop99Done => {
                                            self::refuse_java_script_call_source(state.clone(), "a source binding reference has no actual lexical owner")
                                        }
                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop99Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop99Return(source_ml_source1_temporary_result99) => {
                                            let source_ml_source1_temporary_result99 = *source_ml_source1_temporary_result99;
                                            source_ml_source1_temporary_result99.clone()
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
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value434, _) => {
                                source_ml_source2_data_property_value434
                            }
                        }) == identity_2) }) }; ml_values1.iter().cloned().enumerate().any(|(ml_index1, ml_value1)| ml_callback1(ml_value1)) }) {
                            self::refuse_java_script_call_source(state.clone(), "a source binding owner must be unique in its lexical scope")
                        } else {
                            {
                                let value_3 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                self::lower_java_script_call_source_expression(body.clone(), {
                                    let source_ml_source2_concat_value436 = bindings;
                                    {
                                        let source_ml_source2_concat_value437 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity_2, match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value435) => {
                                                source_ml_source2_data_property_value435
                                            }
                                        })];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source2_concat_value436), source_ml_source2_concat_value437)
                                    }
                                }, match value_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value438, _) => {
                                        let source_ml_source2_data_property_value438 = *source_ml_source2_data_property_value438;
                                        source_ml_source2_data_property_value438.clone()
                                    }
                                }, (depth + 1f64))
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceIf(source_ml_source2_f404, source_ml_source2_f405, source_ml_source2_f406) => {
                        let source_ml_source2_f404 = *source_ml_source2_f404;
                        let source_ml_source2_f405 = *source_ml_source2_f405;
                        let source_ml_source2_f406 = *source_ml_source2_f406;
                        self::lower_java_script_call_source_branch(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f407, source_left, source_right) => {
                        let source_left = *source_left;
                        let source_right = *source_right;
                        {
                            let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings, match left.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value439, _) => {
                                        let source_ml_source2_data_property_value439 = *source_ml_source2_data_property_value439;
                                        source_ml_source2_data_property_value439.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_3 = self::allocate_java_script_call_source_register(match right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value440, _) => {
                                            let source_ml_source2_data_property_value440 = *source_ml_source2_data_property_value440;
                                            source_ml_source2_data_property_value440.clone()
                                        }
                                    });
                                    {
                                        let operator = source_ml_source2_f407.clone();
                                        {
                                            let source_ml_source2_f454 = source_left.clone();
                                            {
                                                let source_ml_source2_f455 = source_right.clone();
                                                if !(self::ml_array::includes(&vec![String::from("add"), String::from("subtract"), String::from("multiply"), String::from("divide"), String::from("remainder"), String::from("power"), String::from("lessThan"), String::from("equal")], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::refuse_java_script_call_source(match result_3.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value471, _) => {
                                                            let source_ml_source2_data_property_value471 = *source_ml_source2_data_property_value471;
                                                            source_ml_source2_data_property_value471.clone()
                                                        }
                                                    }, "source Number lowering requires its exact registered operator")
                                                } else {
                                                    {
                                                        let source_ml_source2_nominal_spread476 = result_3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field477 = match source_ml_source2_nominal_spread476.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value478, _) => {
                                                                    let source_ml_source2_data_property_value478 = *source_ml_source2_data_property_value478;
                                                                    source_ml_source2_data_property_value478.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field479 = match source_ml_source2_nominal_spread476.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value480) => {
                                                                        source_ml_source2_data_property_value480
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override481 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value472, _) => {
                                                                            let source_ml_source2_data_property_value472 = *source_ml_source2_data_property_value472;
                                                                            source_ml_source2_data_property_value472.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::NumberBinary(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value473) => {
                                                                            source_ml_source2_data_property_value473
                                                                        }
                                                                    }, match left.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value474) => {
                                                                            source_ml_source2_data_property_value474
                                                                        }
                                                                    }, match right.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value475) => {
                                                                            source_ml_source2_data_property_value475
                                                                        }
                                                                    }, operator.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override481.clone()), source_ml_source2_nominal_field479)
                                                                }
                                                            }
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value482, _) => {
                                        let source_ml_source2_data_property_value482 = *source_ml_source2_data_property_value482;
                                        source_ml_source2_data_property_value482.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value483, _) => {
                                            let source_ml_source2_data_property_value483 = *source_ml_source2_data_property_value483;
                                            source_ml_source2_data_property_value483.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f499 = source_left_2.clone();
                                        {
                                            let source_ml_source2_f500 = source_right_2.clone();
                                            {
                                                let source_ml_source2_nominal_spread518 = result_4.clone();
                                                {
                                                    let source_ml_source2_nominal_field519 = match source_ml_source2_nominal_spread518.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value520, _) => {
                                                            let source_ml_source2_data_property_value520 = *source_ml_source2_data_property_value520;
                                                            source_ml_source2_data_property_value520.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field521 = match source_ml_source2_nominal_spread518.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value522) => {
                                                                source_ml_source2_data_property_value522
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override523 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value514, _) => {
                                                                    let source_ml_source2_data_property_value514 = *source_ml_source2_data_property_value514;
                                                                    source_ml_source2_data_property_value514.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::SameValue(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value515) => {
                                                                    source_ml_source2_data_property_value515
                                                                }
                                                            }, match left_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value516) => {
                                                                    source_ml_source2_data_property_value516
                                                                }
                                                            }, match right_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value517) => {
                                                                    source_ml_source2_data_property_value517
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override523.clone()), source_ml_source2_nominal_field521)
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value524, _) => {
                                        let source_ml_source2_data_property_value524 = *source_ml_source2_data_property_value524;
                                        source_ml_source2_data_property_value524.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_5 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value525, _) => {
                                            let source_ml_source2_data_property_value525 = *source_ml_source2_data_property_value525;
                                            source_ml_source2_data_property_value525.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f543 = source_left_3.clone();
                                        {
                                            let source_ml_source2_f544 = source_right_3.clone();
                                            {
                                                let source_ml_source2_nominal_spread560 = result_5.clone();
                                                {
                                                    let source_ml_source2_nominal_field561 = match source_ml_source2_nominal_spread560.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value562, _) => {
                                                            let source_ml_source2_data_property_value562 = *source_ml_source2_data_property_value562;
                                                            source_ml_source2_data_property_value562.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field563 = match source_ml_source2_nominal_spread560.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value564) => {
                                                                source_ml_source2_data_property_value564
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override565 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value556, _) => {
                                                                    let source_ml_source2_data_property_value556 = *source_ml_source2_data_property_value556;
                                                                    source_ml_source2_data_property_value556.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::StrictEqual(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value557) => {
                                                                    source_ml_source2_data_property_value557
                                                                }
                                                            }, match left_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value558) => {
                                                                    source_ml_source2_data_property_value558
                                                                }
                                                            }, match right_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value559) => {
                                                                    source_ml_source2_data_property_value559
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override565.clone()), source_ml_source2_nominal_field563)
                                                        }
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
                                            let source_ml_source1_temporary_values96 = arguments;
                                            {
                                                let source_ml_source1_temporary_index96 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s566 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop100(bindings, depth, current.clone(), arguments_, source_ml_source1_temporary_values96, source_ml_source1_temporary_index96.clone());
                                                    match source_ml_source2_s566.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Done(current_2, arguments__2) => {
                                                            let current_2 = *current_2;
                                                            {
                                                                let result_6 = self::allocate_java_script_call_source_register(current_2.clone());
                                                                {
                                                                    let source_ml_source2_nominal_spread569 = result_6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field570 = match source_ml_source2_nominal_spread569.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value571, _) => {
                                                                                let source_ml_source2_data_property_value571 = *source_ml_source2_data_property_value571;
                                                                                source_ml_source2_data_property_value571.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field572 = match source_ml_source2_nominal_spread569.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value573) => {
                                                                                    source_ml_source2_data_property_value573
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override574 = self::append_java_script_call_source_instruction(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value567, _) => {
                                                                                        let source_ml_source2_data_property_value567 = *source_ml_source2_data_property_value567;
                                                                                        source_ml_source2_data_property_value567.clone()
                                                                                    }
                                                                                }, self::JavaScriptCallInstruction::Primitive(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value568) => {
                                                                                        source_ml_source2_data_property_value568
                                                                                    }
                                                                                }, operation.clone(), arguments__2));
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override574.clone()), source_ml_source2_nominal_field572)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value576, _) => {
                                source_ml_source2_data_property_value576
                            }
                        }) != (0f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source recurrence through an active exception handler requires explicit unwinding")
                        } else {
                            if (((arguments_2.len() as f64)) != (match state.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value578, _, _) => {
                                    source_ml_source2_data_property_value578
                                }
                            })) {
                                self::refuse_java_script_call_source(state.clone(), "loop updates require every actual parameter operand")
                            } else {
                                {
                                    let current_3 = state.clone();
                                    {
                                        let arguments__3 = Vec::<f64>::new();
                                        {
                                            let source_ml_source1_temporary_values97 = arguments_2;
                                            {
                                                let source_ml_source1_temporary_index97 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s579 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop101(bindings, depth, current_3.clone(), arguments__3, source_ml_source1_temporary_values97, source_ml_source1_temporary_index97.clone());
                                                    match source_ml_source2_s579.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Done(current_4, arguments__4) => {
                                                            let current_4 = *current_4;
                                                            {
                                                                let index_3 = 0f64;
                                                                {
                                                                    let current_5 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop102(current_4.clone(), &(arguments__4), index_3);
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value580, _) => {
                                        let source_ml_source2_data_property_value580 = *source_ml_source2_data_property_value580;
                                        source_ml_source2_data_property_value580.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let value_4 = self::lower_java_script_call_source_expression(source_value_3.clone(), bindings, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value581, _) => {
                                            let source_ml_source2_data_property_value581 = *source_ml_source2_data_property_value581;
                                            source_ml_source2_data_property_value581.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let result_7 = self::allocate_java_script_call_source_register(match value_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value582, _) => {
                                                let source_ml_source2_data_property_value582 = *source_ml_source2_data_property_value582;
                                                source_ml_source2_data_property_value582.clone()
                                            }
                                        });
                                        {
                                            let write = self::append_java_script_call_source_instruction(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value583, _) => {
                                                    let source_ml_source2_data_property_value583 = *source_ml_source2_data_property_value583;
                                                    source_ml_source2_data_property_value583.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinarySet(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value584) => {
                                                    source_ml_source2_data_property_value584
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value585) => {
                                                    source_ml_source2_data_property_value585
                                                }
                                            }, match key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value586) => {
                                                    source_ml_source2_data_property_value586
                                                }
                                            }, match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value587) => {
                                                    source_ml_source2_data_property_value587
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value588) => {
                                                    source_ml_source2_data_property_value588
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value589) => {
                                                    source_ml_source2_data_property_value589
                                                }
                                            }))), match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value590) => {
                                                    source_ml_source2_data_property_value590
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value591, _) => {
                                        let source_ml_source2_data_property_value591 = *source_ml_source2_data_property_value591;
                                        source_ml_source2_data_property_value591.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_8 = self::allocate_java_script_call_source_register(match key_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value592, _) => {
                                            let source_ml_source2_data_property_value592 = *source_ml_source2_data_property_value592;
                                            source_ml_source2_data_property_value592.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_nominal_spread598 = result_8.clone();
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
                                                    let source_ml_source2_nominal_override603 = self::append_java_script_call_source_instruction(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value593, _) => {
                                                            let source_ml_source2_data_property_value593 = *source_ml_source2_data_property_value593;
                                                            source_ml_source2_data_property_value593.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryGet(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value594) => {
                                                            source_ml_source2_data_property_value594
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value595) => {
                                                            source_ml_source2_data_property_value595
                                                        }
                                                    }, match key_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value596) => {
                                                            source_ml_source2_data_property_value596
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value597) => {
                                                            source_ml_source2_data_property_value597
                                                        }
                                                    }));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override603.clone()), source_ml_source2_nominal_field601)
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
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value604, _) => {
                                            let source_ml_source2_data_property_value604 = *source_ml_source2_data_property_value604;
                                            source_ml_source2_data_property_value604.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let current_6 = match receiver.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value605, _) => {
                                                let source_ml_source2_data_property_value605 = *source_ml_source2_data_property_value605;
                                                source_ml_source2_data_property_value605.clone()
                                            }
                                        };
                                        {
                                            let arguments__5 = Vec::<f64>::new();
                                            {
                                                let source_ml_source1_temporary_values98 = arguments_3;
                                                {
                                                    let source_ml_source1_temporary_index98 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s606 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop103(bindings, depth, current_6.clone(), arguments__5, source_ml_source1_temporary_values98, source_ml_source1_temporary_index98.clone());
                                                        match source_ml_source2_s606.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Done(current_7, arguments__6) => {
                                                                let current_7 = *current_7;
                                                                {
                                                                    let result_9 = self::allocate_java_script_call_source_register(current_7.clone());
                                                                    {
                                                                        let source_ml_source2_nominal_spread611 = result_9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field612 = match source_ml_source2_nominal_spread611.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value613, _) => {
                                                                                    let source_ml_source2_data_property_value613 = *source_ml_source2_data_property_value613;
                                                                                    source_ml_source2_data_property_value613.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field614 = match source_ml_source2_nominal_spread611.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value615) => {
                                                                                        source_ml_source2_data_property_value615
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override616 = self::append_java_script_call_source_instruction(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value607, _) => {
                                                                                            let source_ml_source2_data_property_value607 = *source_ml_source2_data_property_value607;
                                                                                            source_ml_source2_data_property_value607.clone()
                                                                                        }
                                                                                    }, self::JavaScriptCallInstruction::Call(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value608) => {
                                                                                            source_ml_source2_data_property_value608
                                                                                        }
                                                                                    }, match callee.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value609) => {
                                                                                            source_ml_source2_data_property_value609
                                                                                        }
                                                                                    }, match receiver.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value610) => {
                                                                                            source_ml_source2_data_property_value610
                                                                                        }
                                                                                    }, arguments__6));
                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override616.clone()), source_ml_source2_nominal_field614)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
