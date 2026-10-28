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
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value388) => {
            source_ml_source2_data_property_value388.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((depth > (128f64)) || ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value390, _, _, _) => {
                source_ml_source2_data_property_value390
            }
        }) >= (65536f64))) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its structural budget")
        } else {
            {
                let state = {
                    let source_ml_source2_nominal_spread392 = builder.clone();
                    {
                        let source_ml_source2_nominal_field393 = match source_ml_source2_nominal_spread392.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value394, _, _, _, _, _) => {
                                source_ml_source2_data_property_value394
                            }
                        };
                        {
                            let source_ml_source2_nominal_field395 = match source_ml_source2_nominal_spread392.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value396, _, _, _, _) => {
                                    source_ml_source2_data_property_value396
                                }
                            };
                            {
                                let source_ml_source2_nominal_field397 = match source_ml_source2_nominal_spread392.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value398, _, _, _) => {
                                        source_ml_source2_data_property_value398
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field399 = match source_ml_source2_nominal_spread392.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value400, _, _) => {
                                            source_ml_source2_data_property_value400
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field401 = match source_ml_source2_nominal_spread392.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value402, _) => {
                                                source_ml_source2_data_property_value402
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field403 = match source_ml_source2_nominal_spread392.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value404) => {
                                                    source_ml_source2_data_property_value404.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override405 = (match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value391, _, _, _) => {
                                                        source_ml_source2_data_property_value391
                                                    }
                                                } + 1f64);
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field393, source_ml_source2_nominal_field395, source_ml_source2_nominal_override405, source_ml_source2_nominal_field399, source_ml_source2_nominal_field401, source_ml_source2_nominal_field403.clone())
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
                                        let source_ml_source2_nominal_spread415 = result.clone();
                                        {
                                            let source_ml_source2_nominal_field416 = match source_ml_source2_nominal_spread415.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value417, _) => {
                                                    let source_ml_source2_data_property_value417 = *source_ml_source2_data_property_value417;
                                                    source_ml_source2_data_property_value417.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field418 = match source_ml_source2_nominal_spread415.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value419) => {
                                                        source_ml_source2_data_property_value419
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override420 = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value413, _) => {
                                                            let source_ml_source2_data_property_value413 = *source_ml_source2_data_property_value413;
                                                            source_ml_source2_data_property_value413.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Constant(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value414) => {
                                                            source_ml_source2_data_property_value414
                                                        }
                                                    }, Box::new(value.clone())));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override420.clone()), source_ml_source2_nominal_field418)
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
                                    let source_ml_source2_nominal_spread423 = result_2.clone();
                                    {
                                        let source_ml_source2_nominal_field424 = match source_ml_source2_nominal_spread423.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value425, _) => {
                                                let source_ml_source2_data_property_value425 = *source_ml_source2_data_property_value425;
                                                source_ml_source2_data_property_value425.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field426 = match source_ml_source2_nominal_spread423.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value427) => {
                                                    source_ml_source2_data_property_value427
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override428 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value421, _) => {
                                                        let source_ml_source2_data_property_value421 = *source_ml_source2_data_property_value421;
                                                        source_ml_source2_data_property_value421.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Argument(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value422) => {
                                                        source_ml_source2_data_property_value422
                                                    }
                                                }, index));
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override428.clone()), source_ml_source2_nominal_field426)
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
                                let source_ml_source2_nominal_spread431 = value_2.clone();
                                {
                                    let source_ml_source2_nominal_field432 = match source_ml_source2_nominal_spread431.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value433, _) => {
                                            let source_ml_source2_data_property_value433 = *source_ml_source2_data_property_value433;
                                            source_ml_source2_data_property_value433.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field434 = match source_ml_source2_nominal_spread431.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value435) => {
                                                source_ml_source2_data_property_value435
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override436 = self::append_java_script_call_source_instruction(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value429, _) => {
                                                    let source_ml_source2_data_property_value429 = *source_ml_source2_data_property_value429;
                                                    source_ml_source2_data_property_value429.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Throw(match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value430) => {
                                                    source_ml_source2_data_property_value430
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override436.clone()), source_ml_source2_nominal_field434)
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceTry(source_ml_source2_f406, source_ml_source2_f407, source_ml_source2_f408) => {
                        let source_ml_source2_f407 = *source_ml_source2_f407;
                        let source_ml_source2_f408 = *source_ml_source2_f408;
                        self::lower_java_script_call_source_exception(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceBinding(identity) => {
                        if !(self::test_java_script_call_code_coordinate(identity, 65536f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source binding identities require exact nonnegative coordinates")
                        } else {
                            {
                                let index_2 = ((bindings.len() as f64) - 1f64);
                                {
                                    let source_ml_source2_s437 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop100(source.clone(), bindings, state.clone(), index_2);
                                    match source_ml_source2_s437.clone() {
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
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value439, _) => {
                                source_ml_source2_data_property_value439
                            }
                        }) == identity_2) }) }; ml_values1.iter().cloned().enumerate().any(|(ml_index1, ml_value1)| ml_callback1(ml_value1)) }) {
                            self::refuse_java_script_call_source(state.clone(), "a source binding owner must be unique in its lexical scope")
                        } else {
                            {
                                let value_3 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                self::lower_java_script_call_source_expression(body.clone(), {
                                    let source_ml_source2_concat_value441 = bindings;
                                    {
                                        let source_ml_source2_concat_value442 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity_2, match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value440) => {
                                                source_ml_source2_data_property_value440
                                            }
                                        })];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source2_concat_value441), source_ml_source2_concat_value442)
                                    }
                                }, match value_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value443, _) => {
                                        let source_ml_source2_data_property_value443 = *source_ml_source2_data_property_value443;
                                        source_ml_source2_data_property_value443.clone()
                                    }
                                }, (depth + 1f64))
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceIf(source_ml_source2_f409, source_ml_source2_f410, source_ml_source2_f411) => {
                        let source_ml_source2_f409 = *source_ml_source2_f409;
                        let source_ml_source2_f410 = *source_ml_source2_f410;
                        let source_ml_source2_f411 = *source_ml_source2_f411;
                        self::lower_java_script_call_source_branch(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f412, source_left, source_right) => {
                        let source_left = *source_left;
                        let source_right = *source_right;
                        {
                            let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings, match left.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value444, _) => {
                                        let source_ml_source2_data_property_value444 = *source_ml_source2_data_property_value444;
                                        source_ml_source2_data_property_value444.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_3 = self::allocate_java_script_call_source_register(match right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value445, _) => {
                                            let source_ml_source2_data_property_value445 = *source_ml_source2_data_property_value445;
                                            source_ml_source2_data_property_value445.clone()
                                        }
                                    });
                                    {
                                        let operator = source_ml_source2_f412.clone();
                                        {
                                            let source_ml_source2_f459 = source_left.clone();
                                            {
                                                let source_ml_source2_f460 = source_right.clone();
                                                if !(self::ml_array::includes(&vec![String::from("add"), String::from("subtract"), String::from("multiply"), String::from("divide"), String::from("remainder"), String::from("power"), String::from("lessThan"), String::from("equal")], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::refuse_java_script_call_source(match result_3.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value476, _) => {
                                                            let source_ml_source2_data_property_value476 = *source_ml_source2_data_property_value476;
                                                            source_ml_source2_data_property_value476.clone()
                                                        }
                                                    }, "source Number lowering requires its exact registered operator")
                                                } else {
                                                    {
                                                        let source_ml_source2_nominal_spread481 = result_3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field482 = match source_ml_source2_nominal_spread481.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value483, _) => {
                                                                    let source_ml_source2_data_property_value483 = *source_ml_source2_data_property_value483;
                                                                    source_ml_source2_data_property_value483.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field484 = match source_ml_source2_nominal_spread481.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value485) => {
                                                                        source_ml_source2_data_property_value485
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override486 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value477, _) => {
                                                                            let source_ml_source2_data_property_value477 = *source_ml_source2_data_property_value477;
                                                                            source_ml_source2_data_property_value477.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::NumberBinary(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value478) => {
                                                                            source_ml_source2_data_property_value478
                                                                        }
                                                                    }, match left.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value479) => {
                                                                            source_ml_source2_data_property_value479
                                                                        }
                                                                    }, match right.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value480) => {
                                                                            source_ml_source2_data_property_value480
                                                                        }
                                                                    }, operator.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override486.clone()), source_ml_source2_nominal_field484)
                                                                }
                                                            }
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value487, _) => {
                                        let source_ml_source2_data_property_value487 = *source_ml_source2_data_property_value487;
                                        source_ml_source2_data_property_value487.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value488, _) => {
                                            let source_ml_source2_data_property_value488 = *source_ml_source2_data_property_value488;
                                            source_ml_source2_data_property_value488.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f504 = source_left_2.clone();
                                        {
                                            let source_ml_source2_f505 = source_right_2.clone();
                                            {
                                                let source_ml_source2_nominal_spread523 = result_4.clone();
                                                {
                                                    let source_ml_source2_nominal_field524 = match source_ml_source2_nominal_spread523.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value525, _) => {
                                                            let source_ml_source2_data_property_value525 = *source_ml_source2_data_property_value525;
                                                            source_ml_source2_data_property_value525.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field526 = match source_ml_source2_nominal_spread523.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value527) => {
                                                                source_ml_source2_data_property_value527
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override528 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value519, _) => {
                                                                    let source_ml_source2_data_property_value519 = *source_ml_source2_data_property_value519;
                                                                    source_ml_source2_data_property_value519.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::SameValue(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value520) => {
                                                                    source_ml_source2_data_property_value520
                                                                }
                                                            }, match left_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value521) => {
                                                                    source_ml_source2_data_property_value521
                                                                }
                                                            }, match right_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value522) => {
                                                                    source_ml_source2_data_property_value522
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override528.clone()), source_ml_source2_nominal_field526)
                                                        }
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value529, _) => {
                                        let source_ml_source2_data_property_value529 = *source_ml_source2_data_property_value529;
                                        source_ml_source2_data_property_value529.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_5 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value530, _) => {
                                            let source_ml_source2_data_property_value530 = *source_ml_source2_data_property_value530;
                                            source_ml_source2_data_property_value530.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_f548 = source_left_3.clone();
                                        {
                                            let source_ml_source2_f549 = source_right_3.clone();
                                            {
                                                let source_ml_source2_nominal_spread565 = result_5.clone();
                                                {
                                                    let source_ml_source2_nominal_field566 = match source_ml_source2_nominal_spread565.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value567, _) => {
                                                            let source_ml_source2_data_property_value567 = *source_ml_source2_data_property_value567;
                                                            source_ml_source2_data_property_value567.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field568 = match source_ml_source2_nominal_spread565.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value569) => {
                                                                source_ml_source2_data_property_value569
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override570 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value561, _) => {
                                                                    let source_ml_source2_data_property_value561 = *source_ml_source2_data_property_value561;
                                                                    source_ml_source2_data_property_value561.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::StrictEqual(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value562) => {
                                                                    source_ml_source2_data_property_value562
                                                                }
                                                            }, match left_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value563) => {
                                                                    source_ml_source2_data_property_value563
                                                                }
                                                            }, match right_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value564) => {
                                                                    source_ml_source2_data_property_value564
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override570.clone()), source_ml_source2_nominal_field568)
                                                        }
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
                                                    let source_ml_source2_s571 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop101(bindings, depth, current.clone(), arguments_, source_ml_source1_temporary_values97, source_ml_source1_temporary_index97.clone());
                                                    match source_ml_source2_s571.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Done(current_2, arguments__2) => {
                                                            let current_2 = *current_2;
                                                            {
                                                                let result_6 = self::allocate_java_script_call_source_register(current_2.clone());
                                                                {
                                                                    let source_ml_source2_nominal_spread574 = result_6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field575 = match source_ml_source2_nominal_spread574.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value576, _) => {
                                                                                let source_ml_source2_data_property_value576 = *source_ml_source2_data_property_value576;
                                                                                source_ml_source2_data_property_value576.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field577 = match source_ml_source2_nominal_spread574.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value578) => {
                                                                                    source_ml_source2_data_property_value578
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override579 = self::append_java_script_call_source_instruction(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value572, _) => {
                                                                                        let source_ml_source2_data_property_value572 = *source_ml_source2_data_property_value572;
                                                                                        source_ml_source2_data_property_value572.clone()
                                                                                    }
                                                                                }, self::JavaScriptCallInstruction::Primitive(match result_6.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value573) => {
                                                                                        source_ml_source2_data_property_value573
                                                                                    }
                                                                                }, operation.clone(), arguments__2));
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override579.clone()), source_ml_source2_nominal_field577)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value581, _) => {
                                source_ml_source2_data_property_value581
                            }
                        }) != (0f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source recurrence through an active exception handler requires explicit unwinding")
                        } else {
                            if (((arguments_2.len() as f64)) != (match state.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value583, _, _) => {
                                    source_ml_source2_data_property_value583
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
                                                    let source_ml_source2_s584 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop102(bindings, depth, current_3.clone(), arguments__3, source_ml_source1_temporary_values98, source_ml_source1_temporary_index98.clone());
                                                    match source_ml_source2_s584.clone() {
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value585, _) => {
                                        let source_ml_source2_data_property_value585 = *source_ml_source2_data_property_value585;
                                        source_ml_source2_data_property_value585.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let value_4 = self::lower_java_script_call_source_expression(source_value_3.clone(), bindings, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value586, _) => {
                                            let source_ml_source2_data_property_value586 = *source_ml_source2_data_property_value586;
                                            source_ml_source2_data_property_value586.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let result_7 = self::allocate_java_script_call_source_register(match value_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value587, _) => {
                                                let source_ml_source2_data_property_value587 = *source_ml_source2_data_property_value587;
                                                source_ml_source2_data_property_value587.clone()
                                            }
                                        });
                                        {
                                            let write = self::append_java_script_call_source_instruction(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value588, _) => {
                                                    let source_ml_source2_data_property_value588 = *source_ml_source2_data_property_value588;
                                                    source_ml_source2_data_property_value588.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinarySet(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value589) => {
                                                    source_ml_source2_data_property_value589
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value590) => {
                                                    source_ml_source2_data_property_value590
                                                }
                                            }, match key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value591) => {
                                                    source_ml_source2_data_property_value591
                                                }
                                            }, match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value592) => {
                                                    source_ml_source2_data_property_value592
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value593) => {
                                                    source_ml_source2_data_property_value593
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value594) => {
                                                    source_ml_source2_data_property_value594
                                                }
                                            }))), match value_4.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value595) => {
                                                    source_ml_source2_data_property_value595
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value596, _) => {
                                        let source_ml_source2_data_property_value596 = *source_ml_source2_data_property_value596;
                                        source_ml_source2_data_property_value596.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_8 = self::allocate_java_script_call_source_register(match key_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value597, _) => {
                                            let source_ml_source2_data_property_value597 = *source_ml_source2_data_property_value597;
                                            source_ml_source2_data_property_value597.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source2_nominal_spread603 = result_8.clone();
                                        {
                                            let source_ml_source2_nominal_field604 = match source_ml_source2_nominal_spread603.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value605, _) => {
                                                    let source_ml_source2_data_property_value605 = *source_ml_source2_data_property_value605;
                                                    source_ml_source2_data_property_value605.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field606 = match source_ml_source2_nominal_spread603.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value607) => {
                                                        source_ml_source2_data_property_value607
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override608 = self::append_java_script_call_source_instruction(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value598, _) => {
                                                            let source_ml_source2_data_property_value598 = *source_ml_source2_data_property_value598;
                                                            source_ml_source2_data_property_value598.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryGet(match result_8.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value599) => {
                                                            source_ml_source2_data_property_value599
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value600) => {
                                                            source_ml_source2_data_property_value600
                                                        }
                                                    }, match key_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value601) => {
                                                            source_ml_source2_data_property_value601
                                                        }
                                                    }, match object_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value602) => {
                                                            source_ml_source2_data_property_value602
                                                        }
                                                    }));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override608.clone()), source_ml_source2_nominal_field606)
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
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value609, _) => {
                                            let source_ml_source2_data_property_value609 = *source_ml_source2_data_property_value609;
                                            source_ml_source2_data_property_value609.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let current_6 = match receiver.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value610, _) => {
                                                let source_ml_source2_data_property_value610 = *source_ml_source2_data_property_value610;
                                                source_ml_source2_data_property_value610.clone()
                                            }
                                        };
                                        {
                                            let arguments__5 = Vec::<f64>::new();
                                            {
                                                let source_ml_source1_temporary_values99 = arguments_3;
                                                {
                                                    let source_ml_source1_temporary_index99 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s611 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop104(bindings, depth, current_6.clone(), arguments__5, source_ml_source1_temporary_values99, source_ml_source1_temporary_index99.clone());
                                                        match source_ml_source2_s611.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Done(current_7, arguments__6) => {
                                                                let current_7 = *current_7;
                                                                {
                                                                    let result_9 = self::allocate_java_script_call_source_register(current_7.clone());
                                                                    {
                                                                        let source_ml_source2_nominal_spread616 = result_9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field617 = match source_ml_source2_nominal_spread616.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value618, _) => {
                                                                                    let source_ml_source2_data_property_value618 = *source_ml_source2_data_property_value618;
                                                                                    source_ml_source2_data_property_value618.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field619 = match source_ml_source2_nominal_spread616.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value620) => {
                                                                                        source_ml_source2_data_property_value620
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override621 = self::append_java_script_call_source_instruction(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value612, _) => {
                                                                                            let source_ml_source2_data_property_value612 = *source_ml_source2_data_property_value612;
                                                                                            source_ml_source2_data_property_value612.clone()
                                                                                        }
                                                                                    }, self::JavaScriptCallInstruction::Call(match result_9.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value613) => {
                                                                                            source_ml_source2_data_property_value613
                                                                                        }
                                                                                    }, match callee.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value614) => {
                                                                                            source_ml_source2_data_property_value614
                                                                                        }
                                                                                    }, match receiver.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value615) => {
                                                                                            source_ml_source2_data_property_value615
                                                                                        }
                                                                                    }, arguments__6));
                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override621.clone()), source_ml_source2_nominal_field619)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
