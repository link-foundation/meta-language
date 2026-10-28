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
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_branch;
use super::lower_java_script_call_source_exception;
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
pub fn allocate_java_script_call_source_register(builder: self::JavaScriptCallSourceBuilder) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value316) => {
            source_ml_source2_data_property_value316.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value318, _, _, _, _) => {
                source_ml_source2_data_property_value318
            }
        }) >= (4096f64)) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its register budget")
        } else {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                let source_ml_source2_nominal_spread320 = builder.clone();
                {
                    let source_ml_source2_nominal_field321 = match source_ml_source2_nominal_spread320.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value322, _, _, _, _, _) => {
                            source_ml_source2_data_property_value322
                        }
                    };
                    {
                        let source_ml_source2_nominal_field323 = match source_ml_source2_nominal_spread320.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value324, _, _, _, _) => {
                                source_ml_source2_data_property_value324
                            }
                        };
                        {
                            let source_ml_source2_nominal_field325 = match source_ml_source2_nominal_spread320.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value326, _, _, _) => {
                                    source_ml_source2_data_property_value326
                                }
                            };
                            {
                                let source_ml_source2_nominal_field327 = match source_ml_source2_nominal_spread320.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value328, _, _) => {
                                        source_ml_source2_data_property_value328
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field329 = match source_ml_source2_nominal_spread320.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value330, _) => {
                                            source_ml_source2_data_property_value330
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field331 = match source_ml_source2_nominal_spread320.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value332) => {
                                                source_ml_source2_data_property_value332.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override333 = (match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value319, _, _, _, _) => {
                                                    source_ml_source2_data_property_value319
                                                }
                                            } + 1f64);
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field321, source_ml_source2_nominal_override333, source_ml_source2_nominal_field325, source_ml_source2_nominal_field327, source_ml_source2_nominal_field329, source_ml_source2_nominal_field331.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }), match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value334, _, _, _, _) => {
                    source_ml_source2_data_property_value334
                }
            })
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_expression(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value336) => {
            source_ml_source2_data_property_value336.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((depth > (128f64)) || ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value338, _, _, _) => {
                source_ml_source2_data_property_value338
            }
        }) >= (65536f64))) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its structural budget")
        } else {
            {
                let state = {
                    let source_ml_source2_nominal_spread340 = builder.clone();
                    {
                        let source_ml_source2_nominal_field341 = match source_ml_source2_nominal_spread340.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value342, _, _, _, _, _) => {
                                source_ml_source2_data_property_value342
                            }
                        };
                        {
                            let source_ml_source2_nominal_field343 = match source_ml_source2_nominal_spread340.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value344, _, _, _, _) => {
                                    source_ml_source2_data_property_value344
                                }
                            };
                            {
                                let source_ml_source2_nominal_field345 = match source_ml_source2_nominal_spread340.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value346, _, _, _) => {
                                        source_ml_source2_data_property_value346
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field347 = match source_ml_source2_nominal_spread340.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value348, _, _) => {
                                            source_ml_source2_data_property_value348
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field349 = match source_ml_source2_nominal_spread340.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value350, _) => {
                                                source_ml_source2_data_property_value350
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field351 = match source_ml_source2_nominal_spread340.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value352) => {
                                                    source_ml_source2_data_property_value352.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override353 = (match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value339, _, _, _) => {
                                                        source_ml_source2_data_property_value339
                                                    }
                                                } + 1f64);
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field341, source_ml_source2_nominal_field343, source_ml_source2_nominal_override353, source_ml_source2_nominal_field347, source_ml_source2_nominal_field349, source_ml_source2_nominal_field351.clone())
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
                                        let source_ml_source2_nominal_spread363 = result.clone();
                                        {
                                            let source_ml_source2_nominal_field364 = match source_ml_source2_nominal_spread363.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value365, _) => {
                                                    let source_ml_source2_data_property_value365 = *source_ml_source2_data_property_value365;
                                                    source_ml_source2_data_property_value365.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field366 = match source_ml_source2_nominal_spread363.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value367) => {
                                                        source_ml_source2_data_property_value367
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override368 = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value361, _) => {
                                                            let source_ml_source2_data_property_value361 = *source_ml_source2_data_property_value361;
                                                            source_ml_source2_data_property_value361.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Constant(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value362) => {
                                                            source_ml_source2_data_property_value362
                                                        }
                                                    }, Box::new(value.clone())));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override368.clone()), source_ml_source2_nominal_field366)
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
                                    let source_ml_source2_nominal_spread371 = result_2.clone();
                                    {
                                        let source_ml_source2_nominal_field372 = match source_ml_source2_nominal_spread371.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value373, _) => {
                                                let source_ml_source2_data_property_value373 = *source_ml_source2_data_property_value373;
                                                source_ml_source2_data_property_value373.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field374 = match source_ml_source2_nominal_spread371.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value375) => {
                                                    source_ml_source2_data_property_value375
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override376 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value369, _) => {
                                                        let source_ml_source2_data_property_value369 = *source_ml_source2_data_property_value369;
                                                        source_ml_source2_data_property_value369.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Argument(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value370) => {
                                                        source_ml_source2_data_property_value370
                                                    }
                                                }, index));
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override376.clone()), source_ml_source2_nominal_field374)
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
                                let source_ml_source2_nominal_spread379 = value_2.clone();
                                {
                                    let source_ml_source2_nominal_field380 = match source_ml_source2_nominal_spread379.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value381, _) => {
                                            let source_ml_source2_data_property_value381 = *source_ml_source2_data_property_value381;
                                            source_ml_source2_data_property_value381.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field382 = match source_ml_source2_nominal_spread379.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value383) => {
                                                source_ml_source2_data_property_value383
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override384 = self::append_java_script_call_source_instruction(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value377, _) => {
                                                    let source_ml_source2_data_property_value377 = *source_ml_source2_data_property_value377;
                                                    source_ml_source2_data_property_value377.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Throw(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value378) => {
                                                    source_ml_source2_data_property_value378
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override384.clone()), source_ml_source2_nominal_field382)
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceTry(source_ml_source2_f354, source_ml_source2_f355, source_ml_source2_f356) => {
                        let source_ml_source2_f355 = *source_ml_source2_f355;
                        let source_ml_source2_f356 = *source_ml_source2_f356;
                        self::lower_java_script_call_source_exception(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceBinding(identity) => {
                        if !(self::test_java_script_call_code_coordinate(identity, 65536f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source binding identities require exact nonnegative coordinates")
                        } else {
                            {
                                let index_2 = ((bindings.len() as f64) - 1f64);
                                {
                                    let source_ml_source2_s385 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop97(source.clone(), bindings, state.clone(), index_2);
                                    match source_ml_source2_s385.clone() {
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
                    self::JavaScriptCallSource::SourceLet(identity_2, source_value_2, body) => {
                        let source_value_2 = *source_value_2;
                        let body = *body;
                        if (!(self::test_java_script_call_code_coordinate(identity_2, 65536f64)) || { let ml_values1 = bindings.clone(); let ml_callback1 = { let identity_2 = identity_2; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value387, _) => {
                                source_ml_source2_data_property_value387
                            }
                        }) == identity_2) }) }; ml_values1.iter().cloned().enumerate().any(|(ml_index1, ml_value1)| ml_callback1(ml_value1)) }) {
                            self::refuse_java_script_call_source(state.clone(), "a source binding owner must be unique in its lexical scope")
                        } else {
                            {
                                let value_3 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                self::lower_java_script_call_source_expression(body.clone(), {
                                    let source_ml_source2_concat_value389 = bindings;
                                    {
                                        let source_ml_source2_concat_value390 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity_2, match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value388) => {
                                                source_ml_source2_data_property_value388
                                            }
                                        })];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source2_concat_value389), source_ml_source2_concat_value390)
                                    }
                                }, match value_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value391, _) => {
                                        let source_ml_source2_data_property_value391 = *source_ml_source2_data_property_value391;
                                        source_ml_source2_data_property_value391.clone()
                                    }
                                }, (depth + 1f64))
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceIf(source_ml_source2_f357, source_ml_source2_f358, source_ml_source2_f359) => {
                        let source_ml_source2_f357 = *source_ml_source2_f357;
                        let source_ml_source2_f358 = *source_ml_source2_f358;
                        let source_ml_source2_f359 = *source_ml_source2_f359;
                        self::lower_java_script_call_source_branch(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f360, source_left, source_right) => {
                        let source_left = *source_left;
                        let source_right = *source_right;
                        {
                            let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings, match left.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value392, _) => {
                                        let source_ml_source2_data_property_value392 = *source_ml_source2_data_property_value392;
                                        source_ml_source2_data_property_value392.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_3 = self::allocate_java_script_call_source_register(match right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value393, _) => {
                                            let source_ml_source2_data_property_value393 = *source_ml_source2_data_property_value393;
                                            source_ml_source2_data_property_value393.clone()
                                        }
                                    });
                                    {
                                        let operator = source_ml_source2_f360.clone();
                                        {
                                            let source_ml_source2_f407 = source_left.clone();
                                            {
                                                let source_ml_source2_f408 = source_right.clone();
                                                if !(self::ml_array::includes(&vec![String::from("add"), String::from("subtract"), String::from("multiply"), String::from("divide"), String::from("remainder"), String::from("power"), String::from("lessThan"), String::from("equal")], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::refuse_java_script_call_source(match result_3.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value424, _) => {
                                                            let source_ml_source2_data_property_value424 = *source_ml_source2_data_property_value424;
                                                            source_ml_source2_data_property_value424.clone()
                                                        }
                                                    }, "source Number lowering requires its exact registered operator")
                                                } else {
                                                    {
                                                        let source_ml_source2_nominal_spread429 = result_3.clone();
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
                                                                    let source_ml_source2_nominal_override434 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value425, _) => {
                                                                            let source_ml_source2_data_property_value425 = *source_ml_source2_data_property_value425;
                                                                            source_ml_source2_data_property_value425.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::NumberBinary(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value426) => {
                                                                            source_ml_source2_data_property_value426
                                                                        }
                                                                    }, match left.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value427) => {
                                                                            source_ml_source2_data_property_value427
                                                                        }
                                                                    }, match right.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value428) => {
                                                                            source_ml_source2_data_property_value428
                                                                        }
                                                                    }, operator.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override434.clone()), source_ml_source2_nominal_field432)
                                                                }
                                                            }
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value435, _) => {
                                        let source_ml_source2_data_property_value435 = *source_ml_source2_data_property_value435;
                                        source_ml_source2_data_property_value435.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value436, _) => {
                                            let source_ml_source2_data_property_value436 = *source_ml_source2_data_property_value436;
                                            source_ml_source2_data_property_value436.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f452 = source_left_2.clone();
                                        {
                                            let source_ml_source2_f453 = source_right_2.clone();
                                            {
                                                let source_ml_source2_nominal_spread471 = result_4.clone();
                                                {
                                                    let source_ml_source2_nominal_field472 = match source_ml_source2_nominal_spread471.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value473, _) => {
                                                            let source_ml_source2_data_property_value473 = *source_ml_source2_data_property_value473;
                                                            source_ml_source2_data_property_value473.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field474 = match source_ml_source2_nominal_spread471.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value475) => {
                                                                source_ml_source2_data_property_value475
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override476 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value467, _) => {
                                                                    let source_ml_source2_data_property_value467 = *source_ml_source2_data_property_value467;
                                                                    source_ml_source2_data_property_value467.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::SameValue(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value468) => {
                                                                    source_ml_source2_data_property_value468
                                                                }
                                                            }, match left_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value469) => {
                                                                    source_ml_source2_data_property_value469
                                                                }
                                                            }, match right_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value470) => {
                                                                    source_ml_source2_data_property_value470
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override476.clone()), source_ml_source2_nominal_field474)
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value477, _) => {
                                        let source_ml_source2_data_property_value477 = *source_ml_source2_data_property_value477;
                                        source_ml_source2_data_property_value477.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_5 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value478, _) => {
                                            let source_ml_source2_data_property_value478 = *source_ml_source2_data_property_value478;
                                            source_ml_source2_data_property_value478.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f496 = source_left_3.clone();
                                        {
                                            let source_ml_source2_f497 = source_right_3.clone();
                                            {
                                                let source_ml_source2_nominal_spread513 = result_5.clone();
                                                {
                                                    let source_ml_source2_nominal_field514 = match source_ml_source2_nominal_spread513.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value515, _) => {
                                                            let source_ml_source2_data_property_value515 = *source_ml_source2_data_property_value515;
                                                            source_ml_source2_data_property_value515.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field516 = match source_ml_source2_nominal_spread513.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value517) => {
                                                                source_ml_source2_data_property_value517
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override518 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value509, _) => {
                                                                    let source_ml_source2_data_property_value509 = *source_ml_source2_data_property_value509;
                                                                    source_ml_source2_data_property_value509.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::StrictEqual(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value510) => {
                                                                    source_ml_source2_data_property_value510
                                                                }
                                                            }, match left_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value511) => {
                                                                    source_ml_source2_data_property_value511
                                                                }
                                                            }, match right_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value512) => {
                                                                    source_ml_source2_data_property_value512
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override518.clone()), source_ml_source2_nominal_field516)
                                                        }
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
                                                    let source_ml_source2_s519 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop98(bindings, depth, current.clone(), arguments_, source_ml_source1_temporary_values94, source_ml_source1_temporary_index94.clone());
                                                    match source_ml_source2_s519.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Done(current_2, arguments__2) => {
                                                            let current_2 = *current_2;
                                                            {
                                                                let result_6 = self::allocate_java_script_call_source_register(current_2.clone());
                                                                {
                                                                    let source_ml_source2_nominal_spread522 = result_6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field523 = match source_ml_source2_nominal_spread522.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value524, _) => {
                                                                                let source_ml_source2_data_property_value524 = *source_ml_source2_data_property_value524;
                                                                                source_ml_source2_data_property_value524.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field525 = match source_ml_source2_nominal_spread522.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value526) => {
                                                                                    source_ml_source2_data_property_value526
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override527 = self::append_java_script_call_source_instruction(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value520, _) => {
                                                                                        let source_ml_source2_data_property_value520 = *source_ml_source2_data_property_value520;
                                                                                        source_ml_source2_data_property_value520.clone()
                                                                                    }
                                                                                }, self::JavaScriptCallInstruction::Primitive(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value521) => {
                                                                                        source_ml_source2_data_property_value521
                                                                                    }
                                                                                }, operation.clone(), arguments__2));
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override527.clone()), source_ml_source2_nominal_field525)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value529, _) => {
                                source_ml_source2_data_property_value529
                            }
                        }) != (0f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source recurrence through an active exception handler requires explicit unwinding")
                        } else {
                            if (((arguments_2.len() as f64)) != (match state.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value531, _, _) => {
                                    source_ml_source2_data_property_value531
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
                                                    let source_ml_source2_s532 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop99(bindings, depth, current_3.clone(), arguments__3, source_ml_source1_temporary_values95, source_ml_source1_temporary_index95.clone());
                                                    match source_ml_source2_s532.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop99Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop99Done(current_4, arguments__4) => {
                                                            let current_4 = *current_4;
                                                            {
                                                                let index_3 = 0f64;
                                                                {
                                                                    let current_5 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop100(current_4.clone(), &(arguments__4), index_3);
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value533, _) => {
                                        let source_ml_source2_data_property_value533 = *source_ml_source2_data_property_value533;
                                        source_ml_source2_data_property_value533.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let value_4 = self::lower_java_script_call_source_expression(source_value_3.clone(), bindings, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value534, _) => {
                                            let source_ml_source2_data_property_value534 = *source_ml_source2_data_property_value534;
                                            source_ml_source2_data_property_value534.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let result_7 = self::allocate_java_script_call_source_register(match value_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value535, _) => {
                                                let source_ml_source2_data_property_value535 = *source_ml_source2_data_property_value535;
                                                source_ml_source2_data_property_value535.clone()
                                            }
                                        });
                                        {
                                            let write = self::append_java_script_call_source_instruction(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value536, _) => {
                                                    let source_ml_source2_data_property_value536 = *source_ml_source2_data_property_value536;
                                                    source_ml_source2_data_property_value536.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinarySet(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value537) => {
                                                    source_ml_source2_data_property_value537
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value538) => {
                                                    source_ml_source2_data_property_value538
                                                }
                                            }, match key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value539) => {
                                                    source_ml_source2_data_property_value539
                                                }
                                            }, match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value540) => {
                                                    source_ml_source2_data_property_value540
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value541) => {
                                                    source_ml_source2_data_property_value541
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value542) => {
                                                    source_ml_source2_data_property_value542
                                                }
                                            }))), match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value543) => {
                                                    source_ml_source2_data_property_value543
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value544, _) => {
                                        let source_ml_source2_data_property_value544 = *source_ml_source2_data_property_value544;
                                        source_ml_source2_data_property_value544.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_8 = self::allocate_java_script_call_source_register(match key_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value545, _) => {
                                            let source_ml_source2_data_property_value545 = *source_ml_source2_data_property_value545;
                                            source_ml_source2_data_property_value545.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_nominal_spread551 = result_8.clone();
                                        {
                                            let source_ml_source2_nominal_field552 = match source_ml_source2_nominal_spread551.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value553, _) => {
                                                    let source_ml_source2_data_property_value553 = *source_ml_source2_data_property_value553;
                                                    source_ml_source2_data_property_value553.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field554 = match source_ml_source2_nominal_spread551.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value555) => {
                                                        source_ml_source2_data_property_value555
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override556 = self::append_java_script_call_source_instruction(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value546, _) => {
                                                            let source_ml_source2_data_property_value546 = *source_ml_source2_data_property_value546;
                                                            source_ml_source2_data_property_value546.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryGet(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value547) => {
                                                            source_ml_source2_data_property_value547
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value548) => {
                                                            source_ml_source2_data_property_value548
                                                        }
                                                    }, match key_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value549) => {
                                                            source_ml_source2_data_property_value549
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value550) => {
                                                            source_ml_source2_data_property_value550
                                                        }
                                                    }));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override556.clone()), source_ml_source2_nominal_field554)
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
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value557, _) => {
                                            let source_ml_source2_data_property_value557 = *source_ml_source2_data_property_value557;
                                            source_ml_source2_data_property_value557.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let current_6 = match receiver.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value558, _) => {
                                                let source_ml_source2_data_property_value558 = *source_ml_source2_data_property_value558;
                                                source_ml_source2_data_property_value558.clone()
                                            }
                                        };
                                        {
                                            let arguments__5 = Vec::<f64>::new();
                                            {
                                                let source_ml_source1_temporary_values96 = arguments_3;
                                                {
                                                    let source_ml_source1_temporary_index96 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s559 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop101(bindings, depth, current_6.clone(), arguments__5, source_ml_source1_temporary_values96, source_ml_source1_temporary_index96.clone());
                                                        match source_ml_source2_s559.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Done(current_7, arguments__6) => {
                                                                let current_7 = *current_7;
                                                                {
                                                                    let result_9 = self::allocate_java_script_call_source_register(current_7.clone());
                                                                    {
                                                                        let source_ml_source2_nominal_spread564 = result_9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field565 = match source_ml_source2_nominal_spread564.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value566, _) => {
                                                                                    let source_ml_source2_data_property_value566 = *source_ml_source2_data_property_value566;
                                                                                    source_ml_source2_data_property_value566.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field567 = match source_ml_source2_nominal_spread564.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value568) => {
                                                                                        source_ml_source2_data_property_value568
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override569 = self::append_java_script_call_source_instruction(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value560, _) => {
                                                                                            let source_ml_source2_data_property_value560 = *source_ml_source2_data_property_value560;
                                                                                            source_ml_source2_data_property_value560.clone()
                                                                                        }
                                                                                    }, self::JavaScriptCallInstruction::Call(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value561) => {
                                                                                            source_ml_source2_data_property_value561
                                                                                        }
                                                                                    }, match callee.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value562) => {
                                                                                            source_ml_source2_data_property_value562
                                                                                        }
                                                                                    }, match receiver.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value563) => {
                                                                                            source_ml_source2_data_property_value563
                                                                                        }
                                                                                    }, arguments__6));
                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override569.clone()), source_ml_source2_nominal_field567)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
