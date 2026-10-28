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
pub fn allocate_java_script_call_source_register(builder: self::JavaScriptCallSourceBuilder) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value329) => {
            source_ml_source2_data_property_value329.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value331, _, _, _, _) => {
                source_ml_source2_data_property_value331
            }
        }) >= (4096f64)) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its register budget")
        } else {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                let source_ml_source2_nominal_spread333 = builder.clone();
                {
                    let source_ml_source2_nominal_field334 = match source_ml_source2_nominal_spread333.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value335, _, _, _, _, _) => {
                            source_ml_source2_data_property_value335
                        }
                    };
                    {
                        let source_ml_source2_nominal_field336 = match source_ml_source2_nominal_spread333.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value337, _, _, _, _) => {
                                source_ml_source2_data_property_value337
                            }
                        };
                        {
                            let source_ml_source2_nominal_field338 = match source_ml_source2_nominal_spread333.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value339, _, _, _) => {
                                    source_ml_source2_data_property_value339
                                }
                            };
                            {
                                let source_ml_source2_nominal_field340 = match source_ml_source2_nominal_spread333.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value341, _, _) => {
                                        source_ml_source2_data_property_value341
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field342 = match source_ml_source2_nominal_spread333.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value343, _) => {
                                            source_ml_source2_data_property_value343
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field344 = match source_ml_source2_nominal_spread333.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value345) => {
                                                source_ml_source2_data_property_value345.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override346 = (match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value332, _, _, _, _) => {
                                                    source_ml_source2_data_property_value332
                                                }
                                            } + 1f64);
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field334, source_ml_source2_nominal_override346, source_ml_source2_nominal_field338, source_ml_source2_nominal_field340, source_ml_source2_nominal_field342, source_ml_source2_nominal_field344.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }), match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value347, _, _, _, _) => {
                    source_ml_source2_data_property_value347
                }
            })
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_expression(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value349) => {
            source_ml_source2_data_property_value349.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((depth > (128f64)) || ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value351, _, _, _) => {
                source_ml_source2_data_property_value351
            }
        }) >= (65536f64))) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its structural budget")
        } else {
            {
                let state = {
                    let source_ml_source2_nominal_spread353 = builder.clone();
                    {
                        let source_ml_source2_nominal_field354 = match source_ml_source2_nominal_spread353.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value355, _, _, _, _, _) => {
                                source_ml_source2_data_property_value355
                            }
                        };
                        {
                            let source_ml_source2_nominal_field356 = match source_ml_source2_nominal_spread353.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value357, _, _, _, _) => {
                                    source_ml_source2_data_property_value357
                                }
                            };
                            {
                                let source_ml_source2_nominal_field358 = match source_ml_source2_nominal_spread353.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value359, _, _, _) => {
                                        source_ml_source2_data_property_value359
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field360 = match source_ml_source2_nominal_spread353.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value361, _, _) => {
                                            source_ml_source2_data_property_value361
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field362 = match source_ml_source2_nominal_spread353.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value363, _) => {
                                                source_ml_source2_data_property_value363
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field364 = match source_ml_source2_nominal_spread353.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value365) => {
                                                    source_ml_source2_data_property_value365.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override366 = (match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value352, _, _, _) => {
                                                        source_ml_source2_data_property_value352
                                                    }
                                                } + 1f64);
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field354, source_ml_source2_nominal_field356, source_ml_source2_nominal_override366, source_ml_source2_nominal_field360, source_ml_source2_nominal_field362, source_ml_source2_nominal_field364.clone())
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
                                        let source_ml_source2_nominal_spread376 = result.clone();
                                        {
                                            let source_ml_source2_nominal_field377 = match source_ml_source2_nominal_spread376.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value378, _) => {
                                                    let source_ml_source2_data_property_value378 = *source_ml_source2_data_property_value378;
                                                    source_ml_source2_data_property_value378.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field379 = match source_ml_source2_nominal_spread376.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value380) => {
                                                        source_ml_source2_data_property_value380
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override381 = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value374, _) => {
                                                            let source_ml_source2_data_property_value374 = *source_ml_source2_data_property_value374;
                                                            source_ml_source2_data_property_value374.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Constant(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value375) => {
                                                            source_ml_source2_data_property_value375
                                                        }
                                                    }, Box::new(value.clone())));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override381.clone()), source_ml_source2_nominal_field379)
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
                                    let source_ml_source2_nominal_spread384 = result_2.clone();
                                    {
                                        let source_ml_source2_nominal_field385 = match source_ml_source2_nominal_spread384.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value386, _) => {
                                                let source_ml_source2_data_property_value386 = *source_ml_source2_data_property_value386;
                                                source_ml_source2_data_property_value386.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field387 = match source_ml_source2_nominal_spread384.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value388) => {
                                                    source_ml_source2_data_property_value388
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override389 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value382, _) => {
                                                        let source_ml_source2_data_property_value382 = *source_ml_source2_data_property_value382;
                                                        source_ml_source2_data_property_value382.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Argument(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value383) => {
                                                        source_ml_source2_data_property_value383
                                                    }
                                                }, index));
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override389.clone()), source_ml_source2_nominal_field387)
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
                                let source_ml_source2_nominal_spread392 = value_2.clone();
                                {
                                    let source_ml_source2_nominal_field393 = match source_ml_source2_nominal_spread392.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value394, _) => {
                                            let source_ml_source2_data_property_value394 = *source_ml_source2_data_property_value394;
                                            source_ml_source2_data_property_value394.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field395 = match source_ml_source2_nominal_spread392.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value396) => {
                                                source_ml_source2_data_property_value396
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override397 = self::append_java_script_call_source_instruction(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value390, _) => {
                                                    let source_ml_source2_data_property_value390 = *source_ml_source2_data_property_value390;
                                                    source_ml_source2_data_property_value390.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Throw(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value391) => {
                                                    source_ml_source2_data_property_value391
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override397.clone()), source_ml_source2_nominal_field395)
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceTry(source_ml_source2_f367, source_ml_source2_f368, source_ml_source2_f369) => {
                        let source_ml_source2_f368 = *source_ml_source2_f368;
                        let source_ml_source2_f369 = *source_ml_source2_f369;
                        self::lower_java_script_call_source_exception(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceBinding(identity) => {
                        if !(self::test_java_script_call_code_coordinate(identity, 65536f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source binding identities require exact nonnegative coordinates")
                        } else {
                            {
                                let index_2 = ((bindings.len() as f64) - 1f64);
                                {
                                    let source_ml_source2_s398 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop99(source.clone(), bindings, state.clone(), index_2);
                                    match source_ml_source2_s398.clone() {
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
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value400, _) => {
                                source_ml_source2_data_property_value400
                            }
                        }) == identity_2) }) }; ml_values1.iter().cloned().enumerate().any(|(ml_index1, ml_value1)| ml_callback1(ml_value1)) }) {
                            self::refuse_java_script_call_source(state.clone(), "a source binding owner must be unique in its lexical scope")
                        } else {
                            {
                                let value_3 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                self::lower_java_script_call_source_expression(body.clone(), {
                                    let source_ml_source2_concat_value402 = bindings;
                                    {
                                        let source_ml_source2_concat_value403 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity_2, match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value401) => {
                                                source_ml_source2_data_property_value401
                                            }
                                        })];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source2_concat_value402), source_ml_source2_concat_value403)
                                    }
                                }, match value_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value404, _) => {
                                        let source_ml_source2_data_property_value404 = *source_ml_source2_data_property_value404;
                                        source_ml_source2_data_property_value404.clone()
                                    }
                                }, (depth + 1f64))
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceIf(source_ml_source2_f370, source_ml_source2_f371, source_ml_source2_f372) => {
                        let source_ml_source2_f370 = *source_ml_source2_f370;
                        let source_ml_source2_f371 = *source_ml_source2_f371;
                        let source_ml_source2_f372 = *source_ml_source2_f372;
                        self::lower_java_script_call_source_branch(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f373, source_left, source_right) => {
                        let source_left = *source_left;
                        let source_right = *source_right;
                        {
                            let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings, match left.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value405, _) => {
                                        let source_ml_source2_data_property_value405 = *source_ml_source2_data_property_value405;
                                        source_ml_source2_data_property_value405.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_3 = self::allocate_java_script_call_source_register(match right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value406, _) => {
                                            let source_ml_source2_data_property_value406 = *source_ml_source2_data_property_value406;
                                            source_ml_source2_data_property_value406.clone()
                                        }
                                    });
                                    {
                                        let operator = source_ml_source2_f373.clone();
                                        {
                                            let source_ml_source2_f420 = source_left.clone();
                                            {
                                                let source_ml_source2_f421 = source_right.clone();
                                                if !(self::ml_array::includes(&vec![String::from("add"), String::from("subtract"), String::from("multiply"), String::from("divide"), String::from("remainder"), String::from("power"), String::from("lessThan"), String::from("equal")], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::refuse_java_script_call_source(match result_3.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value437, _) => {
                                                            let source_ml_source2_data_property_value437 = *source_ml_source2_data_property_value437;
                                                            source_ml_source2_data_property_value437.clone()
                                                        }
                                                    }, "source Number lowering requires its exact registered operator")
                                                } else {
                                                    {
                                                        let source_ml_source2_nominal_spread442 = result_3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field443 = match source_ml_source2_nominal_spread442.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value444, _) => {
                                                                    let source_ml_source2_data_property_value444 = *source_ml_source2_data_property_value444;
                                                                    source_ml_source2_data_property_value444.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field445 = match source_ml_source2_nominal_spread442.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value446) => {
                                                                        source_ml_source2_data_property_value446
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override447 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value438, _) => {
                                                                            let source_ml_source2_data_property_value438 = *source_ml_source2_data_property_value438;
                                                                            source_ml_source2_data_property_value438.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::NumberBinary(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value439) => {
                                                                            source_ml_source2_data_property_value439
                                                                        }
                                                                    }, match left.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value440) => {
                                                                            source_ml_source2_data_property_value440
                                                                        }
                                                                    }, match right.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value441) => {
                                                                            source_ml_source2_data_property_value441
                                                                        }
                                                                    }, operator.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override447.clone()), source_ml_source2_nominal_field445)
                                                                }
                                                            }
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value448, _) => {
                                        let source_ml_source2_data_property_value448 = *source_ml_source2_data_property_value448;
                                        source_ml_source2_data_property_value448.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value449, _) => {
                                            let source_ml_source2_data_property_value449 = *source_ml_source2_data_property_value449;
                                            source_ml_source2_data_property_value449.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f465 = source_left_2.clone();
                                        {
                                            let source_ml_source2_f466 = source_right_2.clone();
                                            {
                                                let source_ml_source2_nominal_spread484 = result_4.clone();
                                                {
                                                    let source_ml_source2_nominal_field485 = match source_ml_source2_nominal_spread484.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value486, _) => {
                                                            let source_ml_source2_data_property_value486 = *source_ml_source2_data_property_value486;
                                                            source_ml_source2_data_property_value486.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field487 = match source_ml_source2_nominal_spread484.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value488) => {
                                                                source_ml_source2_data_property_value488
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override489 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value480, _) => {
                                                                    let source_ml_source2_data_property_value480 = *source_ml_source2_data_property_value480;
                                                                    source_ml_source2_data_property_value480.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::SameValue(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value481) => {
                                                                    source_ml_source2_data_property_value481
                                                                }
                                                            }, match left_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value482) => {
                                                                    source_ml_source2_data_property_value482
                                                                }
                                                            }, match right_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value483) => {
                                                                    source_ml_source2_data_property_value483
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override489.clone()), source_ml_source2_nominal_field487)
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value490, _) => {
                                        let source_ml_source2_data_property_value490 = *source_ml_source2_data_property_value490;
                                        source_ml_source2_data_property_value490.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_5 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value491, _) => {
                                            let source_ml_source2_data_property_value491 = *source_ml_source2_data_property_value491;
                                            source_ml_source2_data_property_value491.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f509 = source_left_3.clone();
                                        {
                                            let source_ml_source2_f510 = source_right_3.clone();
                                            {
                                                let source_ml_source2_nominal_spread526 = result_5.clone();
                                                {
                                                    let source_ml_source2_nominal_field527 = match source_ml_source2_nominal_spread526.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value528, _) => {
                                                            let source_ml_source2_data_property_value528 = *source_ml_source2_data_property_value528;
                                                            source_ml_source2_data_property_value528.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field529 = match source_ml_source2_nominal_spread526.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value530) => {
                                                                source_ml_source2_data_property_value530
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override531 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value522, _) => {
                                                                    let source_ml_source2_data_property_value522 = *source_ml_source2_data_property_value522;
                                                                    source_ml_source2_data_property_value522.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::StrictEqual(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value523) => {
                                                                    source_ml_source2_data_property_value523
                                                                }
                                                            }, match left_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value524) => {
                                                                    source_ml_source2_data_property_value524
                                                                }
                                                            }, match right_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value525) => {
                                                                    source_ml_source2_data_property_value525
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override531.clone()), source_ml_source2_nominal_field529)
                                                        }
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
                                                    let source_ml_source2_s532 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop100(bindings, depth, current.clone(), arguments_, source_ml_source1_temporary_values96, source_ml_source1_temporary_index96.clone());
                                                    match source_ml_source2_s532.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Done(current_2, arguments__2) => {
                                                            let current_2 = *current_2;
                                                            {
                                                                let result_6 = self::allocate_java_script_call_source_register(current_2.clone());
                                                                {
                                                                    let source_ml_source2_nominal_spread535 = result_6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field536 = match source_ml_source2_nominal_spread535.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value537, _) => {
                                                                                let source_ml_source2_data_property_value537 = *source_ml_source2_data_property_value537;
                                                                                source_ml_source2_data_property_value537.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field538 = match source_ml_source2_nominal_spread535.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value539) => {
                                                                                    source_ml_source2_data_property_value539
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override540 = self::append_java_script_call_source_instruction(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value533, _) => {
                                                                                        let source_ml_source2_data_property_value533 = *source_ml_source2_data_property_value533;
                                                                                        source_ml_source2_data_property_value533.clone()
                                                                                    }
                                                                                }, self::JavaScriptCallInstruction::Primitive(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value534) => {
                                                                                        source_ml_source2_data_property_value534
                                                                                    }
                                                                                }, operation.clone(), arguments__2));
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override540.clone()), source_ml_source2_nominal_field538)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value542, _) => {
                                source_ml_source2_data_property_value542
                            }
                        }) != (0f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source recurrence through an active exception handler requires explicit unwinding")
                        } else {
                            if (((arguments_2.len() as f64)) != (match state.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value544, _, _) => {
                                    source_ml_source2_data_property_value544
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
                                                    let source_ml_source2_s545 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop101(bindings, depth, current_3.clone(), arguments__3, source_ml_source1_temporary_values97, source_ml_source1_temporary_index97.clone());
                                                    match source_ml_source2_s545.clone() {
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value546, _) => {
                                        let source_ml_source2_data_property_value546 = *source_ml_source2_data_property_value546;
                                        source_ml_source2_data_property_value546.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let value_4 = self::lower_java_script_call_source_expression(source_value_3.clone(), bindings, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value547, _) => {
                                            let source_ml_source2_data_property_value547 = *source_ml_source2_data_property_value547;
                                            source_ml_source2_data_property_value547.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let result_7 = self::allocate_java_script_call_source_register(match value_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value548, _) => {
                                                let source_ml_source2_data_property_value548 = *source_ml_source2_data_property_value548;
                                                source_ml_source2_data_property_value548.clone()
                                            }
                                        });
                                        {
                                            let write = self::append_java_script_call_source_instruction(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value549, _) => {
                                                    let source_ml_source2_data_property_value549 = *source_ml_source2_data_property_value549;
                                                    source_ml_source2_data_property_value549.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinarySet(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value550) => {
                                                    source_ml_source2_data_property_value550
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value551) => {
                                                    source_ml_source2_data_property_value551
                                                }
                                            }, match key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value552) => {
                                                    source_ml_source2_data_property_value552
                                                }
                                            }, match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value553) => {
                                                    source_ml_source2_data_property_value553
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value554) => {
                                                    source_ml_source2_data_property_value554
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value555) => {
                                                    source_ml_source2_data_property_value555
                                                }
                                            }))), match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value556) => {
                                                    source_ml_source2_data_property_value556
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value557, _) => {
                                        let source_ml_source2_data_property_value557 = *source_ml_source2_data_property_value557;
                                        source_ml_source2_data_property_value557.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_8 = self::allocate_java_script_call_source_register(match key_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value558, _) => {
                                            let source_ml_source2_data_property_value558 = *source_ml_source2_data_property_value558;
                                            source_ml_source2_data_property_value558.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_nominal_spread564 = result_8.clone();
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
                                                    let source_ml_source2_nominal_override569 = self::append_java_script_call_source_instruction(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value559, _) => {
                                                            let source_ml_source2_data_property_value559 = *source_ml_source2_data_property_value559;
                                                            source_ml_source2_data_property_value559.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryGet(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value560) => {
                                                            source_ml_source2_data_property_value560
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value561) => {
                                                            source_ml_source2_data_property_value561
                                                        }
                                                    }, match key_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value562) => {
                                                            source_ml_source2_data_property_value562
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value563) => {
                                                            source_ml_source2_data_property_value563
                                                        }
                                                    }));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override569.clone()), source_ml_source2_nominal_field567)
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
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value570, _) => {
                                            let source_ml_source2_data_property_value570 = *source_ml_source2_data_property_value570;
                                            source_ml_source2_data_property_value570.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let current_6 = match receiver.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value571, _) => {
                                                let source_ml_source2_data_property_value571 = *source_ml_source2_data_property_value571;
                                                source_ml_source2_data_property_value571.clone()
                                            }
                                        };
                                        {
                                            let arguments__5 = Vec::<f64>::new();
                                            {
                                                let source_ml_source1_temporary_values98 = arguments_3;
                                                {
                                                    let source_ml_source1_temporary_index98 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s572 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop103(bindings, depth, current_6.clone(), arguments__5, source_ml_source1_temporary_values98, source_ml_source1_temporary_index98.clone());
                                                        match source_ml_source2_s572.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Done(current_7, arguments__6) => {
                                                                let current_7 = *current_7;
                                                                {
                                                                    let result_9 = self::allocate_java_script_call_source_register(current_7.clone());
                                                                    {
                                                                        let source_ml_source2_nominal_spread577 = result_9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field578 = match source_ml_source2_nominal_spread577.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value579, _) => {
                                                                                    let source_ml_source2_data_property_value579 = *source_ml_source2_data_property_value579;
                                                                                    source_ml_source2_data_property_value579.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field580 = match source_ml_source2_nominal_spread577.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value581) => {
                                                                                        source_ml_source2_data_property_value581
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override582 = self::append_java_script_call_source_instruction(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value573, _) => {
                                                                                            let source_ml_source2_data_property_value573 = *source_ml_source2_data_property_value573;
                                                                                            source_ml_source2_data_property_value573.clone()
                                                                                        }
                                                                                    }, self::JavaScriptCallInstruction::Call(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value574) => {
                                                                                            source_ml_source2_data_property_value574
                                                                                        }
                                                                                    }, match callee.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value575) => {
                                                                                            source_ml_source2_data_property_value575
                                                                                        }
                                                                                    }, match receiver.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value576) => {
                                                                                            source_ml_source2_data_property_value576
                                                                                        }
                                                                                    }, arguments__6));
                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override582.clone()), source_ml_source2_nominal_field580)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
