// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_math;
use super::ml_string;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_array_expression_loop33;
use super::source_ml_source1_temporary_lower_java_script_call_source_array_expression_loop34;
use super::source_ml_source1_temporary_lower_java_script_call_source_object_expression_loop41;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_property_query_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceStringGet(source_input, source_key, source_prototype) => {
            let source_input = *source_input;
            let source_key = *source_key;
            let source_prototype = *source_prototype;
            {
                let input = self::lower_java_script_call_source_expression(source_input.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let held = self::allocate_java_script_call_source_register(match input.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9167, _) => {
                            let source_ml_source2_data_property_value9167 = *source_ml_source2_data_property_value9167;
                            source_ml_source2_data_property_value9167.clone()
                        }
                    });
                    {
                        let saved = self::append_java_script_call_source_instruction(match held.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9168, _) => {
                                let source_ml_source2_data_property_value9168 = *source_ml_source2_data_property_value9168;
                                source_ml_source2_data_property_value9168.clone()
                            }
                        }, self::JavaScriptCallInstruction::Copy(match held.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9169) => {
                                source_ml_source2_data_property_value9169
                            }
                        }, match input.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9170) => {
                                source_ml_source2_data_property_value9170
                            }
                        }));
                        {
                            let key = self::lower_java_script_call_source_expression(source_key.clone(), bindings.clone(), saved.clone(), (depth + 1f64));
                            {
                                let held_key = self::allocate_java_script_call_source_register(match key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9171, _) => {
                                        let source_ml_source2_data_property_value9171 = *source_ml_source2_data_property_value9171;
                                        source_ml_source2_data_property_value9171.clone()
                                    }
                                });
                                {
                                    let saved_key = self::append_java_script_call_source_instruction(match held_key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9172, _) => {
                                            let source_ml_source2_data_property_value9172 = *source_ml_source2_data_property_value9172;
                                            source_ml_source2_data_property_value9172.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Copy(match held_key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9173) => {
                                            source_ml_source2_data_property_value9173
                                        }
                                    }, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9174) => {
                                            source_ml_source2_data_property_value9174
                                        }
                                    }));
                                    {
                                        let prototype = self::lower_java_script_call_source_expression(source_prototype.clone(), bindings.clone(), saved_key.clone(), (depth + 1f64));
                                        {
                                            let condition = self::allocate_java_script_call_source_register(match prototype.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9175, _) => {
                                                    let source_ml_source2_data_property_value9175 = *source_ml_source2_data_property_value9175;
                                                    source_ml_source2_data_property_value9175.clone()
                                                }
                                            });
                                            {
                                                let result = self::allocate_java_script_call_source_register(match condition.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9176, _) => {
                                                        let source_ml_source2_data_property_value9176 = *source_ml_source2_data_property_value9176;
                                                        source_ml_source2_data_property_value9176.clone()
                                                    }
                                                });
                                                {
                                                    let tested = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9177, _) => {
                                                            let source_ml_source2_data_property_value9177 = *source_ml_source2_data_property_value9177;
                                                            source_ml_source2_data_property_value9177.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Primitive(match condition.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9178) => {
                                                            source_ml_source2_data_property_value9178
                                                        }
                                                    }, self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 32, 107, 101, 121]), std::rc::Rc::new(std::cell::RefCell::new(vec![match held.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9179) => {
                                                            source_ml_source2_data_property_value9179
                                                        }
                                                    }, match held_key.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9180) => {
                                                            source_ml_source2_data_property_value9180
                                                        }
                                                    }]))));
                                                    {
                                                        let branch = { let ml_reference_array74 = match tested.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9181, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value9181.clone()
                                                            }
                                                        }; let ml_reference_projection74 = ml_reference_array74.borrow().len() as f64; ml_reference_projection74 };
                                                        {
                                                            let selected = self::append_java_script_call_source_instruction(tested.clone(), self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9182) => {
                                                                    source_ml_source2_data_property_value9182
                                                                }
                                                            }, (branch + 1f64), 0f64));
                                                            {
                                                                let own = self::append_java_script_call_source_instruction(selected.clone(), self::JavaScriptCallInstruction::Primitive(match result.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9183) => {
                                                                        source_ml_source2_data_property_value9183
                                                                    }
                                                                }, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 32, 107, 101, 121]), std::rc::Rc::new(std::cell::RefCell::new(vec![match held.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9184) => {
                                                                        source_ml_source2_data_property_value9184
                                                                    }
                                                                }, match held_key.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9185) => {
                                                                        source_ml_source2_data_property_value9185
                                                                    }
                                                                }]))));
                                                                {
                                                                    let jump = { let ml_reference_array75 = match own.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9186, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value9186.clone()
                                                                        }
                                                                    }; let ml_reference_projection75 = ml_reference_array75.borrow().len() as f64; ml_reference_projection75 };
                                                                    {
                                                                        let skipped = self::append_java_script_call_source_instruction(own.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                                        {
                                                                            let get = { let ml_reference_array76 = match skipped.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9187, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value9187.clone()
                                                                                }
                                                                            }; let ml_reference_projection76 = ml_reference_array76.borrow().len() as f64; ml_reference_projection76 };
                                                                            {
                                                                                let got = self::append_java_script_call_source_instruction(skipped.clone(), self::JavaScriptCallInstruction::OrdinaryGet(match result.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9188) => {
                                                                                        source_ml_source2_data_property_value9188
                                                                                    }
                                                                                }, match prototype.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9189) => {
                                                                                        source_ml_source2_data_property_value9189
                                                                                    }
                                                                                }, match held_key.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9190) => {
                                                                                        source_ml_source2_data_property_value9190
                                                                                    }
                                                                                }, match held.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9191) => {
                                                                                        source_ml_source2_data_property_value9191
                                                                                    }
                                                                                }));
                                                                                {
                                                                                    let end = { let ml_reference_array77 = match got.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9192, _, _, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value9192.clone()
                                                                                        }
                                                                                    }; let ml_reference_projection77 = ml_reference_array77.borrow().len() as f64; ml_reference_projection77 };
                                                                                    {
                                                                                        let source_ml_source2_constructor_field9194 = match result.clone() {
                                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9193) => {
                                                                                                source_ml_source2_data_property_value9193
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_constructor_field9219 = {
                                                                                                let source_ml_source2_nominal_spread9197 = got.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9198 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9199, _, _, _, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value9199.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9200 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value9201, _, _, _, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value9201
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9202 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value9203, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value9203
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9204 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value9205, _, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value9205
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9206 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value9207, _, _, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value9207
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9208 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value9209, _, _, _, _) => {
                                                                                                                                source_ml_source2_data_property_value9209.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field9210 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value9211, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value9211.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field9212 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value9213, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value9213.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field9214 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value9215, _) => {
                                                                                                                                            source_ml_source2_data_property_value9215.clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field9216 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9217) => {
                                                                                                                                                source_ml_source2_data_property_value9217.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_override9218 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values78 = { let ml_reference_value79 = (match got.clone() {
                                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9195, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                    source_ml_source2_data_property_value9195.clone()
                                                                                                                                                }
                                                                                                                                            }).borrow().clone(); ml_reference_value79 }; let ml_callback78 = { let branch = branch; let condition = condition.clone(); let get = get; let jump = jump; let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, at: f64| -> self::JavaScriptCallInstruction { if (at == branch) {
                                                                                                                                                self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9196) => {
                                                                                                                                                        source_ml_source2_data_property_value9196
                                                                                                                                                    }
                                                                                                                                                }, (branch + 1f64), get)
                                                                                                                                            } else {
                                                                                                                                                if (at == jump) {
                                                                                                                                                    self::JavaScriptCallInstruction::Jump(end)
                                                                                                                                                } else {
                                                                                                                                                    instruction.clone()
                                                                                                                                                }
                                                                                                                                            } }) }; ml_values78.iter().cloned().enumerate().map(|(ml_index78, ml_value78)| ml_callback78(ml_value78, (ml_index78 as f64))).collect::<Vec<_>>() }));
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override9218.clone(), source_ml_source2_nominal_field9200, source_ml_source2_nominal_field9202, source_ml_source2_nominal_field9204, source_ml_source2_nominal_field9206, source_ml_source2_nominal_field9208.clone(), source_ml_source2_nominal_field9210.clone(), source_ml_source2_nominal_field9212.clone(), source_ml_source2_nominal_field9214.clone(), source_ml_source2_nominal_field9216.clone())
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            };
                                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field9219.clone()), source_ml_source2_constructor_field9194)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceGet(source_object, source_key_2, source_receiver) => {
            let source_object = *source_object;
            let source_key_2 = *source_key_2;
            {
                let object = self::lower_java_script_call_source_expression(source_object.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let key_2 = self::lower_java_script_call_source_expression(source_key_2.clone(), bindings.clone(), match object.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9220, _) => {
                            let source_ml_source2_data_property_value9220 = *source_ml_source2_data_property_value9220;
                            source_ml_source2_data_property_value9220.clone()
                        }
                    }, (depth + 1f64));
                    {
                        let supplied_receiver = ((source_receiver.clone().unwrap_or_else(|| (None::<self::JavaScriptCallSource>,))).0).clone();
                        {
                            let receiver = if match (supplied_receiver.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                {
                                    let source_ml_source2_constructor_field9222 = match object.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9221) => {
                                            source_ml_source2_data_property_value9221
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field9224 = match key_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9223, _) => {
                                                let source_ml_source2_data_property_value9223 = *source_ml_source2_data_property_value9223;
                                                source_ml_source2_data_property_value9223.clone()
                                            }
                                        };
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field9224.clone()), source_ml_source2_constructor_field9222)
                                    }
                                }
                            } else {
                                self::lower_java_script_call_source_expression(supplied_receiver.clone().expect("a proven present value"), bindings.clone(), match key_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9225, _) => {
                                        let source_ml_source2_data_property_value9225 = *source_ml_source2_data_property_value9225;
                                        source_ml_source2_data_property_value9225.clone()
                                    }
                                }, (depth + 1f64))
                            };
                            {
                                let result_2 = self::allocate_java_script_call_source_register(match receiver.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9226, _) => {
                                        let source_ml_source2_data_property_value9226 = *source_ml_source2_data_property_value9226;
                                        source_ml_source2_data_property_value9226.clone()
                                    }
                                });
                                {
                                    let source_ml_source2_nominal_field9233 = match result_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9234, _) => {
                                            let source_ml_source2_data_property_value9234 = *source_ml_source2_data_property_value9234;
                                            source_ml_source2_data_property_value9234.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field9235 = match result_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9236) => {
                                                source_ml_source2_data_property_value9236
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override9237 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9227, _) => {
                                                    let source_ml_source2_data_property_value9227 = *source_ml_source2_data_property_value9227;
                                                    source_ml_source2_data_property_value9227.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinaryGet(match result_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9228) => {
                                                    source_ml_source2_data_property_value9228
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9229) => {
                                                    source_ml_source2_data_property_value9229
                                                }
                                            }, match key_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9230) => {
                                                    source_ml_source2_data_property_value9230
                                                }
                                            }, match receiver.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9231) => {
                                                    source_ml_source2_data_property_value9231
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override9237.clone()), source_ml_source2_nominal_field9235)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceDelete(source_object_2, source_key_3) => {
            let source_object_2 = *source_object_2;
            let source_key_3 = *source_key_3;
            {
                let raw_ = self::lower_java_script_call_source_expression(source_object_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let object_2 = self::allocate_java_script_call_source_register(match raw_.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9238, _) => {
                            let source_ml_source2_data_property_value9238 = *source_ml_source2_data_property_value9238;
                            source_ml_source2_data_property_value9238.clone()
                        }
                    });
                    {
                        let saved_2 = self::append_java_script_call_source_instruction(match object_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9239, _) => {
                                let source_ml_source2_data_property_value9239 = *source_ml_source2_data_property_value9239;
                                source_ml_source2_data_property_value9239.clone()
                            }
                        }, self::JavaScriptCallInstruction::Copy(match object_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9240) => {
                                source_ml_source2_data_property_value9240
                            }
                        }, match raw_.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9241) => {
                                source_ml_source2_data_property_value9241
                            }
                        }));
                        {
                            let key_3 = self::lower_java_script_call_source_expression(source_key_3.clone(), bindings.clone(), saved_2.clone(), (depth + 1f64));
                            {
                                let result_3 = self::allocate_java_script_call_source_register(match key_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9242, _) => {
                                        let source_ml_source2_data_property_value9242 = *source_ml_source2_data_property_value9242;
                                        source_ml_source2_data_property_value9242.clone()
                                    }
                                });
                                {
                                    let current = self::append_java_script_call_source_instruction(match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9243, _) => {
                                            let source_ml_source2_data_property_value9243 = *source_ml_source2_data_property_value9243;
                                            source_ml_source2_data_property_value9243.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::OrdinaryDelete(match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9428) => {
                                            source_ml_source2_data_property_value9428
                                        }
                                    }, match object_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9429) => {
                                            source_ml_source2_data_property_value9429
                                        }
                                    }, match key_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9430) => {
                                            source_ml_source2_data_property_value9430
                                        }
                                    }));
                                    {
                                        let current_3 = {
                                            let checked = self::allocate_java_script_call_source_register(current.clone());
                                            {
                                                let current_2 = self::append_java_script_call_source_instruction(match checked.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9618, _) => {
                                                        let source_ml_source2_data_property_value9618 = *source_ml_source2_data_property_value9618;
                                                        source_ml_source2_data_property_value9618.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Primitive(match checked.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9619) => {
                                                        source_ml_source2_data_property_value9619
                                                    }
                                                }, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 100, 101, 108, 101, 116, 101, 32, 115, 117, 99, 99, 101, 115, 115]), std::rc::Rc::new(std::cell::RefCell::new(vec![match result_3.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9620) => {
                                                        source_ml_source2_data_property_value9620
                                                    }
                                                }]))));
                                                current_2.clone()
                                            }
                                        };
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(current_3.clone()), match result_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9621) => {
                                                source_ml_source2_data_property_value9621
                                            }
                                        })
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceHas(source_object_3, source_key_4) => {
            let source_object_3 = *source_object_3;
            let source_key_4 = *source_key_4;
            {
                let raw_2 = self::lower_java_script_call_source_expression(source_object_3.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let object_3 = self::allocate_java_script_call_source_register(match raw_2.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9622, _) => {
                            let source_ml_source2_data_property_value9622 = *source_ml_source2_data_property_value9622;
                            source_ml_source2_data_property_value9622.clone()
                        }
                    });
                    {
                        let saved_3 = self::append_java_script_call_source_instruction(match object_3.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9623, _) => {
                                let source_ml_source2_data_property_value9623 = *source_ml_source2_data_property_value9623;
                                source_ml_source2_data_property_value9623.clone()
                            }
                        }, self::JavaScriptCallInstruction::Copy(match object_3.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9624) => {
                                source_ml_source2_data_property_value9624
                            }
                        }, match raw_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9625) => {
                                source_ml_source2_data_property_value9625
                            }
                        }));
                        {
                            let key_4 = self::lower_java_script_call_source_expression(source_key_4.clone(), bindings.clone(), saved_3.clone(), (depth + 1f64));
                            {
                                let result_4 = self::allocate_java_script_call_source_register(match key_4.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9626, _) => {
                                        let source_ml_source2_data_property_value9626 = *source_ml_source2_data_property_value9626;
                                        source_ml_source2_data_property_value9626.clone()
                                    }
                                });
                                {
                                    let current_4 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9627, _) => {
                                            let source_ml_source2_data_property_value9627 = *source_ml_source2_data_property_value9627;
                                            source_ml_source2_data_property_value9627.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::OrdinaryHas(match result_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9815) => {
                                            source_ml_source2_data_property_value9815
                                        }
                                    }, match object_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9816) => {
                                            source_ml_source2_data_property_value9816
                                        }
                                    }, match key_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9817) => {
                                            source_ml_source2_data_property_value9817
                                        }
                                    }));
                                    {
                                        let current_5 = current_4.clone();
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(current_5.clone()), match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10002) => {
                                                source_ml_source2_data_property_value10002
                                            }
                                        })
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 113, 117, 101, 114, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_array_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceArrayAccumulation(source_prototype, parts, source_iterator_key, realm) => {
            let source_prototype = *source_prototype;
            let source_iterator_key = *source_iterator_key;
            if (((({ let ml_reference_array80 = parts.clone(); let ml_reference_projection80 = ml_reference_array80.borrow().len() as f64; ml_reference_projection80 }) > (4096f64)) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 99, 99, 117, 109, 117, 108, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 98, 111, 117, 110, 100, 101, 100, 32, 112, 97, 114, 116, 115, 32, 97, 110, 100, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109])))
            } else {
                {
                    let prototype = self::lower_java_script_call_source_expression(source_prototype.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let object = self::allocate_java_script_call_source_register(match prototype.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10179, _) => {
                                let source_ml_source2_data_property_value10179 = *source_ml_source2_data_property_value10179;
                                source_ml_source2_data_property_value10179.clone()
                            }
                        });
                        {
                            let current = self::append_java_script_call_source_instruction(match object.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10180, _) => {
                                    let source_ml_source2_data_property_value10180 = *source_ml_source2_data_property_value10180;
                                    source_ml_source2_data_property_value10180.clone()
                                }
                            }, self::JavaScriptCallInstruction::ArrayAllocate(match object.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10181) => {
                                    source_ml_source2_data_property_value10181
                                }
                            }, match prototype.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10182) => {
                                    source_ml_source2_data_property_value10182
                                }
                            }, 0f64));
                            {
                                let iterator_key = self::lower_java_script_call_source_expression(source_iterator_key.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                                {
                                    let saved_key = self::allocate_java_script_call_source_register(match iterator_key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10183, _) => {
                                            let source_ml_source2_data_property_value10183 = *source_ml_source2_data_property_value10183;
                                            source_ml_source2_data_property_value10183.clone()
                                        }
                                    });
                                    {
                                        let keyed = self::append_java_script_call_source_instruction(match saved_key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10184, _) => {
                                                let source_ml_source2_data_property_value10184 = *source_ml_source2_data_property_value10184;
                                                source_ml_source2_data_property_value10184.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match saved_key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10185) => {
                                                source_ml_source2_data_property_value10185
                                            }
                                        }, match iterator_key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10186) => {
                                                source_ml_source2_data_property_value10186
                                            }
                                        }));
                                        {
                                            let index = self::allocate_java_script_call_source_register(keyed.clone());
                                            {
                                                let current_2 = self::append_java_script_call_source_instruction(match index.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10187, _) => {
                                                        let source_ml_source2_data_property_value10187 = *source_ml_source2_data_property_value10187;
                                                        source_ml_source2_data_property_value10187.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Constant(match index.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10188) => {
                                                        source_ml_source2_data_property_value10188
                                                    }
                                                }, Box::new(self::JavaScriptSemanticValue::Number(0f64))));
                                                {
                                                    let source_ml_source1_temporary_index31 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s10189 = self::source_ml_source1_temporary_lower_java_script_call_source_array_expression_loop33(bindings.clone(), depth, realm.clone(), object.clone(), current_2.clone(), saved_key.clone(), index.clone(), parts.clone(), source_ml_source1_temporary_index31.clone());
                                                        match source_ml_source2_s10189.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Done(current_3) => {
                                                                let current_3 = *current_3;
                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(current_3.clone()), match object.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10190) => {
                                                                        source_ml_source2_data_property_value10190
                                                                    }
                                                                })
                                                            }
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Return(source_ml_source1_temporary_result33) => {
                                                                let source_ml_source1_temporary_result33 = *source_ml_source1_temporary_result33;
                                                                source_ml_source1_temporary_result33.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceArray(source_prototype_2, length, elements, realm_2) => {
            let source_prototype_2 = *source_prototype_2;
            if ((((((!(self::ml_math::is_integer(length)) || (length < (0f64))) || (length > (4294967295f64))) || {
                let source_ml_source2_same_value_operand10192 = (-0f64);
                (((length == source_ml_source2_same_value_operand10192) && ((length != (0f64)) || (((1f64 / length)) == ((1f64 / source_ml_source2_same_value_operand10192))))) || ((length != length) && (source_ml_source2_same_value_operand10192 != source_ml_source2_same_value_operand10192)))
            }) || (({ let ml_reference_array81 = elements.clone(); let ml_reference_projection81 = ml_reference_array81.borrow().len() as f64; ml_reference_projection81 }) > (4096f64))) || (realm_2 <= (self::ml::Big::from_i128(0)))) || (realm_2 > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112, 97, 114, 115, 101, 32, 108, 101, 110, 103, 116, 104, 44, 32, 98, 111, 117, 110, 100, 101, 100, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])))
            } else {
                {
                    let prototype_2 = self::lower_java_script_call_source_expression(source_prototype_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let object_2 = self::allocate_java_script_call_source_register(match prototype_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10193, _) => {
                                let source_ml_source2_data_property_value10193 = *source_ml_source2_data_property_value10193;
                                source_ml_source2_data_property_value10193.clone()
                            }
                        });
                        {
                            let current_4 = self::append_java_script_call_source_instruction(match object_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10194, _) => {
                                    let source_ml_source2_data_property_value10194 = *source_ml_source2_data_property_value10194;
                                    source_ml_source2_data_property_value10194.clone()
                                }
                            }, self::JavaScriptCallInstruction::ArrayAllocate(match object_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10195) => {
                                    source_ml_source2_data_property_value10195
                                }
                            }, match prototype_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10196) => {
                                    source_ml_source2_data_property_value10196
                                }
                            }, length));
                            {
                                let previous = (-1f64);
                                {
                                    let source_ml_source1_temporary_index32 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s10197 = self::source_ml_source1_temporary_lower_java_script_call_source_array_expression_loop34(bindings.clone(), depth, realm_2.clone(), length, object_2.clone(), current_4.clone(), previous, elements.clone(), source_ml_source1_temporary_index32.clone());
                                        match source_ml_source2_s10197.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Done(current_5) => {
                                                let current_5 = *current_5;
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(current_5.clone()), match object_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10198) => {
                                                        source_ml_source2_data_property_value10198
                                                    }
                                                })
                                            }
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Return(source_ml_source1_temporary_result34) => {
                                                let source_ml_source1_temporary_result34 = *source_ml_source1_temporary_result34;
                                                source_ml_source1_temporary_result34.clone()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 97, 114, 114, 97, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_object_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceObject(source_prototype, properties, realm) => {
            let source_prototype = *source_prototype;
            if ((realm <= (self::ml::Big::from_i128(0))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 111, 98, 106, 101, 99, 116, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 114, 101, 97, 108, 109, 32, 105, 100, 101, 110, 116, 105, 116, 121])))
            } else {
                if (({ let ml_reference_array82 = properties.clone(); let ml_reference_projection82 = ml_reference_array82.borrow().len() as f64; ml_reference_projection82 }) > (4096f64)) {
                    self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 111, 98, 106, 101, 99, 116, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 114, 117, 99, 116, 117, 114, 97, 108, 32, 98, 117, 100, 103, 101, 116])))
                } else {
                    {
                        let prototype = self::lower_java_script_call_source_expression(source_prototype.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                        {
                            let object = self::allocate_java_script_call_source_register(match prototype.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10380, _) => {
                                    let source_ml_source2_data_property_value10380 = *source_ml_source2_data_property_value10380;
                                    source_ml_source2_data_property_value10380.clone()
                                }
                            });
                            {
                                let current = self::append_java_script_call_source_instruction(match object.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10381, _) => {
                                        let source_ml_source2_data_property_value10381 = *source_ml_source2_data_property_value10381;
                                        source_ml_source2_data_property_value10381.clone()
                                    }
                                }, self::JavaScriptCallInstruction::OrdinaryAllocate(match object.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10382) => {
                                        source_ml_source2_data_property_value10382
                                    }
                                }, match prototype.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10383) => {
                                        source_ml_source2_data_property_value10383
                                    }
                                }));
                                {
                                    let source_ml_source1_temporary_index35 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s10384 = self::source_ml_source1_temporary_lower_java_script_call_source_object_expression_loop41(bindings.clone(), depth, realm.clone(), object.clone(), current.clone(), properties.clone(), source_ml_source1_temporary_index35.clone());
                                        match source_ml_source2_s10384.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Done(current_2) => {
                                                let current_2 = *current_2;
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(current_2.clone()), match object.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10385) => {
                                                        source_ml_source2_data_property_value10385
                                                    }
                                                })
                                            }
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Return(source_ml_source1_temporary_result41) => {
                                                let source_ml_source1_temporary_result41 = *source_ml_source1_temporary_result41;
                                                source_ml_source1_temporary_result41.clone()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 111, 98, 106, 101, 99, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}
