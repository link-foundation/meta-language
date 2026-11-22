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
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9113, _) => {
                            let source_ml_source2_data_property_value9113 = *source_ml_source2_data_property_value9113;
                            source_ml_source2_data_property_value9113.clone()
                        }
                    });
                    {
                        let saved = self::append_java_script_call_source_instruction(match held.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9114, _) => {
                                let source_ml_source2_data_property_value9114 = *source_ml_source2_data_property_value9114;
                                source_ml_source2_data_property_value9114.clone()
                            }
                        }, self::JavaScriptCallInstruction::Copy(match held.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9115) => {
                                source_ml_source2_data_property_value9115
                            }
                        }, match input.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9116) => {
                                source_ml_source2_data_property_value9116
                            }
                        }));
                        {
                            let key = self::lower_java_script_call_source_expression(source_key.clone(), bindings.clone(), saved.clone(), (depth + 1f64));
                            {
                                let held_key = self::allocate_java_script_call_source_register(match key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9117, _) => {
                                        let source_ml_source2_data_property_value9117 = *source_ml_source2_data_property_value9117;
                                        source_ml_source2_data_property_value9117.clone()
                                    }
                                });
                                {
                                    let saved_key = self::append_java_script_call_source_instruction(match held_key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9118, _) => {
                                            let source_ml_source2_data_property_value9118 = *source_ml_source2_data_property_value9118;
                                            source_ml_source2_data_property_value9118.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Copy(match held_key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9119) => {
                                            source_ml_source2_data_property_value9119
                                        }
                                    }, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9120) => {
                                            source_ml_source2_data_property_value9120
                                        }
                                    }));
                                    {
                                        let prototype = self::lower_java_script_call_source_expression(source_prototype.clone(), bindings.clone(), saved_key.clone(), (depth + 1f64));
                                        {
                                            let condition = self::allocate_java_script_call_source_register(match prototype.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9121, _) => {
                                                    let source_ml_source2_data_property_value9121 = *source_ml_source2_data_property_value9121;
                                                    source_ml_source2_data_property_value9121.clone()
                                                }
                                            });
                                            {
                                                let result = self::allocate_java_script_call_source_register(match condition.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9122, _) => {
                                                        let source_ml_source2_data_property_value9122 = *source_ml_source2_data_property_value9122;
                                                        source_ml_source2_data_property_value9122.clone()
                                                    }
                                                });
                                                {
                                                    let tested = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9123, _) => {
                                                            let source_ml_source2_data_property_value9123 = *source_ml_source2_data_property_value9123;
                                                            source_ml_source2_data_property_value9123.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Primitive(match condition.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9124) => {
                                                            source_ml_source2_data_property_value9124
                                                        }
                                                    }, self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 32, 107, 101, 121]), std::rc::Rc::new(std::cell::RefCell::new(vec![match held.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9125) => {
                                                            source_ml_source2_data_property_value9125
                                                        }
                                                    }, match held_key.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9126) => {
                                                            source_ml_source2_data_property_value9126
                                                        }
                                                    }]))));
                                                    {
                                                        let branch = { let ml_reference_array74 = match tested.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9127, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value9127.clone()
                                                            }
                                                        }; let ml_reference_projection74 = ml_reference_array74.borrow().len() as f64; ml_reference_projection74 };
                                                        {
                                                            let selected = self::append_java_script_call_source_instruction(tested.clone(), self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9128) => {
                                                                    source_ml_source2_data_property_value9128
                                                                }
                                                            }, (branch + 1f64), 0f64));
                                                            {
                                                                let own = self::append_java_script_call_source_instruction(selected.clone(), self::JavaScriptCallInstruction::Primitive(match result.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9129) => {
                                                                        source_ml_source2_data_property_value9129
                                                                    }
                                                                }, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 32, 107, 101, 121]), std::rc::Rc::new(std::cell::RefCell::new(vec![match held.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9130) => {
                                                                        source_ml_source2_data_property_value9130
                                                                    }
                                                                }, match held_key.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9131) => {
                                                                        source_ml_source2_data_property_value9131
                                                                    }
                                                                }]))));
                                                                {
                                                                    let jump = { let ml_reference_array75 = match own.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9132, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value9132.clone()
                                                                        }
                                                                    }; let ml_reference_projection75 = ml_reference_array75.borrow().len() as f64; ml_reference_projection75 };
                                                                    {
                                                                        let skipped = self::append_java_script_call_source_instruction(own.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                                        {
                                                                            let get = { let ml_reference_array76 = match skipped.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9133, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value9133.clone()
                                                                                }
                                                                            }; let ml_reference_projection76 = ml_reference_array76.borrow().len() as f64; ml_reference_projection76 };
                                                                            {
                                                                                let got = self::append_java_script_call_source_instruction(skipped.clone(), self::JavaScriptCallInstruction::OrdinaryGet(match result.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9134) => {
                                                                                        source_ml_source2_data_property_value9134
                                                                                    }
                                                                                }, match prototype.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9135) => {
                                                                                        source_ml_source2_data_property_value9135
                                                                                    }
                                                                                }, match held_key.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9136) => {
                                                                                        source_ml_source2_data_property_value9136
                                                                                    }
                                                                                }, match held.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9137) => {
                                                                                        source_ml_source2_data_property_value9137
                                                                                    }
                                                                                }));
                                                                                {
                                                                                    let end = { let ml_reference_array77 = match got.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9138, _, _, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value9138.clone()
                                                                                        }
                                                                                    }; let ml_reference_projection77 = ml_reference_array77.borrow().len() as f64; ml_reference_projection77 };
                                                                                    {
                                                                                        let source_ml_source2_constructor_field9140 = match result.clone() {
                                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9139) => {
                                                                                                source_ml_source2_data_property_value9139
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_constructor_field9165 = {
                                                                                                let source_ml_source2_nominal_spread9143 = got.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9144 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9145, _, _, _, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value9145.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9146 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value9147, _, _, _, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value9147
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9148 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value9149, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value9149
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9150 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value9151, _, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value9151
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9152 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value9153, _, _, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value9153
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9154 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value9155, _, _, _, _) => {
                                                                                                                                source_ml_source2_data_property_value9155.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field9156 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value9157, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value9157.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field9158 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value9159, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value9159.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field9160 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value9161, _) => {
                                                                                                                                            source_ml_source2_data_property_value9161.clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field9162 = match source_ml_source2_nominal_spread9143.clone() {
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9163) => {
                                                                                                                                                source_ml_source2_data_property_value9163.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_override9164 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values78 = { let ml_reference_value79 = (match got.clone() {
                                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9141, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                    source_ml_source2_data_property_value9141.clone()
                                                                                                                                                }
                                                                                                                                            }).borrow().clone(); ml_reference_value79 }; let ml_callback78 = { let branch = branch; let condition = condition.clone(); let get = get; let jump = jump; let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, at: f64| -> self::JavaScriptCallInstruction { if (at == branch) {
                                                                                                                                                self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9142) => {
                                                                                                                                                        source_ml_source2_data_property_value9142
                                                                                                                                                    }
                                                                                                                                                }, (branch + 1f64), get)
                                                                                                                                            } else {
                                                                                                                                                if (at == jump) {
                                                                                                                                                    self::JavaScriptCallInstruction::Jump(end)
                                                                                                                                                } else {
                                                                                                                                                    instruction.clone()
                                                                                                                                                }
                                                                                                                                            } }) }; ml_values78.iter().cloned().enumerate().map(|(ml_index78, ml_value78)| ml_callback78(ml_value78, (ml_index78 as f64))).collect::<Vec<_>>() }));
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override9164.clone(), source_ml_source2_nominal_field9146, source_ml_source2_nominal_field9148, source_ml_source2_nominal_field9150, source_ml_source2_nominal_field9152, source_ml_source2_nominal_field9154.clone(), source_ml_source2_nominal_field9156.clone(), source_ml_source2_nominal_field9158.clone(), source_ml_source2_nominal_field9160.clone(), source_ml_source2_nominal_field9162.clone())
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
                                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field9165.clone()), source_ml_source2_constructor_field9140)
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
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9166, _) => {
                            let source_ml_source2_data_property_value9166 = *source_ml_source2_data_property_value9166;
                            source_ml_source2_data_property_value9166.clone()
                        }
                    }, (depth + 1f64));
                    {
                        let supplied_receiver = ((source_receiver.clone().unwrap_or_else(|| (None::<self::JavaScriptCallSource>,))).0).clone();
                        {
                            let receiver = if match (supplied_receiver.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                {
                                    let source_ml_source2_constructor_field9168 = match object.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9167) => {
                                            source_ml_source2_data_property_value9167
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field9170 = match key_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9169, _) => {
                                                let source_ml_source2_data_property_value9169 = *source_ml_source2_data_property_value9169;
                                                source_ml_source2_data_property_value9169.clone()
                                            }
                                        };
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field9170.clone()), source_ml_source2_constructor_field9168)
                                    }
                                }
                            } else {
                                self::lower_java_script_call_source_expression(supplied_receiver.clone().expect("a proven present value"), bindings.clone(), match key_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9171, _) => {
                                        let source_ml_source2_data_property_value9171 = *source_ml_source2_data_property_value9171;
                                        source_ml_source2_data_property_value9171.clone()
                                    }
                                }, (depth + 1f64))
                            };
                            {
                                let result_2 = self::allocate_java_script_call_source_register(match receiver.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9172, _) => {
                                        let source_ml_source2_data_property_value9172 = *source_ml_source2_data_property_value9172;
                                        source_ml_source2_data_property_value9172.clone()
                                    }
                                });
                                {
                                    let source_ml_source2_nominal_field9179 = match result_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9180, _) => {
                                            let source_ml_source2_data_property_value9180 = *source_ml_source2_data_property_value9180;
                                            source_ml_source2_data_property_value9180.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field9181 = match result_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9182) => {
                                                source_ml_source2_data_property_value9182
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override9183 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9173, _) => {
                                                    let source_ml_source2_data_property_value9173 = *source_ml_source2_data_property_value9173;
                                                    source_ml_source2_data_property_value9173.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinaryGet(match result_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9174) => {
                                                    source_ml_source2_data_property_value9174
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9175) => {
                                                    source_ml_source2_data_property_value9175
                                                }
                                            }, match key_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9176) => {
                                                    source_ml_source2_data_property_value9176
                                                }
                                            }, match receiver.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9177) => {
                                                    source_ml_source2_data_property_value9177
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override9183.clone()), source_ml_source2_nominal_field9181)
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
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9184, _) => {
                            let source_ml_source2_data_property_value9184 = *source_ml_source2_data_property_value9184;
                            source_ml_source2_data_property_value9184.clone()
                        }
                    });
                    {
                        let saved_2 = self::append_java_script_call_source_instruction(match object_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9185, _) => {
                                let source_ml_source2_data_property_value9185 = *source_ml_source2_data_property_value9185;
                                source_ml_source2_data_property_value9185.clone()
                            }
                        }, self::JavaScriptCallInstruction::Copy(match object_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9186) => {
                                source_ml_source2_data_property_value9186
                            }
                        }, match raw_.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9187) => {
                                source_ml_source2_data_property_value9187
                            }
                        }));
                        {
                            let key_3 = self::lower_java_script_call_source_expression(source_key_3.clone(), bindings.clone(), saved_2.clone(), (depth + 1f64));
                            {
                                let result_3 = self::allocate_java_script_call_source_register(match key_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9188, _) => {
                                        let source_ml_source2_data_property_value9188 = *source_ml_source2_data_property_value9188;
                                        source_ml_source2_data_property_value9188.clone()
                                    }
                                });
                                {
                                    let current = self::append_java_script_call_source_instruction(match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9189, _) => {
                                            let source_ml_source2_data_property_value9189 = *source_ml_source2_data_property_value9189;
                                            source_ml_source2_data_property_value9189.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::OrdinaryDelete(match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9374) => {
                                            source_ml_source2_data_property_value9374
                                        }
                                    }, match object_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9375) => {
                                            source_ml_source2_data_property_value9375
                                        }
                                    }, match key_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9376) => {
                                            source_ml_source2_data_property_value9376
                                        }
                                    }));
                                    {
                                        let current_3 = {
                                            let checked = self::allocate_java_script_call_source_register(current.clone());
                                            {
                                                let current_2 = self::append_java_script_call_source_instruction(match checked.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9564, _) => {
                                                        let source_ml_source2_data_property_value9564 = *source_ml_source2_data_property_value9564;
                                                        source_ml_source2_data_property_value9564.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Primitive(match checked.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9565) => {
                                                        source_ml_source2_data_property_value9565
                                                    }
                                                }, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 100, 101, 108, 101, 116, 101, 32, 115, 117, 99, 99, 101, 115, 115]), std::rc::Rc::new(std::cell::RefCell::new(vec![match result_3.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9566) => {
                                                        source_ml_source2_data_property_value9566
                                                    }
                                                }]))));
                                                current_2.clone()
                                            }
                                        };
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(current_3.clone()), match result_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9567) => {
                                                source_ml_source2_data_property_value9567
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
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9568, _) => {
                            let source_ml_source2_data_property_value9568 = *source_ml_source2_data_property_value9568;
                            source_ml_source2_data_property_value9568.clone()
                        }
                    });
                    {
                        let saved_3 = self::append_java_script_call_source_instruction(match object_3.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9569, _) => {
                                let source_ml_source2_data_property_value9569 = *source_ml_source2_data_property_value9569;
                                source_ml_source2_data_property_value9569.clone()
                            }
                        }, self::JavaScriptCallInstruction::Copy(match object_3.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9570) => {
                                source_ml_source2_data_property_value9570
                            }
                        }, match raw_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9571) => {
                                source_ml_source2_data_property_value9571
                            }
                        }));
                        {
                            let key_4 = self::lower_java_script_call_source_expression(source_key_4.clone(), bindings.clone(), saved_3.clone(), (depth + 1f64));
                            {
                                let result_4 = self::allocate_java_script_call_source_register(match key_4.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9572, _) => {
                                        let source_ml_source2_data_property_value9572 = *source_ml_source2_data_property_value9572;
                                        source_ml_source2_data_property_value9572.clone()
                                    }
                                });
                                {
                                    let current_4 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9573, _) => {
                                            let source_ml_source2_data_property_value9573 = *source_ml_source2_data_property_value9573;
                                            source_ml_source2_data_property_value9573.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::OrdinaryHas(match result_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9761) => {
                                            source_ml_source2_data_property_value9761
                                        }
                                    }, match object_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9762) => {
                                            source_ml_source2_data_property_value9762
                                        }
                                    }, match key_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9763) => {
                                            source_ml_source2_data_property_value9763
                                        }
                                    }));
                                    {
                                        let current_5 = current_4.clone();
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(current_5.clone()), match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9948) => {
                                                source_ml_source2_data_property_value9948
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
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10125, _) => {
                                let source_ml_source2_data_property_value10125 = *source_ml_source2_data_property_value10125;
                                source_ml_source2_data_property_value10125.clone()
                            }
                        });
                        {
                            let current = self::append_java_script_call_source_instruction(match object.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10126, _) => {
                                    let source_ml_source2_data_property_value10126 = *source_ml_source2_data_property_value10126;
                                    source_ml_source2_data_property_value10126.clone()
                                }
                            }, self::JavaScriptCallInstruction::ArrayAllocate(match object.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10127) => {
                                    source_ml_source2_data_property_value10127
                                }
                            }, match prototype.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10128) => {
                                    source_ml_source2_data_property_value10128
                                }
                            }, 0f64));
                            {
                                let iterator_key = self::lower_java_script_call_source_expression(source_iterator_key.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                                {
                                    let saved_key = self::allocate_java_script_call_source_register(match iterator_key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10129, _) => {
                                            let source_ml_source2_data_property_value10129 = *source_ml_source2_data_property_value10129;
                                            source_ml_source2_data_property_value10129.clone()
                                        }
                                    });
                                    {
                                        let keyed = self::append_java_script_call_source_instruction(match saved_key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10130, _) => {
                                                let source_ml_source2_data_property_value10130 = *source_ml_source2_data_property_value10130;
                                                source_ml_source2_data_property_value10130.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match saved_key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10131) => {
                                                source_ml_source2_data_property_value10131
                                            }
                                        }, match iterator_key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10132) => {
                                                source_ml_source2_data_property_value10132
                                            }
                                        }));
                                        {
                                            let index = self::allocate_java_script_call_source_register(keyed.clone());
                                            {
                                                let current_2 = self::append_java_script_call_source_instruction(match index.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10133, _) => {
                                                        let source_ml_source2_data_property_value10133 = *source_ml_source2_data_property_value10133;
                                                        source_ml_source2_data_property_value10133.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Constant(match index.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10134) => {
                                                        source_ml_source2_data_property_value10134
                                                    }
                                                }, Box::new(self::JavaScriptSemanticValue::Number(0f64))));
                                                {
                                                    let source_ml_source1_temporary_index31 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s10135 = self::source_ml_source1_temporary_lower_java_script_call_source_array_expression_loop33(bindings.clone(), depth, realm.clone(), object.clone(), current_2.clone(), saved_key.clone(), index.clone(), parts.clone(), source_ml_source1_temporary_index31.clone());
                                                        match source_ml_source2_s10135.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Done(current_3) => {
                                                                let current_3 = *current_3;
                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(current_3.clone()), match object.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10136) => {
                                                                        source_ml_source2_data_property_value10136
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
                let source_ml_source2_same_value_operand10138 = (-0f64);
                (((length == source_ml_source2_same_value_operand10138) && ((length != (0f64)) || (((1f64 / length)) == ((1f64 / source_ml_source2_same_value_operand10138))))) || ((length != length) && (source_ml_source2_same_value_operand10138 != source_ml_source2_same_value_operand10138)))
            }) || (({ let ml_reference_array81 = elements.clone(); let ml_reference_projection81 = ml_reference_array81.borrow().len() as f64; ml_reference_projection81 }) > (4096f64))) || (realm_2 <= (self::ml::Big::from_i128(0)))) || (realm_2 > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112, 97, 114, 115, 101, 32, 108, 101, 110, 103, 116, 104, 44, 32, 98, 111, 117, 110, 100, 101, 100, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])))
            } else {
                {
                    let prototype_2 = self::lower_java_script_call_source_expression(source_prototype_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let object_2 = self::allocate_java_script_call_source_register(match prototype_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10139, _) => {
                                let source_ml_source2_data_property_value10139 = *source_ml_source2_data_property_value10139;
                                source_ml_source2_data_property_value10139.clone()
                            }
                        });
                        {
                            let current_4 = self::append_java_script_call_source_instruction(match object_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10140, _) => {
                                    let source_ml_source2_data_property_value10140 = *source_ml_source2_data_property_value10140;
                                    source_ml_source2_data_property_value10140.clone()
                                }
                            }, self::JavaScriptCallInstruction::ArrayAllocate(match object_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10141) => {
                                    source_ml_source2_data_property_value10141
                                }
                            }, match prototype_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10142) => {
                                    source_ml_source2_data_property_value10142
                                }
                            }, length));
                            {
                                let previous = (-1f64);
                                {
                                    let source_ml_source1_temporary_index32 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s10143 = self::source_ml_source1_temporary_lower_java_script_call_source_array_expression_loop34(bindings.clone(), depth, realm_2.clone(), length, object_2.clone(), current_4.clone(), previous, elements.clone(), source_ml_source1_temporary_index32.clone());
                                        match source_ml_source2_s10143.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Done(current_5) => {
                                                let current_5 = *current_5;
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(current_5.clone()), match object_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10144) => {
                                                        source_ml_source2_data_property_value10144
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
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10326, _) => {
                                    let source_ml_source2_data_property_value10326 = *source_ml_source2_data_property_value10326;
                                    source_ml_source2_data_property_value10326.clone()
                                }
                            });
                            {
                                let current = self::append_java_script_call_source_instruction(match object.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10327, _) => {
                                        let source_ml_source2_data_property_value10327 = *source_ml_source2_data_property_value10327;
                                        source_ml_source2_data_property_value10327.clone()
                                    }
                                }, self::JavaScriptCallInstruction::OrdinaryAllocate(match object.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10328) => {
                                        source_ml_source2_data_property_value10328
                                    }
                                }, match prototype.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10329) => {
                                        source_ml_source2_data_property_value10329
                                    }
                                }));
                                {
                                    let source_ml_source1_temporary_index35 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s10330 = self::source_ml_source1_temporary_lower_java_script_call_source_object_expression_loop41(bindings.clone(), depth, realm.clone(), object.clone(), current.clone(), properties.clone(), source_ml_source1_temporary_index35.clone());
                                        match source_ml_source2_s10330.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Done(current_2) => {
                                                let current_2 = *current_2;
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(current_2.clone()), match object.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10331) => {
                                                        source_ml_source2_data_property_value10331
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
