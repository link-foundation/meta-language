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
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value310) => {
            source_ml_source2_data_property_value310.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value312, _, _, _) => {
                source_ml_source2_data_property_value312
            }
        }) >= (4096f64)) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its register budget")
        } else {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                let source_ml_source2_nominal_spread314 = builder.clone();
                {
                    let source_ml_source2_nominal_field315 = match source_ml_source2_nominal_spread314.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value316, _, _, _, _) => {
                            source_ml_source2_data_property_value316
                        }
                    };
                    {
                        let source_ml_source2_nominal_field317 = match source_ml_source2_nominal_spread314.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value318, _, _, _) => {
                                source_ml_source2_data_property_value318
                            }
                        };
                        {
                            let source_ml_source2_nominal_field319 = match source_ml_source2_nominal_spread314.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value320, _, _) => {
                                    source_ml_source2_data_property_value320
                                }
                            };
                            {
                                let source_ml_source2_nominal_field321 = match source_ml_source2_nominal_spread314.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value322, _) => {
                                        source_ml_source2_data_property_value322
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field323 = match source_ml_source2_nominal_spread314.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value324) => {
                                            source_ml_source2_data_property_value324.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override325 = (match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value313, _, _, _) => {
                                                source_ml_source2_data_property_value313
                                            }
                                        } + 1f64);
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field315, source_ml_source2_nominal_override325, source_ml_source2_nominal_field319, source_ml_source2_nominal_field321, source_ml_source2_nominal_field323.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }), match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value326, _, _, _) => {
                    source_ml_source2_data_property_value326
                }
            })
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_expression(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value328) => {
            source_ml_source2_data_property_value328.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((depth > (128f64)) || ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value330, _, _) => {
                source_ml_source2_data_property_value330
            }
        }) >= (65536f64))) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its structural budget")
        } else {
            {
                let state = {
                    let source_ml_source2_nominal_spread332 = builder.clone();
                    {
                        let source_ml_source2_nominal_field333 = match source_ml_source2_nominal_spread332.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value334, _, _, _, _) => {
                                source_ml_source2_data_property_value334
                            }
                        };
                        {
                            let source_ml_source2_nominal_field335 = match source_ml_source2_nominal_spread332.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value336, _, _, _) => {
                                    source_ml_source2_data_property_value336
                                }
                            };
                            {
                                let source_ml_source2_nominal_field337 = match source_ml_source2_nominal_spread332.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value338, _, _) => {
                                        source_ml_source2_data_property_value338
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field339 = match source_ml_source2_nominal_spread332.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value340, _) => {
                                            source_ml_source2_data_property_value340
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field341 = match source_ml_source2_nominal_spread332.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value342) => {
                                                source_ml_source2_data_property_value342.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override343 = (match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value331, _, _) => {
                                                    source_ml_source2_data_property_value331
                                                }
                                            } + 1f64);
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field333, source_ml_source2_nominal_field335, source_ml_source2_nominal_override343, source_ml_source2_nominal_field339, source_ml_source2_nominal_field341.clone())
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
                                        let source_ml_source2_nominal_spread350 = result.clone();
                                        {
                                            let source_ml_source2_nominal_field351 = match source_ml_source2_nominal_spread350.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value352, _) => {
                                                    let source_ml_source2_data_property_value352 = *source_ml_source2_data_property_value352;
                                                    source_ml_source2_data_property_value352.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field353 = match source_ml_source2_nominal_spread350.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value354) => {
                                                        source_ml_source2_data_property_value354
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override355 = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value348, _) => {
                                                            let source_ml_source2_data_property_value348 = *source_ml_source2_data_property_value348;
                                                            source_ml_source2_data_property_value348.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Constant(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value349) => {
                                                            source_ml_source2_data_property_value349
                                                        }
                                                    }, Box::new(value.clone())));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override355.clone()), source_ml_source2_nominal_field353)
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
                                    let source_ml_source2_s356 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop97(source.clone(), bindings, state.clone(), index);
                                    match source_ml_source2_s356.clone() {
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
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value358, _) => {
                                source_ml_source2_data_property_value358
                            }
                        }) == identity_2) }) }; ml_values1.iter().cloned().enumerate().any(|(ml_index1, ml_value1)| ml_callback1(ml_value1)) }) {
                            self::refuse_java_script_call_source(state.clone(), "a source binding owner must be unique in its lexical scope")
                        } else {
                            {
                                let value_2 = self::lower_java_script_call_source_expression(source_value.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                self::lower_java_script_call_source_expression(body.clone(), {
                                    let source_ml_source2_concat_value360 = bindings;
                                    {
                                        let source_ml_source2_concat_value361 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity_2, match value_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value359) => {
                                                source_ml_source2_data_property_value359
                                            }
                                        })];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source2_concat_value360), source_ml_source2_concat_value361)
                                    }
                                }, match value_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value362, _) => {
                                        let source_ml_source2_data_property_value362 = *source_ml_source2_data_property_value362;
                                        source_ml_source2_data_property_value362.clone()
                                    }
                                }, (depth + 1f64))
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceIf(source_ml_source2_f344, source_ml_source2_f345, source_ml_source2_f346) => {
                        let source_ml_source2_f344 = *source_ml_source2_f344;
                        let source_ml_source2_f345 = *source_ml_source2_f345;
                        let source_ml_source2_f346 = *source_ml_source2_f346;
                        self::lower_java_script_call_source_branch(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f347, source_left, source_right) => {
                        let source_left = *source_left;
                        let source_right = *source_right;
                        {
                            let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings, match left.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value363, _) => {
                                        let source_ml_source2_data_property_value363 = *source_ml_source2_data_property_value363;
                                        source_ml_source2_data_property_value363.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_2 = self::allocate_java_script_call_source_register(match right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value364, _) => {
                                            let source_ml_source2_data_property_value364 = *source_ml_source2_data_property_value364;
                                            source_ml_source2_data_property_value364.clone()
                                        }
                                    });
                                    {
                                        let operator = source_ml_source2_f347.clone();
                                        {
                                            let source_ml_source2_f373 = source_left.clone();
                                            {
                                                let source_ml_source2_f374 = source_right.clone();
                                                if !(self::ml_array::includes(&vec![String::from("add"), String::from("subtract"), String::from("multiply"), String::from("divide"), String::from("remainder"), String::from("power"), String::from("lessThan"), String::from("equal")], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::refuse_java_script_call_source(match result_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value390, _) => {
                                                            let source_ml_source2_data_property_value390 = *source_ml_source2_data_property_value390;
                                                            source_ml_source2_data_property_value390.clone()
                                                        }
                                                    }, "source Number lowering requires its exact registered operator")
                                                } else {
                                                    {
                                                        let source_ml_source2_nominal_spread395 = result_2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field396 = match source_ml_source2_nominal_spread395.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value397, _) => {
                                                                    let source_ml_source2_data_property_value397 = *source_ml_source2_data_property_value397;
                                                                    source_ml_source2_data_property_value397.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field398 = match source_ml_source2_nominal_spread395.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value399) => {
                                                                        source_ml_source2_data_property_value399
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override400 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value391, _) => {
                                                                            let source_ml_source2_data_property_value391 = *source_ml_source2_data_property_value391;
                                                                            source_ml_source2_data_property_value391.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::NumberBinary(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value392) => {
                                                                            source_ml_source2_data_property_value392
                                                                        }
                                                                    }, match left.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value393) => {
                                                                            source_ml_source2_data_property_value393
                                                                        }
                                                                    }, match right.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value394) => {
                                                                            source_ml_source2_data_property_value394
                                                                        }
                                                                    }, operator.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override400.clone()), source_ml_source2_nominal_field398)
                                                                }
                                                            }
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value401, _) => {
                                        let source_ml_source2_data_property_value401 = *source_ml_source2_data_property_value401;
                                        source_ml_source2_data_property_value401.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_3 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value402, _) => {
                                            let source_ml_source2_data_property_value402 = *source_ml_source2_data_property_value402;
                                            source_ml_source2_data_property_value402.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f413 = source_left_2.clone();
                                        {
                                            let source_ml_source2_f414 = source_right_2.clone();
                                            {
                                                let source_ml_source2_nominal_spread432 = result_3.clone();
                                                {
                                                    let source_ml_source2_nominal_field433 = match source_ml_source2_nominal_spread432.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value434, _) => {
                                                            let source_ml_source2_data_property_value434 = *source_ml_source2_data_property_value434;
                                                            source_ml_source2_data_property_value434.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field435 = match source_ml_source2_nominal_spread432.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value436) => {
                                                                source_ml_source2_data_property_value436
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override437 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value428, _) => {
                                                                    let source_ml_source2_data_property_value428 = *source_ml_source2_data_property_value428;
                                                                    source_ml_source2_data_property_value428.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::SameValue(match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value429) => {
                                                                    source_ml_source2_data_property_value429
                                                                }
                                                            }, match left_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value430) => {
                                                                    source_ml_source2_data_property_value430
                                                                }
                                                            }, match right_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value431) => {
                                                                    source_ml_source2_data_property_value431
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override437.clone()), source_ml_source2_nominal_field435)
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value438, _) => {
                                        let source_ml_source2_data_property_value438 = *source_ml_source2_data_property_value438;
                                        source_ml_source2_data_property_value438.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value439, _) => {
                                            let source_ml_source2_data_property_value439 = *source_ml_source2_data_property_value439;
                                            source_ml_source2_data_property_value439.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f452 = source_left_3.clone();
                                        {
                                            let source_ml_source2_f453 = source_right_3.clone();
                                            {
                                                let source_ml_source2_nominal_spread469 = result_4.clone();
                                                {
                                                    let source_ml_source2_nominal_field470 = match source_ml_source2_nominal_spread469.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value471, _) => {
                                                            let source_ml_source2_data_property_value471 = *source_ml_source2_data_property_value471;
                                                            source_ml_source2_data_property_value471.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field472 = match source_ml_source2_nominal_spread469.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value473) => {
                                                                source_ml_source2_data_property_value473
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override474 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value465, _) => {
                                                                    let source_ml_source2_data_property_value465 = *source_ml_source2_data_property_value465;
                                                                    source_ml_source2_data_property_value465.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::StrictEqual(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value466) => {
                                                                    source_ml_source2_data_property_value466
                                                                }
                                                            }, match left_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value467) => {
                                                                    source_ml_source2_data_property_value467
                                                                }
                                                            }, match right_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value468) => {
                                                                    source_ml_source2_data_property_value468
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override474.clone()), source_ml_source2_nominal_field472)
                                                        }
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
                                                    let source_ml_source2_s475 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop98(bindings, depth, current.clone(), arguments_, source_ml_source1_temporary_values94, source_ml_source1_temporary_index94.clone());
                                                    match source_ml_source2_s475.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Done(current_2, arguments__2) => {
                                                            let current_2 = *current_2;
                                                            {
                                                                let result_5 = self::allocate_java_script_call_source_register(current_2.clone());
                                                                {
                                                                    let source_ml_source2_nominal_spread478 = result_5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field479 = match source_ml_source2_nominal_spread478.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value480, _) => {
                                                                                let source_ml_source2_data_property_value480 = *source_ml_source2_data_property_value480;
                                                                                source_ml_source2_data_property_value480.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field481 = match source_ml_source2_nominal_spread478.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value482) => {
                                                                                    source_ml_source2_data_property_value482
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override483 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value476, _) => {
                                                                                        let source_ml_source2_data_property_value476 = *source_ml_source2_data_property_value476;
                                                                                        source_ml_source2_data_property_value476.clone()
                                                                                    }
                                                                                }, self::JavaScriptCallInstruction::Primitive(match result_5.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value477) => {
                                                                                        source_ml_source2_data_property_value477
                                                                                    }
                                                                                }, operation.clone(), arguments__2));
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override483.clone()), source_ml_source2_nominal_field481)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value485, _) => {
                                source_ml_source2_data_property_value485
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
                                                let source_ml_source2_s486 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop99(bindings, depth, current_3.clone(), arguments__3, source_ml_source1_temporary_values95, source_ml_source1_temporary_index95.clone());
                                                match source_ml_source2_s486.clone() {
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value487, _) => {
                                        let source_ml_source2_data_property_value487 = *source_ml_source2_data_property_value487;
                                        source_ml_source2_data_property_value487.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let value_3 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value488, _) => {
                                            let source_ml_source2_data_property_value488 = *source_ml_source2_data_property_value488;
                                            source_ml_source2_data_property_value488.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let result_6 = self::allocate_java_script_call_source_register(match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value489, _) => {
                                                let source_ml_source2_data_property_value489 = *source_ml_source2_data_property_value489;
                                                source_ml_source2_data_property_value489.clone()
                                            }
                                        });
                                        {
                                            let write = self::append_java_script_call_source_instruction(match result_6.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value490, _) => {
                                                    let source_ml_source2_data_property_value490 = *source_ml_source2_data_property_value490;
                                                    source_ml_source2_data_property_value490.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinarySet(match result_6.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value491) => {
                                                    source_ml_source2_data_property_value491
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value492) => {
                                                    source_ml_source2_data_property_value492
                                                }
                                            }, match key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value493) => {
                                                    source_ml_source2_data_property_value493
                                                }
                                            }, match value_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value494) => {
                                                    source_ml_source2_data_property_value494
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value495) => {
                                                    source_ml_source2_data_property_value495
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_6.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value496) => {
                                                    source_ml_source2_data_property_value496
                                                }
                                            }))), match value_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value497) => {
                                                    source_ml_source2_data_property_value497
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value498, _) => {
                                        let source_ml_source2_data_property_value498 = *source_ml_source2_data_property_value498;
                                        source_ml_source2_data_property_value498.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_7 = self::allocate_java_script_call_source_register(match key_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value499, _) => {
                                            let source_ml_source2_data_property_value499 = *source_ml_source2_data_property_value499;
                                            source_ml_source2_data_property_value499.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_nominal_spread505 = result_7.clone();
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
                                                    let source_ml_source2_nominal_override510 = self::append_java_script_call_source_instruction(match result_7.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value500, _) => {
                                                            let source_ml_source2_data_property_value500 = *source_ml_source2_data_property_value500;
                                                            source_ml_source2_data_property_value500.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryGet(match result_7.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value501) => {
                                                            source_ml_source2_data_property_value501
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value502) => {
                                                            source_ml_source2_data_property_value502
                                                        }
                                                    }, match key_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value503) => {
                                                            source_ml_source2_data_property_value503
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value504) => {
                                                            source_ml_source2_data_property_value504
                                                        }
                                                    }));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override510.clone()), source_ml_source2_nominal_field508)
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
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value511, _) => {
                                            let source_ml_source2_data_property_value511 = *source_ml_source2_data_property_value511;
                                            source_ml_source2_data_property_value511.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let current_6 = match receiver.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value512, _) => {
                                                let source_ml_source2_data_property_value512 = *source_ml_source2_data_property_value512;
                                                source_ml_source2_data_property_value512.clone()
                                            }
                                        };
                                        {
                                            let arguments__5 = Vec::<f64>::new();
                                            {
                                                let source_ml_source1_temporary_values96 = arguments_3;
                                                {
                                                    let source_ml_source1_temporary_index96 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s513 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop101(bindings, depth, current_6.clone(), arguments__5, source_ml_source1_temporary_values96, source_ml_source1_temporary_index96.clone());
                                                        match source_ml_source2_s513.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Done(current_7, arguments__6) => {
                                                                let current_7 = *current_7;
                                                                {
                                                                    let result_8 = self::allocate_java_script_call_source_register(current_7.clone());
                                                                    {
                                                                        let source_ml_source2_nominal_spread518 = result_8.clone();
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
                                                                                    let source_ml_source2_nominal_override523 = self::append_java_script_call_source_instruction(match result_8.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value514, _) => {
                                                                                            let source_ml_source2_data_property_value514 = *source_ml_source2_data_property_value514;
                                                                                            source_ml_source2_data_property_value514.clone()
                                                                                        }
                                                                                    }, self::JavaScriptCallInstruction::Call(match result_8.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value515) => {
                                                                                            source_ml_source2_data_property_value515
                                                                                        }
                                                                                    }, match callee.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value516) => {
                                                                                            source_ml_source2_data_property_value516
                                                                                        }
                                                                                    }, match receiver.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value517) => {
                                                                                            source_ml_source2_data_property_value517
                                                                                        }
                                                                                    }, arguments__6));
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
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
