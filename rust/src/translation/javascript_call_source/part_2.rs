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
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value347) => {
            source_ml_source2_data_property_value347.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value349, _, _, _, _) => {
                source_ml_source2_data_property_value349
            }
        }) >= (4096f64)) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its register budget")
        } else {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                let source_ml_source2_nominal_spread351 = builder.clone();
                {
                    let source_ml_source2_nominal_field352 = match source_ml_source2_nominal_spread351.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value353, _, _, _, _, _) => {
                            source_ml_source2_data_property_value353
                        }
                    };
                    {
                        let source_ml_source2_nominal_field354 = match source_ml_source2_nominal_spread351.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value355, _, _, _, _) => {
                                source_ml_source2_data_property_value355
                            }
                        };
                        {
                            let source_ml_source2_nominal_field356 = match source_ml_source2_nominal_spread351.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value357, _, _, _) => {
                                    source_ml_source2_data_property_value357
                                }
                            };
                            {
                                let source_ml_source2_nominal_field358 = match source_ml_source2_nominal_spread351.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value359, _, _) => {
                                        source_ml_source2_data_property_value359
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field360 = match source_ml_source2_nominal_spread351.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value361, _) => {
                                            source_ml_source2_data_property_value361
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field362 = match source_ml_source2_nominal_spread351.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value363) => {
                                                source_ml_source2_data_property_value363.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override364 = (match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value350, _, _, _, _) => {
                                                    source_ml_source2_data_property_value350
                                                }
                                            } + 1f64);
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field352, source_ml_source2_nominal_override364, source_ml_source2_nominal_field356, source_ml_source2_nominal_field358, source_ml_source2_nominal_field360, source_ml_source2_nominal_field362.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }), match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value365, _, _, _, _) => {
                    source_ml_source2_data_property_value365
                }
            })
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_expression(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value367) => {
            source_ml_source2_data_property_value367.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((depth > (128f64)) || ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value369, _, _, _) => {
                source_ml_source2_data_property_value369
            }
        }) >= (65536f64))) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its structural budget")
        } else {
            {
                let state = {
                    let source_ml_source2_nominal_spread371 = builder.clone();
                    {
                        let source_ml_source2_nominal_field372 = match source_ml_source2_nominal_spread371.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value373, _, _, _, _, _) => {
                                source_ml_source2_data_property_value373
                            }
                        };
                        {
                            let source_ml_source2_nominal_field374 = match source_ml_source2_nominal_spread371.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value375, _, _, _, _) => {
                                    source_ml_source2_data_property_value375
                                }
                            };
                            {
                                let source_ml_source2_nominal_field376 = match source_ml_source2_nominal_spread371.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value377, _, _, _) => {
                                        source_ml_source2_data_property_value377
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field378 = match source_ml_source2_nominal_spread371.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value379, _, _) => {
                                            source_ml_source2_data_property_value379
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field380 = match source_ml_source2_nominal_spread371.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value381, _) => {
                                                source_ml_source2_data_property_value381
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field382 = match source_ml_source2_nominal_spread371.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value383) => {
                                                    source_ml_source2_data_property_value383.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override384 = (match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value370, _, _, _) => {
                                                        source_ml_source2_data_property_value370
                                                    }
                                                } + 1f64);
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field372, source_ml_source2_nominal_field374, source_ml_source2_nominal_override384, source_ml_source2_nominal_field378, source_ml_source2_nominal_field380, source_ml_source2_nominal_field382.clone())
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
                                        let source_ml_source2_nominal_spread394 = result.clone();
                                        {
                                            let source_ml_source2_nominal_field395 = match source_ml_source2_nominal_spread394.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value396, _) => {
                                                    let source_ml_source2_data_property_value396 = *source_ml_source2_data_property_value396;
                                                    source_ml_source2_data_property_value396.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field397 = match source_ml_source2_nominal_spread394.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value398) => {
                                                        source_ml_source2_data_property_value398
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override399 = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value392, _) => {
                                                            let source_ml_source2_data_property_value392 = *source_ml_source2_data_property_value392;
                                                            source_ml_source2_data_property_value392.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Constant(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value393) => {
                                                            source_ml_source2_data_property_value393
                                                        }
                                                    }, Box::new(value.clone())));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override399.clone()), source_ml_source2_nominal_field397)
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
                                    let source_ml_source2_nominal_spread402 = result_2.clone();
                                    {
                                        let source_ml_source2_nominal_field403 = match source_ml_source2_nominal_spread402.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value404, _) => {
                                                let source_ml_source2_data_property_value404 = *source_ml_source2_data_property_value404;
                                                source_ml_source2_data_property_value404.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field405 = match source_ml_source2_nominal_spread402.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value406) => {
                                                    source_ml_source2_data_property_value406
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override407 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value400, _) => {
                                                        let source_ml_source2_data_property_value400 = *source_ml_source2_data_property_value400;
                                                        source_ml_source2_data_property_value400.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Argument(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value401) => {
                                                        source_ml_source2_data_property_value401
                                                    }
                                                }, index));
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override407.clone()), source_ml_source2_nominal_field405)
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
                                let source_ml_source2_nominal_spread410 = value_2.clone();
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
                                            let source_ml_source2_nominal_override415 = self::append_java_script_call_source_instruction(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value408, _) => {
                                                    let source_ml_source2_data_property_value408 = *source_ml_source2_data_property_value408;
                                                    source_ml_source2_data_property_value408.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Throw(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value409) => {
                                                    source_ml_source2_data_property_value409
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override415.clone()), source_ml_source2_nominal_field413)
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceTry(source_ml_source2_f385, source_ml_source2_f386, source_ml_source2_f387) => {
                        let source_ml_source2_f386 = *source_ml_source2_f386;
                        let source_ml_source2_f387 = *source_ml_source2_f387;
                        self::lower_java_script_call_source_exception(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceBinding(identity) => {
                        if !(self::test_java_script_call_code_coordinate(identity, 65536f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source binding identities require exact nonnegative coordinates")
                        } else {
                            {
                                let index_2 = ((bindings.len() as f64) - 1f64);
                                {
                                    let source_ml_source2_s416 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop99(source.clone(), bindings, state.clone(), index_2);
                                    match source_ml_source2_s416.clone() {
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
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value418, _) => {
                                source_ml_source2_data_property_value418
                            }
                        }) == identity_2) }) }; ml_values1.iter().cloned().enumerate().any(|(ml_index1, ml_value1)| ml_callback1(ml_value1)) }) {
                            self::refuse_java_script_call_source(state.clone(), "a source binding owner must be unique in its lexical scope")
                        } else {
                            {
                                let value_3 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                self::lower_java_script_call_source_expression(body.clone(), {
                                    let source_ml_source2_concat_value420 = bindings;
                                    {
                                        let source_ml_source2_concat_value421 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity_2, match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value419) => {
                                                source_ml_source2_data_property_value419
                                            }
                                        })];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source2_concat_value420), source_ml_source2_concat_value421)
                                    }
                                }, match value_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value422, _) => {
                                        let source_ml_source2_data_property_value422 = *source_ml_source2_data_property_value422;
                                        source_ml_source2_data_property_value422.clone()
                                    }
                                }, (depth + 1f64))
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceIf(source_ml_source2_f388, source_ml_source2_f389, source_ml_source2_f390) => {
                        let source_ml_source2_f388 = *source_ml_source2_f388;
                        let source_ml_source2_f389 = *source_ml_source2_f389;
                        let source_ml_source2_f390 = *source_ml_source2_f390;
                        self::lower_java_script_call_source_branch(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f391, source_left, source_right) => {
                        let source_left = *source_left;
                        let source_right = *source_right;
                        {
                            let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings, match left.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value423, _) => {
                                        let source_ml_source2_data_property_value423 = *source_ml_source2_data_property_value423;
                                        source_ml_source2_data_property_value423.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_3 = self::allocate_java_script_call_source_register(match right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value424, _) => {
                                            let source_ml_source2_data_property_value424 = *source_ml_source2_data_property_value424;
                                            source_ml_source2_data_property_value424.clone()
                                        }
                                    });
                                    {
                                        let operator = source_ml_source2_f391.clone();
                                        {
                                            let source_ml_source2_f438 = source_left.clone();
                                            {
                                                let source_ml_source2_f439 = source_right.clone();
                                                if !(self::ml_array::includes(&vec![String::from("add"), String::from("subtract"), String::from("multiply"), String::from("divide"), String::from("remainder"), String::from("power"), String::from("lessThan"), String::from("equal")], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::refuse_java_script_call_source(match result_3.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value455, _) => {
                                                            let source_ml_source2_data_property_value455 = *source_ml_source2_data_property_value455;
                                                            source_ml_source2_data_property_value455.clone()
                                                        }
                                                    }, "source Number lowering requires its exact registered operator")
                                                } else {
                                                    {
                                                        let source_ml_source2_nominal_spread460 = result_3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field461 = match source_ml_source2_nominal_spread460.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value462, _) => {
                                                                    let source_ml_source2_data_property_value462 = *source_ml_source2_data_property_value462;
                                                                    source_ml_source2_data_property_value462.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field463 = match source_ml_source2_nominal_spread460.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value464) => {
                                                                        source_ml_source2_data_property_value464
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override465 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value456, _) => {
                                                                            let source_ml_source2_data_property_value456 = *source_ml_source2_data_property_value456;
                                                                            source_ml_source2_data_property_value456.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::NumberBinary(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value457) => {
                                                                            source_ml_source2_data_property_value457
                                                                        }
                                                                    }, match left.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value458) => {
                                                                            source_ml_source2_data_property_value458
                                                                        }
                                                                    }, match right.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value459) => {
                                                                            source_ml_source2_data_property_value459
                                                                        }
                                                                    }, operator.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override465.clone()), source_ml_source2_nominal_field463)
                                                                }
                                                            }
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value466, _) => {
                                        let source_ml_source2_data_property_value466 = *source_ml_source2_data_property_value466;
                                        source_ml_source2_data_property_value466.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value467, _) => {
                                            let source_ml_source2_data_property_value467 = *source_ml_source2_data_property_value467;
                                            source_ml_source2_data_property_value467.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f483 = source_left_2.clone();
                                        {
                                            let source_ml_source2_f484 = source_right_2.clone();
                                            {
                                                let source_ml_source2_nominal_spread502 = result_4.clone();
                                                {
                                                    let source_ml_source2_nominal_field503 = match source_ml_source2_nominal_spread502.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value504, _) => {
                                                            let source_ml_source2_data_property_value504 = *source_ml_source2_data_property_value504;
                                                            source_ml_source2_data_property_value504.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field505 = match source_ml_source2_nominal_spread502.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value506) => {
                                                                source_ml_source2_data_property_value506
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override507 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value498, _) => {
                                                                    let source_ml_source2_data_property_value498 = *source_ml_source2_data_property_value498;
                                                                    source_ml_source2_data_property_value498.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::SameValue(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value499) => {
                                                                    source_ml_source2_data_property_value499
                                                                }
                                                            }, match left_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value500) => {
                                                                    source_ml_source2_data_property_value500
                                                                }
                                                            }, match right_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value501) => {
                                                                    source_ml_source2_data_property_value501
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override507.clone()), source_ml_source2_nominal_field505)
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value508, _) => {
                                        let source_ml_source2_data_property_value508 = *source_ml_source2_data_property_value508;
                                        source_ml_source2_data_property_value508.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_5 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value509, _) => {
                                            let source_ml_source2_data_property_value509 = *source_ml_source2_data_property_value509;
                                            source_ml_source2_data_property_value509.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f527 = source_left_3.clone();
                                        {
                                            let source_ml_source2_f528 = source_right_3.clone();
                                            {
                                                let source_ml_source2_nominal_spread544 = result_5.clone();
                                                {
                                                    let source_ml_source2_nominal_field545 = match source_ml_source2_nominal_spread544.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value546, _) => {
                                                            let source_ml_source2_data_property_value546 = *source_ml_source2_data_property_value546;
                                                            source_ml_source2_data_property_value546.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field547 = match source_ml_source2_nominal_spread544.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value548) => {
                                                                source_ml_source2_data_property_value548
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override549 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value540, _) => {
                                                                    let source_ml_source2_data_property_value540 = *source_ml_source2_data_property_value540;
                                                                    source_ml_source2_data_property_value540.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::StrictEqual(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value541) => {
                                                                    source_ml_source2_data_property_value541
                                                                }
                                                            }, match left_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value542) => {
                                                                    source_ml_source2_data_property_value542
                                                                }
                                                            }, match right_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value543) => {
                                                                    source_ml_source2_data_property_value543
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override549.clone()), source_ml_source2_nominal_field547)
                                                        }
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
                                                    let source_ml_source2_s550 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop100(bindings, depth, current.clone(), arguments_, source_ml_source1_temporary_values96, source_ml_source1_temporary_index96.clone());
                                                    match source_ml_source2_s550.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Done(current_2, arguments__2) => {
                                                            let current_2 = *current_2;
                                                            {
                                                                let result_6 = self::allocate_java_script_call_source_register(current_2.clone());
                                                                {
                                                                    let source_ml_source2_nominal_spread553 = result_6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field554 = match source_ml_source2_nominal_spread553.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value555, _) => {
                                                                                let source_ml_source2_data_property_value555 = *source_ml_source2_data_property_value555;
                                                                                source_ml_source2_data_property_value555.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field556 = match source_ml_source2_nominal_spread553.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value557) => {
                                                                                    source_ml_source2_data_property_value557
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override558 = self::append_java_script_call_source_instruction(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value551, _) => {
                                                                                        let source_ml_source2_data_property_value551 = *source_ml_source2_data_property_value551;
                                                                                        source_ml_source2_data_property_value551.clone()
                                                                                    }
                                                                                }, self::JavaScriptCallInstruction::Primitive(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value552) => {
                                                                                        source_ml_source2_data_property_value552
                                                                                    }
                                                                                }, operation.clone(), arguments__2));
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override558.clone()), source_ml_source2_nominal_field556)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value560, _) => {
                                source_ml_source2_data_property_value560
                            }
                        }) != (0f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source recurrence through an active exception handler requires explicit unwinding")
                        } else {
                            if (((arguments_2.len() as f64)) != (match state.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value562, _, _) => {
                                    source_ml_source2_data_property_value562
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
                                                    let source_ml_source2_s563 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop101(bindings, depth, current_3.clone(), arguments__3, source_ml_source1_temporary_values97, source_ml_source1_temporary_index97.clone());
                                                    match source_ml_source2_s563.clone() {
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value564, _) => {
                                        let source_ml_source2_data_property_value564 = *source_ml_source2_data_property_value564;
                                        source_ml_source2_data_property_value564.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let value_4 = self::lower_java_script_call_source_expression(source_value_3.clone(), bindings, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value565, _) => {
                                            let source_ml_source2_data_property_value565 = *source_ml_source2_data_property_value565;
                                            source_ml_source2_data_property_value565.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let result_7 = self::allocate_java_script_call_source_register(match value_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value566, _) => {
                                                let source_ml_source2_data_property_value566 = *source_ml_source2_data_property_value566;
                                                source_ml_source2_data_property_value566.clone()
                                            }
                                        });
                                        {
                                            let write = self::append_java_script_call_source_instruction(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value567, _) => {
                                                    let source_ml_source2_data_property_value567 = *source_ml_source2_data_property_value567;
                                                    source_ml_source2_data_property_value567.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinarySet(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value568) => {
                                                    source_ml_source2_data_property_value568
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value569) => {
                                                    source_ml_source2_data_property_value569
                                                }
                                            }, match key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value570) => {
                                                    source_ml_source2_data_property_value570
                                                }
                                            }, match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value571) => {
                                                    source_ml_source2_data_property_value571
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value572) => {
                                                    source_ml_source2_data_property_value572
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value573) => {
                                                    source_ml_source2_data_property_value573
                                                }
                                            }))), match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value574) => {
                                                    source_ml_source2_data_property_value574
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value575, _) => {
                                        let source_ml_source2_data_property_value575 = *source_ml_source2_data_property_value575;
                                        source_ml_source2_data_property_value575.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_8 = self::allocate_java_script_call_source_register(match key_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value576, _) => {
                                            let source_ml_source2_data_property_value576 = *source_ml_source2_data_property_value576;
                                            source_ml_source2_data_property_value576.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_nominal_spread582 = result_8.clone();
                                        {
                                            let source_ml_source2_nominal_field583 = match source_ml_source2_nominal_spread582.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value584, _) => {
                                                    let source_ml_source2_data_property_value584 = *source_ml_source2_data_property_value584;
                                                    source_ml_source2_data_property_value584.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field585 = match source_ml_source2_nominal_spread582.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value586) => {
                                                        source_ml_source2_data_property_value586
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override587 = self::append_java_script_call_source_instruction(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value577, _) => {
                                                            let source_ml_source2_data_property_value577 = *source_ml_source2_data_property_value577;
                                                            source_ml_source2_data_property_value577.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryGet(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value578) => {
                                                            source_ml_source2_data_property_value578
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value579) => {
                                                            source_ml_source2_data_property_value579
                                                        }
                                                    }, match key_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value580) => {
                                                            source_ml_source2_data_property_value580
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value581) => {
                                                            source_ml_source2_data_property_value581
                                                        }
                                                    }));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override587.clone()), source_ml_source2_nominal_field585)
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
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value588, _) => {
                                            let source_ml_source2_data_property_value588 = *source_ml_source2_data_property_value588;
                                            source_ml_source2_data_property_value588.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let current_6 = match receiver.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value589, _) => {
                                                let source_ml_source2_data_property_value589 = *source_ml_source2_data_property_value589;
                                                source_ml_source2_data_property_value589.clone()
                                            }
                                        };
                                        {
                                            let arguments__5 = Vec::<f64>::new();
                                            {
                                                let source_ml_source1_temporary_values98 = arguments_3;
                                                {
                                                    let source_ml_source1_temporary_index98 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s590 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop103(bindings, depth, current_6.clone(), arguments__5, source_ml_source1_temporary_values98, source_ml_source1_temporary_index98.clone());
                                                        match source_ml_source2_s590.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Done(current_7, arguments__6) => {
                                                                let current_7 = *current_7;
                                                                {
                                                                    let result_9 = self::allocate_java_script_call_source_register(current_7.clone());
                                                                    {
                                                                        let source_ml_source2_nominal_spread595 = result_9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field596 = match source_ml_source2_nominal_spread595.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value597, _) => {
                                                                                    let source_ml_source2_data_property_value597 = *source_ml_source2_data_property_value597;
                                                                                    source_ml_source2_data_property_value597.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field598 = match source_ml_source2_nominal_spread595.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value599) => {
                                                                                        source_ml_source2_data_property_value599
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override600 = self::append_java_script_call_source_instruction(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value591, _) => {
                                                                                            let source_ml_source2_data_property_value591 = *source_ml_source2_data_property_value591;
                                                                                            source_ml_source2_data_property_value591.clone()
                                                                                        }
                                                                                    }, self::JavaScriptCallInstruction::Call(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value592) => {
                                                                                            source_ml_source2_data_property_value592
                                                                                        }
                                                                                    }, match callee.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value593) => {
                                                                                            source_ml_source2_data_property_value593
                                                                                        }
                                                                                    }, match receiver.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value594) => {
                                                                                            source_ml_source2_data_property_value594
                                                                                        }
                                                                                    }, arguments__6));
                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override600.clone()), source_ml_source2_nominal_field598)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
